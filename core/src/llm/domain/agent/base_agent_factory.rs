use std::error::Error;

use langchain_rust::chain::Chain;
use mockall::automock;

#[derive(Default)]
pub struct CreateAgentArgs {
    pub model: String,
    pub temperature: Option<f32>,
}

#[automock]
#[async_trait::async_trait]
pub trait BaseAgentFactory: Send + Sync {
    fn is_compatible(&self, agent_id: &str) -> bool;

    async fn create(&self, args: &CreateAgentArgs)
        -> Result<Box<dyn Chain>, Box<dyn Error + Send>>;
}
