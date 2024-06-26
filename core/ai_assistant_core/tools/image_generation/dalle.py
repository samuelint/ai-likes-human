from langchain_core.tools import tool
from langchain_community.utilities.dalle_image_generator import DallEAPIWrapper

dall_e = DallEAPIWrapper()


@tool
def dall_e_tool(query: str) -> str:
    """Generate an image using DALL-E. Returns image URL"""
    return dall_e.run(query)
