from unittest.mock import patch
from decoy import Decoy
import pytest

from ai_assistant_core.llm.domain.local_model_index_repository import (
    LocalLLMModelIndexRepository,
)
from ai_assistant_core.llm.domain.local_model_service import LocalLLMModelService
from tests.llm.domain.__dto_factories__ import (
    LLMModelIndexPolyfactory,
)


@pytest.fixture
def index_repository(decoy: Decoy) -> LocalLLMModelIndexRepository:
    repo = decoy.mock(cls=LocalLLMModelIndexRepository)

    return repo


@pytest.fixture
def instance(index_repository: LocalLLMModelIndexRepository) -> LocalLLMModelService:
    return LocalLLMModelService(index_repository=index_repository)


class TestDelete:
    def test_on_delete_index_is_removed_from_repo(
        self,
        decoy: Decoy,
        instance: LocalLLMModelService,
        index_repository: LocalLLMModelIndexRepository,
    ):
        some_deleted_index_item = LLMModelIndexPolyfactory.build()
        decoy.when(index_repository.delete(model_name="a")).then_return(
            some_deleted_index_item
        )

        result = instance.delete(id="a")

        assert result is some_deleted_index_item

    def test_on_delete_local_q4_gguf_file_are_deleted(
        self,
        decoy: Decoy,
        instance: LocalLLMModelService,
        index_repository: LocalLLMModelIndexRepository,
    ):

        some_deleted_index_item = LLMModelIndexPolyfactory.build(local_path="./q4.gguf")
        decoy.when(index_repository.delete(model_name="a")).then_return(
            some_deleted_index_item
        )

        with patch("os.remove") as mock_remove, patch(
            "os.path.exists"
        ) as mock_path_exists:
            mock_path_exists.side_effect = lambda path: path == "./q4.gguf"
            instance.delete(id="a")

            mock_remove.assert_called_once_with("./q4.gguf")
