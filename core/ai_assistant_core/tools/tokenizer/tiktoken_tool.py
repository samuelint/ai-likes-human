from langchain_core.tools import tool
import tiktoken


encoder = tiktoken.encoding_for_model("gpt-3.5-turbo")


@tool("token_size")
def token_size_tool(query: str) -> str:
    """Get token size query"""
    tokens = encoder.encode(query)
    return len(tokens)
