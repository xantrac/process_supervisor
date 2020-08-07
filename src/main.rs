mod workers;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

fn main() {
    let (event_requester, event_request): (Sender<String>, Receiver<String>) = mpsc::channel();
    fn callback (value: String) { println!("Executing callback with value: {}", value) };
    let workers = workers::start_supervisor(5, event_requester, callback);
    let mut workers_map: HashMap<String, workers::Worker> = HashMap::new();
    for worker in workers {
        workers_map.insert(String::from(&worker.name), worker);
    }

    loop {
        let free_worker_name = event_request.recv().unwrap();
        match workers_map.get(&free_worker_name) {
            Some(worker) => worker.sender.send(format!("Please do this {}!", worker.id)).unwrap(),
            None => println!("Boooo"),
        };
    }
}
