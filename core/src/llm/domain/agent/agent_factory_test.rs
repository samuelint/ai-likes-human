#[cfg(test)]
mod test {
    use langchain_rust::{
        chain::{Chain, ChainError},
        language_models::GenerateResult,
        prompt::PromptArgs,
    };
    use mockall::{mock, predicate::eq};
    use std::{collections::HashMap, sync::Arc};

    use crate::llm::domain::agent::{
        base_agent_factory::{CreateAgentArgs, MockBaseAgentFactory},
        AgentFactory,
    };

    mock! {
        ChainStub {
        }

        impl Clone for ChainStub {
            fn clone(&self) -> Self {
                ChainStub {}
            }
        }

        #[async_trait::async_trait]
        impl Chain for ChainStub {
            async fn call(&self, input_variables: PromptArgs) -> Result<GenerateResult, ChainError> {
                todo!()
            }

        }
    }

    #[tokio::test]
    async fn test_agent_corresponding_to_id_is_returned() {
        let mut factory1 = MockBaseAgentFactory::new();
        let mut factory2 = MockBaseAgentFactory::new();
        factory1
            .expect_is_compatible()
            .with(eq("AAA"))
            .return_const(true);
        factory1.expect_create().returning(|_| {
            let mut chain_mock = MockChainStub::new();
            chain_mock.expect_call().returning(|_| {
                Ok(GenerateResult {
                    generation: "some gen result".to_string(),
                    ..GenerateResult::default()
                })
            });

            Ok(Box::new(chain_mock))
        });
        factory2.expect_is_compatible().return_const(false);
        let instance = AgentFactory::new(vec![Arc::new(factory1), Arc::new(factory2)]);

        let result = instance
            .create("AAA", CreateAgentArgs::default())
            .await
            .unwrap();
        let gen_result = result.call(HashMap::new()).await.unwrap();

        assert_eq!(gen_result.generation, "some gen result");
    }
}
