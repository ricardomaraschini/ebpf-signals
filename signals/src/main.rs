pub mod emitters;

use log::info;
use std::sync::mpsc::sync_channel;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let (tx, rx) = sync_channel(100);

    let mut _bpf = emitters::SignalEmitter::new(tx)?;
    _bpf.attach()?;

    info!("tasks started, awaiting for events");
    for msg in rx {
        info!("{:?}", msg);
    }
    Ok(())
}
