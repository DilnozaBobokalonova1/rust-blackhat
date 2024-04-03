use common::api::{self, Job};

use super::Client;
use crate::{cli::jobs, config, Error};


impl Client {
    pub fn list_jobs(&self) -> Result<Vec<Job>, Error> {

        let jobs_list_route: String = format!("{}/api/jobs", config::SERVER_URL);

        let api_response = self.http_client.get(jobs_list_route).send()?;
        let res: api::Response<api::JobsList> = api_response.json()?;

        // if let Some(jobs_data) = res.data {
        //     return Ok(res.data.unwrap().jobs);
        // }

        // return Err(Error::Internal(res.error.unwrap().message));

        if let Some(err) = res.error {
            return Err(Error::Internal(err.message));
        }

        return Ok(res.data.unwrap().jobs);
    }
}