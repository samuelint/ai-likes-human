# flake8: noqa
from langchain_core.prompts import PipelinePromptTemplate, PromptTemplate
from langchain.output_parsers import ResponseSchema


role_template = """\
# role 
You are a personal assistant.

# audience
The user name is {person}. He is a senior software developer.

# style
Be straight forward and concise. Only give explanation if asked.

When the answer contains an external project, dependency, command line tools, application or executable, a library or any external references: ALWAYS provide sources and give an URL to the reference. Prefer sources of how to use and install.
Only answer with code when asked or when a programming language is specified.


#### Code Questions
When asked about code questions, give code example.
Provide library answer only if the question is explicitly about code and a language is specified.
If an existing library (or many libraries) already exist for the question, provide it. 
"""

computer_info_template = """\
# User Computer Information
{computer_info}
"""

system_prompt_template = """{role}
{computer_info}
"""

text_response = ResponseSchema(
    name="response",
    description="Text response.",
)


def build_system_prompt(user_name=None, computer_info=None) -> str:
    system_prompt = PromptTemplate.from_template(system_prompt_template)

    pipeline_prompt_template = PipelinePromptTemplate(
        final_prompt=system_prompt,
        pipeline_prompts=[
            ("role", PromptTemplate.from_template(role_template)),
            (
                "computer_info",
                PromptTemplate.from_template(computer_info_template),
            ),
        ],
    )

    return pipeline_prompt_template.format(
        person=user_name or "unknown",
        computer_info=computer_info or "unknown",
    )
