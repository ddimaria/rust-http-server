use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    #[allow(dead_code)]
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// Type alias to hold the clojure type for the execute() method
type Message = Box<dyn FnOnce() + Send + 'static>;

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

    // Create a new Message instance
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let message = Box::new(f);
        self.sender.send(message).expect("Send could not send message.");
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
                .expect("Receiver mutex poisoned.")
                .recv()
                .expect("Receiver could not send message.");
            message()
        });

        Worker {
            thread: Some(thread),
        }
    }
}
