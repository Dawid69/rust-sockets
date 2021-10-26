use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

const SOCKET_PATH: &str = "/tmp/rust.sock";

fn main() {
    // create IPC thread to try receive data from the socket
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    // create new socket
    let socket = Path::new(SOCKET_PATH);

    socket_rx(socket, tx.clone());

    socket_tx(socket, "Hello");

    thread::spawn(move || {
        for _ in 0..5 {
            socket_tx(&socket, "Hello");

            thread::sleep(Duration::from_secs(1));
        }
    });

    loop {
        match rx.try_recv() {
            Ok(x) => println!("{}", x),
            Err(_) => (),
        }
    }
}

// Do Result error handling
fn socket_rx(socket: &Path, tx_channel: Sender<String>) -> Result<(), String> {
    if socket.exists() {
        fs::remove_file(&socket).unwrap();
    }

    let listener = match UnixListener::bind(socket) {
        Ok(sock) => sock,
        Err(e) => {
            // println!("Couldn't connect to socket: {:?}", e);
            let x = format!("Couldn't connect to socket: {:?}", e);
            return Err(x);
        }
    };

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => handle_client(stream, tx_channel.clone()),
                Err(e) => {
                    println!("Error has occurred: {}", e);
                    break;
                }
            }
        }
    });

    Ok(())
}

fn handle_client(stream: UnixStream, tx: Sender<String>) {
    let stream = BufReader::new(stream);

    for line in stream.lines() {
        let x = line.unwrap();
        // unwrap line into string
        tx.send(x).unwrap();
        //send line
    }
}

// Do Result error handling
fn socket_tx(socket: &Path, payload: &str) -> Result<(), String> {
    let mut socket = match UnixStream::connect(socket) {
        Ok(sock) => sock,
        Err(e) => {
            return Err(format!("Couldn't connect to socket: {:?}", e));
        }
    };

    let x = payload.as_bytes();

    socket.write_all(x).unwrap();

    Ok(())
}
