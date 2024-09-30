#[cfg(test)]
mod test_get_api_key {

    use std::{env, sync::Arc};

    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;

    use crate::{
        configuration::{domain::repository::MockConfigurationRepository, ConfigurationDto},
        llm::domain::api_key_service::{ApiKeyService, ApiKeyServiceImpl},
    };

    #[tokio::test]
    async fn test_get_api_key_from_repository() {
        let mut configuration_repository = MockConfigurationRepository::new();
        configuration_repository
            .expect_find_by_key()
            .with(eq("SOME_KEY"))
            .returning(|_| {
                Ok(Some(ConfigurationDto {
                    key: "SOME_KEY".to_string(),
                    value: "SOME_VALUE".to_string(),
                }))
            });
        let instance = ApiKeyServiceImpl::new(Arc::new(configuration_repository));

        let found_key = instance.get_api_key("SOME_KEY").await.unwrap();

        assert_eq!(found_key, "SOME_VALUE");
    }

    #[tokio::test]
    async fn test_get_api_key_not_in_repository_is_from_env() {
        let mut configuration_repository = MockConfigurationRepository::new();
        configuration_repository
            .expect_find_by_key()
            .with(eq("SOME_KEY"))
            .returning(|_| Ok(None));
        env::set_var("SOME_KEY", "SOME");
        let instance = ApiKeyServiceImpl::new(Arc::new(configuration_repository));

        let found_key = instance.get_api_key("SOME_KEY").await.unwrap();

        assert_eq!(found_key, "SOME");
    }

    #[tokio::test]
    async fn test_get_api_key_is_in_repository_with_empty_string_is_from_env() {
        let mut configuration_repository = MockConfigurationRepository::new();
        configuration_repository
            .expect_find_by_key()
            .with(eq("SOME_KEY"))
            .returning(|_| {
                Ok(Some(ConfigurationDto {
                    key: "SOME_KEY".to_string(),
                    value: "".to_string(),
                }))
            });
        env::set_var("SOME_KEY", "SOME");
        let instance = ApiKeyServiceImpl::new(Arc::new(configuration_repository));

        let found_key = instance.get_api_key("SOME_KEY").await.unwrap();

        assert_eq!(found_key, "SOME");
    }

    #[tokio::test]
    async fn test_get_api_key_not_in_repository_or_env_raise_an_error() {
        let mut configuration_repository = MockConfigurationRepository::new();
        configuration_repository
            .expect_find_by_key()
            .with(eq("SOME_OTHER_KEY"))
            .returning(|_| Ok(None));
        let instance = ApiKeyServiceImpl::new(Arc::new(configuration_repository));

        let result = instance.get_api_key("SOME_OTHER_KEY").await;

        assert!(result.is_err());
    }
}
