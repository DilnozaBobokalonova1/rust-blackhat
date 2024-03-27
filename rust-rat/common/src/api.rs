use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, String>>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T: Serialize> {
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl<T:Serialize> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        return Response { data: Some(data), error: None };
    }

    pub fn error(err: Error) -> Response<()> {
        return Response::<()> {
            data: None,
            error: Some(err.into())
        };
    }
}