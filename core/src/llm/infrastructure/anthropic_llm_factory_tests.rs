#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate::eq;

    pub use crate::llm::domain::llm_factory::CreateLLMParameters;
    use crate::llm::{
        domain::{api_key_service::MockApiKeyService, llm_factory::LLMFactory},
        infrastructure::anthropic_llm_factory::AnthropicLLMFactory,
    };

    #[test]
    fn test_model_containing_openai_is_compatible() {
        let api_key_service = Arc::new(MockApiKeyService::new());
        let instance = AnthropicLLMFactory::new(api_key_service);

        assert!(instance.is_compatible("anthropic"));
    }

    #[test]
    fn test_model_containing_openai_in_uppercase_is_compatible() {
        let api_key_service = Arc::new(MockApiKeyService::new());
        let instance = AnthropicLLMFactory::new(api_key_service);

        assert!(instance.is_compatible("ANTHROPIC"));
    }

    #[test]
    fn test_model_containing_openai_and_model_is_compatible() {
        let api_key_service = Arc::new(MockApiKeyService::new());
        let instance = AnthropicLLMFactory::new(api_key_service);

        assert!(instance.is_compatible("anthropic:claude"));
    }

    #[test]
    fn test_model_not_containing_openai_is_not_compatible() {
        let api_key_service = Arc::new(MockApiKeyService::new());
        let instance = AnthropicLLMFactory::new(api_key_service);

        assert!(!instance.is_compatible("openai"));
    }

    #[tokio::test]
    async fn test_create_with_semicolon_is_created() {
        let mut api_key_service = MockApiKeyService::new();
        api_key_service
            .expect_get_api_key()
            .with(eq("ANTHROPIC_API_KEY"))
            .returning(|_| Ok("ABC".to_string()));
        let instance = AnthropicLLMFactory::new(Arc::new(api_key_service));
        let result = instance
            .create(&CreateLLMParameters {
                model: "anthropic:gpt-4o".to_string(),
                ..CreateLLMParameters::default()
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_with_model_is_created() {
        let mut api_key_service = MockApiKeyService::new();
        api_key_service
            .expect_get_api_key()
            .with(eq("ANTHROPIC_API_KEY"))
            .returning(|_| Ok("ABC".to_string()));
        let instance = AnthropicLLMFactory::new(Arc::new(api_key_service));
        let result = instance
            .create(&CreateLLMParameters {
                model: "gpt-4o".to_string(),
                ..CreateLLMParameters::default()
            })
            .await;

        assert!(result.is_ok());
    }
}
