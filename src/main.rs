use std::{
    thread::{
    spawn,
    sleep,
    JoinHandle
}
    time::{
        Duration
    }
};

macro_rules! to_str {
    ($string:expr) => {
      String::from($string)
  };
}

#[tokio::main]
async fn main() {
    let to_string_string = to_str!("Hello world!");

    println!("{:?}", to_string_string);

    timer().await;
    println!("async this time?");
}

async fn timer() {
    let mut jobs: Vec<JoinHandle<()>> = vec![];

    for i in 0..5 {
        jobs.push(
            spawn(move ||
                loop {
                    println!("Async jobs go! {}", i);
                    sleep();
                }
            )
        )
    }

    for job in jobs {
        job.join().unwrap();
    }
}
