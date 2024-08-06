use sqlx::{Pool, Postgres};
mod agents;
mod jobs;
use crate::repository::Repository;

pub const ENCRYPTED_JOB_MAX_SIZE: usize = 512_000;
pub const ENCYRPTED_JOB_RESULT_MAX_SIZE: usize = 2_000_000;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: Pool<Postgres>,
    config: config::Config,
}

impl Service {
    pub fn new(&self, repo: &Repository, db: &Pool<Postgres>, config: config::Config) -> Self {
        Service {
            repo: repo,
            db: db,
            config: config,
        }
    }
}
