# Threadder
A simple multithreading implementation.
Examples:

```rust
use std::{
    thread::sleep,
    time::Duration
}

fn main() {
    // Warning: Threadpools MUST be declared as mutable!
    // If the threadpool is not mutable, it won't be able to switch threads.
    // Create a 4-thread threadpool
    let mut my_threadpool = ThreadPool::new(4);
    for _ in 0..4 {
        // Clojures can (optionally) be boxed.
        my_threadpool.send(
            || {
                println!("Goodnight, world!");
                sleep(Duration::from_secs(3));
                println!("Goodmorning, world!");
            }
        );
    }
    // MUST be ran at the end of the program. Ensures all threads exit PROPERLY. Without this, 
    // all threads will shut down as soon as the 'main' thread shuts down -- which isn't desirable 
    // when your thread finishes 3 seconds AFTER main does.
    my_threadpool.stop();
    // This drops the transmitter which breaks the listener 
    // loop and joins the thread, allowing it to exit properly without landing the program in an endless loop
}
```