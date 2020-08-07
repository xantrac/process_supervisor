use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use std::fmt;


struct Worker {
    name: String,
    id: i32,
    sender: Sender<i32>,
    receiver: Receiver<i32>,
}

pub fn start_supervisor(n_workers: i32, event_requester: Sender<String>) -> () {
    let stuff: Vec<i32> = (1..n_workers)
        .map(|n| {
            let cloned_event_requester = Sender::clone(&event_requester);
            let (consumer_sender, consumer_receiver): (Sender<String>, Receiver<String>) = channel();

            thread::spawn(move || start_worker(n, cloned_event_requester, consumer_receiver));
            n
        })
        .collect();

}

fn start_worker(name: i32, event_requester: Sender<String>, worker_receiver: Receiver<String>) -> () {
    loop {
        let value = format!("Next event for worker {} please", name);
         event_requester.send(value).unwrap();       
        thread::sleep(Duration::from_secs(3));

    }
}
