"""anthripic api key

Revision ID: c9e26e1e7481
Revises: 4fc18413d4ba
Create Date: 2024-06-29 22:12:39.921146

"""

from typing import Sequence, Union

from alembic import op
from sqlalchemy.sql import table, column
from sqlalchemy import Integer, String


# revision identifiers, used by Alembic.
revision: str = "c9e26e1e7481"
down_revision: Union[str, None] = "4fc18413d4ba"
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
            {"key": "ANTHROPIC_API_KEY", "value": ""},
        ],
    )


def downgrade() -> None:
    pass
