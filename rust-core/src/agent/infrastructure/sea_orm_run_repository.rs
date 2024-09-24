use crate::agent::domain::run_repository::{CreateRunParams, RunRepository};
use crate::entities::run;
use crate::utils::time::current_time_with_timezone;
use crate::utils::PageRequest;
use anyhow::anyhow;
use sea_orm::{
    ActiveValue, ColumnTrait, ConnectionTrait, DatabaseConnection, DeleteResult, EntityTrait,
    QueryFilter, QuerySelect,
};
use std::error::Error;
use std::sync::Arc;

pub struct SeaOrmRunRepository {
    connection: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
impl RunRepository for SeaOrmRunRepository {
    async fn find(&self, id: i32) -> Result<Option<run::Model>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let r = run::Entity::find_by_id(id).one(conn.as_ref()).await?;

        Ok(r)
    }

    async fn create(&self, item: CreateRunParams) -> Result<run::Model, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let model = run::ActiveModel {
            created_at: ActiveValue::Set(current_time_with_timezone()),
            assistant_id: ActiveValue::Set(item.assistant_id),
            thread_id: ActiveValue::Set(Some(item.thread_id)),
            model: ActiveValue::Set(item.model.to_owned()),
            status: ActiveValue::Set(item.status.to_owned()),
            instructions: ActiveValue::Set(item.instructions),
            temperature: ActiveValue::Set(item.temperature),
            metadata: ActiveValue::Set(item.metadata.to_owned()),
            ..Default::default()
        };

        let r = run::Entity::insert(model)
            .exec_with_returning(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        Ok(r)
    }

    async fn list_by_thread_paginated(
        &self,
        thread_id: i32,
        page: PageRequest,
    ) -> Result<Vec<run::Model>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let mut cursor = run::Entity::find()
            .filter(run::Column::ThreadId.eq(thread_id))
            .cursor_by(run::Column::Id);
        cursor.after(page.after).before(page.before);

        let mut cursor = if let Some(limit) = page.limit {
            cursor.limit(limit)
        } else {
            cursor
        };

        let result = cursor.all(conn.as_ref()).await?;

        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        run::Entity::delete_by_id(id).exec(conn.as_ref()).await?;

        Ok(())
    }
}

impl SeaOrmRunRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    pub async fn tx_delete_by_thread_id<'a, C>(
        &self,
        conn: &'a C,
        id: i32,
    ) -> Result<DeleteResult, Box<dyn Error>>
    where
        C: ConnectionTrait,
    {
        let result = run::Entity::delete_many()
            .filter(run::Column::ThreadId.eq(id))
            .exec(conn)
            .await?;

        Ok(result)
    }
}
