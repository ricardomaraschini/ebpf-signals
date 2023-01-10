use log::{error, info};
use signals_common::Signal;
use std::collections::HashMap;
use std::fs::remove_file;
use std::io::ErrorKind;
use std::process;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::broadcast;

// SOCKET_PATH is the path to the unix socket that will be used to communicate
// with the daemon.
const SOCKET_PATH: &str = "/var/run/signals";

// create_socket creates the unix socket file. if it already exists drops it
// first and then create a new one.
fn create_socket() -> UnixListener {
    let _ = remove_file(SOCKET_PATH);
    let listener = match UnixListener::bind(SOCKET_PATH) {
        Ok(listener) => listener,
        Err(err) => {
            error!("error binding to socket {}: {}", SOCKET_PATH, err);
            process::exit(1);
        }
    };
    listener
}

// start starts the unix socket listener and handles incoming connections.
pub async fn start(from: broadcast::Sender<Signal>) {
    let listener = create_socket();
    info!("awaiting for new connections");
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let local_rx = from.subscribe();
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

// handle_stream handles a single connection to the unix socket.
async fn handle_stream(stream: UnixStream, mut from: broadcast::Receiver<Signal>) {
    info!("new connection accepted");
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
