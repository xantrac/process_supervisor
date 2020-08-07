mod workers;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

fn main() {
    let (event_requester, event_request): (Sender<String>, Receiver<String>) = mpsc::channel();

    workers::start_supervisor(5, event_requester);
    loop {
        let request = event_request.recv().unwrap();
        println!("Got: {}", request);
    }
}
