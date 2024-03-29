use crate::api::AppState;
use common::api;

use std::{sync::Arc, time::Duration};
use uuid::Uuid;
use warp::{http::StatusCode, Rejection};

pub async fn create_job(
    state: Arc<AppState>,
    input: api::CreateJob,
) -> Result<impl warp::Reply, warp::Rejection> {
    let job = state.service.create_job(input).await?;
    let job: api::Job = job.into();

    let res = api::Response::ok(job);
    let res_json = warp::reply::json(&res);
    Ok(warp::reply::with_status(res_json, StatusCode::Ok))
}
