from langchain.output_parsers import ResponseSchema

confidence_percent_instructions = "Give a confidence percentage level for each answers."
confidence_percent_response_schema = ResponseSchema(
    name="confidence_percent",
    description=confidence_percent_instructions,
)
