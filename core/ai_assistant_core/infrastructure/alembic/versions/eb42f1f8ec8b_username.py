"""USERNAME

Revision ID: eb42f1f8ec8b
Revises: ac6b1a443629
Create Date: 2024-07-03 11:15:08.811587

"""

from typing import Sequence, Union

from alembic import op
from sqlalchemy.sql import table, column
from sqlalchemy import Integer, String

# revision identifiers, used by Alembic.
revision: str = "eb42f1f8ec8b"
down_revision: Union[str, None] = "ac6b1a443629"
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
            {"key": "USERNAME", "value": ""},
        ],
    )


def downgrade() -> None:
    pass
