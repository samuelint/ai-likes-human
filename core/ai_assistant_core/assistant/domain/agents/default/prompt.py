# flake8: noqa
from langchain_core.prompts import PipelinePromptTemplate, PromptTemplate
from langchain.output_parsers import ResponseSchema


role_template = """\
# role 
You are a personal assistant.

# audience
The user name is {person}.

# style
Be straight forward and concise. Only give explanation if asked.
"""

system_prompt_template = """{role}
"""

text_response = ResponseSchema(
    name="response",
    description="Text response.",
)


def build_system_prompt(user_name=None) -> str:
    system_prompt = PromptTemplate.from_template(system_prompt_template)

    pipeline_prompt_template = PipelinePromptTemplate(
        final_prompt=system_prompt,
        pipeline_prompts=[
            ("role", PromptTemplate.from_template(role_template)),
        ],
    )

    return pipeline_prompt_template.format(person=user_name or "unknown")
