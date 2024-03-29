use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

type Job = Box<dyn FnOnce() -> () + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Job>();

        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            workers.push(Worker::new(id, receiver));
        }
        Self { workers, sender }
    }

    pub fn excute<F>(&self, f: F)
    where
        F: FnOnce() -> () + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });
        Self { id, thread }
    }
}
