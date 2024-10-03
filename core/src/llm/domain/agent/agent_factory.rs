#[cfg(test)]
#[path = "./agent_factory_test.rs"]
mod agent_factory_test;

use super::base_agent_factory::{BaseAgentFactory, CreateAgentArgs};
use anyhow::anyhow;
use langchain_rust::chain::Chain;
use std::{error::Error, sync::Arc};

pub struct AgentFactory {
    agent_factories: Vec<Arc<dyn BaseAgentFactory>>,
}

impl AgentFactory {
    pub fn new(agent_factories: Vec<Arc<dyn BaseAgentFactory>>) -> Self {
        Self { agent_factories }
    }

    fn find_factory(&self, agent_id: &str) -> Option<Arc<dyn BaseAgentFactory>> {
        for factory in &self.agent_factories {
            if factory.is_compatible(agent_id) {
                return Some(factory.clone());
            }
        }

        None
    }
}

impl AgentFactory {
    pub async fn create(
        &self,
        agent_id: &str,
        args: &CreateAgentArgs,
    ) -> Result<Box<dyn Chain>, Box<dyn Error + Send>> {
        let chain = match self.find_factory(agent_id) {
            Some(factory) => factory.create(args).await,
            None => Err(anyhow!("Agent not found: {}", agent_id).into()),
        };

        chain
    }
}
