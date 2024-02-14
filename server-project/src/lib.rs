use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool with the specified number of threads.
    ///
    /// Panics if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // Move ownership of the closure into the thread    
                workers.push(Worker::new(id));

        
        }


        ThreadPool { workers }
        
    }

    

    /// Execute a given closure in a thread from the pool.
    ///
    /// The closure must be `Send` and `'static`.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Move ownership of the closure into the thread
        let handle = thread::spawn(|| {
            f();
        });
        // Collect the JoinHandle so the thread doesn't get dropped prematurely
    }

}

    struct Worker {
        id: usize,
        thread: thread::JoinHandle<()>,
    }

    impl Worker {
        fn new(id: usize) -> Worker {
            let thread = thread::spawn(|| {});

            Worker {id, thread}
        }
    }
