use std::io;
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::time::Duration;
//use std::fs;

// bind a port to the program
// invoke a request (on text command?)
// listen for a stream
// when you get one, print its contents
// and flush the stream

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        break;
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET MY MESSAGE";

    // if not our request message
    if !buffer.starts_with(get) {
        panic!("Incorrent Header");
    }

    println!("Message Received!");

    println!("Please enter a message to send:");
    let mut response = String::new();

    io::stdin().read_line(&mut response)
            .expect("Failed to read line");

    // sleep for 2 seconds
    thread::sleep(Duration::from_secs(1));

    stream.write(response.trim().as_bytes()).unwrap();

    stream.flush().unwrap();
}