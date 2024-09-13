.PHONY: build
build:
	@cargo build

# DATABASE_TMP_PATH=$(shell mktemp -d)
DATABASE_TMP_PATH:=$(shell cd $(HOME)/Desktop && pwd)
DATABASE_FILE_PATH=$(DATABASE_TMP_PATH)/data222.db
DATABASE_URL?=sqlite://$(DATABASE_FILE_PATH)?mode=rwc

.PHONY: migration_generate_entities
migration_generate_entities:
	@mkdir -p $(DATABASE_TMP_PATH)
	sea-orm-cli migrate refresh -u $(DATABASE_URL)
	sea-orm-cli generate entity -u $(DATABASE_URL) -o src/entities --with-serde both

.PHONY: test
test:
	@cargo test -- --nocapture


# %: target is a catch-all pattern rule. It prevents make from failing when you pass additional arguments
%:
	@: