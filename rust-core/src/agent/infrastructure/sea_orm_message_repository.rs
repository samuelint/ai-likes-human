use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use std::error::Error;
use std::sync::Arc;

use crate::agent::domain::message_repository::{MessageRepository, NewMessageModel};
use crate::entities::message;

pub struct SeaOrmMessageRepository {
    connection: Arc<DatabaseConnection>,
}

impl SeaOrmMessageRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl MessageRepository for SeaOrmMessageRepository {
    async fn find(&self, id: i32) -> Result<Option<message::Model>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let r = message::Entity::find_by_id(id).one(conn.as_ref()).await?;

        Ok(r)
    }

    async fn create(&self, item: NewMessageModel) -> Result<message::Model, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let model = message::ActiveModel {
            content: ActiveValue::Set(item.content.to_owned()),
            role: ActiveValue::Set(item.role.to_owned()),
            attachments: ActiveValue::Set(item.attachments.to_owned()),
            metadata: ActiveValue::Set(item.metadata.to_owned()),
            ..Default::default()
        };

        let r = message::Entity::insert(model)
            .exec_with_returning(conn.as_ref())
            .await?;

        Ok(r)
    }

    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        message::Entity::delete_by_id(id)
            .exec(conn.as_ref())
            .await?;

        Ok(())
    }
}
