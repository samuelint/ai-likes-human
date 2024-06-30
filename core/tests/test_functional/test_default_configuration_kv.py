from tests.test_functional.functional_test_utils import create_test_client

test_api = create_test_client()


base_route = "/configuration/kv"


class TestDefaultConfigurationKeyValue:

    def test_contains_empty_OPENAI_API_KEY(self):
        response = test_api.get(f"{base_route}/OPENAI_API_KEY")
        response_body = response.json()

        assert response.status_code == 200
        assert response_body["key"] == "OPENAI_API_KEY"
        assert response_body["value"] == ""

    def test_contains_empty_ANTHROPIC_API_KEY(self):
        response = test_api.get(f"{base_route}/ANTHROPIC_API_KEY")
        response_body = response.json()

        assert response.status_code == 200
        assert response_body["key"] == "ANTHROPIC_API_KEY"
        assert response_body["value"] == ""

    def test_contains_empty_SERPER_API_KEY(self):
        response = test_api.get(f"{base_route}/SERPER_API_KEY")
        response_body = response.json()

        assert response.status_code == 200
        assert response_body["key"] == "SERPER_API_KEY"
        assert response_body["value"] == ""
