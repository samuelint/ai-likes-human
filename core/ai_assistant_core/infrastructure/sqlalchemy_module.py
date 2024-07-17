from injector import Module, provider, singleton
from sqlalchemy.orm import Session


from ai_assistant_core.app_configuration import AppConfiguration
from ai_assistant_core.infrastructure.session_factory import SqlAlchemySessionFactory


class SqlAlchemyModule(Module):
    @singleton
    @provider
    def provide_sqlalchemy_session(self, configuration: AppConfiguration) -> Session:
        database_url = configuration.database_url
        factory = SqlAlchemySessionFactory(database_url)

        return factory.create()
