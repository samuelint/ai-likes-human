import os
import sys
import subprocess
import importlib.util
from base_assistant_extension import BaseExtension


class WhlExtensionLoader:
    def __init__(self, wheel_path: str) -> None:
        self.wheel_path = wheel_path

        file_path = os.path.basename(wheel_path)
        self.module_name = file_path.split("-")[0]

    def install(self):
        subprocess.check_call(
            [
                sys.executable,
                "-m",
                "pip",
                "install",
                "--force-reinstall",
                "--no-deps",
                self.wheel_path,
            ]
        )

    def uninstall(self):
        subprocess.check_call(
            [
                sys.executable,
                "-m",
                "pip",
                "uninstall",
                "-y",
                self.wheel_path,
            ]
        )
        if self.module_name in sys.modules:
            del sys.modules[self.module_name]

    def is_installed(self):
        try:
            self.load()
            return True
        except ImportError:
            return False

    def load(self) -> BaseExtension:
        module = importlib.import_module(self.module_name)

        return module.Extension()
