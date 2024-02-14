use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job =Box<dyn FnOnce() + Send +'static>;

impl ThreadPool {
    /// Create a new ThreadPool with the specified number of threads.
    ///
    /// Panics if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel(); 
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // Move ownership of the closure into the thread    
                workers.push(
                    Worker::new(id, Arc::clone(&receiver))
                );

        
        }


        ThreadPool { workers, sender}
        
    }

    

    /// Execute a given closure in a thread from the pool.
    ///
    /// The closure must be `Send` and `'static`.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
        }
        // Collect the JoinHandle so the thread doesn't get dropped prematurely
    }


    struct Worker {
        id: usize,
        thread: thread::JoinHandle<()>,
    }

    impl Worker {
        fn new(
            id: usize,
            receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
        ) -> Worker {
            let thread = thread::spawn(move || loop{
                let job = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();
            println!("Worker  {id} got a job, executing..");
            job();
            });

            Worker {id, thread}
        }
    }
