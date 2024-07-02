"""serper api key

Revision ID: ac6b1a443629
Revises: c9e26e1e7481
Create Date: 2024-06-29 22:12:45.464608

"""

from typing import Sequence, Union

from alembic import op
from sqlalchemy.sql import table, column
from sqlalchemy import Integer, String


# revision identifiers, used by Alembic.
revision: str = "ac6b1a443629"
down_revision: Union[str, None] = "c9e26e1e7481"
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
            {"key": "SERPER_API_KEY", "value": ""},
        ],
    )


def downgrade() -> None:
    pass
