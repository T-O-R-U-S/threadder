# Threadder
A simple multithreading implementation.
Examples:

```rust
use std::{
    thread::sleep,
    time::Duration
};

fn main() {
    // Warning: Threadpools MUST be declared as mutable!
    // Create a 4-thread threadpool
    let mut my_threadpool = ThreadPool::new(4);
    // Due to a limiation of Rust, all clojures sent must be boxed to have a 'size known at compile time'... :'(
    for _ in 0..4 {
        my_threadpool.send(
            Box::new(|| {
                println!("Goodnight, world!");
                sleep(Duration::from_secs(3));
                println!("Goodmorning, world!");
            })
        );
    }
    // MUST be ran at the end of the program. Ensures all threads exit PROPERLY. Without this, all threads will shut down as soon as the 'main' thread shuts down -- which isn't desirable when your thread finishes 3 seconds AFTER main does.
    my_threadpool.stop();
}
```
