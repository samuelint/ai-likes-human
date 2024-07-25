import os
from ai_assistant_core.app_configuration import AppConfiguration
from tests.test_functional.functional_test_utils import create_test_client
from tests.test_functional.assets.assets import joke_extension_whl_file_path

test_api = create_test_client()
base_route = "/extension"


class TestExtensions:
    appConfig = AppConfiguration(database_url="sqlite:///:memory:")

    def test_upload_file_must_have_whl_extension(self):
        with open(joke_extension_whl_file_path, "rb") as file:
            response = test_api.post(
                f"{base_route}/upload",
                files={"file": ("joke_extension", file, "application/zip")},
            )
            body = response.json()

            assert response.status_code == 400
            assert body["detail"] == "invalid file format"

    def test_upload_read_delete(self):
        # Upload
        with open(joke_extension_whl_file_path, "rb") as file:
            response = test_api.post(
                f"{base_route}/upload",
                files={
                    "file": (
                        "joke_extension-0.1.0-py3-none-any.whl",
                        file,
                        "application/zip",
                    )
                },
            )
            upload_body = response.json()

            assert response.status_code == 200
            assert upload_body["name"] == "joke-extension"
            assert upload_body["version"] == "0.1.0"
            assert upload_body["author"] == "Samuel"

        # List
        list_response = test_api.get(base_route)
        list_response_body = list_response.json()
        assert list_response.status_code == 200
        assert len(list_response_body) > 0
        assert any(item["name"] == "joke-extension" for item in list_response_body)

        # Read
        got_response = test_api.get(f"{base_route}/joke-extension")
        got_configuration_item = got_response.json()
        assert got_response.status_code == 200
        assert got_configuration_item["name"] == upload_body["name"]
        assert got_configuration_item["version"] == upload_body["version"]

        # Delete
        delete_response = test_api.delete(f"{base_route}/joke-extension")
        assert delete_response.status_code == 200
        list_response = test_api.get(base_route)
        list_response_body = list_response.json()
        assert list_response.status_code == 200
        assert not any(item["name"] == "joke-extension" for item in list_response_body)

    def test_on_upload_file_is_uploaded(self):
        with open(joke_extension_whl_file_path, "rb") as file:
            test_api.post(
                f"{base_route}/upload",
                files={
                    "file": (
                        "joke_extension-0.1.0-py3-none-any.whl",
                        file,
                        "application/zip",
                    )
                },
            )

        assert os.path.exists(
            os.path.join(
                self.appConfig.extensions_directory,
                "joke_extension-0.1.0-py3-none-any.whl",
            )
        )

    def test_file_is_deleted_from_disk_delete(self):
        with open(joke_extension_whl_file_path, "rb") as file:
            test_api.post(
                f"{base_route}/upload",
                files={
                    "file": (
                        "joke_extension-0.1.0-py3-none-any.whl",
                        file,
                        "application/zip",
                    )
                },
            )
        test_api.delete(f"{base_route}/joke-extension")

        assert not os.path.exists(
            os.path.join(
                self.appConfig.extensions_directory,
                "joke_extension-0.1.0-py3-none-any.whl",
            )
        )
