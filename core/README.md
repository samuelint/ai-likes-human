# AI Assistant Project 💁

Proof of Concept 🧪 for an AI personalized assistant

## Goals of the assistant:

The assistant does everything a real human assistant would do while being behind a computer.

- Helpful (not a toy)
- Access all of my personnal data
- Answer questions on my data (by me)
- Answer question on my for other people (restricted to some data)
- Platform agnostic,
  - Answer access data on multiple platform
  - Can use local or hosted LLM
- Gives confidence %
- Gives sources when answer is related to my external data
- Gives inspirational quotes when needed
- Act on my behalf (with my permission)

## Roadmap

- [] Gives confidence level
- [] Access data from
  - [] Google drive
  - [] Microsoft OneDrive
  - [] Google Gmail
  - [] LinkedIn
  - [] Others.. TBD. Let's see how it goes with aboves before adding more
- [] Give sources
- [] API to interact with the assistant
- [] UI
  - [] Webbase
  - [] Desktop (Electro)
- [] Automatically answer emails

## Setup

`poetry install`
`poetry shell`

## Commands

### Run tests

| Command                         | Command                |
| ------------------------------- | ---------------------- |
| Run Tests                       | `make test`            |
| Watch Tests (Run on change)     | `make test-watch`      |
| Server Dev                      | `make run`, `make dev` |
| Bundle app to single executable | `make bundle`          |
