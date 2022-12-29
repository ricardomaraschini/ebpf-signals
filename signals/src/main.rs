pub mod emitters;

use log::info;
use tokio::sync::broadcast;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let (tx, _) = broadcast::channel(100);
    let mut bpf = emitters::SignalEmitter::new()?;
    bpf.attach(tx.clone())?;

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
        handle.await?;
    }
    Ok(())
}
