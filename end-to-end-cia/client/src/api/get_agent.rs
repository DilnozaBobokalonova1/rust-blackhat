use super::Client;
use common::api;
use crate::{config, Error};

impl Client {
    pub fn get_agent(&self, agent_id: &str) -> Result<api::Agent, Error> {
        let result = self.http_client.get(
            format!("{}/api/agents/{}", config::SERVER_URL, agent_id)
        ).send().map_err(|e| {
            return Error::ConnectionError(e.to_string());
        })?;

        let api_result: api::Response<api::Agent> = result.json()?;
        if let Some(err) = api_result.error {
            return Err(Error::Internal(err.message));
        }

        Ok(api_result.data.unwrap())
    }
}