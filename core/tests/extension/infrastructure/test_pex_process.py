import io
import os
import signal
import sys
import time

import pytest
from ai_assistant_core.extension.infrastructure.pex_process import PexProcess

test_file_dir = os.path.dirname(os.path.abspath(__file__))
test_pext_file_path = os.path.join(test_file_dir, "script_for_test.py")


@pytest.fixture
def instance() -> PexProcess:
    return PexProcess(pex_path=test_pext_file_path)


class TestPexProcess:

    def test_process_stdout_is_forwarded_to_stdout(self, instance: PexProcess):
        stdout_capture = io.StringIO()
        original_stdout = sys.stdout
        try:
            sys.stdout = stdout_capture

            instance.start()
            time.sleep(1)
        finally:
            sys.stdout = original_stdout

        stdout_contents = stdout_capture.getvalue()
        assert "forwarded stdout -- 0" in stdout_contents

    def test_process_stderr_is_forwarded_to_stderr(self, instance: PexProcess):
        stderr_capture = io.StringIO()
        original_stderr = sys.stderr
        try:
            sys.stderr = stderr_capture

            instance.start()
            time.sleep(1)
        finally:
            sys.stderr = original_stderr

        stderr_contents = stderr_capture.getvalue()
        assert "forwarded stderr -- 0" in stderr_contents

    def test_process_is_started_when_started(self, instance: PexProcess):
        instance.start()

        assert instance.isStarted() is True

    def test_process_is_not_started_when_never_started(self, instance: PexProcess):
        assert instance.isStarted() is False

    def test_process_is_not_started_when_started_and_then_stopped(
        self, instance: PexProcess
    ):
        instance.start()
        time.sleep(0.2)
        instance.stop()

        assert instance.isStarted() is False

    def test_when_process_is_started_log_forward_is_alive(self, instance: PexProcess):
        instance.start()

        assert instance.stdout_thread.is_alive() is True
        assert instance.stderr_thread.is_alive() is True

    def test_when_process_is_stopped_log_forward_is_stopped(self, instance: PexProcess):
        instance.start()
        instance.stop()

        assert instance.stdout_thread.is_alive() is False
        assert instance.stderr_thread.is_alive() is False

    def test_when_process_is_terminated_externally_log_forward_is_stopped(
        self, instance: PexProcess
    ):
        instance.start()
        os.kill(instance.process.pid, signal.SIGTERM)
        time.sleep(0.01)

        assert instance.stdout_thread.is_alive() is False
        assert instance.stderr_thread.is_alive() is False

    def test_ipc_port_is_pass_as_argument(self):
        instance = PexProcess(pex_path=test_pext_file_path, ipc_port=456)

        stdout_capture = io.StringIO()
        original_stdout = sys.stdout
        try:
            sys.stdout = stdout_capture

            instance.start()
            time.sleep(1)
        finally:
            sys.stdout = original_stdout

        stdout_contents = stdout_capture.getvalue()
        assert "['--port', '456']" in stdout_contents

    def test_inference_url_is_pass_as_argument(self):
        instance = PexProcess(
            pex_path=test_pext_file_path,
            inference_url="http://127.0.0.1",
        )

        stdout_capture = io.StringIO()
        original_stdout = sys.stdout
        try:
            sys.stdout = stdout_capture

            instance.start()
            time.sleep(1)
        finally:
            sys.stdout = original_stdout

        stdout_contents = stdout_capture.getvalue()
        assert "['--inference-url', 'http://127.0.0.1']" in stdout_contents
