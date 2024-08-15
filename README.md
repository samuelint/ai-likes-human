# AI Assistant

[![License: CC BY-NC-ND 4.0](https://img.shields.io/badge/License-CC_BY--NC--ND_4.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-nd/4.0/) [![GitHub release](https://img.shields.io/github/v/release/samuelint/ai-assistant)](https://github.com/samuelint/ai-assistant/releases/latest)

A personal AI assistant.
This is **NOT** another LLM UI wrapper. But a framework to provide a set of tools for local AI agents and improve productivity using LangGraph.

Open possibilities for a wide range of private automation & productivity.

## Usage

Download the latest version from the release page.
[![GitHub release](https://img.shields.io/github/v/release/samuelint/ai-assistant)](https://github.com/samuelint/ai-assistant/releases/latest)

## Features

### Tools

- ✅ Local inference for private data
- ✅ Computer screenshot
- ✅ Web Search
- ✅ Image generation

### LLM Vendors

- ✅ Local Phi3, LLama3
- ✅ OpenAI
- ✅ Anthropic

### Compatible on

- ✅ macOS
- ✅ Windows
  - :warning: Extensions are not supported yet
- ✅ Ubuntu/Debian

https://github.com/samuelint/ai-assistant/assets/5473142/50c44730-1fc7-40d8-9681-78e772fb9bc1

## Project Structure

Project is currently in a mono repo. This structure simplify the integration of the `core` backend into the standalone desktop app.

- `core` | Core Web Server, AI & Agent integration
- `webapp` | Webapp / Desktop App

## Development

#### Prerequisites:

First install the following on your machine

- **Core**
  - `python` \>= 3.9 | https://www.python.org/downloads/
  - `poetry` | https://python-poetry.org/docs/
- **Webapp**
  - `pnpm` | https://pnpm.io/installation

##### For Windows Development

- Windows SDK, CMake | Install Visual Studio (https://visualstudio.microsoft.com/) and install
  - Desktop development with C++
- `make` |

#### Build

At the root of the project, execute.

```bash
make build
```

Once completed, a directory `./dist` should be created at the root of the project, containing binaries of the app.

#### Dev

In order to start the development environment, you should build the `core` first. Follow [buid](#build) steps and copy paste the binaries from `./dist` to `./webapp/src-tauri/bin`.

In `core` directory, install dependencies with `peotry install`. Then `make dev` or `make run` to start the server.

Once started, `core` should be served on port `8000`. You can get a health status of the server on `localhost:8000`.

In the `webapp` directory, install dependencies with `pnpm install` and then run `pnpm dev`.

## Extensions

Extensions need to implement the following project:
https://github.com/samuelint/ai-assistant-extension

A full example is available here: https://github.com/samuelint/ai-assistant/tree/main/extensions/joke

## What is this project solving?

### Rapidly Evolving LLM Models:

- The best large language model (LLM) changes every 3 to 6 months, requiring frequent subscription changes.
- Switching between LLM vendors often results in the loss of certain features, leading to inconsistencies.

### AI Agent Adoption Challenges:

- Creating an AI agent prototype is easy, a product is hard. Deployment & reliability takes a lot of efforts.
- Using Agent is a long & hard process. They can do incredible things, but are still hard to deploy & use rapidly.
- Most services are cloud based, but it seem to be contradictory with business needs (they do not want to share sensitive data to the cloud).
- There are a lot of agents/GPT/libraries that do not work well or expected. Adding reluctance to AI Agent adoption.
- Non-technical users may have difficulty identifying what constitutes an AI agent.

### Privacy and Data Concerns:

- Some LLM vendors retain users' personal information.
- Businesses are hesitant to use LLMs for sensitive or private data.

### Lack of personalization & Repetitive Question Formulation:

- Users often ask the same types of questions to LLMs, but in different ways.
- Users need to translate their visual observations into text to communicate with LLMs.

### Subscription-based AI Services:

- There are multiple AI services available on a subscription basis.
- Many of these services cannot be tested until paid for, and often do not perform well.
- Some of these services can be replicated locally at a fraction of the cost.

Let's try to fix that by creating a personal assistant running locally on user computer.

## License

Project is under **Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International (CC BY-NC-ND 4.0)** license.

https://creativecommons.org/licenses/by-nc-nd/4.0/

If you want to use this code for commercial use, please contact me for permission.
