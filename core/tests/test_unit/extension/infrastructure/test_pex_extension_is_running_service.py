from decoy import Decoy
import pytest
from ai_assistant_core.extension.infrastructure.pex_extension_api_factory import (
    PexExtensionApiFactory,
)
from ai_assistant_core.extension.infrastructure.pex_extension_is_running_service import (
    PexExtensionIsRunningService,
)
from ai_assistant_core.extension.infrastructure.pex_extension_metadata_api import (
    PexExtensionApi,
)


@pytest.fixture
def extension_api_factory(decoy: Decoy) -> PexExtensionApiFactory:
    return decoy.mock(cls=PexExtensionApiFactory)


@pytest.fixture
def instance(
    extension_api_factory: PexExtensionApiFactory,
) -> PexExtensionIsRunningService:
    return PexExtensionIsRunningService(extension_api_factory=extension_api_factory)


class TestPexExtensionIsRunningService__is_running:

    def test_loaded_extension_ip_is_up(
        self,
        decoy: Decoy,
        instance: PexExtensionIsRunningService,
        extension_api_factory: PexExtensionApiFactory,
    ):
        api_mock = decoy.mock(cls=PexExtensionApi)
        decoy.when(api_mock.is_up()).then_return(True)
        decoy.when(
            extension_api_factory.create_from_extension_name(extension_name="test")
        ).then_return(api_mock)

        result = instance.is_running(extension_name="test")

        assert result is True

    def test_loaded_extension_down_is_not_up(
        self,
        decoy: Decoy,
        instance: PexExtensionIsRunningService,
        extension_api_factory: PexExtensionApiFactory,
    ):
        api_mock = decoy.mock(cls=PexExtensionApi)
        decoy.when(api_mock.is_up()).then_return(False)
        decoy.when(
            extension_api_factory.create_from_extension_name(extension_name="test")
        ).then_return(api_mock)

        result = instance.is_running(extension_name="test")

        assert result is False

    def test_no_loaded_extension_is_not_up(
        self,
        decoy: Decoy,
        instance: PexExtensionIsRunningService,
        extension_api_factory: PexExtensionApiFactory,
    ):
        decoy.when(
            extension_api_factory.create_from_extension_name(extension_name="test")
        ).then_raise(ValueError("No such extension"))

        result = instance.is_running(extension_name="test")

        assert result is False


class TestPexExtensionIsRunningService__wait_for_running:

    def test_returns_when_up(
        self,
        decoy: Decoy,
        instance: PexExtensionIsRunningService,
        extension_api_factory: PexExtensionApiFactory,
    ):
        api_mock = decoy.mock(cls=PexExtensionApi)
        decoy.when(api_mock.is_up()).then_return(True)
        decoy.when(
            extension_api_factory.create_from_extension_name(extension_name="test")
        ).then_return(api_mock)

        result = instance.wait_for_running(extension_name="test")

        assert result is True

    def test_returns_when_become_up_after_a_some_time(
        self,
        decoy: Decoy,
        instance: PexExtensionIsRunningService,
        extension_api_factory: PexExtensionApiFactory,
    ):
        api_mock = decoy.mock(cls=PexExtensionApi)
        global is_up_call_count
        is_up_call_count = 0

        def mock_is_up():
            global is_up_call_count
            is_up_call_count += 1

            if is_up_call_count == 1:
                return False
            else:
                return True

        decoy.when(api_mock.is_up()).then_do(mock_is_up)
        decoy.when(
            extension_api_factory.create_from_extension_name(extension_name="test")
        ).then_return(api_mock)

        result = instance.wait_for_running(extension_name="test")

        assert result is True

    def test_returns_throws_when_never_up(
        self,
        decoy: Decoy,
        instance: PexExtensionIsRunningService,
        extension_api_factory: PexExtensionApiFactory,
    ):
        api_mock = decoy.mock(cls=PexExtensionApi)
        decoy.when(api_mock.is_up()).then_return(False)
        decoy.when(
            extension_api_factory.create_from_extension_name(extension_name="test")
        ).then_return(api_mock)

        with pytest.raises(TimeoutError):
            instance.wait_for_running(extension_name="test", timeout_sec=1)
