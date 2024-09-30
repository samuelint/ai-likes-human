# AI Likes Human

An alternative multi-platform LLM client.
[![License: CC BY-NC-ND 4.0](https://img.shields.io/badge/License-CC_BY--NC--ND_4.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-nd/4.0/) [![GitHub release](https://img.shields.io/github/v/release/samuelint/ai-assistant)](https://github.com/samuelint/ai-assistant/releases/latest)

## Structure

- `core/`: The core AI functionality implemented in Rust.
- `webapp/`: The web interface implemented in React.
- `desktop/`: The desktop application implemented in Tauri.

## Development

### Core

- Install the Sea ORM CLI with `cargo install sea-orm-cli`.
- Run `cargo test --all-features` to run all tests.

### Webapp

- Run `pnpm run lint` to run linting.
- Run `pnpm run test` to run tests.
- Run `pnpm run build` to build the webapp.

### Desktop

- Run `pnpm run build` to build the desktop application.

## GitHub Actions

The GitHub Actions workflow is defined in `.github/workflows/quality.yml`. It runs all tests and builds the webapp and desktop applications on every push.
