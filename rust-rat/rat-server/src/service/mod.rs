use crate::repository;
use sqlx::{Pool, Postgres};
use repository::Repository;
mod agents;
mod jobs;


#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: Pool<Postgres>,
}

impl Service {
    pub fn new(db: Pool<Postgres>) -> Service {
        let repo = Repository {};
        Service { repo, db}
    }
}