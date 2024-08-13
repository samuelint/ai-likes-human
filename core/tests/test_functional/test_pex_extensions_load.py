import psutil
import pytest
from ai_assistant_core.app_configuration import AppConfiguration
from tests.test_functional.functional_test_utils import create_test_client
from tests.test_functional.assets.assets import (
    joke_extension_pex_file_path,
    joke_extension_pex_file_name,
)

test_api = create_test_client()
base_route = "/extension"


class TestPexExtensionLoadOnInstall:
    appConfig = AppConfiguration(database_url="sqlite:///:memory:")

    def upload_extension(self) -> str:
        with open(joke_extension_pex_file_path, "rb") as file:
            response = test_api.post(
                f"{base_route}/upload",
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

    def test_installing_an_extension_does_not_loads_it(self):
        extension_name = self.upload_extension()

        extension_state = test_api.get(f"{base_route}/{extension_name}").json()

        assert extension_state["is_loaded"] is False

    def test_not_loaded_extension_has_no_pid(self):
        extension_name = self.upload_extension()

        extension_state = test_api.get(f"{base_route}/{extension_name}").json()

        assert extension_state["pid"] is None

    def test_not_loaded_extension_has_no_ipc_port(self):
        extension_name = self.upload_extension()

        extension_state = test_api.get(f"{base_route}/{extension_name}").json()

        assert extension_state["ipc_port"] is None


class TestPexExtensionLoad:
    appConfig = AppConfiguration(database_url="sqlite:///:memory:")

    def upload_extension(self) -> str:
        with open(joke_extension_pex_file_path, "rb") as file:
            response = test_api.post(
                f"{base_route}/upload",
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

    def load_extension(self, extension_name: str) -> None:
        test_api.post(f"{base_route}/{extension_name}/load")

    def unload_extension(self, extension_name: str) -> None:
        test_api.post(f"{base_route}/{extension_name}/unload")

    @pytest.fixture(scope="module", autouse=True)
    def setup_and_teardown(self):
        # Setup
        extension_name = self.upload_extension()
        self.load_extension(extension_name=extension_name)

        yield extension_name
        # Teardown
        self.unload_extension(extension_name=extension_name)

    def test_loaded_extension_has_process(self, setup_and_teardown):
        extension_name = setup_and_teardown
        extension_state = test_api.get(f"{base_route}/{extension_name}").json()

        assert extension_state["pid"] > 0
        process = psutil.Process(extension_state["pid"])
        assert process.is_running()

    def test_loaded_extension_has_ipc_port(self, setup_and_teardown):
        extension_name = setup_and_teardown
        extension_state = test_api.get(f"{base_route}/{extension_name}").json()

        assert extension_state["ipc_port"] > 0

    def test_loaded_extensions_are_listed(self, setup_and_teardown):
        extension_name = setup_and_teardown
        extensions = test_api.get(f"{base_route}").json()

        assert any(item["name"] == extension_name for item in extensions)
