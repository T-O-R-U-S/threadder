use std::{
    sync::mpsc::{channel, Sender},
    thread::{spawn, JoinHandle},
};

pub struct Job(JoinHandle<()>, Sender<Box<dyn FnOnce() + Send>>);

impl Job {
    pub fn new() -> Job {
        let (transmitter, reciever) = channel::<Box<dyn FnOnce() + Send>>();
        Job(
            spawn(move || loop {
                match reciever.recv() {
                    Ok(data) => data(),
                    Err(_) => {
                        println!("Thread terminated safely.");
                        break;
                    }
                };
            }),
            transmitter,
        )
    }
}

trait Threading {
    fn send(&self, task: Box<dyn FnOnce() + Send>) -> ();
}

impl Threading for Job {
    fn send(&self, task: Box<dyn FnOnce() + Send>) {
        self.1.send(task).unwrap();
    }
}

#[tokio::main]
async fn main() {
    let mut job_pool:Vec<Job> = vec![];
    for _ in 0..10 {
        &job_pool.push(Job::new());
    }
    threadpool(job_pool);
}

fn threadpool(jobs: Vec<Job>) {
    for job in jobs {
        job.send(Box::new(move || {
            println!("Hello world!");
        }));
        job.send(Box::new(move || {
            println!("Hey there!");
        }));
    }
}
