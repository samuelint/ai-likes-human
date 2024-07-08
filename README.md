# AI Assistant

[![License: CC BY-NC-ND 4.0](https://img.shields.io/badge/License-CC_BY--NC--ND_4.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-nd/4.0/)

A personalized AI assistant framework


https://github.com/samuelint/ai-assistant/assets/5473142/50c44730-1fc7-40d8-9681-78e772fb9bc1



## Project Structure

Project is currently in a mono repo. This structure simplify the integration of the `core` backend into the standalone desktop app.

- `core` | Core Web Server, AI & Agent integration
- `webapp` | Webapp / Desktop App

## What is this project solving?

### Rapidly Evolving LLM Models:

- The best large language model (LLM) changes every 3 to 6 months, requiring frequent subscription changes.
- Switching between LLM vendors often results in the loss of certain features, leading to inconsistencies.

### AI Agent Adoption Challenges:

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

## Roadmap

- V0 - Foundation for what's next
    - Easy to deploy - Installed as a regular app (no need of docker or install third parties). Everything works out of the box (mac, windows, linux).
    - Access to Local LLM (LLamaCPP)
    - Access to hosted LLM (OpenAI, Anthropic, Groq, etc...)
    - Conversations history
    - Take computer screenshots as context
    - Simple (do not look like a plane control board)
- V1
    - Terminal Client
        - Talk to personallized LLM from any terminal windows (client to the app localhost server)
    - Curated & useful AI agents (using LangGraph)
    Use POC:
        - Resume maker
        - Headshot maker
        - Code Generator

## License

Project is under **Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International (CC BY-NC-ND 4.0)** license.

https://creativecommons.org/licenses/by-nc-nd/4.0/

If you want to use this code for commercial use, please contact me for permission.
