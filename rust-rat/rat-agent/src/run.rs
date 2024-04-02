use std::{thread::sleep, time::Duration};
use uuid::Uuid;
use common::api;
use crate::consts;
use std::process::Command;

pub fn run(api_client: &ureq::Agent, agent_id: Uuid) -> ! {
    let sleep_for = Duration::from_secs(3);
    let get_job_route = format!("{}/api/agents/{}/job", consts::SERVER_URL, agent_id);
    let post_job_result_route = format!("{}/api/jobs/result", consts::SERVER_URL);

    loop {
        // retrieve a job from server using agent_id
        let server_res = match api_client.get(get_job_route.as_str()).call() {
            Ok(res) => res,
            Err(err) => {
                log::debug!("Error getting job from the server: {}", err);
                sleep(sleep_for);
                continue;
            }
        };

        // deserialize server's JSON response of type ureq::Response into api::Response 
        let api_res: api::Response<api::AgentJob> = match server_res.into_json() {
            Ok(res) => res,
            Err(err) => {
                log::debug!("Error parsing JSON: {}", err);
                sleep(sleep_for);
                continue;
            }
        };

        log::debug!("API response successfully received and deserialized! {:?}", api_res);

        let job: api::AgentJob = match api_res.data {
            Some(job) => job,
            None => {
                log::debug!("No job found. Trying again in a few seconds! {:?}", sleep_for);
                sleep(sleep_for);
                continue;
            }
        };
        
        let output = execute_command(job.command, job.args);

        let job_result = api::UpdateJobResult {
            job_id: job.id,
            output
        };

        //now post the job result post-Agent's execution of the instructed commands
        match api_client
            .post(post_job_result_route.as_str())
            .send_json(ureq::json!(job_result)) {
                Ok(_) => {},
                Err(err) => {
                    log::debug!("Error sending job's result back: {}", err);
                }
        };
    }
}

fn execute_command(command: String, args: Vec<String>) -> String {

    let output = match Command::new(command).args(&args).output() {
        Ok(output) => output,
        Err(err) => {
            log::debug!("Error executing command: {}", err);
            return String::new();
        }
    };

    match String::from_utf8(output.stdout) {
        Ok(stdout) => {
            return stdout
        },
        Err(err) => {
            log::debug!("Error converting command's output to Syring: {}", err);
            return String::new();
        }
    };
}