use std::thread;
use std::sync::mpsc;

use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("All Worker finished");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Finish: Worker {}", worker.id);

            // worker.thread.join().unwrap();
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>){
        (*self)()
    }
}

// struct Job;

// type Job = Box<FnOnce() + Send + 'static>;
type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool { 
        assert!(size>0);

        // let mut threads = Vec::with_capacity(size);

        // for _ in 0..size {

        // }
        // ThreadPool{
        //     threads
        // }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
            {
                let job = Box::new(f);
                // self.sender.send(job).unwrap();
                self.sender.send(Message::NewJob(job)).unwrap();
            }
}


struct Worker {
    id: usize,
    // thread: thread::JoinHandle<()>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move|| {
            // receiver;
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                
                match message {
                    Message::NewJob(job) => {
                        println!("Start: Worker {}", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Finish message received: Worker {}", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}