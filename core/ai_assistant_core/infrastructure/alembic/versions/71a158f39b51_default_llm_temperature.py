"""default_llm_temperature

Revision ID: 71a158f39b51
Revises: 83120276fe1a
Create Date: 2024-07-08 12:03:26.699995

"""

from typing import Sequence, Union

from alembic import op
from sqlalchemy.sql import table, column
from sqlalchemy import Integer, String


# revision identifiers, used by Alembic.
revision: str = "71a158f39b51"
down_revision: Union[str, None] = "83120276fe1a"
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
            {"key": "DEFAULT_LLM_TEMPERATURE", "value": "0.1"},
        ],
    )


def downgrade() -> None:
    pass
