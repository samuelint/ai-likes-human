use anyhow::anyhow;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseConnection, DeleteResult,
    EntityTrait, InsertResult, QueryFilter, QuerySelect,
};
use std::error::Error;
use std::num::ParseIntError;
use std::sync::Arc;

use crate::assistant::domain::dto::{
    DbCreateThreadMessageDto, DbUpdateThreadMessageDto, PageRequest, PageResponse, ThreadMessageDto,
};
use crate::assistant::domain::message::MessageRepository;
use crate::entities::message;

pub struct SeaOrmMessageRepository {
    connection: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
impl MessageRepository for SeaOrmMessageRepository {
    async fn find(&self, id: String) -> Result<Option<ThreadMessageDto>, Box<dyn Error>> {
        let id: i32 = id.parse()?;
        let conn = Arc::clone(&self.connection);
        let r = message::Entity::find_by_id(id).one(conn.as_ref()).await?;

        if r.is_none() {
            return Ok(None);
        }

        let r = r.unwrap();
        let r: ThreadMessageDto = r.into();

        Ok(Some(r))
    }

    async fn list_by_thread_id_paginated(
        &self,
        thread_id: &str,
        page: &PageRequest,
    ) -> Result<PageResponse<ThreadMessageDto>, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let mut cursor = message::Entity::find()
            .filter(message::Column::ThreadId.eq(thread_id))
            .cursor_by(message::Column::Id);

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

        let result = cursor.all(conn.as_ref()).await.map_err(|e| anyhow!(e))?;
        let result: Vec<ThreadMessageDto> = result.iter().map(|r| r.clone().into()).collect();

        Ok(PageResponse {
            first_id: result.first().map(|r| r.id.to_string()).unwrap_or_default(),
            last_id: result.last().map(|r| r.id.to_string()).unwrap_or_default(),
            has_more: result.len() > 0,
            data: result,
        })
    }

    async fn update(
        &self,
        id: &str,
        update_dto: &DbUpdateThreadMessageDto,
    ) -> Result<ThreadMessageDto, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let id: i32 = id.parse().map_err(|e: ParseIntError| anyhow!(e))?;

        let existing = message::Entity::find_by_id(id)
            .one(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        if existing.is_none() {
            return Err(anyhow!("Message not found").into());
        }

        let mut model: message::ActiveModel = match existing {
            Some(m) => m.into(),
            None => return Err(anyhow!("Message not found").into()),
        };

        match &update_dto.status {
            Some(status) => {
                model.status = ActiveValue::Set(status.clone());
            }
            None => {}
        };

        match &update_dto.content {
            Some(new_content) => {
                let json_content = serde_json::to_string(&new_content).unwrap();
                model.content = ActiveValue::Set(json_content);
            }
            None => {}
        };

        match &update_dto.assistant_id {
            Some(new_assistant_id) => {
                model.assistant_id = match new_assistant_id {
                    Some(new_assistant_id) => {
                        let assistant_id = new_assistant_id
                            .parse()
                            .map_err(|_| anyhow!("Cannot parse assistant id"))?;
                        ActiveValue::Set(Some(assistant_id))
                    }
                    None => ActiveValue::Set(None),
                }
            }
            None => {}
        };

        match &update_dto.metadata {
            Some(new_metadata) => {
                let json_metadata = serde_json::to_string(&new_metadata).unwrap();
                model.metadata = ActiveValue::Set(json_metadata);
            }
            None => {}
        };

        let updated_model = model.update(conn.as_ref()).await.map_err(|e| anyhow!(e))?;

        Ok(updated_model.into())
    }

    async fn create(
        &self,
        item: DbCreateThreadMessageDto,
    ) -> Result<ThreadMessageDto, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let model: message::ActiveModel = (&item).into();

        let r = message::Entity::insert(model)
            .exec_with_returning(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        Ok(r.into())
    }

    async fn create_many(
        &self,
        messages: Vec<DbCreateThreadMessageDto>,
    ) -> Result<(), Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        self.tx_create_many(conn.as_ref(), &messages).await?;

        Ok(())
    }

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>> {
        let id: i32 = id.parse()?;
        let conn = Arc::clone(&self.connection);
        message::Entity::delete_by_id(id)
            .exec(conn.as_ref())
            .await?;

        Ok(())
    }
}

impl SeaOrmMessageRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    pub async fn tx_create_many<'a, C>(
        &self,
        conn: &'a C,
        messages: &Vec<DbCreateThreadMessageDto>,
    ) -> Result<InsertResult<message::ActiveModel>, Box<dyn Error + Send>>
    where
        C: ConnectionTrait,
    {
        let models: Vec<message::ActiveModel> = messages.iter().map(move |m| m.into()).collect();

        let insert_result = message::Entity::insert_many(models)
            .exec(conn)
            .await
            .map_err(|e| anyhow!(e))?;

        Ok(insert_result)
    }

    pub async fn tx_delete_by_thread_id<'a, C>(
        &self,
        conn: &'a C,
        id: i32,
    ) -> Result<DeleteResult, Box<dyn Error>>
    where
        C: ConnectionTrait,
    {
        let result = message::Entity::delete_many()
            .filter(message::Column::ThreadId.eq(id))
            .exec(conn)
            .await?;

        Ok(result)
    }
}
