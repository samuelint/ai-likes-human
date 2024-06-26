# flake8: noqa
from langchain_core.prompts import PipelinePromptTemplate, PromptTemplate
from langchain.output_parsers import ResponseSchema, StructuredOutputParser
from ai_assistant_core.prompts.confidence_level import (
    confidence_percent_response_schema,
)

role_template = """\
#role 
You are {person} personal assistant.
#style
Be straight forward and concise. Only give explanation if asked, otherwise only answer with the response value.
#objective
When an url is provided by user, load the content of the url and use it for the answer
"""
commands_template = """\
#commands
When "/<command> <args>" is written, execute the corresponding function and return only the raw output value without text. 
For example, 
- "/magic_number 6" outputs "8".
- "/magic_number 2" outputs "4".
"""

system_prompt_template = """{role}

{commands}
"""

text_response = ResponseSchema(
    name="response",
    description="Text response.",
)


class AssistantPromptBuilder:
    def __init__(self, person_name=None, confidence_percentage=True):
        self.person_name = person_name
        self.confidence_percentage = confidence_percentage

    def build_output_parser(self) -> StructuredOutputParser:
        response_schemas = [text_response]
        if self.confidence_percentage:
            response_schemas.append(confidence_percent_response_schema)

        return StructuredOutputParser.from_response_schemas(response_schemas)

    def build_system_prompt(self) -> str:
        # output_parser = self.build_output_parser()
        # format_instructions = output_parser.get_format_instructions()
        system_prompt = PromptTemplate.from_template(system_prompt_template)

        pipeline_prompt_template = PipelinePromptTemplate(
            final_prompt=system_prompt,
            pipeline_prompts=[
                ("role", PromptTemplate.from_template(role_template)),
                ("commands", PromptTemplate.from_template(commands_template)),
            ],
        )

        return pipeline_prompt_template.format(person=self.person_name)
