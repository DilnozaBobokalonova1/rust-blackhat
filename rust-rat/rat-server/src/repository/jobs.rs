use crate::entities::Job;
use sqlx::{Pool, Postgres};
use log::error;
use crate::Error;
use uuid::Uuid;

use super::Repository;

impl Repository {

    pub async fn create_job(&self, db: &Pool<Postgres>, job: &Job) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO jobs
            (id, created_at, executed_at, command, args, output, agent_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)";
        
        match sqlx::query(QUERY)
            .bind(job.id)
            .bind(job.created_at)
            .bind(job.executed_at)
            .bind(&job.command)
            .bind(&job.args)
            .bind(&job.output)
            .bind(job.agent_id)
            .execute(db)
            .await {
                Ok(_) => Ok(()),
                Err(err) => {
                    error!("created_job: Inserting job: {}", &err);
                    Err(err.into())
                }
            }
    }

    pub async fn update_job(&self, db: &Pool<Postgres>, job: &Job) -> Result<(), Error> {

        const QUERY: &str = "UPDATE jobs
            SET executed_at = $1, output = $2
            WHERE id = $3";

        match sqlx::query(QUERY)
            .bind(job.executed_at)
            .bind(&job.output)
            .bind(job.id)
            .execute(db)
            .await {
                Ok(_) => Ok(()),
                Err(err) => {
                    error!("update_job: updating job: {}", &err);
                    Err(err.into())
                }
            }
    }

    pub async fn find_job_by_id(&self, db: &Pool<Postgres>, job_id: Uuid) -> Result<Job, Error> {
        const QUERY: &str = "SELECT * FROM jobs WHERE id = $1";

        match sqlx::query_as::<Postgres, Job>(QUERY).bind(job_id).fetch_optional(db).await {
            Ok(Some(res)) => Ok(res),
            Ok(None) => Err(Error::NotFound("Job not found.".to_string())),
            Err(err) => {
                error!("find_job_by_id: finding job: {}", &err);
                Err(err.into())
            }
        }
    }

    pub async fn find_job_for_agent(&self, db: &Pool<Postgres>, agent_id: Uuid) -> Result<Job, Error> {

        const QUERY: &str = "SELECT * FROM jobs
            WHERE agent_id = $1 AND output is NULL
            LIMIT 1";

        match sqlx::query_as::<_, Job>(QUERY)
            .bind(agent_id)
            .fetch_optional(db)
            .await {
                Ok(Some(res)) => Ok(res),
                Ok(None) => Err(Error::NotFound("No Job found for Agent.".to_string())),
                Err(err) => {
                    error!("find_job_where_output_is_null: finding job: {}", &err);
                    Err(err.into())
                }
            }
    }

    pub async fn find_all_jobs(&self, db: &Pool<Postgres>) -> Result<Vec<Job>, Error> {
        const QUERY: &str = "SELECT * FROM jobs ORDER BY created_at";

        match sqlx::query_as::<_, Job>(QUERY).fetch_all(db).await {
            Ok(res) => Ok(res),
            Err(err) => {
                error!("find_all_jobs: finding jobs: {}", &err);
                Err(err.into())
            }
        }
    }
}