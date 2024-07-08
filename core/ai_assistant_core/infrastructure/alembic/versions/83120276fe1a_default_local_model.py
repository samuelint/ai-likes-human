"""default-local-model

Revision ID: 83120276fe1a
Revises: 06f67a4a60b0
Create Date: 2024-07-05 12:02:15.948995

"""

import os
from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa
from sqlalchemy.sql import table


# revision identifiers, used by Alembic.
revision: str = "83120276fe1a"
down_revision: Union[str, None] = "06f67a4a60b0"
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    try:
        op.create_table(
            "local_model_index",
            sa.Column("name", sa.String, primary_key=True),
            sa.Column("type", sa.String),
            sa.Column("local_path", sa.String, nullable=True),
            sa.Column("repo_id", sa.String, nullable=True),
            sa.Column("remote_path", sa.String, nullable=True),
        )
    except sa.exc.OperationalError:
        # table already exists
        pass

    local_model_index_table = table(
        "local_model_index",
        sa.Column("name", sa.String),
        sa.Column("type", sa.String),
        sa.Column("local_path", sa.String),
        sa.Column("repo_id", sa.String),
        sa.Column("remote_path", sa.String),
    )

    local_path = os.path.join(
        os.path.expanduser("~/.cache/lm-studio/models"),
        "lmstudio-community/Meta-Llama-3-8B-Instruct-GGUF/Meta-Llama-3-8B-Instruct-Q4_K_M.gguf",
    )
    op.bulk_insert(
        local_model_index_table,
        [
            {"name": "llama3", "type": "local", "local_path": local_path},
        ],
    )


def downgrade() -> None:
    pass
