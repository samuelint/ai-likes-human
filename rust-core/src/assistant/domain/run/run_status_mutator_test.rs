#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::sync::Arc;

    use crate::assistant::domain::{
        dto::{DbUpdateRunDto, RunDto},
        run::{run_repository::MockRunRepository, run_status_mutator::RunStatusMutator},
    };

    #[tokio::test]
    async fn test_mutating_status_is_presisted() {
        let some_run = RunDto {
            id: "some-id".to_string(),
            status: "queued".to_string(),
            ..RunDto::default()
        };
        let run_repository = create_run_repository_mock();
        let instance = RunStatusMutator::new(Arc::new(run_repository));

        let updated_run_dto = instance.mutate(&some_run, "in-progress").await.unwrap();

        assert_eq!(updated_run_dto.status, "in-progress");
    }

    #[tokio::test]
    async fn test_mutating_to_completed_is_presisted() {
        let some_run = RunDto {
            id: "some-id".to_string(),
            status: "queued".to_string(),
            ..RunDto::default()
        };
        let run_repository = create_run_repository_mock();
        let instance = RunStatusMutator::new(Arc::new(run_repository));

        let updated_run_dto = instance.mutate_to_completed(&some_run).await.unwrap();

        assert_eq!(updated_run_dto.status, "completed");
    }

    #[tokio::test]
    async fn test_mutating_to_in_progress_is_presisted() {
        let some_run = RunDto {
            id: "some-id".to_string(),
            status: "queued".to_string(),
            ..RunDto::default()
        };
        let run_repository = create_run_repository_mock();
        let instance = RunStatusMutator::new(Arc::new(run_repository));

        let updated_run_dto = instance.mutate_to_in_progress(&some_run).await.unwrap();

        assert_eq!(updated_run_dto.status, "in-progress");
    }

    fn create_run_repository_mock() -> MockRunRepository {
        let mut run_repository = MockRunRepository::new();
        run_repository
            .expect_update()
            .withf(|run_id: &str, _dto: &DbUpdateRunDto| run_id == "some-id")
            .returning(|run_id: &str, dto: &DbUpdateRunDto| {
                let status = dto.status.as_ref().unwrap().clone();
                let updated_dto = RunDto {
                    id: run_id.to_string(),
                    status,
                    ..RunDto::default()
                };
                Box::pin(async { Ok(updated_dto) })
            });

        run_repository
    }
}
