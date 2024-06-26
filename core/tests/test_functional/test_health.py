from fastapi.testclient import TestClient
from ai_assistant_core.main import api

test_api = TestClient(api)


def test_health():
    response = test_api.get("/health")

    assert response.status_code == 200
    assert response.text == "healthy"
