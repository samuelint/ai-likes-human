# AI Likes Human Core

## Domains

### Assistant

Everything related to Assistant and it's storage.
Thread, Message, Run, Etc..

Very closed to OpenAI Assistant API.

### Chat Completion

Everything related to LLM calling in the OpenAI Chat Completion API format.

### Configuration

App configuration. Mainly storage of various configurations.
For example:

- Api Keys
- Selected LLM
- LLM Parameters

### Entities

Database entities generated by `sea-orm` migration system.

### infrastructure

Infrastructure code used by every other domains.

### llm

LLM Inference. Implementation of every used LLM in this app.
For example:

- OpenAI
- Anthropic Claude
- LLamaCPP
- Etc...

Also implement agent creation. Creation of a custom agent based on existing LLM (using langgraph rust).
Can add, tool, chain of tought, etc..

### profile

User profiles. For example system prompt for a `Software Engineer`

### utils

Utilities functions used in every other domains