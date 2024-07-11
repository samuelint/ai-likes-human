export PYTHONPATH=$(shell pwd)

.PHONY: bundle build
bundle build:
	@cd core && make build
	@cd webapp && pnpm run build
	@mkdir -p dist
	@cp -r webapp/src-tauri/target/release/bundle/* dist/

