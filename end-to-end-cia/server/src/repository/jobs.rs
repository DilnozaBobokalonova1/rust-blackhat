use super::Repository;
use crate::{entities::Job, Error};
use common::api::Job;
use log::error;
use sqlx::{Pool, Postgres};

impl Repository {
    pub async fn create_job(&self, db: &Pool<Postgres>, job: &Job) -> Result<(), Error>;
}
