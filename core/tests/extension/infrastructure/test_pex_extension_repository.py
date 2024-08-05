from ai_assistant_core.extension.infrastructure.pex_extension_repository import (
    PexExtensionRepository,
)
from unittest import mock

extensions_directory = "/tmp/my-extensions"
mock_entry_1 = mock.Mock(
    is_file=mock.Mock(return_value=True),
    name="extension_1.pex",
    path=f"{extensions_directory}/extension_1.pex",
)
mock_entries = [
    mock_entry_1,
    mock.Mock(is_file=mock.Mock(return_value=False), name="dir1"),
    mock.Mock(
        is_file=mock.Mock(return_value=True),
        name="some.txt",
        path=f"{extensions_directory}/some.txt",
    ),
    mock.Mock(
        is_file=mock.Mock(return_value=True),
        name="extension_2.pex",
        path=f"{extensions_directory}/extension_2.pex",
    ),
]


class TestListAvailable:
    def test_list_available(self):
        with mock.patch("os.scandir") as mock_scandir:
            mock_scandir.side_effect = lambda path: path == extensions_directory and [
                mock_entry_1
            ]

            service = PexExtensionRepository(extensions_directory=extensions_directory)
            result = service.list()

            assert result[0].name == "extension_1"
            assert result[0].uri == f"{extensions_directory}/extension_1.pex"

    def test_extension_listed_are_files_with_pex_extension(self):
        with mock.patch("os.scandir") as mock_scandir:
            mock_scandir.side_effect = (
                lambda path: path == extensions_directory and mock_entries
            )
            instance = PexExtensionRepository(extensions_directory=extensions_directory)
            files = instance.list()

            assert len(files) == 2
            assert files[0].uri.endswith("extension_1.pex")
            assert files[1].uri.endswith("extension_2.pex")
