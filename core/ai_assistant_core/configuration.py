import os
from pydantic import BaseModel
from dotenv import load_dotenv, find_dotenv

load_dotenv(find_dotenv())


# !!! REMOVE THOSES HARDCODED KEYS ONCE THE APP ALLOWS TO CONFIGURE SECRETS WITHIN UI !!! #
class Configuration(BaseModel):
    openai_api_key: str
    anthropic_api_key: str

    serper_api_key: str

    def load():
        config = Configuration(
            openai_api_key=os.getenv(
                "OPENAI_API_KEY",
                "sk-temp-in-app-hardcoded-Ywq7GiavQy6enRGzD6oNT3BlbkFJ37YEwq1JfFWsWo2BUfTD",
            ),
            anthropic_api_key=os.getenv(
                "ANTHROPIC_API_KEY",
                # flake8: noqa: E501
                "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",
            ),
            serper_api_key=os.getenv(
                "SERPER_API_KEY", "546b9c8c39df5f519feae9a2285fd773dd7772a6"
            ),
        )
        os.environ["OPENAI_API_KEY"] = config.openai_api_key
        os.environ["ANTHROPIC_API_KEY"] = config.anthropic_api_key
        os.environ["SERPER_API_KEY"] = config.serper_api_key

        return config
