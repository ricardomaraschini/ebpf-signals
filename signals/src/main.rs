pub mod conn_handler;
pub mod emitters;

use env_logger::Env;
use log::{error, info};
use std::process;
use tokio::sync::broadcast;
use users::get_current_uid;

#[tokio::main]
async fn main() {
    let env_with_def_loglevel = Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env_with_def_loglevel).init();

    print_welcome();

    if get_current_uid() != 0 {
        error!("running as non root is not supported");
        process::exit(1);
    }

    info!("loading and attaching to the ebpf program");
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

    // start won't return until the program is terminated.
    conn_handler::start(tx).await;
}

fn print_welcome() {
    info!(r"       / /\        /\ \       /\ \        ");
    info!(r"      / /  \       \ \ \     /  \ \       ");
    info!(r"     / / /\ \__    /\ \_\   / /\ \_\      ");
    info!(r"    / / /\ \___\  / /\/_/  / / /\/_/      ");
    info!(r"    \ \ \ \/___/ / / /    / / / _____     ");
    info!(r"     \ \ \      / / /    / / / /\_____\   ");
    info!(r" _    \ \ \    / / /    / / /  \/____ /   ");
    info!(r"/_/\__/ / /___/ / /__  / / /_____/ / /    ");
    info!(r"\ \/___/ //\__\/_/___\/ / /______\/ /     ");
    info!(r" \_____\/ \/_________/\/___________/      ");
    info!("signal emitter development version ");
}
