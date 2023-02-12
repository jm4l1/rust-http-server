use std::cmp::{Eq, PartialEq};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

use super::traits::FnBox;

enum Message {
    NewJob(Job),
    Terminate,
}
struct Worker {
    thread: Option<JoinHandle<()>>,
    id: Id,
}
pub struct ThreadPool {
    size: usize,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnBox + Send + 'static>;
pub struct Id {
    value: usize,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        println!("Initializing thread pools of size {size}");
        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::spawn(id, Arc::clone(&receiver)));
            println!("Worker {id} started");
        }
        ThreadPool {
            size,
            workers,
            sender,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down work {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn spawn(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing", id);
                    job.call_box();
                }
                Message::Terminate => {
                    println!("Worker {} got terminate signal", id);
                    break;
                }
            }
        });
        Worker {
            id: Id { value: id },
            thread: Some(thread),
        }
    }
}

impl Id {
    pub fn new(value: usize) -> Id {
        Id { value }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Id {}
