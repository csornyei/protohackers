use std::{
    env,
    io::prelude::*,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
};

use smoke_test::ThreadPool;

fn main() {
    let port = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    let listener = TcpListener::bind(address).unwrap();
    let pool = ThreadPool::new(5);

    println!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Handling connection from {}", stream.peer_addr().unwrap());

    let mut handled_data: usize = 0;

    loop {
        let mut receive_buffer = [0; 1024];
        match stream.read(&mut receive_buffer) {
            Ok(0) => break,
            Ok(size) => {
                handled_data += size;
                if handled_data > 10 * 1024 {
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                    break;
                }
                stream.write(&receive_buffer).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
