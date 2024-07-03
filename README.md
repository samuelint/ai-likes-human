# AI Assistant

[![License: CC BY-NC-ND 4.0](https://img.shields.io/badge/License-CC_BY--NC--ND_4.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-nd/4.0/)

A personalized AI assistant framework

## Project Structure

Project is currently in a mono repo. This structure simplify the integration of the `core` backend into the standalone desktop app.

- `core` | Core Web Server, AI & Agent integration
- `webapp` | Webapp / Desktop App

## What is this project solving?

#### Rapidly Evolving LLM Models:

- The best large language model (LLM) changes every 3 to 6 months, requiring frequent subscription changes.
- Switching between LLM vendors often results in the loss of certain features, leading to inconsistencies.

#### Repetitive Question Formulation:

- Users often ask the same types of questions to LLMs, but in different ways.
- Users need to translate their visual observations into text to communicate with LLMs.

#### Privacy and Data Concerns:

- Some LLM vendors retain users' personal information.
- Businesses are hesitant to use LLMs for sensitive or private data.

#### Adoption Challenges:

- AI agents are not widely used, and when run on a server, personal information may be stored by third parties.
- Non-technical users may have difficulty identifying what constitutes an AI agent.

#### Subscription-based AI Services:

- There are multiple AI services available on a subscription basis.
- Many of these services cannot be tested until paid for, and often do not perform well.
- Some of these services can be replicated locally at a fraction of the cost.

Let's try to fix that by creating a personal assistant running locally on user computer.

## License

Project is under **Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International (CC BY-NC-ND 4.0)** license.

https://creativecommons.org/licenses/by-nc-nd/4.0/

If you want to use this code for commercial use, please contact me for permission.
