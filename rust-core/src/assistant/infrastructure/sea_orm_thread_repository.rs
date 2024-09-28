use crate::assistant::domain::dto::{
    DbCreateThreadDto, DbCreateThreadMessageDto, DbUpdateThreadDto, PageRequest, PageResponse,
    ThreadDto,
};
use crate::assistant::domain::thread::ThreadRepository;
use crate::entities::thread;
use crate::utils::time::TimeBuilder;
use anyhow::anyhow;
use sea_orm::{
    ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, QuerySelect, TransactionTrait,
};
use std::error::Error;
use std::sync::Arc;

use super::metadata::serialize_metadata_opt;
use super::page_request_adapter::DbPageRequest;
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
        let id: i32 = id.parse()?;
        let r = thread::Entity::find_by_id(id)
            .one(self.connection.as_ref())
            .await?;

        if r.is_none() {
            return Ok(None);
        }
        let r: ThreadDto = r.unwrap().into();

        Ok(Some(r))
    }

    async fn create(
        &self,
        new_thread: DbCreateThreadDto,
    ) -> Result<ThreadDto, Box<dyn Error + Send>> {
        let txn = self.connection.begin().await.map_err(|e| anyhow!(e))?;

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
            let messages: Vec<DbCreateThreadMessageDto> = new_thread
                .messages
                .iter()
                .map(|mesg| DbCreateThreadMessageDto {
                    content: mesg.content.clone(),
                    role: mesg.role.clone(),
                    thread_id: Some(model.id.to_string()),
                    ..DbCreateThreadMessageDto::default()
                })
                .collect();

            self.message_repository
                .tx_create_many(&txn, &messages)
                .await?;
        }

        txn.commit().await.map_err(|e| anyhow!(e))?;

        Ok(model.into())
    }

    async fn update(&self, thread: DbUpdateThreadDto) -> Result<ThreadDto, Box<dyn Error>> {
        let thread_id: i32 = thread.id.parse()?;

        let existing = thread::Entity::find_by_id(thread_id)
            .one(self.connection.as_ref())
            .await?;

        if existing.is_none() {
            return Err(format!("Thread {} not found", thread.id).into());
        }

        let mut model: thread::ActiveModel = existing.unwrap().into();
        model.metadata = ActiveValue::Set(serialize_metadata_opt(thread.metadata));

        let updated_model = model.update(self.connection.as_ref()).await?;

        Ok(updated_model.into())
    }

    async fn list_by_page(
        &self,
        page: &PageRequest,
    ) -> Result<PageResponse<ThreadDto>, Box<dyn Error>> {
        let mut cursor = thread::Entity::find().cursor_by(thread::Column::Id);

        let db_page_request: DbPageRequest = page.into();
        if db_page_request.after.is_some() {
            cursor.after(db_page_request.after.unwrap());
        };
        if db_page_request.before.is_some() {
            cursor.before(db_page_request.before.unwrap());
        };
        let mut cursor = if let Some(limit) = db_page_request.limit {
            cursor.limit(limit as u64)
        } else {
            cursor
        };

        let result = cursor.all(self.connection.as_ref()).await?;
        let result: Vec<ThreadDto> = result.iter().map(|r| r.clone().into()).collect();

        Ok(PageResponse {
            first_id: result.first().map(|r| r.id.to_string()).unwrap_or_default(),
            last_id: result.last().map(|r| r.id.to_string()).unwrap_or_default(),
            has_more: result.len() > 0,
            data: result,
        })
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let id = id.parse()?;

        let txn = self.connection.begin().await?;

        thread::Entity::delete_by_id(id).exec(&txn).await?;
        self.message_repository
            .tx_delete_by_thread_id(&txn, id)
            .await?;
        self.run_repository.tx_delete_by_thread_id(&txn, id).await?;

        txn.commit().await?;

        Ok(())
    }
}
