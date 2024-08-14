import subprocess
import sys
import threading
from typing import IO, Optional, TextIO


class PexProcess:

    process: Optional[subprocess.Popen] = None
    stdout_thread: Optional[threading.Thread] = None
    stderr_thread: Optional[threading.Thread] = None

    @property
    def pid(self) -> int:
        if self.process is None or self.process.pid is None:
            raise ValueError("PexProcess not started yet")
        return self.process.pid

    def __init__(
        self,
        pex_path: str,
        ipc_port: Optional[int] = None,
        inference_url: Optional[str] = None,
    ):
        self.pex_path = pex_path

        self.ipc_port = ipc_port
        self.inference_url = inference_url

    def start(self):
        command = self._build_command()
        self.process = subprocess.Popen(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )

        self._start_log_forward(self.process)

    def stop(self):
        if self.process:
            self.process.terminate()
            self.process.wait()

            self._stop_log_forward()

    def isStarted(self):
        if self.process is None:
            return False

        return self.process.poll() is None

    def _build_command(self) -> list[str]:
        command = [sys.executable, self.pex_path]
        if self.ipc_port is not None:
            command += ["--port", f"{self.ipc_port}"]

        if self.inference_url is not None:
            command += ["--inference_url", self.inference_url]

        return command

    def _start_log_forward(self, process: subprocess.Popen):
        self.stdout_thread = threading.Thread(
            target=self._stream_output, args=(process.stdout, sys.stdout)
        )
        self.stderr_thread = threading.Thread(
            target=self._stream_output, args=(process.stderr, sys.stderr)
        )

        self.stdout_thread.start()
        self.stderr_thread.start()

    def _stop_log_forward(self):
        if self.stdout_thread:
            self.stdout_thread.join()
        if self.stderr_thread:
            self.stderr_thread.join()

    def _stream_output(self, pipe: IO, output: TextIO):
        try:
            for line in iter(pipe.readline, ""):
                if line:
                    output.write(line)
        finally:
            pipe.close()
