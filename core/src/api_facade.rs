use std::sync::Arc;

use crate::{
    chat_completion::{
        inference::{Inference, InferenceArgs},
        ChatCompletionMessageDto, ChatCompletionResult, ChatCompletionStream,
    },
    configuration::ConfigurationDto,
    profile::domain::dto::ProfileDto,
    AppContainer,
};

pub struct ApiFacade {
    container: Arc<AppContainer>,
}

impl ApiFacade {
    pub fn new(container: Arc<AppContainer>) -> Self {
        Self { container }
    }

    pub async fn chat_completion_invoke(
        &self,
        model: &str,
        messages: &Vec<ChatCompletionMessageDto>,
    ) -> ChatCompletionResult {
        let inference = self.container.chat_completion_module.get_llm_inference();

        inference
            .invoke(InferenceArgs {
                model: model.to_string(),
                messages: messages.clone(),
                ..Default::default()
            })
            .await
    }

    pub async fn chat_completion_stream(
        &self,
        model: &str,
        messages: &Vec<ChatCompletionMessageDto>,
    ) -> ChatCompletionStream {
        let inference = self.container.chat_completion_module.get_llm_inference();

        inference
            .stream(InferenceArgs {
                model: model.to_string(),
                messages: messages.clone(),
                ..Default::default()
            })
            .await
    }

    pub async fn find_configuration(
        &self,
        key: &str,
    ) -> Result<Option<ConfigurationDto>, Box<dyn std::error::Error + Send>> {
        self.container
            .configuration_module
            .get_configuration_service()
            .find(key)
            .await
    }

    pub async fn upsert_configuration(
        &self,
        key: &str,
        value: &str,
    ) -> Result<ConfigurationDto, Box<dyn std::error::Error + Send>> {
        let configuration_service = self
            .container
            .configuration_module
            .get_configuration_service();

        configuration_service.upsert(key, value).await
    }

    pub async fn get_selected_profiles(
        &self,
    ) -> Result<Vec<ProfileDto>, Box<dyn std::error::Error + Send>> {
        self.container
            .profile_module
            .get_selected_profiles_service()
            .find_selected_profiles()
            .await
    }
}
