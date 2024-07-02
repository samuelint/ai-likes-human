"""configuration_items

Revision ID: a6d449d9bab9
Revises:
Create Date: 2024-06-29 13:56:04.115189

"""

from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa


# revision identifiers, used by Alembic.
revision: str = "a6d449d9bab9"
down_revision: Union[str, None] = None
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade():
    try:
        op.create_table(
            "configuration_item",
            sa.Column("id", sa.Integer, primary_key=True),
            sa.Column("key", sa.String(), unique=True, index=True),
            sa.Column("value", sa.String()),
        )
    except sa.exc.OperationalError:
        # table already exists
        pass


def downgrade() -> None:
    pass
    op.drop_table("configuration_item")
