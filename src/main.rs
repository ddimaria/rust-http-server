#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use std::io::Result;

use crate::server::server;

mod config;
mod handlers;
mod headers;
mod request;
mod response;
mod server;
mod storage;
mod threadpool;

fn main() -> Result<()> {
    server().expect("Could not start server");
    Ok(())
}
