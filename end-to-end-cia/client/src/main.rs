use clap::{Arg, Command};

mod api;
mod error;
mod config;
mod cli;

pub use error::Error;

fn main() {
    println!("Hello, world!");
}
