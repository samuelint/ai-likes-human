use crate::assistant::domain::dto::{DbCreateRunDto, DbUpdateRunDto, PageRequest, PageResponse, RunDto};
use crate::assistant::domain::run::RunRepository;
use crate::entities::run;
use crate::utils::time::TimeBuilder;

use anyhow::anyhow;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseConnection, DeleteResult,
    EntityTrait, QueryFilter, QuerySelect,
};
use std::error::Error;
use std::num::ParseIntError;
use std::sync::Arc;

use super::metadata::{serialize_metadata, serialize_metadata_opt};

pub struct SeaOrmRunRepository {
    connection: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
impl RunRepository for SeaOrmRunRepository {
    async fn find(&self, id: &str) -> Result<Option<RunDto>, Box<dyn Error>> {
        let id: i32 = id.parse()?;
        let conn = Arc::clone(&self.connection);
        let r = run::Entity::find_by_id(id).one(conn.as_ref()).await?;

        if r.is_none() {
            return Ok(None);
        }

        let r: RunDto = r.unwrap().into();

        Ok(Some(r))
    }

    async fn create(&self, item: DbCreateRunDto) -> Result<RunDto, Box<dyn Error + Send>> {
        let conn: Arc<DatabaseConnection> = Arc::clone(&self.connection);
        let thread_id: i32 = item
            .thread_id
            .parse()
            .map_err(|e: ParseIntError| anyhow!(e))?;

        let model = run::ActiveModel {
            created_at: ActiveValue::Set(TimeBuilder::now().into()),
            assistant_id: ActiveValue::Set(item.assistant_id),
            thread_id: ActiveValue::Set(Some(thread_id)),
            model: ActiveValue::Set(item.model.to_owned()),
            status: ActiveValue::Set(item.status.to_owned()),
            instructions: ActiveValue::Set(item.instructions),
            temperature: ActiveValue::Set(item.temperature),
            metadata: ActiveValue::Set(serialize_metadata_opt(item.metadata)),
            ..Default::default()
        };

        let r = run::Entity::insert(model)
            .exec_with_returning(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        Ok(r.into())
    }

    async fn update(
        &self,
        id: &str,
        update_dto: &DbUpdateRunDto,
    ) -> Result<RunDto, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let id: i32 = id.parse().map_err(|e: ParseIntError| anyhow!(e))?;

        let existing = run::Entity::find_by_id(id)
            .one(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        if existing.is_none() {
            return Err(anyhow!("Message not found").into());
        }

        let mut model: run::ActiveModel = match existing {
            Some(m) => m.into(),
            None => return Err(anyhow!("Message not found").into()),
        };

        match &update_dto.status {
            Some(updated_status) => model.status = ActiveValue::Set(updated_status.clone()),
            None => (),
        }

        match &update_dto.metadata {
            Some(updated_metadata) => {
                model.metadata = ActiveValue::Set(serialize_metadata(updated_metadata))
            }
            None => (),
        }

        let updated_model = model.update(conn.as_ref()).await.map_err(|e| anyhow!(e))?;

        Ok(updated_model.into())
    }

    async fn list_by_thread_paginated(
        &self,
        thread_id: &str,
        page: PageRequest,
    ) -> Result<PageResponse<RunDto>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let mut cursor = run::Entity::find()
            .filter(run::Column::ThreadId.eq(thread_id))
            .cursor_by(run::Column::Id);

        if page.after.is_some() {
            cursor.after(page.after);
        }
        if page.before.is_some() {
            cursor.after(page.after);
        }

        let mut cursor = if let Some(limit) = page.limit {
            cursor.limit(limit)
        } else {
            cursor
        };

        let result = cursor.all(conn.as_ref()).await?;
        let result: Vec<RunDto> = result.iter().map(|r| r.clone().into()).collect();

        Ok(PageResponse {
            first_id: result.first().map(|r| r.id.to_string()).unwrap_or_default(),
            last_id: result.last().map(|r| r.id.to_string()).unwrap_or_default(),
            has_more: result.len() > 0,
            data: result,
        })
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let id: i32 = id.parse()?;
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
