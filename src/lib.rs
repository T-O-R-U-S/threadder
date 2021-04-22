/*!
 The Threadder crate is a simple threadpool crate. It can be used like so:
 ```rust
  use threadder::ThreadPool;
  use std::thread::sleep;
  use std::time::Duration;
 
  fn main() {
    // MUST be mutable so that instructions may be sent to thread!
    let mut my_threadpool = ThreadPool::new(2);
    // ThreadPool::new()'s parameter is equal to number of cores. Must be a usize larger than 0.
    for _ in 0..4 {
      my_threadpool.send(Box::new(|| {
          // Simulate heavy load
          sleep(Duration::from_secs(4));
          println!("hello world!")
        })
      );
    }
  }
  ```
 */

use std::{
    panic::panic_any,
    sync::mpsc::{channel, Sender, TryRecvError},
    thread::{spawn, JoinHandle},
};

/**
 * The TaskCarrier struct is a thread that can have tasks sent to it.
 * ```rust
 * use threadder::TaskCarrier;
 * 
 * fn main() {
 *  let my_job = TaskCarrier::new();
 *  my_job.send(
 *      Box::new(|| println!("Hello world!"))
 *  );
 * }
 * ```
 */
pub struct TaskCarrier<T>(JoinHandle<()>, Sender<T>);

impl<T> TaskCarrier<T> {
    /**
     * Makes a new TaskCarrier type. This is the easiest clean way to make a new TaskCarrier.
     * ```rust
     * use threadder::TaskCarrier;
     * 
     * fn main() {
     *  let my_taskcarrier = TaskCarrier::new();
     *  my_taskcarrier.send(|| {
     *      println!("Hello world!");
     *  });
     * }
     * ```
     */
    pub fn new() -> TaskCarrier<T>
        where T: FnOnce() + 'static + Send {
        let (transmitter, reciever) = channel::<T>();
        TaskCarrier::<T>(
            spawn(move || loop {
                match reciever.try_recv() {
                    Ok(data) => data(),
                    Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => {}
                };
            }),
            transmitter,
        )
    }
    /**
     * Sends a task to a thread
     * ```rust
     * use threadder::TaskCarrier;
     * 
     * fn main() {
     *  let my_job = TaskCarrier::new();
     *  my_job.send(
     *    Box::new(|| println!("Hello world!"))
     *  ).unwrap();
     * }
     * ```
     */
    pub fn send(&self, task: T) -> Result<(), &'static str>
        where T: FnOnce() + 'static + Send {
        self.1.send(task).unwrap();
        Ok(())
    }
}

/**
 The main attraction -- a threadpooling solution.
 ```rust
  use threadder::ThreadPool;
  use std::thread::sleep;
  use std::time::Duration;
 
  fn main() {
    // MUST be mutable so that instructions may be sent to thread!
    let mut my_threadpool = ThreadPool::new(2);
    // ThreadPool::new()'s parameter is equal to number of cores. Must be above 0, and a usize!
    for _ in 0..4 {
      my_threadpool.send(Box::new(|| {
          // Simulate heavy load
          sleep(Duration::from_secs(4));
          println!("hello world!")
        })
      );
    }
  }
  ```
 */
pub struct ThreadPool<T>(Vec<TaskCarrier<T>>, usize);

impl<T> ThreadPool<T> {
    pub fn new(num_threads: usize) -> ThreadPool<T>
        where T: FnOnce() + 'static + Send {
        let mut threads = Vec::with_capacity(num_threads);
        if num_threads == 0 {
            panic_any("Empty threadpool!")
        }
        for _ in 0..num_threads {
            threads.push(TaskCarrier::new());
        }
        ThreadPool(threads, 1)
    }
    pub fn send(&mut self, task: T)
        where T: FnOnce() + 'static + Send {
        if self.0.len() == 0 {
            panic_any("Empty threadpool!")
        }
        self.0
        [   // Cycle around self.0's index.
            // E.g: An array has a len of 2. [0, 1].
            // If you gave it vec[2], it would return vec[0]
            self.1%self.0.len()
        ]
        .send(task)
        .expect("Failed to send task to thread");
        self.1 += 1;
    }
    pub fn stop(self) {
        for thread in self.0 {
            drop(thread.1);
            thread.0.join().unwrap();
        }
    }
}

#[test]
// Sends 3 tasks to the threadpool
fn send_thread() {
    let mut thread_pool = ThreadPool::new(3);
    for _ in 0..3 {
        thread_pool.send(Box::new(|| {
            println!("Hello world!");
            println!("Hello world!!!!!");
        }));
    }
    thread_pool.stop();
}
#[test]
fn single_thread_test() {
    let mut threadpool = ThreadPool::new(1);
    for _ in 0..3 {
        threadpool.send(Box::new(|| {
            std::thread::sleep(
                std::time::Duration::from_secs(2)
            );
            println!("Hello world!");
        }));
    }
}
#[test]
// Tests if thread exits properly
// Expected duration 3 seconds
fn test_safe_exit() {
    // Warning: Threadpools MUST be declared as mutable!
    // Create a 4-thread threadpool
    let mut my_threadpool = ThreadPool::new(4);
    // Due to a limiation of Rust, all clojures sent must be boxed to have a 'size known at compile time'... :'(
    for _ in 0..4 {
        my_threadpool.send(Box::new(|| {
            println!("Goodnight, world!");
            std::thread::sleep(
                // Simulate heavy load
                std::time::Duration::from_secs(3),
            );
            println!("Goodmorning, world!");
        }));
    }
    // MUST be ran at the end of the program. Ensures all threads exit PROPERLY. Without this, all threads will shut down as soon as the 'main' thread shuts down -- which isn't desirable when your thread finishes 3 seconds AFTER main does.
    my_threadpool.stop();
}
#[test]
// Test if threadpool properly load-balances.
// Six seconds expected speed
fn test_load_balance() {
    let mut my_threadpool = ThreadPool::new(3);
    for _ in 0..6 {
        my_threadpool.send(Box::new(|| {
            std::thread::sleep(
                // Simulate heavy load
                std::time::Duration::from_secs(3),
            );
            println!("Loaded B)");
        }));
    }
    my_threadpool.stop()
}

/*
#[test]
fn threadpool(jobs: Vec<TaskCarrier>) {
    for job in jobs.iter() {
        job.send(Box::new(move || {
            println!("Hello world!");
        }));
        job.send(Box::new(move || {
            println!("Hey there!");
        }));
    }
}
*/
