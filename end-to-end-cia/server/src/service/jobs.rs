use super::Service;
use crate::Error;
use crate::{entities, Repository};
use common::api;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

impl Service {
    pub async fn register_agent(
        &self,
        agent_data: api::RegisterAgent,
    ) -> Result<entities::Agent, Error> {

    }
    
    pub async fn find_agent(&self, agent_id: Uuid) -> Result<entities::Agent, Error> {
        self.repo.find_agent_by_id(&self.db, agent_id).await
    }
    
    pub async fn list_agents(&self) -> Result<Vec<entities::Agent>, Error> {
        self.repo.find_all_agents(&self.db).await
    }

}
