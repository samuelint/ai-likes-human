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

    def __init__(self, database_url: Optional[str] = None):
        os.environ["APP_DIRECTORY"] = self.app_directory = user_data_dir(
            appname=app_name,
        )
        os.environ["EXTENSIONS_DIRECTORY"] = self.extensions_directory = os.path.join(
            self.app_directory, "extensions"
        )
        os.environ["DATABASE_URL"] = self.database_url = (
            database_url or f"sqlite:///{self.app_directory}/data.db"
        )


class AppConfigurationModule(Module):
    def __init__(self, database_url: Optional[str] = None):
        self.database_url = database_url

    def configure(self, binder: Binder):
        configuration = AppConfiguration(database_url=self.database_url)
        binder.bind(AppConfiguration, to=configuration, scope=singleton)
