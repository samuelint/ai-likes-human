import re
from langchain_core.tools import tool
from langchain_community.document_loaders.recursive_url_loader import RecursiveUrlLoader
from bs4 import BeautifulSoup as Soup


@tool("url_content_loader")
def url_content_loader_tool(url: str) -> dict:
    """Load url content"""

    loader = RecursiveUrlLoader(
        url=url,
        use_async=True,
        max_depth=2,
        extractor=lambda x: Soup(x, "html.parser").text,
    )
    data = loader.load()
    str_data = [__remove_consecutive_newlines(doc.page_content) for doc in data]

    return __remove_trailing_and_leading_newlines("".join(str_data))


def __remove_trailing_and_leading_newlines(text: str) -> str:
    return re.sub(r"^\n+|\n+$", "", text)


def __remove_consecutive_newlines(text: str) -> str:
    return re.sub(r"\n+", "\n", text)
