import os

import psutil
from ai_assistant_core.app_configuration import AppConfiguration
from tests.test_functional.functional_test_utils import create_test_client
from tests.test_functional.assets.assets import (
    joke_extension_pex_file_path,
    joke_extension_pex_file_name,
)

test_api = create_test_client()
base_route = "/extension"


class TestPexExtensionCRUD:
    appConfig = AppConfiguration(database_url="sqlite:///:memory:")

    def test_upload_file_must_have_pex_extension(self):
        with open(joke_extension_pex_file_path, "rb") as file:
            response = test_api.post(
                f"{base_route}/pex/upload",
                files={"file": ("joke_extension", file, "application/zip")},
            )
            body = response.json()

            assert response.status_code == 400
            assert body["detail"] == "invalid file format"

    def test_upload_read_delete(self):
        # Upload
        with open(joke_extension_pex_file_path, "rb") as file:
            response = test_api.post(
                f"{base_route}/pex/upload",
                files={
                    "file": (
                        joke_extension_pex_file_name,
                        file,
                        "application/zip",
                    )
                },
            )
            upload_body = response.json()

            assert response.status_code == 200
            assert upload_body["name"] == "joke_extension_v2"

        # List
        list_response = test_api.get(base_route)
        list_response_body = list_response.json()
        assert list_response.status_code == 200
        assert len(list_response_body) > 0
        assert any(item["name"] == "joke_extension_v2" for item in list_response_body)

        # Read
        got_response = test_api.get(f"{base_route}/joke_extension_v2")
        got_configuration_item = got_response.json()
        assert got_response.status_code == 200
        assert got_configuration_item["name"] == upload_body["name"]
        assert got_configuration_item["version"] == upload_body["version"]

        # Delete
        delete_response = test_api.delete(f"{base_route}/joke_extension_v2")
        assert delete_response.status_code == 200
        list_response = test_api.get(base_route)
        list_response_body = list_response.json()
        assert list_response.status_code == 200
        assert not any(
            item["name"] == "joke_extension_v2" for item in list_response_body
        )

    def test_on_upload_file_is_uploaded(self):
        with open(joke_extension_pex_file_path, "rb") as file:
            test_api.post(
                f"{base_route}/pex/upload",
                files={
                    "file": (
                        joke_extension_pex_file_name,
                        file,
                        "application/zip",
                    )
                },
            )

        assert os.path.exists(
            os.path.join(
                self.appConfig.extensions_directory,
                joke_extension_pex_file_name,
            )
        )

    def test_file_is_deleted_from_disk_delete(self):
        with open(joke_extension_pex_file_path, "rb") as file:
            test_api.post(
                f"{base_route}/pex/upload",
                files={
                    "file": (
                        joke_extension_pex_file_name,
                        file,
                        "application/zip",
                    )
                },
            )
        test_api.delete(f"{base_route}/joke_extension_v2")

        assert not os.path.exists(
            os.path.join(
                self.appConfig.extensions_directory,
                joke_extension_pex_file_name,
            )
        )


class TestPexExtensionLoad:
    appConfig = AppConfiguration(database_url="sqlite:///:memory:")

    def upload_extension(self) -> str:
        with open(joke_extension_pex_file_path, "rb") as file:
            response = test_api.post(
                f"{base_route}/pex/upload",
                files={
                    "file": (
                        joke_extension_pex_file_name,
                        file,
                        "application/zip",
                    )
                },
            )
            upload_body = response.json()

            assert response.status_code == 200
            return upload_body["name"]

    def test_installing_an_extension_loads_it(self):
        extension_name = self.upload_extension()

        extension_state = test_api.get(f"{base_route}/{extension_name}").json()

        assert extension_state["is_loaded"] is True

    def test_loaded_extension_has_process(self):
        extension_name = self.upload_extension()

        extension_state = test_api.get(f"{base_route}/{extension_name}").json()

        assert extension_state["pid"] > 0
        process = psutil.Process(extension_state["pid"])
        assert process.is_running()
