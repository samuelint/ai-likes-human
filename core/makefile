
.PHONY: dev run
dev run:
	poetry env use ./.venv/bin/python
	DEBUG=true poetry run uvicorn ai_assistant_core.main:api --reload

.PHONY: test
test:
	poetry run pytest

.PHONY: test-watch
test-watch:
	poetry run ptw

.PHONY: lint
lint:
	poetry run flake8 ai-assistant-core tests

.PHONY: bundle build
bundle build:
	poetry run pyinstaller -y --clean ai-assistant-core.spec