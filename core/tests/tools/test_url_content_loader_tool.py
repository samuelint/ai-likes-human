from unittest.mock import patch

from ai_assistant_core.tools.webpage_loader.url_content_loader import (
    url_content_loader_tool,
)
from langchain_core.documents import Document


class TestUrlContentLoaderTool:
    @patch(
        "langchain_community.document_loaders.recursive_url_loader.RecursiveUrlLoader.load",
        autospec=True,
    )
    async def test_returns_url_string_content(self, recursive_loader_aload):
        documents = [Document(page_content="hello"), Document(page_content=" world!")]
        recursive_loader_aload.return_value = documents

        result = url_content_loader_tool("https://www.google.com")

        assert result == "hello world!"

    @patch(
        "langchain_community.document_loaders.recursive_url_loader.RecursiveUrlLoader.load",
        autospec=True,
    )
    def test_trailing_and_leading_newlines_are_removed(self, loader_mock):
        documents = [
            Document(page_content="\n\n\n\n\n\nHello\n"),
            Document(page_content="World\n"),
            Document(page_content="!\n\n\n\n"),
        ]
        loader_mock.return_value = documents

        result = url_content_loader_tool("https://www.google.com")

        assert result == "Hello\nWorld\n!"
