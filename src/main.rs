mod workers;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let (event_requester, event_request): (Sender<String>, Receiver<String>) = mpsc::channel();
    fn callback(value: String) {
        println!("Executing callback with value: {}", value)
    };
    let workers = workers::start_supervisor(5, event_requester, callback);
    let mut workers_map: HashMap<String, workers::Worker> = HashMap::new();
    for worker in workers {
        workers_map.insert(String::from(&worker.name), worker);
    }
    let endless_event_list = vec!["event1", "event2", "event3", "event4", "event5", "event6"];
    let mut mutex = Mutex::new(endless_event_list);

    loop {
        let updated_event_list = mutex.get_mut().unwrap();

        match updated_event_list.len() {
            0 => {
                println!("Nothing to do");
                thread::sleep(Duration::from_secs(3));
            }
            _ => {
                let free_worker_name = event_request.recv().unwrap();
                match workers_map.get(&free_worker_name) {
                    Some(worker) => {
                        worker
                            .sender
                            .send(format!("Please do this {}!", updated_event_list[0]))
                            .unwrap();
                        updated_event_list.remove(0);
                    }
                    None => println!("No workers available"),
                };
            }
        }
    }
}
