use std::{
    sync::mpsc::{channel, Sender},
    thread::{spawn, JoinHandle},
};

struct Job(JoinHandle<()>, Sender<Box<dyn FnOnce() + Send>>);

impl Job {
    fn new() -> Job {
        let (transmitter, reciever) = channel::<Box<dyn FnOnce() + Send>>();
        Job(spawn(move || {
            loop {
                let recv_job = reciever.recv().unwrap();
                recv_job();
            }
        }), transmitter)
    }
}

trait Start {
    fn send(&self, task:Box<dyn FnOnce() + Send>) -> ();
}

impl Start for Job {
    fn send(&self, task:Box<dyn FnOnce() + Send>) {
        self.1.send(task).unwrap();
    }
}

#[tokio::main]
async fn main() {
    threadpool(vec![Job::new(), Job::new()]);
}

fn threadpool(jobs: Vec<Job>) {

    for job in jobs {
        loop {
            job.send(Box::new(|| {
                println!("Hello world!");
            }));
        }
        
    }
}
