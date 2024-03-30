
mod jobs;
mod agents;
mod index;

use warp::Filter;
use index::index;
use agents::{get_agents, post_agents};
use std::sync::Arc;
use std::convert::Infallible;
use super::AppState;

pub fn routes(app_state: Arc<AppState>) {

    

}
