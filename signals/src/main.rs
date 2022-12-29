pub mod emitters;

use log::error;
use log::info;
use std::process;
use tokio::sync::broadcast;
use tokio::task;
use users::get_current_uid;

#[tokio::main]
async fn main() {
    env_logger::init();

    if get_current_uid() != 0 {
        error!("running as non root is not supported");
        process::exit(1);
    }

    let (tx, _) = broadcast::channel(100);
    let mut bpf = match emitters::SignalEmitter::new() {
        Ok(bpf) => bpf,
        Err(err) => {
            error!("unable to load ebpf program: {}", err);
            process::exit(1);
        }
    };

    if let Err(err) = bpf.attach(tx.clone()) {
        error!("unable to attach to ebpf program: {}", err);
        process::exit(1);
    }

    let mut join_handles: Vec<task::JoinHandle<()>> = vec![];
    for i in 0..2 {
        let mut local_rx = tx.subscribe();
        join_handles.push(tokio::spawn(async move {
            loop {
                let msg = match local_rx.recv().await {
                    Ok(msg) => msg,
                    Err(_) => break,
                };

                info!("[{}] {:?}", i, msg);
            }
        }));
    }

    drop(tx);

    for handle in join_handles {
        if let Err(err) = handle.await {
            error!("error waiting for thread: {}", err);
            process::exit(1);
        }
    }
}
