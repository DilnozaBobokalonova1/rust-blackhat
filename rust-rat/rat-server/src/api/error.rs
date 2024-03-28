use common::api;
use std::{collections::HashMap, convert::Infallible};
use warp::http::StatusCode;
use warp::{Rejection, Reply};

const EXTENSION_KEY_CODE: &str = "code";
const CODE_NOT_FOUND: &str = "NOT_FOUND";
const CODE_INTERNAL: &str = "INTERNAL";

impl std::convert::Into<api::Error> for crate::Error {
    
    fn into(self) -> api::Error {
        match self {
            crate::Error::NotFound(err) => {
                let mut extensions: HashMap<String, String> = HashMap::new();
                extensions.insert(EXTENSION_KEY_CODE.into(), CODE_NOT_FOUND.into());

                api::Error {
                    message: err.to_string(),
                    extensions: Some(extensions)
                }
            }

            crate::Error::Internal(_) => {
                let mut extensions = HashMap::new();
                extensions.insert(EXTENSION_KEY_CODE.into(), CODE_INTERNAL.into());

                api::Error {
                    message: self.to_string(),
                    extensions: Some(extensions),
                }
            }

            _ => api::Error {
                message: self.to_string(),
                extensions: None,
            }
        }
    }
}