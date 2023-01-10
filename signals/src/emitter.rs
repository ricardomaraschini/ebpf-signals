use aya::include_bytes_aligned;
use aya::maps::perf::AsyncPerfEventArray;
use aya::programs::TracePoint;
use aya::util::online_cpus;
use aya::Bpf;
use aya_log::BpfLogger;
use bytes::BytesMut;
use log::{debug, error, info};
use signals_common::Signal;
use std::mem::size_of;
use tokio::sync::broadcast;

// load loads the bpf program into the kernel and attaches the tracepoint.
fn load() -> Result<Bpf, anyhow::Error> {
    info!("loading and attaching to the ebpf program");

    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/signals"
    ))?;

    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/signals"
    ))?;

    BpfLogger::init(&mut bpf).unwrap();

    let program: &mut TracePoint = bpf.program_mut("signals").unwrap().try_into()?;
    program.load()?;
    program.attach("signal", "signal_generate")?;

    Ok(bpf)
}

// load_and_attach loads the bpf program into kernel and then attaches to it, all events received
// from the bpf program are forwarded to the provided Sender. This function spawns a thread for
// each cpu.
pub fn load_and_attach(dst: broadcast::Sender<Signal>) -> Result<Bpf, anyhow::Error> {
    let bpf = load()?;

    let signal_struct_size: usize = size_of::<Signal>();
    let mut perf_array = AsyncPerfEventArray::try_from(bpf.map_mut("SIGNALS")?)?;
    for cpu_id in online_cpus()? {
        debug!("spawning task for cpu {}", cpu_id);
        let mut parray = perf_array.open(cpu_id, None)?;
        let local_dst = dst.clone();
        tokio::spawn(async move {
            debug!("task for cpu awaiting for events {}", cpu_id);
            let mut buffers = (0..100)
                .map(|_| BytesMut::with_capacity(signal_struct_size))
                .collect::<Vec<_>>();

            loop {
                let events = match parray.read_events(&mut buffers).await {
                    Ok(events) => events,
                    Err(error) => {
                        error!("fail to read events from the perf, bailing out: {}", error);
                        return;
                    }
                };

                if events.lost > 0 {
                    error!("queues are getting full, lost {} signals", events.lost);
                }

                for i in 0..events.read {
                    let buf = &mut buffers[i];
                    let ptr = buf.as_ptr() as *const Signal;
                    let signal = unsafe { ptr.read_unaligned() };
                    match local_dst.send(signal) {
                        Ok(_) => continue,
                        Err(err) => {
                            // if no one is listening the error returned. XXX find a
                            // better way of handling this.
                            let errstr = format!("{}", err);
                            if !errstr.contains("channel closed") {
                                error!("failed to send signal internally: {}", err);
                            }
                        }
                    }
                }
            }
        });
    }
    Ok(bpf)
}
