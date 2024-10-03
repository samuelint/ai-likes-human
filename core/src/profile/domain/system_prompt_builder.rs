#[cfg(test)]
#[path = "./system_prompt_builder_test.rs"]
mod system_prompt_builder_test;

use super::{computer_info_service::get_computer_info, dto::ProfileDto};

pub struct SystemPromptBuilder {
    prompt_chunks: Vec<String>,
}

impl SystemPromptBuilder {
    pub fn new() -> Self {
        Self {
            prompt_chunks: vec![],
        }
    }

    pub fn with_computer_info(self) -> Self {
        self.with_section("Computer Info", &get_computer_info().to_string())
    }

    pub fn with_profile(self, profile: &ProfileDto) -> Self {
        self.with(&profile.prompt)
    }

    pub fn with_profiles(self, profiles: &Vec<ProfileDto>) -> Self {
        profiles
            .iter()
            .fold(self, |acc, profile| acc.with_profile(profile))
    }

    pub fn with_section(mut self, title: &str, content: &str) -> Self {
        self.prompt_chunks
            .push(format!("# {}\n{}\n", title, content));

        self
    }

    pub fn with_personal_assistant_role(self) -> Self {
        self.with_section("role", "You are a personal assistant.")
    }

    pub fn with(mut self, content: &str) -> Self {
        self.prompt_chunks.push(format!("{}\n", content));

        self
    }

    pub fn build(self) -> String {
        self.prompt_chunks.join("\n")
    }
}
