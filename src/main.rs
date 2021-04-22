mod lib;
use lib::ThreadPool;

fn main() {
    let mut my_threadpool = ThreadPool::new(3);
    my_threadpool.send(|| {
        println!("Hello world!")
    });
    my_threadpool.stop();
}