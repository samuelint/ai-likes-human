import os
from injector import inject
from sqlalchemy import Pool, QueuePool, StaticPool, create_engine
from sqlalchemy.orm import sessionmaker, Session

from ai_assistant_core.infrastructure.migrator import run_database_migration

from .sqlalchemy import Base
from urllib.parse import urlparse


@inject
class SqlAlchemySessionFactory:

    def __init__(self, database_url: str) -> None:
        self.database_url = database_url

    def create(
        self,
    ) -> Session:
        database_url = self.database_url
        self.__create_sqlite_path(database_url)
        engine = create_engine(
            database_url,
            poolclass=self.__get_pool_class(database_url),
        )

        Base.metadata.bind = engine
        Base.metadata.create_all(engine)

        with engine.begin() as connection:
            run_database_migration(connection=connection)

        return sessionmaker(autocommit=False, bind=engine)()

    def __get_pool_class(self, database_url: str) -> Pool:
        if database_url.startswith("sqlite"):
            return StaticPool
        else:
            return QueuePool

    def __create_sqlite_path(self, database_url: str) -> str:
        parsed_url = urlparse(database_url)

        if parsed_url.scheme in ["sqlite"]:
            database_path = os.path.abspath(parsed_url.path).removeprefix("/")
            directory = os.path.dirname(database_path)

            if not os.path.exists(directory):
                os.makedirs(directory)
