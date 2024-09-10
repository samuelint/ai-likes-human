.PHONY: build
build:
	@cargo build

.PHONY: migration_create
migration_create:
	@diesel migration generate $(filter-out $@,$(MAKECMDGOALS))

.PHONY: migration_up
migration_up:
	@diesel migration run

.PHONY: migration_down
migration_down:
	@diesel migration redo


# %: target is a catch-all pattern rule. It prevents make from failing when you pass additional arguments
%:
	@: