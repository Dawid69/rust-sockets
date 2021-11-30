// sockets module that is used to communicate with standalone processes using UNIX sockets
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::mpsc::Sender;

use std::fs;
use std::thread;

// Opens an IPC channel to receiuve data on the socket... Needs a call to channel.try_recv
pub fn open_socket_rx(socket: &Path, tx_channel: Sender<String>) -> Result<(), String> {
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

// Passes the incoming data into a string that can be sent out of the rx thread
fn handle_client(stream: UnixStream, tx: Sender<String>) {
    let stream = BufReader::new(stream);

    for line in stream.lines() {
        let x = line.unwrap();
        // unwrap line into string
        tx.send(x).unwrap();
        //send line
    }
}

pub fn socket_tx(socket: &Path, payload: &str) -> Result<(), String> {
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
