#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::sync::Arc;

    use crate::assistant::domain::{
        dto::{DbUpdateThreadMessageDto, ThreadMessageDto},
        message::{
            message_repository::MockMessageRepository, message_status_mutator::MessageStatusMutator,
        },
    };

    #[tokio::test]
    async fn test_mutating_status_is_presisted() {
        let some_message = ThreadMessageDto {
            id: "some-id".to_string(),
            status: "queued".to_string(),
            ..ThreadMessageDto::default()
        };
        let run_repository = create_run_repository_mock();
        let instance = MessageStatusMutator::new(Arc::new(run_repository));

        let updated_run_dto = instance.mutate(&some_message, "in-progress").await.unwrap();

        assert_eq!(updated_run_dto.status, "in-progress");
    }

    #[tokio::test]
    async fn test_mutating_to_completed_is_presisted() {
        let some_run = ThreadMessageDto {
            id: "some-id".to_string(),
            status: "queued".to_string(),
            ..ThreadMessageDto::default()
        };
        let run_repository = create_run_repository_mock();
        let instance = MessageStatusMutator::new(Arc::new(run_repository));

        let updated_run_dto = instance.mutate_to_completed(&some_run).await.unwrap();

        assert_eq!(updated_run_dto.status, "completed");
    }

    fn create_run_repository_mock() -> MockMessageRepository {
        let mut repository = MockMessageRepository::new();
        repository
            .expect_update()
            .withf(|run_id: &str, _dto: &DbUpdateThreadMessageDto| run_id == "some-id")
            .returning(|run_id: &str, dto: &DbUpdateThreadMessageDto| {
                let status = dto.status.as_ref().unwrap().clone();
                let updated_dto = ThreadMessageDto {
                    id: run_id.to_string(),
                    status,
                    ..ThreadMessageDto::default()
                };
                Box::pin(async { Ok(updated_dto) })
            });

        repository
    }
}
