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
                self.wheel_path,
            ]
        )

    def uninstall(self):
        if self.module_name in sys.modules:
            del sys.modules[self.module_name]

    def is_installed(self):
        try:
            importlib.import_module(self.module_name)
            return True
        except ImportError:
            return False

    def load(self) -> BaseExtension:
        self.install()
        module = importlib.import_module(self.module_name)

        return module.Extension()
