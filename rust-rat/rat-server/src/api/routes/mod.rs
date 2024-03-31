
mod jobs;
mod agents;
mod index;

use warp::Filter;
use index::index;
use agents::{get_agents, post_agents};
use std::sync::Arc;
use std::convert::Infallible;
use jobs::{get_jobs, create_job, post_job_result, get_job_result, get_agent_job};

use super::AppState;

pub fn routes(
    app_state: Arc<AppState>
) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {

    let api = warp::path("api");
    //making sure there is a provided shared application state
    let api_with_state = api.and(super::with_state(app_state));

    //building index route by chaining several filters for serving GET HTTP reqs calling /api
    let index = api.and(warp::path::end()).and(warp::get()).and_then(index);

    //call to GET /api/jobs
    let get_jobs = api_with_state
        .clone()
        .and(warp::path("jobs"))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(get_jobs);

    //POST /api/jobs
    let post_jobs = api_with_state
        .clone()
        .and(warp::path("jobs"))
        .and(warp::path::end())
        .and(warp::post())
        .and(super::json_body())
        .and_then(create_job);

    //GET /api/jobs/{job_id}/result
    let get_job = api_with_state
        .clone()
        .and(warp::path("jobs"))
        .and(warp::path::param())
        .and(warp::path("result"))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(get_job_result);

    // POST /api/jobs/result
    let post_job_result = api_with_state
        .clone()
        .and(warp::path("jobs"))
        .and(warp::path("result"))
        .and(warp::path::end())
        .and(warp::post())
        .and(super::json_body())
        .and_then(post_job_result);

    // POST /api/agents
    let post_agents = api_with_state
        .clone()
        .and(warp::path("agents"))
        .and(warp::path::end())
        .and(warp::post())
        .and_then(post_agents);

    let get_agents = api_with_state
        .clone()
        .and(warp::path("agents"))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(get_agents);

    //GET /api/agents/{agent_id}/job
    let get_agents_job = api_with_state
        .clone()
        .and(warp::path("agents"))
        .and(warp::path::param())
        .and(warp::path("job"))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(get_agent_job);

    let routes = index
        .or(get_jobs)
        .or(post_jobs)
        .or(get_job)
        .or(post_job_result)
        .or(post_agents)
        .or(get_agents)
        .or(get_agents_job)
        .with(warp::log("server"))
        .recover(super::handle_error);
    
    routes
}
