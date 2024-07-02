"""openai api key

Revision ID: 4fc18413d4ba
Revises: a6d449d9bab9
Create Date: 2024-06-29 22:10:24.366401

"""

from typing import Sequence, Union

from alembic import op
from sqlalchemy.sql import table, column
from sqlalchemy import Integer, String

# revision identifiers, used by Alembic.
revision: str = "4fc18413d4ba"
down_revision: Union[str, None] = "a6d449d9bab9"
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    configuration_item_table = table(
        "configuration_item",
        column("id", Integer),
        column("key", String),
        column("value", String),
    )

    op.bulk_insert(
        configuration_item_table,
        [
            {"key": "OPENAI_API_KEY", "value": ""},
        ],
    )


def downgrade() -> None:
    pass
