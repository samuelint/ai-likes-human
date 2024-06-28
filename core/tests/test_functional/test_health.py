from tests.test_functional.functional_test_utils import create_test_client

test_api = create_test_client()


def test_health():
    response = test_api.get("/health")

    assert response.status_code == 200
    assert response.text == "healthy"
