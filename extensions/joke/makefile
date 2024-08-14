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
	poetry run pex . -m joke_extension.main \
	--platform=macosx_11_0_arm64-cp-3.12.0-cp312 \
	--platform=linux-x86_64-cp-3.12.0-cp312 \
	-o dist/extension.pex