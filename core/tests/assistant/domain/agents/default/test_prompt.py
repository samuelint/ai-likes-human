from ai_assistant_core.assistant.domain.agents.default.prompt import (
    build_system_prompt,
)


class TestSystemPromptTest:
    def test_person_name(self):
        # When building system prompt
        result = build_system_prompt(user_name="John")

        # Then prompt contains input
        assert "The user name is John." in result

    def test_no_person_name(self):
        # Given no person name
        # When building system prompt
        result = build_system_prompt()

        # Then None is the person name
        assert "The user name is unknown." in result
