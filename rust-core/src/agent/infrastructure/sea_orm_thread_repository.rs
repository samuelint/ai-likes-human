use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, QuerySelect};
use std::error::Error;
use std::sync::Arc;

use crate::agent::domain::thread_repository::{ListByPage, NewThreadModel, ThreadRepository};
use crate::entities::thread;

pub struct SeaOrmThreadRepository {
    connection: Arc<DatabaseConnection>,
}

impl SeaOrmThreadRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl ThreadRepository for SeaOrmThreadRepository {
    async fn find(&self, id: i32) -> Result<Option<thread::Model>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let r = thread::Entity::find_by_id(id).one(conn.as_ref()).await?;

        Ok(r)
    }

    async fn create(&self, item: NewThreadModel) -> Result<thread::Model, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let model = thread::ActiveModel {
            metadata: ActiveValue::Set(item.metadata.to_owned()),
            ..Default::default()
        };

        let r = thread::Entity::insert(model)
            .exec_with_returning(conn.as_ref())
            .await?;

        Ok(r)
    }

    async fn list_by_page(&self, args: ListByPage) -> Result<Vec<thread::Model>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let mut cursor = thread::Entity::find().cursor_by(thread::Column::Id);
        cursor.after(args.after).before(args.before);

        let mut cursor = if let Some(limit) = args.limit {
            cursor.limit(limit)
        } else {
            cursor
        };

        let result = cursor.all(conn.as_ref()).await?;

        Ok(result)
    }

    async fn delete(&self, id: i32) -> Result<(), Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        thread::Entity::delete_by_id(id).exec(conn.as_ref()).await?;

        Ok(())
    }
}
