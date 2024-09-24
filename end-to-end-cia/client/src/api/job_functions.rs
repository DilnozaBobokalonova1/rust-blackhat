use common::api;
use common::api::AgentsList;
use uuid::Uuid;
use std::fmt::Display;
use super::Client;
use crate::config;
use crate::Error;

#[derive(Debug,)]
pub enum ApiRoute {
    CreateJob,
    GetJobResult(Uuid),
    ListAgents,
    GetAgent(Uuid),
}

impl Display for ApiRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiRoute::CreateJob => write!(f, "{}/api/jobs", config::SERVER_URL),
            ApiRoute::GetAgent(agent_id) => write!(f, "{}/api/agents/{}", config::SERVER_URL, agent_id),
            ApiRoute::GetJobResult(job_id) => write!(f, "{}/api/jobs/{}/result", config::SERVER_URL, job_id),
            ApiRoute::ListAgents => write!(f, "{}/api/agents", config::SERVER_URL),
        }
    }
}

impl Client {

    fn send_request<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        route: ApiRoute,
        body: Option<&B>,
    ) -> Result<T, Error> {
        let request = self.http_client.get(route.to_string());
        let request = if let Some(body) = body {
            request.json(body)
        } else {
            request
        };

        let response = request.send()?;
        let api_result: api::Response<T> = response.json()?;

        if let Some(err) = api_result.error {
            return Err(Error::Internal(err.message));
        }

        Ok(api_result.data.unwrap())
    }


    pub fn create_job(&self, input: api::CreateJob) -> Result<Uuid, Error> {
        self.send_request(ApiRoute::CreateJob, Some(&input)).map(|res: api::Response<api::Job>| res.data.unwrap().job_id)
    }

    pub fn get_job_result(&self, job_id: uuid::Uuid) -> Result<Option<api::Job>, Error> {
        self.send_request::<api::Response<api::Job>, ()>(ApiRoute::GetJobResult(job_id), None).map(|res: api::Response<api::Job>| res.data)
    }

    pub fn get_agent(&self, agent_id: Uuid) -> Result<api::Agent, Error> {
        self.send_request::<api::Response<api::Agent>, ()>(ApiRoute::GetAgent(agent_id), None).map(|res: api::Response<api::Agent>| res.data.unwrap())
    }

    pub fn list_agents(&self,) -> Result<AgentsList, Error> {
        self.send_request::<api::Response<api::AgentsList>, ()>(ApiRoute::ListAgents, None).map(|result: api::Response<AgentsList>| result.data.unwrap())
    }
}