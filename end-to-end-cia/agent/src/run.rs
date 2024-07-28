use std::{thread::sleep, time::Duration};

use common::api;

use crate::config;


pub fn run(api_client: &ureq::Agent, conf: config::Config) -> ! {
    let sleep_for = Duration::from_secs(1);
    let get_job_route = format!("{}/api/agents/{}/job". config::SERVER_URL, conf.agent_id);
    let post_job_result_route = format!("{}/api/jobs/result", config::SERVER_URL);

    while {
        let server_res = match api_client.get(get_job_route.as_str()).call() {
            Ok(res) => res,
            Err(err) => {
                log::debug!("Error greeting job from server: {}", err);
                sleep(sleep_for);
                continue;
            }
        };

        let api_res: api::Response<api::AgentJob>
    }


}