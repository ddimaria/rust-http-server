use std::sync::{Arc, RwLock};

use crate::request::Request;
use crate::response::{respond, HttpStatusCode};
use crate::storage::{Storage};

// Handle GET requests.
pub fn get(request: Request, storage: Arc<RwLock<Storage>>) {
    // let storage = storage.read().expect("RwLock read poisoned");
    respond(request.stream, HttpStatusCode::OK, None, None);
}

// Handle POST requests.
pub fn post(request: Request, storage: Arc<RwLock<Storage>>) {
    // let mut storage = storage.write().expect("RwLock write poisoned");
    respond(request.stream, HttpStatusCode::OK, None, None);
}

// Handle DELETE requests.
pub fn delete(request: Request, storage: Arc<RwLock<Storage>>) {
    // let mut storage = storage.write().expect("RwLock write poisoned");
    respond(request.stream, HttpStatusCode::OK, None, None);
}
