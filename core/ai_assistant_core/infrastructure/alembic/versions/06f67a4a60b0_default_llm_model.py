"""default_llm_model

Revision ID: 06f67a4a60b0
Revises: eb42f1f8ec8b
Create Date: 2024-07-04 09:13:28.419477

"""

from typing import Sequence, Union

from alembic import op
from sqlalchemy.sql import table, column
from sqlalchemy import Integer, String


# revision identifiers, used by Alembic.
revision: str = "06f67a4a60b0"
down_revision: Union[str, None] = "eb42f1f8ec8b"
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
            {"key": "DEFAULT_LLM_MODEL", "value": "openai:gpt-3.5-turbo"},
        ],
    )


def downgrade() -> None:
    pass
