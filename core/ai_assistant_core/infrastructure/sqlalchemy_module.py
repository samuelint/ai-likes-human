import os
from injector import Module, provider, singleton
from sqlalchemy import Pool, QueuePool, StaticPool, create_engine
from sqlalchemy.orm import sessionmaker, Session

from ai_assistant_core.app_configuration import AppConfiguration

from .sqlalchemy import Base
from urllib.parse import urlparse


class SqlAlchemyModule(Module):

    @singleton
    @provider
    def provide_sqlalchemy_session(self, configuration: AppConfiguration) -> Session:
        database_url = configuration.database_url
        self.create_sqlite_path(database_url)
        engine = create_engine(
            database_url,
            poolclass=self.get_pool_class(database_url),
        )

        Base.metadata.bind = engine
        Base.metadata.create_all(engine)
        session = sessionmaker(autocommit=False, bind=engine)()

        return session

    def get_pool_class(self, database_url: str) -> Pool:
        if database_url.startswith("sqlite"):
            return StaticPool
        else:
            return QueuePool

    def create_sqlite_path(self, database_url: str) -> str:
        parsed_url = urlparse(database_url)

        if parsed_url.scheme in ["sqlite"]:
            database_path = os.path.abspath(parsed_url.path)
            directory = os.path.dirname(database_path)

            if not os.path.exists(directory):
                os.makedirs(directory)
