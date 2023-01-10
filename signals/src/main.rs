pub mod conn_handler;
pub mod emitter;

use env_logger::Env;
use log::{error, info};
use std::fs::remove_file;
use std::process;
use tokio::net::UnixListener;
use tokio::sync::broadcast;
use users::get_current_uid;

// SOCKET_PATH is the path to the unix socket that will be used to communicate
// with the daemon.
const SOCKET_PATH: &str = "/var/run/signals";

#[tokio::main]
async fn main() {
    let env_with_def_loglevel = Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env_with_def_loglevel).init();

    print_welcome();

    if get_current_uid() != 0 {
        error!("running as non root is not supported");
        process::exit(1);
    }

    // tx is the transmit side of the broadcast, we will send this over to the
    // bpf handler. from the reader part of the broadcast we can receive all
    // signals intercepted. We use a buffered channel in an attempt to avoid
    // blocking.
    let (tx, _) = broadcast::channel(100);

    // we need to keep a reference to the Bpf program in memory for the duration
    // of the program otherwise the Bpf program will stop.
    let _bpf = emitter::load_and_attach(tx.clone()).expect("failed to load bpf");

    // creates a unix socket and starts listening for connections on it. messages
    // are forwarded directly from the tx to the socket.
    let socket = create_socket();
    conn_handler::start(socket, tx).await;
}

// create_socket creates the unix socket file. if it already exists drops it
// first and creates a new one.
fn create_socket() -> UnixListener {
    let _ = remove_file(SOCKET_PATH);
    let listener = match UnixListener::bind(SOCKET_PATH) {
        Ok(listener) => listener,
        Err(err) => {
            error!("error binding to socket {}: {}", SOCKET_PATH, err);
            process::exit(1);
        }
    };
    info!("unix socket created on {}", SOCKET_PATH);
    listener
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
    info!(r"signal emitter development version        ");
}
