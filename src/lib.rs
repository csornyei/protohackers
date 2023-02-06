use std::{
    env,
    io::prelude::*,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
}

fn get_address() -> SocketAddr {
    let port = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)
}

fn create_listener() -> std::net::TcpListener {
    let address = get_address();
    let listener = std::net::TcpListener::bind(address).unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    listener
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

pub fn serve(pool: &ThreadPool) {
    let listener = create_listener();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
