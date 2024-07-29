mod config;
mod error;
mod init;
mod run;
pub use error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent("di_agent/0.1")
        .build();

    let conf = init::init(&api_client)?;

    run::run(&api_client, conf);
}
