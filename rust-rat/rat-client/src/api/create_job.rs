use common::api;
use uuid::Uuid;

use crate::{config, Error};

use super::Client;

impl Client {

    pub fn create_job(&self, input: api::CreateJob) -> Result<Uuid, Error> {
        let post_request_job_route = format!("{}/api/jobs", config::SERVER_URL);

        let res = self.http_client.post(post_request_job_route).send()?;
        let api_result: api::Response<api::Job> = res.json()?;

        if let Some(err) = api_result.error {
            return Err(Error::Internal(err.message));
        }

        Ok(api_result.data.unwrap().id)
    }
}