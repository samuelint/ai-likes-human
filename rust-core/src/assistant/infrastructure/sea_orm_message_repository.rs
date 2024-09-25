use anyhow::anyhow;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseConnection, DeleteResult,
    EntityTrait, InsertResult, QueryFilter,
};
use std::error::Error;
use std::num::ParseIntError;
use std::sync::Arc;

use crate::assistant::domain::dto::{ThreadMessageDto, UpdateThreadMessageDto};
use crate::assistant::domain::message_repository::{CreateMessageParams, MessageRepository};
use crate::entities::message;

use super::metadata::to_concrete_metadata;

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

    async fn find_by_thread_id(&self, id: String) -> Result<Vec<ThreadMessageDto>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let models: Vec<message::Model> = message::Entity::find()
            .filter(message::Column::ThreadId.eq(id))
            .all(conn.as_ref())
            .await?;

        let models: Vec<ThreadMessageDto> = models.iter().map(|m| m.clone().into()).collect();
        Ok(models)
    }

    async fn update(
        &self,
        message: UpdateThreadMessageDto,
    ) -> Result<ThreadMessageDto, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let id: i32 = message.id.parse().map_err(|e: ParseIntError| anyhow!(e))?;

        let existing = message::Entity::find_by_id(id)
            .one(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        if existing.is_none() {
            return Err(anyhow!("Message not found").into());
        }

        let mut model: message::ActiveModel = existing.unwrap().into();

        if message.status.is_some() {
            model.status = ActiveValue::Set(message.status.unwrap());
        }

        if message.content.is_some() {
            model.content = ActiveValue::Set(message.content.unwrap());
        }

        if message.assistant_id.is_some() {
            let assistant_id = message.assistant_id.unwrap();

            model.assistant_id = if assistant_id.is_some() {
                let assistant_id = assistant_id
                    .unwrap()
                    .parse()
                    .map_err(|_| anyhow!("Cannot parse assistant id"))?;

                ActiveValue::Set(Some(assistant_id))
            } else {
                ActiveValue::Set(None)
            }
        }

        if message.metadata.is_some() {
            let metadata = message.metadata.unwrap();
            model.metadata = ActiveValue::Set(Some(to_concrete_metadata(metadata)));
        }

        let updated_model = model.update(conn.as_ref()).await.map_err(|e| anyhow!(e))?;

        Ok(updated_model.into())
    }

    async fn create(
        &self,
        item: CreateMessageParams,
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
        messages: Vec<CreateMessageParams>,
    ) -> Result<(), Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        self.tx_create_many(conn.as_ref(), messages).await?;

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
        messages: Vec<CreateMessageParams>,
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
