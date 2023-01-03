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

    // let _ = socket::open_socket_rx(socket, ipc_tx);

    let x = socket::socket_tx(socket, "Hello");
    println!("{:?}", x);

    if let Err(e) = socket::socket_tx(socket, "Hello") {
        println!("Socket tx err: {}",e)
    }

    println!("Sent");

    thread::sleep(Duration::from_millis(SOCKET_SLEEP_DELAY));

       
//    if let Ok(socket_payload) = ipc_rx.try_recv() {
//        println!("{}", socket_payload);
//        }


    
}
