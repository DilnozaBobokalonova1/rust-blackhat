use clap::{Arg, Command};

mod api;
mod cli;
mod config;
mod error;

pub use error::Error;

fn main() {
    println!("Hello, world!");
}
