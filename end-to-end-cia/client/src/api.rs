use std::time::Duration;
use common::api::{self, Agent, AgentsList, JobResult};
use reqwest::{blocking::Client as BlockingClient, redirect};
use uuid::Uuid;
use crate::{config, Error};

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

    pub fn get_agent(&self, agent_id: &str) -> Result<Agent, Error> {
        let get_agent_route = format!("{}/api/agents/{}", config::SERVER_URL, agent_id);
        let res = self.http_client.get(get_agent_route).send()?;
        let api_res: api::Response<api::Agent> = res.json()?;

        if let Some(err) = api_res.error {
            return Err(Error::Internal(err.message));
        }

        Ok(api_res.data.unwrap())
    }

    pub fn list_agents(&self) -> Result<Vec<api::Agent>, Error> {
        let get_agents_route = format!("{}/api/agents", config::SERVER_URL);
        let res = self.http_client.get(get_agents_route).send()?;
        let api_response: api::Response<AgentsList> = res.json()?;
        if let Some(err) = api_response.error {
            return Err(Error::Internal(err.message));
        }

        Ok(api_response.data.unwrap().agents)
    }

    pub fn create_job(&self, input: api::CreateJob) -> Result<Uuid, Error> {
        let post_job_route = format!("{}/api/jobs", config::SERVER_URL);
        let result = self.http_client.post(post_job_route).json(&input).send()?;
        let api_result: api::Response<api::Job> = result.json()?;

        if let Some(err) = api_result.error {
            return Err(Error::Internal(err.message));
        }

        Ok(api_result.data.unwrap().job_id)

    }

    pub fn get_job_result(&self, job_id: Uuid) -> Result<Option<api::Job>, Error> {
        let get_job_result_route = format!("{}/api/jobs/{}/result", config::SERVER_URL, job_id);
        let result = self.http_client.get(get_job_result_route).send()?;
        let api_res: api::Response<api::Job> = result.json()?;

        if let Some(error) = api_res.error {
            return Err(Error::Internal(error.message));
        }

        Ok(api_res.data) // might not have a job result when getting so Optional
    }
}
