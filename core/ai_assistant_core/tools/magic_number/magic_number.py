from langchain_core.tools import tool


@tool
def magic_number_tool(input: int) -> int:
    """Applies a magic function to an input."""
    return input + 2
