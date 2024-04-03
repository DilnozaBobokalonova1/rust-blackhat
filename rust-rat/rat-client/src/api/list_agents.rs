use super::Client;
use crate::{config, Error};
use common::api;

impl Client {

    // Returns a list of available agents
    pub fn list_agents(&self) -> Result<Vec<api::Agent>, Error> {
        let agents_get_route = format!("{}/api/agents", config::SERVER_URL);

        let res = self.http_client.get(agents_get_route).send()?;
        let api_res: api::Response<api::AgentsList> = res.json()?;

        if let Some(error) = api_res.error {
            return Err(Error::Internal(error.message));
        }

        Ok(api_res.data.unwrap().agents)
    }
}