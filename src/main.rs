use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let lines: Vec<_> = reader.lines().map(|l| l.unwrap()).collect();

    println!("Request: {}", lines.join(""));

    match stream.write_all(lines.join("").as_bytes()) {
        Ok(_) => println!("OK"),
        Err(e) => println!("Error: {}", e),
    }
}
