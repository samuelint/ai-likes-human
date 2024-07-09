import pytest
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from sqlalchemy.orm.session import Session
from ai_assistant_core.assistant.infrastructure.sqlalchemy_thread_repository import (
    SqlalchemyThreadRepository,
)
from ai_assistant_core.infrastructure.sqlalchemy import Base


@pytest.fixture(scope="session")
def session() -> Session:
    engine = create_engine("sqlite:///:memory:", echo=False)
    SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
    db = SessionLocal()
    Base.metadata.create_all(bind=engine)

    return db


@pytest.fixture(scope="session")
def instance(session) -> SqlalchemyThreadRepository:
    return SqlalchemyThreadRepository(db=session)


class TestList:
    def test_list(self, instance: SqlalchemyThreadRepository):
        thread1 = instance.create()
        thread2 = instance.create()
        thread3 = instance.create()

        result = instance.list().data

        assert len(result) == 3
        assert result[0].id == thread1.id
        assert result[1].id == thread2.id
        assert result[2].id == thread3.id

    def test_list_limit(self, instance: SqlalchemyThreadRepository):
        thread1 = instance.create()
        thread2 = instance.create()
        instance.create()

        result = instance.list(limit=2).data

        assert len(result) == 2
        assert result[0].id == thread1.id
        assert result[1].id == thread2.id

    def test_list_order_asc(self, instance: SqlalchemyThreadRepository):
        thread1 = instance.create(created_at=10000)
        thread2 = instance.create(created_at=20000)
        thread3 = instance.create(created_at=30000)

        result = instance.list(order="asc").data

        assert result[0].id == thread1.id
        assert result[1].id == thread2.id
        assert result[2].id == thread3.id

    def test_list_order_desc(self, instance: SqlalchemyThreadRepository):
        thread1 = instance.create(created_at=10000)
        thread2 = instance.create(created_at=20000)
        thread3 = instance.create(created_at=30000)

        result = instance.list(order="desc").data

        assert result[0].id == thread3.id
        assert result[1].id == thread2.id
        assert result[2].id == thread1.id

    def test_list_order_desc_with_limit(self, instance: SqlalchemyThreadRepository):
        instance.create(created_at=10000)
        thread2 = instance.create(created_at=20000)
        thread3 = instance.create(created_at=30000)

        result = instance.list(limit=2, order="desc").data

        assert result[0].id == thread3.id
        assert result[1].id == thread2.id

    def test_list_after(self, instance: SqlalchemyThreadRepository):
        thread1 = instance.create(created_at=10000)
        thread2 = instance.create(created_at=20000)
        thread3 = instance.create(created_at=30000)

        result = instance.list(after=thread1.id).data

        assert len(result) == 2
        assert result[0].id == thread2.id
        assert result[1].id == thread3.id

    def test_list_before(self, instance: SqlalchemyThreadRepository):
        thread1 = instance.create(created_at=10000)
        thread2 = instance.create(created_at=20000)
        instance.create(created_at=30000)

        result = instance.list(before=thread2.id).data

        assert len(result) == 1
        assert result[0].id == thread1.id
