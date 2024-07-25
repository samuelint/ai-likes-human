import zipfile
import pytest

from ai_assistant_core.extension.infrastructure.whl_metadata_loader import (
    WhlMetadataLoader,
)


@pytest.fixture
def wheel_file(tmp_path):
    # Create a temporary .whl file with minimal necessary structure
    wheel_content = {
        "example_package-1.0.0.dist-info/METADATA": (
            "Metadata-Version: 2.1\n"
            "Name: example_package\n"
            "Version: 1.0.0\n"
            "Summary: An example package\n"
            "Author: Author Name\n"
            "License: MIT\n"
        )
    }

    wheel_path = tmp_path / "example_package-1.0.0-py3-none-any.whl"
    with zipfile.ZipFile(wheel_path, "w") as whl:
        for file_name, file_content in wheel_content.items():
            whl.writestr(file_name, file_content)

    return wheel_path


def test_wheel_metadata(wheel_file):
    metadata = WhlMetadataLoader(wheel_file)
    metadata_dict = metadata.read_metadata_as_set()

    assert metadata_dict["metadata-version"] == "2.1"
    assert metadata_dict["name"] == "example_package"
    assert metadata_dict["version"] == "1.0.0"
    assert metadata_dict["summary"] == "An example package"
    assert metadata_dict["author"] == "Author Name"
    assert metadata_dict["license"] == "MIT"


if __name__ == "__main__":
    pytest.main()
