import os
import pytest
from unittest.mock import patch
from ai_assistant_core.infrastructure.session_factory import SqlAlchemySessionFactory
from platformdirs import user_data_dir


@pytest.mark.skipif(os.name != "posix", reason="Test runs only on Unix-based systems")
class TestUnix:
    local_db_path = "/tmp/some/sub-dir/test.db"

    def test_local_database_directory_is_created(self):
        instance = SqlAlchemySessionFactory(
            database_url=f"sqlite:///{self.local_db_path}"
        )
        with patch("os.makedirs") as mock_makedirs, patch(
            "os.path.exists", side_effect=lambda path: path != "/tmp/some/sub-dir"
        ):
            instance._create_sqlite_path()

            mock_makedirs.assert_called_once_with("/tmp/some/sub-dir")


@pytest.mark.skipif(os.name != "nt", reason="Test runs only on Windows")
class TestWindows:
    local_db_path = f"{user_data_dir()}\\some\\test.db"

    def test_local_database_directory_is_created(self):
        instance = SqlAlchemySessionFactory(
            database_url=f"sqlite:///{self.local_db_path}"
        )
        with patch("os.makedirs") as mock_makedirs, patch(
            "os.path.exists",
            side_effect=lambda path: path != f"{user_data_dir()}\\some",
        ):
            instance._create_sqlite_path()

            mock_makedirs.assert_called_once_with(f"{user_data_dir()}\\some")
