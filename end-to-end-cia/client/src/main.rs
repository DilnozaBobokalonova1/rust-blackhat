use clap::{Arg, Command};

mod api_v2;
mod cli;
mod config;
mod error;
mod api;

pub use error::Error;

fn main() {
    println!("Hello, world!");
}
