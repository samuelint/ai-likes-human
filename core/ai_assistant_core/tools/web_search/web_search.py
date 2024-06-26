from langchain_core.tools import tool
from langchain_community.utilities import GoogleSerperAPIWrapper

search = GoogleSerperAPIWrapper()


@tool
def web_search_tool(query: str) -> str:
    """Search the web (internet) about a given topic"""
    return search.run(query)
