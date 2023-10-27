use std::env;

use anyhow::{Result, Ok};
use clap::{Arg, Command};

mod cli;
mod common_ports;
mod dns;
mod error;
mod ports;
mod modules;
pub use error::Error;

fn main() -> Result<()> {
    // set the global log level to "info" & specify that log messages 
    // from the trust_dns_proto module with a severity level of "error" 
    // should be displayed
    env::set_var("RUST_LOG", "info,trust_dns_proto=error");
    env_logger::init();

    let cli = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(Command::new("modules").about("List all modules"))
        .subcommand(Command::new("scan").about("Scan a target").arg(
            Arg::new("target").help("The domain name to scan").required(true).index(1),
        ),)
        .arg_required_else_help(true)
        .get_matches();
        
        if let Some(_) = cli.subcommand_matches("modules") {
            cli::modules();
        } else if let Some(matches) = cli.subcommand_matches("scan") {
            let target = matches.get_one::<String>("target").unwrap();
            cli::scan(target)?;
        }
        
        Ok(())
}