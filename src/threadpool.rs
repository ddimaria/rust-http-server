use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// Type alias to hold the clojure type for the execute() method
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Use channels to send requests to threads
        let (sender, receiver) = mpsc::channel();

        // Allow for thread-save multiple consumers/receivers
        let receiver = Arc::new(Mutex::new(receiver));

        // Preallocate space
        let mut workers = Vec::with_capacity(size);

        // For each work, pass in the channel receiver
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // Create a new Job instance
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .send(Message::NewJob(job))
            .expect("Sender could not send message to workers");
    }
}

// Deal with thread pools when they go out of scope
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Tell all workers to terminate
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // Join threads together
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Sends code from the threadpool to the thread.
// Workers fetch work from the threadpool queue.
struct Worker {
    #[allow(dead_code)]
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // Spawn the threads
        let thread = thread::spawn(move || loop {
            // Acquire the lock, receive a message from the channel (blocking),
            // then send the message.
            let message = receiver
                .lock()
                .expect("Receiver mutex poisoned")
                .recv()
                .expect("Receiver could not send job");

            match message {
                Message::NewJob(job) => job(),
                Message::Terminate => break,
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}
