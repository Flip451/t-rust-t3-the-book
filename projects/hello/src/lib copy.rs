use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool<F>
where
    F: FnOnce() -> () + Send + 'static,
{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job<F>>,
}

struct Job<F>
where
    F: FnOnce() -> () + Send + 'static,
{
    f: F,
}

impl<F> ThreadPool<F>
where
    F: FnOnce() -> () + Send + 'static,
{
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn excute(&self, f: F) {
        self.sender.send(Job { f }).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new<F>(id: usize, receiver: Arc<Mutex<Receiver<Job<F>>>>) -> Self
    where
        F: FnOnce() -> () + Send + 'static,
    {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap();
                if let Ok(Job { f: job }) = job.try_recv() {
                    job();
                }
            }
        });
        Self { id, thread }
    }
}
