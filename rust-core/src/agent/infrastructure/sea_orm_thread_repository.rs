use crate::agent::domain::thread_repository::{CreateThreadDto, ThreadRepository, UpdateThreadDto};
use crate::entities::thread;
use crate::utils::PageRequest;
use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, QuerySelect};
use std::error::Error;
use std::sync::Arc;

use super::{SeaOrmMessageRepository, SeaOrmRunRepository};

pub struct SeaOrmThreadRepository {
    connection: Arc<DatabaseConnection>,
    message_repository: Arc<SeaOrmMessageRepository>,
    run_repository: Arc<SeaOrmRunRepository>,
}

impl SeaOrmThreadRepository {
    pub fn new(
        connection: Arc<DatabaseConnection>,
        message_repository: Arc<SeaOrmMessageRepository>,
        run_repository: Arc<SeaOrmRunRepository>,
    ) -> Self {
        Self {
            connection,
            message_repository,
            run_repository,
        }
    }
}

#[async_trait::async_trait]
impl ThreadRepository for SeaOrmThreadRepository {
    async fn find(&self, id: i32) -> Result<Option<thread::Model>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let r = thread::Entity::find_by_id(id).one(conn.as_ref()).await?;

        Ok(r)
    }

    async fn create(&self, new_thread: CreateThreadDto) -> Result<thread::Model, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);

        let txn = conn.begin().await?;

        let model = thread::ActiveModel {
            metadata: ActiveValue::Set(new_thread.metadata.to_owned()),
            ..Default::default()
        };

        let model = thread::Entity::insert(model)
            .exec_with_returning(&txn)
            .await?;

        if new_thread.messages.len() > 0 {
            self.message_repository
                .tx_create_many(&txn, new_thread.messages)
                .await?;
        }

        txn.commit().await?;

        Ok(model)
    }

    async fn update(&self, thread: UpdateThreadDto) -> Result<thread::Model, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);

        let existing = thread::Entity::find_by_id(thread.id)
            .one(conn.as_ref())
            .await?;

        if existing.is_none() {
            return Err(format!("Thread {} not found", thread.id).into());
        }

        let mut model: thread::ActiveModel = existing.unwrap().into();
        model.metadata = ActiveValue::Set(thread.metadata.to_owned());

        let updated_model = model.update(conn.as_ref()).await?;

        Ok(updated_model)
    }

    async fn list_by_page(&self, args: PageRequest) -> Result<Vec<thread::Model>, Box<dyn Error>> {
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

        let txn = conn.begin().await?;

        thread::Entity::delete_by_id(id).exec(&txn).await?;
        self.message_repository
            .tx_delete_by_thread_id(&txn, id)
            .await?;
        self.run_repository.tx_delete_by_thread_id(&txn, id).await?;

        txn.commit().await?;

        Ok(())
    }
}
