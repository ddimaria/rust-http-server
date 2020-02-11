use std::io::Result;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};

use crate::config::CONFIG;
use crate::handlers::{delete, get, post};
use crate::request::{Method, Request};
use crate::storage::Storage;
use crate::threadpool::ThreadPool;

// Starts the server.
// Improvement: Multi-thread the server.
// Improvement: Make async.
// Improvement: Handle different types of http requests.
pub fn server() -> Result<TcpListener> {
    env_logger::init();
    let listener = TcpListener::bind(&CONFIG.server)?;

    // Keep the threadpool at 4, but can be increased.
    let pool = ThreadPool::new(4);

    // Share ownership of storage and user a reader-writer lock.
    // Allows multiple readers and a single writer.
    let storage = Arc::new(RwLock::new(Storage::new()));

    info!("Starting server at {}", CONFIG.server);

    // Read in the streams into an iterator.
    for stream in listener.incoming() {
        // Get a new Arc instance
        let storage = Arc::clone(&storage);

        // Spawn a new thread in the threadpool
        pool.execute(|| {
            let request = Request::new(stream.expect("Could not acquire the request stream."))
                .expect("Could not read from the request stream.");

            // Handle routing
            match request.method {
                Method::Delete => delete(request, storage),
                Method::Get => get(request, storage),
                Method::Post => post(request, storage),
            }
        });
    }

    Ok(listener)
}
