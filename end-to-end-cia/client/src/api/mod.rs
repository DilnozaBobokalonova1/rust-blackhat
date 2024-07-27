use std::time::Duration;
use reqwest::{blocking::Client as BlockingClient, redirect};

pub mod get_agent;
pub mod get_job_result;
pub mod create_job;
pub mod list_agents;

#[derive(Debug)]
pub struct Client {
    pub http_client: BlockingClient,
    server_url: String,
}

impl Client {
    pub fn new(server_url: String) -> Client {
        let http_timeout = Duration::from_secs(8);
        let http_client = BlockingClient::builder()
            .redirect(redirect::Policy::limited(4))
            .timeout(http_timeout)
            .build()
            .expect("api: Building HTTP client");

        Client {
            http_client,
            server_url
        }
    }
}