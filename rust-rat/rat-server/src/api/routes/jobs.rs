use crate::api::AppState;
use common::api;

use std::{sync::Arc, time::Duration};
use uuid::Uuid;
use warp::{http::StatusCode, serve, Rejection};

pub async fn create_job(
    state: Arc<AppState>,
    input: api::CreateJob,
) -> Result<impl warp::Reply, warp::Rejection> {
    let job = state.service.create_job(input).await?;
    let job: api::Job = job.into();

    let res = api::Response::ok(job);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}

pub async fn post_job_result(
    state: Arc<AppState>,
    input: api::UpdateJobResult,
) -> Result<impl warp::Reply, warp::Rejection> {
    state.service.update_job_result(input).await?;

    let res = api::Response::ok(true);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}

/**
 * Long-Polling implementation for getting a job.
 * Note: using u64 can lead to better performance, especially when running RAT
 * on 64-bit architectures where working with 64-bit ints may be more efficient
 * than the narrower inferred by compiler types.
 */
pub async fn get_job_result(
    state: Arc<AppState>,
    job_id: Uuid,
) -> Result<impl warp::Reply, warp::Rejection> {
    let sleep_for = Duration::from_secs(1);

    for _ in 0..10u64 {
        let job = state.service.find_job(job_id).await?;
        match &job.output {
            //job result found in this case, meaning it both exists and is done running or is not currently used.
            Some(_) => {
                let job: api::Job = job.into();
                let res = api::Response::ok(job);
                let res_json = warp::reply::json(&res);
                return Ok(warp::reply::with_status(res_json, StatusCode::OK));
            }
            None => tokio::time::sleep(sleep_for).await,
        }
    }

    //in this case, we have not found the job so return empy OK response
    let res = api::Response::<Option<()>>::ok(None); //unit type response
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}

pub async fn get_agent_job(state: Arc<AppState>, agent_id: Uuid) 
    -> Result<impl warp::Reply, warp::Rejection> {
    let sleep_for = Duration::from_secs(1);

    for _ in 0..10u64 {

        match state.service.get_agent_job(agent_id).await? {
            Some(job) => {
                let agent_job = api::AgentJob {
                    id: job.id,
                    command: job.command,
                    args: job.args.0,
                };

                let res = api::Response::ok(agent_job);
                let res_json = warp::reply::json(&res);
                return Ok(warp::reply::with_status(res_json, StatusCode::OK));
            }
            None => tokio::time::sleep(sleep_for).await,
        }
    }

    let res = api::Response::<Option<()>>::ok(None);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))
}

pub async fn get_jobs(state: Arc<AppState>) -> Result<impl warp::Reply, warp::Rejection> {
    let jobs = state.service.list_jobs().await?;
    let jobs = jobs.into_iter().map(Into::into).collect();
    let res = api::JobsList { jobs };

    let res = api::Response::ok(res);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::OK))

}