use aya::include_bytes_aligned;
use aya::maps::perf::AsyncPerfEventArray;
use aya::programs::TracePoint;
use aya::util::online_cpus;
use aya::Bpf;
use aya_log::BpfLogger;
use bytes::BytesMut;
use log::debug;
use log::error;
use signals_common::Signal;
use std::mem::size_of;
use tokio::sync::broadcast;

pub struct SignalEmitter {
    bpf: Bpf,
}

impl SignalEmitter {
    // new returns a new Signal that reads a bpf program from disk and loads it
    // into the kernel.
    pub fn new() -> Result<Self, anyhow::Error> {
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

        Ok(Self { bpf })
    }

    // attach starts to read signal events from the kernel and pipe them through the
    // provided destination. spawns a task per cpu, each task process events from its
    // own perf array.
    pub fn attach(&mut self, dst: broadcast::Sender<Signal>) -> Result<(), anyhow::Error> {
        let signal_struct_size: usize = size_of::<Signal>();
        let mut perf_array = AsyncPerfEventArray::try_from(self.bpf.map_mut("SIGNALS")?)?;
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

        Ok(())
    }
}
