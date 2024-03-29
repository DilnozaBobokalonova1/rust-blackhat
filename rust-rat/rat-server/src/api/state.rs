use crate::Service;
use std::sync::Arc;
use warp::Filter;

#[derive(Debug)]
pub struct AppState {
    pub service: Service,
}

impl AppState {
    pub fn new(service: Service) -> AppState {
        AppState { service }
    }
}

/**
 * Using with_state in Warp to inject the shared state into our request handlers.
 * Cloning the Arc ensures that each part of our application that needs to access
 * the state can do so safely and independently, without directly sharing the
 * mutable state or risking data races.
 */
pub fn with_state(
    state: Arc<AppState>,
) -> impl Filter<Extract = (Arc<AppState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
