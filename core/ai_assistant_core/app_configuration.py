from typing import Optional, Union
from injector import Binder, Module, singleton
import os
from dotenv import load_dotenv, find_dotenv
from platformdirs import user_data_dir

load_dotenv(find_dotenv())

app_name = "com.samuelint.assistant"


# !!! REMOVE THOSES HARDCODED KEYS ONCE THE APP ALLOWS TO CONFIGURE SECRETS WITHIN UI !!! #
class AppConfiguration:
    openai_api_key: str
    anthropic_api_key: str

    serper_api_key: str

    app_directory: str
    database_url: str

    def __init__(self, database_url: Optional[str] = None):
        os.environ["APP_DIRECTORY"] = self.app_directory = user_data_dir(
            appname=app_name,
        )
        os.environ["DATABASE_URL"] = self.database_url = (
            database_url or f"sqlite:///{self.app_directory}/data.db"
        )
        os.environ["OPENAI_API_KEY"] = self.openai_api_key = os.getenv(
            "OPENAI_API_KEY",
            "sk-temp-in-app-hardcoded-Ywq7GiavQy6enRGzD6oNT3BlbkFJ37YEwq1JfFWsWo2BUfTD",
        )
        os.environ["ANTHROPIC_API_KEY"] = self.anthropic_api_key = os.getenv(
            "ANTHROPIC_API_KEY",
            # flake8: noqa: E501
            "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",
        )
        os.environ["SERPER_API_KEY"] = self.serper_api_key = os.getenv(
            "SERPER_API_KEY", "546b9c8c39df5f519feae9a2285fd773dd7772a6"
        )


class AppConfigurationModule(Module):
    def __init__(self, database_url: Optional[str] = None):
        self.database_url = database_url

    def configure(self, binder: Binder):
        configuration = AppConfiguration(database_url=self.database_url)
        binder.bind(AppConfiguration, to=configuration, scope=singleton)
