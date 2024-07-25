class InvalidFileFormat(ValueError):
    def __init__(self) -> None:
        super().__init__("invalid file format")
