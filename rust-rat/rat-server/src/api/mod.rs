use serde::de::DeserializeOwned;
use warp::Filter;

mod error;
mod state;

pub mod routes;
pub use error::handle_error;

pub use state::{with_state, AppState};

/**
 * This function constructs a filter that first checks if the incoming
 * request's body is under a size limit and then attempts to deserialize
 * it into a specified type.
 * 
 * If any of the previous 2 steps fail, the filter chain will not proceed,
 * and a warp::Rejection error will be returned.
 * 
 * Size limit serves as a safeguard to prevent large payloads from being
 * processed which could potentially be a vector for denial-of-service attacks.
 */
pub fn json_body<T: DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
