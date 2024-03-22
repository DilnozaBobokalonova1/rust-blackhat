use anyhow::Ok;
use futures::{stream, StreamExt, lock::Mutex};
use reqwest::Client;
use std::{
    env,
    time::{Duration, Instant}, sync::Arc,
};

mod error;
pub use error::Error;
mod model;
mod ports;
mod subdomains;
use model::Subdomain;
mod common_ports;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		return Err(Error::CliUsage.into());
	}

	let target = args[1].as_str();

	let http_timeout = Duration::from_secs(10);
	let http_client = Client::builder().timeout(http_timeout).build()?;

	let ports_concurrency = 200;
	let subdomains_concurrency = 100;
	let scan_start = Instant::now();

	let subdomains = subdomains::enumerate(&http_client, target).await?;

	let res: Arc<Mutex<Vec<Subdomain>>> = Arc::new(Mutex::new(Vec::new()));

	stream::iter(subdomains.into_iter()).for_each_concurrent(
		subdomains_concurrency, |subdomain| {
			//cloning of async reference counting pointer to empty mutable Vec
			let res = res.clone();
			async move {
				let subdomain = ports::scan_ports(ports_concurrency, subdomain).await;
				res.lock().await.push(subdomain)
			}
	}).await;

	
	let scan_duration = scan_start.elapsed();
	println!("Scan completed in {:?}", scan_duration);

	let mutex_guard = res.lock().await;
	for subdomain in mutex_guard.iter() {
		println!("{}", &subdomain.domain);
		for port in &subdomain.open_ports {
			println!("		{}: open", port.port);
		}
		println!("");
	}
	// for subdomain in res.lock().unwrap().iter() {
	// 	println!("{}", &subdomain.domain);
	// 	for port in &subdomain.open_ports {
	// 		println!("		{}: open", port.port);
	// 	}

	// 	println!("");
	// }
	
	Ok(())
}
