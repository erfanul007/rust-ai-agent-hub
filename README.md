# Rust AI Agent Hub

A Rust-native application for creating and managing multiple, specialized AI agents powered by Large Language Models (LLMs).

This project is designed to be a robust, configurable, and extensible framework for experimenting with different AI personas and capabilities. It is built with a focus on clean architecture, modern tooling, and a CLI-first approach.

For a detailed, step-by-step guide on how this project is being built, please see the [Development Plan](DEVELOPMENT.md).

---

## Features

- **Multi-Agent Architecture:** Define and manage multiple agents, each with a unique system prompt, persona, and purpose.
- **Configuration-Driven:** Agents are defined in a simple `agents.toml` file, allowing for easy modification without changing code.
- **CLI-First:** A powerful command-line interface for interacting with agents, listing available agents, and managing conversations.
- **Asynchronous Core:** Built on Tokio for efficient, non-blocking performance.

---

## Getting Started

### Prerequisites

- **Rust Toolchain:** Ensure you have Rust and Cargo installed. You can get them from [rustup.rs](https://rustup.rs/).
- **LLM API Key:** You will need an API key from an LLM provider (e.g., OpenAI, Anthropic).

### Installation & Setup

1.  **Clone the Repository:**
    ```sh
    git clone <repository-url>
    cd rust-ai-agent-hub
    ```

2.  **Configure Environment Variables:**
    Create a `.env` file in the project root based on `.env.example`:
    ```
    # .env
    OPENAI_API_KEY=your_openai_api_key_here
    OPENAI_BASE_URL=https://api.openai.com/v1
    OPENAI_MODEL=gpt-3.5-turbo
    OPENAI_TIMEOUT_SECONDS=120
    ```
    
    **Required Variables:**
    - `OPENAI_API_KEY`: Your OpenAI API key
    - `OPENAI_BASE_URL`: The API endpoint (default: OpenAI's official endpoint)
    - `OPENAI_MODEL`: The model to use (e.g., gpt-3.5-turbo, gpt-4)
    - `OPENAI_TIMEOUT_SECONDS`: Request timeout in seconds

3.  **Build the Project:**
    ```sh
    cargo build --release
    ```

---

### Usage

Once the application is built, you can interact with it via the command line.

- **List Available Agents:**
  ```sh
  cargo run -- list-agents
  ```

- **Chat with the Default Agent:**
  ```sh
  cargo run -- chat
  ```

- **Chat with a Specific Agent:**
  ```sh
  cargo run -- chat --agent <AGENT_NAME>
  ```

---

## Development

This project follows a milestone-driven development process. For complete details on the architecture, upcoming features, and the implementation roadmap, please see [DEVELOPMENT.md](DEVELOPMENT.md).

To contribute, please fork the repository, create a feature branch, and submit a pull request.

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.