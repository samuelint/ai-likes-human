export PYTHONPATH=$(shell pwd)

.PHONY: test
test:
	poetry run pytest


.PHONY: build
build:
	poetry build

.PHONY: bundle
bundle:
	@mkdir -p dist
	poetry run pex . -m joke_extension.main -o dist/extension.pex