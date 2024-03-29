mod api;
mod config;
mod db;
mod entities;
mod error;
mod repository;
mod service;
pub use service::Service;

use config::Config;
pub use error::Error;

fn main() {
    println!("Hello, world!");
}
