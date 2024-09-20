use sea_orm::{
    ActiveValue, ColumnTrait, ConnectionTrait, DatabaseConnection, DeleteResult, EntityTrait,
    InsertResult, QueryFilter,
};
use std::error::Error;
use std::sync::Arc;

use crate::agent::domain::message_repository::{CreateMessageParams, MessageRepository};
use crate::entities::message;
use crate::utils::time::current_time_with_timezone;

pub struct SeaOrmMessageRepository {
    connection: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
impl MessageRepository for SeaOrmMessageRepository {
    async fn find(&self, id: i32) -> Result<Option<message::Model>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let r = message::Entity::find_by_id(id).one(conn.as_ref()).await?;

        Ok(r)
    }

    async fn find_by_thread_id(&self, id: i32) -> Result<Vec<message::Model>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let models: Vec<message::Model> = message::Entity::find()
            .filter(message::Column::ThreadId.eq(id))
            .all(conn.as_ref())
            .await?;

        Ok(models)
    }

    async fn create(&self, item: CreateMessageParams) -> Result<message::Model, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let model = self.to_active_model(&item);

        let r = message::Entity::insert(model)
            .exec_with_returning(conn.as_ref())
            .await?;

        Ok(r)
    }

    async fn create_many(&self, messages: Vec<CreateMessageParams>) -> Result<(), Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        self.tx_create_many(conn.as_ref(), messages).await?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
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
    ) -> Result<InsertResult<message::ActiveModel>, Box<dyn Error>>
    where
        C: ConnectionTrait,
    {
        let models: Vec<message::ActiveModel> = messages
            .iter()
            .map(move |m| self.to_active_model(m))
            .collect();

        let insert_result = message::Entity::insert_many(models).exec(conn).await?;

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

impl SeaOrmMessageRepository {
    fn to_active_model(&self, item: &CreateMessageParams) -> message::ActiveModel {
        message::ActiveModel {
            created_at: ActiveValue::Set(current_time_with_timezone()),
            content: ActiveValue::Set(item.content.to_owned()),
            role: ActiveValue::Set(item.role.to_owned()),
            thread_id: ActiveValue::Set(item.thread_id),
            run_id: ActiveValue::Set(item.run_id),
            attachments: ActiveValue::Set(item.attachments.to_owned()),
            metadata: ActiveValue::Set(item.metadata.to_owned()),
            ..Default::default()
        }
    }
}
