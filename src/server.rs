use std::io::Result;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};

use crate::config::CONFIG;
use crate::data::Data;
use crate::handlers::{delete, get, post};
use crate::request::{Method, Request};
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

    // Share ownership of data and user a reader-writer lock.
    // Allows multiple readers and a single writer.
    let mut data_inner = Data::new();
    data_inner.insert("first".into(), "TEMP DATA TO STORE");
    let data = Arc::new(RwLock::new(data_inner));

    info!("Starting server at {}", CONFIG.server);

    // Read in the streams into an iterator.
    for stream in listener.incoming() {
        // Get a new Arc instance
        let data = Arc::clone(&data);

        // Spawn a new thread in the threadpool
        pool.execute(|| {
            let request = Request::new(stream.expect("Could not acquire the request stream."))
                .expect("Could not read from the request stream.");

            // Handle routing
            match request.method {
                Method::Delete => delete(request, data),
                Method::Get => get(request, data),
                Method::Post => post(request, data),
            }
        });
    }

    Ok(listener)
}
