from typing import Optional
from injector import Binder, Module, singleton
import os
from dotenv import load_dotenv, find_dotenv
from platformdirs import user_data_dir

load_dotenv(find_dotenv())

app_name = "com.samuelint.assistant"


class AppConfiguration:
    app_directory: str
    database_url: str
    extensions_directory: str
    self_url: str

    def __init__(
        self,
        server_port: Optional[int] = None,
        database_url: Optional[str] = None,
        self_host: Optional[str] = None,
    ):
        os.environ["APP_DIRECTORY"] = self.app_directory = user_data_dir(
            appname=app_name,
        )
        os.environ["EXTENSIONS_DIRECTORY"] = self.extensions_directory = os.path.join(
            self.app_directory, "extensions"
        )
        os.environ["DATABASE_URL"] = self.database_url = (
            database_url or f"sqlite:///{self.app_directory}/data.db"
        )

        server_port = server_port or 8000
        self_host = self_host or "localhost"
        os.environ["SELF_URL"] = self.self_url = f"http://{self_host}:{server_port}"


class AppConfigurationModule(Module):
    def __init__(
        self,
        server_port: int,
        self_host: Optional[str] = None,
        database_url: Optional[str] = None,
    ):
        self.database_url = database_url
        self.self_host = self_host
        self.server_port = server_port

    def configure(self, binder: Binder):
        configuration = AppConfiguration(
            server_port=self.server_port,
            database_url=self.database_url,
            self_host=self.self_host,
        )

        binder.bind(AppConfiguration, to=configuration, scope=singleton)
