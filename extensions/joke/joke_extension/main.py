from base_assistant_extension import (
    entry,
)
from joke_extension.extension import Extension


def main():
    entry(extension=Extension())


if __name__ == "__main__":
    main()
