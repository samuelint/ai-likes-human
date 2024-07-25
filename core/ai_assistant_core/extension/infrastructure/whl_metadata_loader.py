import os
import zipfile


class WhlMetadataLoader:
    def __init__(self, wheel_path: str) -> None:
        self.wheel_path = wheel_path

        file_path = os.path.basename(wheel_path)
        self.module_name = file_path.split("-")[0]

    def read_metadata_as_string(self) -> str:
        with zipfile.ZipFile(self.wheel_path, "r") as wheel:
            for file in wheel.namelist():
                if file.endswith("METADATA"):
                    with wheel.open(file) as metadata_file:
                        metadata = metadata_file.read().decode("utf-8")
                        return metadata
            raise ValueError("No METADATA file found in the wheel archive")

    def read_metadata_as_set(self):
        metadata = {}
        str_metadata = self.read_metadata_as_string()
        metadata_lines = str_metadata.split("\n")

        for line in metadata_lines:
            if ": " in line:
                key, value = line.split(": ", 1)
                metadata[key.lower()] = value

        return metadata
