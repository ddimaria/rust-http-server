use std::sync::{Arc, RwLock};

use crate::data::Data;
use crate::request::Request;
use crate::response::{respond, HttpStatusCode};

// Handle GET requests.
pub fn get<T>(request: Request, data: Arc<RwLock<Data<T>>>) {
    // let data = data.read().expect("RwLock read poisoned");
    respond(request.stream, HttpStatusCode::OK, None, None);
}

// Handle POST requests.
pub fn post<T>(request: Request, data: Arc<RwLock<Data<T>>>) {
    // let mut data = data.write().expect("RwLock write poisoned");
    respond(request.stream, HttpStatusCode::OK, None, None);
}

// Handle DELETE requests.
pub fn delete<T>(request: Request, data: Arc<RwLock<Data<T>>>) {
    // let mut data = data.write().expect("RwLock write poisoned");
    respond(request.stream, HttpStatusCode::OK, None, None);
}
