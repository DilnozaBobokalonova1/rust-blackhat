use serde::de::DeserializeOwned;
use warp::Filter;

mod error;
mod state;

pub mod routes;
pub use routes::handle_error;

pub use state::{with_state, AppState};

