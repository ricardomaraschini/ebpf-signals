use log::{error, info};
use signals_common::Signal;
use std::collections::HashMap;
use std::io::ErrorKind;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::broadcast;

// start starts accepts and handles connections in the provided socket. this fn
// never returns. For each new connection accepted a new thread is created.
pub async fn start(socket: UnixListener, from: broadcast::Sender<Signal>) {
    info!("awaiting for new connections");
    loop {
        match socket.accept().await {
            Ok((stream, _addr)) => {
                let local_rx = from.subscribe();
                tokio::spawn(async move {
                    handle_conn(stream, local_rx).await;
                });
            }
            Err(err) => {
                error!("error accepting connection: {}", err);
                continue;
            }
        }
    }
}

// handle_stream handles a single connection to the unix socket. Bails out in case
// of errors or if the connection has been closed.
async fn handle_conn(stream: UnixStream, mut from: broadcast::Receiver<Signal>) {
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
