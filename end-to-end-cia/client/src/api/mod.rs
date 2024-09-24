use std::time::Duration;
use reqwest::redirect;
mod job_functions;

#[derive(Debug)]
pub struct Client {
    pub http_client: reqwest::blocking::Client,
    server_url: String,
}

impl Client {
    pub fn new(server_url: String) -> Client {
        let http_timeout = Duration::from_secs(9);
        let http_client = reqwest::blocking::Client::builder()
            .redirect(redirect::Policy::limited(6))
            .timeout(http_timeout)
            .build()
            .expect("api: Building HTTP client!");

        Client {
            http_client,
            server_url,
        }
    }
}