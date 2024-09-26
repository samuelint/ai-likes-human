use crate::assistant::domain::dto::{CreateThreadMessageDto, ThreadDto, ThreadMessageDto};
use crate::assistant::domain::thread_repository::{
    CreateThreadParams, ThreadRepository, UpdateThreadParams,
};
use crate::entities::{message, thread};
use crate::utils::time::TimeBuilder;
use crate::utils::PageRequest;
use anyhow::anyhow;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QuerySelect, TransactionTrait,
};
use std::error::Error;
use std::sync::Arc;

use super::metadata::serialize_metadata_opt;
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
    async fn find(&self, id: &str) -> Result<Option<ThreadDto>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let id: i32 = id.parse()?;
        let r = thread::Entity::find_by_id(id).one(conn.as_ref()).await?;

        if r.is_none() {
            return Ok(None);
        }
        let r: ThreadDto = r.unwrap().into();

        Ok(Some(r))
    }

    async fn find_messages(
        &self,
        id: &str,
    ) -> Result<Vec<ThreadMessageDto>, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);
        let models: Vec<message::Model> = message::Entity::find()
            .filter(message::Column::ThreadId.eq(id))
            .all(conn.as_ref())
            .await
            .map_err(|e| anyhow!(e))?;

        let models: Vec<ThreadMessageDto> =
            models.iter().map(|model| model.clone().into()).collect();

        Ok(models)
    }

    async fn create(
        &self,
        new_thread: CreateThreadParams,
    ) -> Result<ThreadDto, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);

        let txn = conn.begin().await.map_err(|e| anyhow!(e))?;

        let model = thread::ActiveModel {
            metadata: ActiveValue::Set(serialize_metadata_opt(new_thread.metadata)),
            created_at: ActiveValue::Set(TimeBuilder::now().into()),
            ..Default::default()
        };

        let model: thread::Model = thread::Entity::insert(model)
            .exec_with_returning(&txn)
            .await
            .map_err(|e| anyhow!(e))?;

        if new_thread.messages.len() > 0 {
            let messages: Vec<CreateThreadMessageDto> = new_thread
                .messages
                .iter()
                .map(|mesg| CreateThreadMessageDto {
                    content: mesg.content.clone(),
                    role: mesg.role.clone(),
                    thread_id: Some(model.id.to_string()),
                    ..CreateThreadMessageDto::default()
                })
                .collect();

            self.message_repository
                .tx_create_many(&txn, &messages)
                .await?;
        }

        txn.commit().await.map_err(|e| anyhow!(e))?;

        Ok(model.into())
    }

    async fn update(&self, thread: UpdateThreadParams) -> Result<ThreadDto, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let thread_id: i32 = thread.id.parse()?;

        let existing = thread::Entity::find_by_id(thread_id)
            .one(conn.as_ref())
            .await?;

        if existing.is_none() {
            return Err(format!("Thread {} not found", thread.id).into());
        }

        let mut model: thread::ActiveModel = existing.unwrap().into();
        model.metadata = ActiveValue::Set(serialize_metadata_opt(thread.metadata));

        let updated_model = model.update(conn.as_ref()).await?;

        Ok(updated_model.into())
    }

    async fn list_by_page(&self, args: PageRequest) -> Result<Vec<ThreadDto>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let mut cursor = thread::Entity::find().cursor_by(thread::Column::Id);

        if args.after.is_some() {
            cursor.after(args.after);
        }
        if args.before.is_some() {
            cursor.after(args.after);
        }

        let mut cursor = if let Some(limit) = args.limit {
            cursor.limit(limit)
        } else {
            cursor
        };

        let result = cursor.all(conn.as_ref()).await?;

        Ok(result.iter().map(|r| r.clone().into()).collect())
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let id = id.parse()?;
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
