use std::{
    panic::panic_any,
    sync::mpsc::{channel, Sender, TryRecvError},
    thread::{spawn, JoinHandle},
};

pub struct Job(JoinHandle<()>, Sender<Box<dyn FnOnce() + Send>>);

impl Job {
    pub fn new() -> Job {
        let (transmitter, reciever) = channel::<Box<dyn FnOnce() + Send>>();
        Job(
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
}

trait Threading {
    fn send(&self, task: Box<dyn FnOnce() + Send>) -> Result<(), &'static str>;
}

impl Threading for Job {
    fn send(&self, task: Box<dyn FnOnce() + Send>) -> Result<(), &'static str> {
        self.1.send(task).unwrap();
        Ok(())
    }
}

pub struct ThreadPool(Vec<Job>, usize);

trait ThreadPoolManagement {
    fn send(&mut self, task: Box<dyn FnOnce() + Send>) -> ();
    fn stop(self) -> ();
}

impl ThreadPool {
    pub fn new(num_threads: u16) -> ThreadPool {
        let mut threads = vec![];
        if num_threads == 0 {
            panic_any("Empty threadpool!")
        }
        for _ in 0..num_threads {
            threads.push(Job::new());
        }
        ThreadPool(threads, 1)
    }
}

impl ThreadPoolManagement for ThreadPool {
    fn send(&mut self, task: Box<dyn FnOnce() + Send>) {
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
    fn stop(self) {
        for thread in self.0 {
            drop(thread.1);
            thread.0.join().unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let mut thread_pool = ThreadPool::new(3);
    thread_pool.send(Box::new(|| {
        println!("Hello world!");
        println!("Hello world!!!!!");
    }));
    thread_pool.send(Box::new(|| {
        println!("Hello world!");
        println!("Hello world!!!!!");
    }));
    thread_pool.send(Box::new(|| {
        println!("Hello world!");
        println!("Hello world!!!!!");
    }));

    /*
    let mut job_pool:Vec<Job> = vec![];
    for _ in 0..10 {
        &job_pool.push(Job::new());
    }
    threadpool(job_pool);
    */
}
/*
fn threadpool(jobs: Vec<Job>) {
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
