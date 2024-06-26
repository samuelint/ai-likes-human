from ai_assistant_core.assistant_prompt_builder import (
    AssistantPromptBuilder,
)


class TestSystemPromptTest:
    def test_person_name(self):
        # When building system prompt
        result = AssistantPromptBuilder(person_name="John").build_system_prompt()

        # Then prompt contains input
        assert "You are John personal assistant." in result

    def test_no_person_name(self):
        # Given no person name
        builder = AssistantPromptBuilder()

        # When building system prompt
        result = builder.build_system_prompt()

        # Then None is the person name
        assert "You are None personal assistant." in result
