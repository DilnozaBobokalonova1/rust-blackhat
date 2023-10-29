use crate::{
    modules::{HttpFinding, HttpModule, Module},
    Error,
};

use async_trait::async_trait;
use reqwest::Client;

pub struct GitLabOpenRegistrations {}

impl GitLabOpenRegistrations {
    pub fn new() -> Self {
        GitLabOpenRegistrations {}
    }
}

impl Module for GitLabOpenRegistrations {
    fn name(&self) -> String {
        String::from("http/gitlab_open_registration")
    }

    fn description(&self) -> String {
        String::from("Check if the GitLab instance is open to registrations")
    }
}

#[async_trait]
impl HttpModule for GitLabOpenRegistrations {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}", &endpoint);
        let res = http_client.get(&url).send().await?;

        if !res.status().is_success() {
            return Ok(None);
        }

        let body = res.text().await?;
        if body.contains("This is a self-managed instance of GitLab") && body.contains("Register") {
            return Ok(Some(HttpFinding::GitLabOpenRegistrations(url)));
        }

        Ok(None)
    }
}
