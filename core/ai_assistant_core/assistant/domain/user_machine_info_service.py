import platform
import sys
from injector import inject


@inject
class UserMachineInfoService:
    def __init__(self):
        pass

    def get_machine_info(self) -> str:
        system = platform.system()
        architecture = platform.machine()
        release = platform.release()
        version = sys.version_info

        return f"System: {system}\nArchitecture: {architecture}\nRelease: {release}\nVersion: {version}"
