use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

mod socket;

const SOCKET_PATH: &str = "/tmp/rust.sock";

const SOCKET_SLEEP_DELAY: u64 = 20;

fn main() {
    // create IPC thread to try receive data from the socket
    let (ipc_tx, ipc_rx): (Sender<String>, Receiver<String>) = channel();

    // create new socket
    let socket = Path::new(SOCKET_PATH);

    let _ = socket::open_socket_rx(socket, ipc_tx);

    let _ = socket::socket_tx(socket, "Hello");

    thread::sleep(Duration::from_millis(SOCKET_SLEEP_DELAY));

    match ipc_rx.try_recv() {
        Ok(x) => println!("{}", &x),
        Err(x) => println!("Error: {}", &x),
    }
}
