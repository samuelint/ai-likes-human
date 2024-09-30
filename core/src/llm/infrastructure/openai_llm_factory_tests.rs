#[cfg(test)]
mod tests {
    pub use crate::llm::domain::llm_factory::CreateLLMParameters;
    use crate::llm::{
        domain::llm_factory::LLMFactory, infrastructure::openai_llm_factory::OpenAILLMFactory,
    };

    #[test]
    fn test_model_containing_openai_is_compatible() {
        let instance = OpenAILLMFactory {};

        assert!(instance.is_compatible("openai"));
    }

    #[test]
    fn test_model_containing_openai_in_uppercase_is_compatible() {
        let instance = OpenAILLMFactory {};

        assert!(instance.is_compatible("OPENAI"));
    }

    #[test]
    fn test_model_containing_openai_and_model_is_compatible() {
        let instance = OpenAILLMFactory {};

        assert!(instance.is_compatible("openai:gpt4-o"));
    }

    #[test]
    fn test_model_not_containing_openai_is_not_compatible() {
        let instance = OpenAILLMFactory {};

        assert!(!instance.is_compatible("anthropic"));
    }

    #[test]
    fn test_create_with_semicolon_is_created() {
        let instance = OpenAILLMFactory {};
        let result = instance.create(CreateLLMParameters {
            model: "openai:gpt-4o".to_string(),
        });

        assert!(result.is_ok());
    }

    #[test]
    fn test_create_only_with_model_is_created() {
        let instance = OpenAILLMFactory {};
        let result = instance.create(CreateLLMParameters {
            model: "gpt-4o".to_string(),
        });

        assert!(result.is_ok());
    }
}
