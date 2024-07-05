from tests.test_functional.functional_test_utils import create_test_client

test_api = create_test_client()


class TestLLMLocalModel:

    def test_theres_at_least_one_local_model(self):
        list_response = test_api.get("/configuration/llm/local")
        list_response_body = list_response.json()
        assert list_response.status_code == 200
        assert len(list_response_body) > 0
