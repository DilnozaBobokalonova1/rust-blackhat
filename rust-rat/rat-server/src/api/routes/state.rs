use crate::Service;
use std::sync::Arc;
use warp::Filter;

#[derive(Debug)]
pub struct AppState {
    pub service: Service,
}