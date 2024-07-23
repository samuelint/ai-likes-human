"""available-models

Revision ID: 08974f9a248f
Revises: 71a158f39b51
Create Date: 2024-07-23 12:09:48.342321

"""

from typing import Sequence, Union

from alembic import op
from sqlalchemy.sql import table, column
from sqlalchemy import Integer, String


# revision identifiers, used by Alembic.
revision: str = "08974f9a248f"
down_revision: Union[str, None] = "71a158f39b51"
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
            {
                "key": "AVAILABLE_MODELS",
                "value": "openai:gpt-4o;openai:gpt-4o-mini;anthropic:claude-3-5-sonnet-20240620;local:llama3",
            },
        ],
    )


def downgrade() -> None:
    pass
