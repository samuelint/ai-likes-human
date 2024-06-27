import os
from typing import Union
from langchain_core.tools import tool
from langchain_community.utilities import GoogleSerperAPIWrapper

serper_client: Union[GoogleSerperAPIWrapper, None] = None


def _get_serper_client():
    global serper_client
    if serper_client is None:
        serper_client = GoogleSerperAPIWrapper(
            serper_api_key=os.getenv("SERPER_API_KEY")
        )
    return serper_client


@tool
def web_search_tool(query: str) -> str:
    """Search the web (internet) about a given topic"""
    serper_client = _get_serper_client()
    return serper_client.run(query)
