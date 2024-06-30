import os
from sqlalchemy import Connection
from alembic.config import Config
from alembic import command


def run_database_migration(connection: Connection) -> None:
    current_file_path = os.path.dirname(os.path.abspath(__file__))
    alembic_cfg = Config()
    alembic_cfg.set_main_option("script_location", f"{current_file_path}/alembic")

    alembic_cfg.attributes["connection"] = connection
    command.upgrade(alembic_cfg, "head")
