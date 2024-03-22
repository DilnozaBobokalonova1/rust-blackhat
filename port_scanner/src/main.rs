use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};
use std::{env, time::{Duration, Instant}};

mod error;
pub use error::Error;
mod models;
mod ports;
mod subdomains;
use models::Subdomain;
mod common_ports;
fn main() -> Result<(), anyhow::Error> {
    
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].as_str();

    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;
    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();
    //lets start the scan!
    let scan_start = Instant::now();

    pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_iter()
            .map(ports::scan_ports)
            .collect();
        
        println!("Scan completed in {:?}", scan_start.elapsed());

        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in subdomain.open_ports {
                println!("  {}", port.port);
            }

            println!();
        }
    });
    Ok(())
}
