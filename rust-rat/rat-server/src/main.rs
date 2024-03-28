mod error;
mod config;
mod db;
mod api;
mod service;
mod entities;
pub use service::Service;

pub use error::Error;
use config::Config;


fn main() {
    println!("Hello, world!");

}
