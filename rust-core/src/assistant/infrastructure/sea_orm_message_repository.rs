use sea_orm::{ActiveValue, EntityTrait};
use shaku::Provider;
use std::error::Error;
use std::sync::Arc;

use crate::assistant::domain::message_repository::{MessageRepository, NewMessageModel};
use crate::entities::message;
use crate::infrastructure::sea_orm::connection_provider::ConnectionProvider;

#[derive(Provider)]
#[shaku(interface = MessageRepository)]
pub struct SeaOrmMessageRepository {
    #[shaku(inject)]
    connection: Arc<dyn ConnectionProvider>,
}

#[async_trait::async_trait]
impl MessageRepository for SeaOrmMessageRepository {
    async fn find(&self, id: i32) -> Result<Option<message::Model>, Box<dyn Error>> {
        let conn = self.connection.get();
        let r = message::Entity::find_by_id(id).one(conn.as_ref()).await?;

        Ok(r)
    }

    async fn create(&self, item: NewMessageModel) -> Result<message::Model, Box<dyn Error>> {
        let conn = self.connection.get();
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
        let conn = self.connection.get();
        message::Entity::delete_by_id(id)
            .exec(conn.as_ref())
            .await?;

        Ok(())
    }
}
