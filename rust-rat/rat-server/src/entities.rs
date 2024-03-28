use chrono::{DateTime, Utc};
use common::api;
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
    pub command: String,
    pub args: Json<Vec<String>>,
    pub output: Option<String>,

    pub agent_id: Uuid,
}

