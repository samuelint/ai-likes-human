#[cfg(test)]
mod tests {
    use crate::profile::domain::{dto::ProfileDto, SystemPromptBuilder};

    #[test]
    fn test_prompt_have_personal_assistant_role() {
        let builder = SystemPromptBuilder::new().with_personal_assistant_role();

        let prompt = builder.build();

        assert!(prompt.contains("# role\nYou are a personal assistant."));
    }

    #[test]
    fn test_prompt_have_computer_info() {
        let builder = SystemPromptBuilder::new().with_computer_info();

        let prompt = builder.build();

        assert!(prompt.contains("# Computer Info"));
    }

    #[test]
    fn test_prompt_with_profile() {
        let profile = &ProfileDto {
            name: "some".to_string(),
            prompt: "Your are a super hero.".to_string(),
        };
        let builder = SystemPromptBuilder::new().with_profile(profile);

        let prompt = builder.build();

        assert!(prompt.contains("Your are a super hero."));
    }

    #[test]
    fn test_prompt_with_multiple_profiles() {
        let profile1 = ProfileDto {
            name: "some".to_string(),
            prompt: "Your are a super hero.".to_string(),
        };
        let profile2 = ProfileDto {
            name: "other".to_string(),
            prompt: "Be gentle".to_string(),
        };
        let builder = SystemPromptBuilder::new().with_profiles(&vec![profile1, profile2]);

        let prompt = builder.build();

        assert!(prompt.contains("Your are a super hero."));
        assert!(prompt.contains("Be gentle"));
    }
}
