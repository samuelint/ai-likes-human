#[cfg(test)]
#[path = "./message_status_mutator_test.rs"]
mod message_status_mutator_test;

use std::sync::Arc;

use crate::assistant::domain::dto::{DbUpdateThreadMessageDto, ThreadMessageDto};

use super::MessageRepository;

pub struct MessageStatusMutator {
    message_repository: Arc<dyn MessageRepository>,
}

impl MessageStatusMutator {
    pub fn new(message_repository: Arc<dyn MessageRepository>) -> Self {
        Self { message_repository }
    }

    pub async fn mutate(
        &self,
        message: &ThreadMessageDto,
        status: &str,
    ) -> Result<ThreadMessageDto, Box<dyn std::error::Error + Send>> {
        self.message_repository
            .update(
                &message.id,
                &DbUpdateThreadMessageDto {
                    status: Some(status.to_string()),
                    ..DbUpdateThreadMessageDto::default()
                },
            )
            .await
    }

    pub async fn mutate_to_completed(
        &self,
        run: &ThreadMessageDto,
    ) -> Result<ThreadMessageDto, Box<dyn std::error::Error + Send>> {
        self.mutate(run, "completed").await
    }
}
