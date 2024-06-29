from tests.test_functional.functional_test_utils import create_test_client

test_api = create_test_client()


base_route = "/configuration/kv"


class TestConfigurationKeyValueCrud:

    def test_initial_list_contains_no_configurations(self):
        list_response = test_api.get(base_route)
        list_response_body = list_response.json()

        assert list_response.status_code == 200
        assert len(list_response_body) == 0

    def test_create_read_update_delete(self):
        # Create
        response = test_api.put(
            base_route, json={"key": "some-key", "value": "some-value"}
        )
        created_configuration_item = response.json()
        assert response.status_code == 200
        assert created_configuration_item["key"] == "some-key"
        assert created_configuration_item["value"] == "some-value"

        # List
        list_response = test_api.get(base_route)
        list_response_body = list_response.json()
        assert list_response.status_code == 200
        assert len(list_response_body) == 1
        assert list_response_body[0]["key"] == created_configuration_item["key"]

        # Read
        got_response = test_api.get(f"{base_route}/{created_configuration_item['key']}")
        got_configuration_item = got_response.json()
        assert got_response.status_code == 200
        assert got_configuration_item["key"] == created_configuration_item["key"]
        assert got_configuration_item["value"] == created_configuration_item["value"]

        # Update
        update_response = test_api.put(
            f"{base_route}/{created_configuration_item['key']}",
            json={"key": "some-key", "value": "some-value2"},
        )
        updated_configuration_item = update_response.json()
        assert update_response.status_code == 200
        assert updated_configuration_item["key"] == "some-key"
        assert updated_configuration_item["value"] == "some-value2"

        # Delete
        delete_response = test_api.delete(
            f"{base_route}/{created_configuration_item['key']}"
        )
        assert delete_response.status_code == 200
        list_response = test_api.get(base_route)
        list_response_body = list_response.json()
        assert list_response.status_code == 200
        assert len(list_response_body) == 0
