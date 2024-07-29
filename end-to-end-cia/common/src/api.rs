use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::crypto;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T: Serialize> {
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        return Response {
            data: Some(data),
            error: None,
        }
    }

    pub fn err(err: Error) -> Response<()> {
        return Response::<()> {
            data: None,
            error: Some(err.into()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, String>>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterAgent {
    pub identity_public_key: [u8; crypto::ED25519_PUBLIC_KEY_SIZE],
    pub public_prekey: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
    pub public_prekey_signature: Vec<u8>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentRegistered {
    pub id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPayload {
    pub command: String,
    pub args: Vec<String>,
    pub result_ephemeral_public_key: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub identity_public_key: [u8; crypto::ED25519_PUBLIC_KEY_SIZE],
    pub public_prekey: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
    pub public_prekey_signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsList {
    pub agents: Vec<Agent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentJob {
    pub id: Uuid,
    pub encrypted_job: Vec<u8>,
    pub ephemeral_public_key: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
    pub nonce: [u8; crypto::XCHACHA20_POLY1305_NONCE_SIZE],
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResult {
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateJobResult {
    pub job_id: Uuid,
    pub encrypted_job_result: Vec<u8>,
    pub ephemeral_public_key: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
    pub nonce: [u8; crypto::XCHACHA20_POLY1305_NONCE_SIZE],
    pub signature: Vec<u8>,
}
