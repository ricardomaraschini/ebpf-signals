pub mod emitters;

use env_logger::Env;
use log::{error, info};
use signals_common::Signal;
use std::collections::HashMap;
use std::fs::remove_file;
use std::io::ErrorKind;
use std::process;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::broadcast;
use users::get_current_uid;

const SOCKET_PATH: &str = "/var/run/signals";

async fn handle_stream(stream: UnixStream, mut from: broadcast::Receiver<Signal>) {
    info!("new connection from accepted");
    loop {
        let sig = match from.recv().await {
            Ok(sig) => sig,
            Err(err) => {
                error!("error reading signal: {}", err);
                return;
            }
        };

        let mut hm = HashMap::new();
        hm.insert("pid", sig.pid as u64);
        hm.insert("signr", sig.signr as u64);
        let mut serial = match serde_json::to_string(&hm) {
            Ok(serial) => serial,
            Err(err) => {
                error!("error serializing signal: {}", err);
                return;
            }
        };

        serial.push('\n');
        if let Err(err) = stream.try_write(serial.as_bytes()) {
            if err.kind() == ErrorKind::BrokenPipe {
                info!("client closed connection");
                return;
            }
            error!("unable to write data to the stream: {}", err);
            return;
        }
    }
}

#[tokio::main]
async fn main() {
    let env_with_def_loglevel = Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env_with_def_loglevel).init();

    print_welcome();

    if get_current_uid() != 0 {
        error!("running as non root is not supported");
        process::exit(1);
    }

    let _ = remove_file(SOCKET_PATH);
    let listener = match UnixListener::bind(SOCKET_PATH) {
        Ok(listener) => listener,
        Err(err) => {
            error!("error binding to socket {}: {}", SOCKET_PATH, err);
            process::exit(1);
        }
    };

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

    info!("awaiting for new connections");
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let local_rx = tx.subscribe();
                tokio::spawn(async move {
                    handle_stream(stream, local_rx).await;
                });
            }
            Err(err) => {
                error!("error accepting connection: {}", err);
                continue;
            }
        }
    }
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
    info!("unix socket listening on {}", SOCKET_PATH);
}
