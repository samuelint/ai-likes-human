from ai_assistant_core.extension.infrastructure.whl_extension_repository import (
    WhlExtensionRepository,
)
from unittest import mock

extensions_directory = "/tmp/my-extensions"
mock_entry_1 = mock.Mock(
    is_file=mock.Mock(return_value=True),
    name="extension_1.whl",
    path=f"{extensions_directory}/extension_1.whl",
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
        name="extension_2.whl",
        path=f"{extensions_directory}/extension_2.whl",
    ),
]


class TestListAvailable:
    def test_list_available(self):
        with mock.patch("os.scandir") as mock_scandir, mock.patch(
            "ai_assistant_core.extension.infrastructure.whl_extension_service.WhlMetadataLoader"
        ) as mock_metadata_loader:
            mock_scandir.side_effect = lambda path: path == extensions_directory and [
                mock_entry_1
            ]
            mock_metadata_loader_instance = mock_metadata_loader.return_value
            mock_metadata_loader_instance.read_metadata_as_set.return_value = {
                "name": "extension1",
                "version": "1.0.0",
                "author": "Author Name",
            }

            service = WhlExtensionRepository(extensions_directory=extensions_directory)
            result = service.list_available()

            assert result[0].author == "Author Name"
            assert result[0].name == "extension1"
            assert result[0].version == "1.0.0"
            assert result[0].uri == f"{extensions_directory}/extension_1.whl"

    def test_extension_listed_are_files_with_whl_extension(self):
        with mock.patch("os.scandir") as mock_scandir:
            mock_scandir.side_effect = (
                lambda path: path == extensions_directory and mock_entries
            )
            instance = WhlExtensionRepository(extensions_directory=extensions_directory)
            files = instance._list_whl_files_in_extension_directory()

            assert len(files) == 2
            assert files[0].endswith("extension_1.whl")
            assert files[1].endswith("extension_2.whl")
