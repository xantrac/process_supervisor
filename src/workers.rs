use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub struct Worker {
    pub name: String,
    pub id: i32,
    pub sender: Sender<String>,
}

pub fn start_supervisor(n_workers: i32, event_requester: Sender<String>) -> Vec<Worker> {
    let workers: Vec<Worker> = (1..n_workers)
        .map(|n| {
            let cloned_event_requester = Sender::clone(&event_requester);
            let (consumer_sender, consumer_receiver): (Sender<String>, Receiver<String>) =
                channel();
            thread::spawn(move || {
                start_worker(
                    format!("worker#{}", n),
                    cloned_event_requester,
                    consumer_receiver,
                )
            });
            Worker {
                name: format!("worker#{}", n),
                id: n,
                sender: consumer_sender,
            }
        })
        .collect();
    workers
}

fn start_worker(
    name: String,
    event_requester: Sender<String>,
    worker_receiver: Receiver<String>,
) -> () {
    loop {
        match worker_receiver.recv_timeout(Duration::from_secs(3)) {
            Ok(value) => println!("I got some stuff to do {}", value),
            Err(_) => {
                let value = format!("{}", name);
                event_requester.send(value).unwrap();
            }
        }
    }
}
