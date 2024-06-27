import os
from typing import Union
from langchain_core.tools import tool
from langchain_community.utilities.dalle_image_generator import DallEAPIWrapper


dall_e_client: Union[DallEAPIWrapper, None] = None


def _get_dall_e_client():
    global dall_e_client
    if dall_e_client is None:
        dall_e_client = DallEAPIWrapper(openai_api_key=os.getenv("OPENAI_API_KEY"))
    return dall_e_client


@tool
def dall_e_tool(query: str) -> str:
    """Generate an image using DALL-E. Returns image URL"""
    dall_e = _get_dall_e_client()
    return dall_e.run(query)
