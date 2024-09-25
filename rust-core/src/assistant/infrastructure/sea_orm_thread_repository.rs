use crate::assistant::domain::dto::ThreadDto;
use crate::assistant::domain::thread_repository::{
    CreateThreadParams, ThreadRepository, UpdateThreadParams,
};
use crate::assistant::domain::CreateMessageParams;
use crate::entities::thread;
use crate::utils::time::current_time_with_timezone;
use crate::utils::PageRequest;
use anyhow::anyhow;
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

    fn to_concrete_metadata(&self, metadata: Option<String>) -> String {
        metadata
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or("{}".to_string())
    }
}

#[async_trait::async_trait]
impl ThreadRepository for SeaOrmThreadRepository {
    async fn find(&self, id: String) -> Result<Option<ThreadDto>, Box<dyn Error>> {
        let conn = Arc::clone(&self.connection);
        let id: i32 = id.parse()?;
        let r = thread::Entity::find_by_id(id).one(conn.as_ref()).await?;

        if r.is_none() {
            return Ok(None);
        }
        let r: ThreadDto = r.unwrap().into();

        Ok(Some(r))
    }

    async fn create(
        &self,
        new_thread: CreateThreadParams,
    ) -> Result<ThreadDto, Box<dyn Error + Send>> {
        let conn = Arc::clone(&self.connection);

        let txn = conn.begin().await.map_err(|e| anyhow!(e))?;

        let model = thread::ActiveModel {
            metadata: ActiveValue::Set(self.to_concrete_metadata(new_thread.metadata)),
            created_at: ActiveValue::Set(current_time_with_timezone()),
            ..Default::default()
        };

        let model: thread::Model = thread::Entity::insert(model)
            .exec_with_returning(&txn)
            .await
            .map_err(|e| anyhow!(e))?;

        if new_thread.messages.len() > 0 {
            let messages: Vec<CreateMessageParams> = new_thread
                .messages
                .iter()
                .map(|mesg| CreateMessageParams {
                    content: mesg.content.clone(),
                    role: mesg.role.clone(),
                    thread_id: Some(model.id.to_string()),
                    run_id: None,
                    attachments: None,
                    metadata: None,
                })
                .collect();

            self.message_repository
                .tx_create_many(&txn, messages)
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
        model.metadata = ActiveValue::Set(self.to_concrete_metadata(thread.metadata));

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

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>> {
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
