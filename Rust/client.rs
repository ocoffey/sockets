use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
//use std::fs;

// bind to a port
// listen for a request
// on request, open a file, and send its contents
// flush

fn main() {
    loop {
        println!("Please enter Y to continue.");

        let mut input = String::with_capacity(1);
    
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim() {
            "Y" => break,
            _ => continue,
        }
    }

    let intro = b"GET MY MESSAGE";

    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    stream.write(intro).unwrap();

    stream.flush().unwrap();

    println!("Message Sent!");

    let mut buffer = [0; 512];
    
    stream.read(&mut buffer).unwrap();
    
    let response = std::str::from_utf8(&mut buffer).unwrap();
    
    print!("Message Received: ");

    print!("{}", response);

    stream.flush().unwrap();
}