use smoke_test::{serve, ThreadPool};

fn main() {
    let pool = ThreadPool::new(5);
    serve(&pool);
}
