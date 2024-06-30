
.PHONY: dev run
dev run:
	poetry env use ./.venv/bin/python
	DEBUG=true poetry run uvicorn ai_assistant_core.main:create_app --reload

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


# Usage: `make add-migration m="YOUR MIGRATION NAME"`
.PHONY: add-migration
add-migration:
	@if [ -z "$(m)" ]; then \
		echo "Error: Missing migration name. Use 'make add-migration m=\"YOUR MIGRATION NAME\"'"; \
		exit 1; \
	fi
	@cd ./ai_assistant_core/infrastructure && poetry run alembic revision -m "$(m)"