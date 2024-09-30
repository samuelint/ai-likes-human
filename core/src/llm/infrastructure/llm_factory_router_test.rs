#[cfg(test)]
mod test_is_compatible {
    use std::sync::Arc;

    use crate::llm::{
        domain::llm_factory::{LLMFactory, MockLLMFactory},
        infrastructure::llm_factory_router::LLMFactoryRouter,
    };
    use mockall::*;

    #[test]
    fn test_is_compatible_when_one_of_factories_is_compatible() {
        let mut factory1 = MockLLMFactory::new();
        let mut factory2 = MockLLMFactory::new();
        factory1
            .expect_is_compatible()
            .with(predicate::eq("openai:gpt-4o"))
            .returning(|_| false);
        factory2
            .expect_is_compatible()
            .with(predicate::eq("openai:gpt-4o"))
            .returning(|_| true);

        let instance = LLMFactoryRouter {
            llm_factories: vec![Arc::new(factory1), Arc::new(factory2)],
        };

        assert!(instance.is_compatible("openai:gpt-4o"));
    }

    #[test]
    fn test_is_not_compatible_when_none_of_factories_are_compatible() {
        let mut factory1 = MockLLMFactory::new();
        let mut factory2 = MockLLMFactory::new();
        factory1.expect_is_compatible().returning(|_| false);
        factory2.expect_is_compatible().returning(|_| false);

        let instance = LLMFactoryRouter {
            llm_factories: vec![Arc::new(factory1), Arc::new(factory2)],
        };

        assert!(!instance.is_compatible("openai:gpt-4o"));
    }

    #[test]
    fn test_is_not_compatible_when_theres_no_factories() {
        let instance = LLMFactoryRouter {
            llm_factories: vec![],
        };

        assert!(!instance.is_compatible("openai:gpt-4o"));
    }
}

#[cfg(test)]
mod test_create {
    use std::{pin::Pin, sync::Arc};

    use crate::llm::{
        domain::llm_factory::{CreateLLMParameters, LLMFactory, MockLLMFactory},
        infrastructure::llm_factory_router::LLMFactoryRouter,
    };
    use futures::Stream;
    use langchain_rust::{
        language_models::{llm::LLM, GenerateResult, LLMError},
        schemas::{Message, StreamData},
    };
    use mockall::*;

    mock! {
        LLMStub {
        }

        impl Clone for LLMStub {
            fn clone(&self) -> Self {
                LLMStub {}
            }
        }

        impl LLM for LLMStub {
            #[must_use]
            #[allow(
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn generate<'life0, 'life1, 'async_trait>(
                &'life0 self,
                messages: &'life1 [Message],
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<Output = Result<GenerateResult, LLMError>>
                        + ::core::marker::Send
                        + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                Self: 'async_trait,
            {
                todo!()
            }

            #[must_use]
            #[allow(
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn stream<'life0, 'life1, 'async_trait>(
                &'life0 self,
                _messages: &'life1 [Message],
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                            Output = Result<
                                Pin<Box<dyn Stream<Item = Result<StreamData, LLMError>> + Send>>,
                                LLMError,
                            >,
                        > + ::core::marker::Send
                        + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                Self: 'async_trait,
            {
                todo!()
            }
        }
    }

    #[test]
    fn test_returns_compatible_llm() {
        let mut factory1 = MockLLMFactory::new();
        factory1
            .expect_is_compatible()
            .with(predicate::eq("openai:gpt-4o"))
            .returning(|_| true);
        factory1
            .expect_create()
            .returning(|_| Ok(Box::new(MockLLMStub::new())));

        let instance = LLMFactoryRouter {
            llm_factories: vec![Arc::new(factory1)],
        };

        let result = instance.create(CreateLLMParameters {
            model: "openai:gpt-4o".to_string(),
        });
        let result = result;

        assert!(result.is_ok());
    }

    #[test]
    fn test_returns_error_when_no_compatible_llm_is_found() {
        let mut factory1 = MockLLMFactory::new();
        factory1
            .expect_is_compatible()
            .with(predicate::eq("openai:gpt-4o"))
            .returning(|_| false);

        let instance = LLMFactoryRouter {
            llm_factories: vec![Arc::new(factory1)],
        };

        let result = instance.create(CreateLLMParameters {
            model: "openai:gpt-4o".to_string(),
        });

        assert!(result.is_err());
    }
}
