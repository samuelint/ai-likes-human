use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, QuerySelect};
use std::error::Error;
use std::sync::Arc;

use crate::agent::domain::run_repository::{ListByPage, NewRunModel, RunRepository};
use crate::entities::run;

pub struct SeaOrmRunRepository {
    connection: Arc<DatabaseConnection>,
}

impl SeaOrmRunRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl RunRepository for SeaOrmRunRepository {
    async fn find(&self, id: i32) -> Result<Option<run::Model>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let r = run::Entity::find_by_id(id).one(conn.as_ref()).await?;

        Ok(r)
    }

    async fn create(&self, item: NewRunModel) -> Result<run::Model, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let model = run::ActiveModel {
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
            .await?;

        Ok(r)
    }

    async fn list_by_page(&self, args: ListByPage) -> Result<Vec<run::Model>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let mut cursor = run::Entity::find().cursor_by(run::Column::Id);
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
        run::Entity::delete_by_id(id).exec(conn.as_ref()).await?;

        Ok(())
    }
}
