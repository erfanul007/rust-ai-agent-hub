# Rust AI Agent Hub

A Rust CLI application for chatting with specialized AI agents powered by OpenAI's API. Create conversations with different AI personalities, each designed for specific tasks and expertise areas.

## Features

- **Multiple AI Agents**: Chat with different specialized agents, each with unique personalities and expertise
- **Streaming Responses**: Real-time token-by-token response display
- **Configuration-Based**: Agents are defined in a simple `agents.toml` file
- **CLI Interface**: Easy-to-use command-line interface with agent selection
- **Async/Await**: Built with Tokio for efficient async operations

## Available Agents

- **default** - General-purpose helpful assistant
- **solution-architect** (alias: `architect`) - Software architecture and system design expert
- **pirate-explorer** (aliases: `pirate`, `one-piece`, `straw-hat`) - Fun One Piece-inspired adventure agent
- **detective-l** (aliases: `investigator`, `l`, `detective`) - Analytical investigation specialist

## Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- OpenAI API key

## Setup

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd rust-ai-agent-hub
   ```

2. **Set up environment variables:**
   
   Copy `.env.example` to `.env` and add your OpenAI API key:
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` file:
   ```
   OPENAI_API_KEY=your_actual_api_key_here
   OPENAI_BASE_URL=https://api.openai.com/v1
   OPENAI_MODEL=gpt-3.5-turbo
   OPENAI_TIMEOUT_SECONDS=120
   ```

3. **Build the project:**
   ```bash
   cargo build --release
   ```

## Usage

**List all available agents:**
```bash
cargo run -- list-agents
```

**Chat with the default agent:**
```bash
cargo run -- chat
```

**Chat with a specific agent:**
```bash
cargo run -- chat --agent solution-architect
# or use aliases:
cargo run -- chat --agent architect
cargo run -- chat --agent pirate
cargo run -- chat --agent detective
```

**During chat:**
- Type your messages and press Enter
- Type `exit`, `quit`, or `bye` to end the conversation
- Type `agents` to see available agents

## Project Structure

```
src/
├── main.rs              # Entry point and CLI handling
└── core/
    ├── mod.rs           # Module declarations
    ├── agent.rs         # Agent configuration and management
    ├── chat.rs          # Chat session handling
    ├── cli.rs           # Command-line interface definitions
    ├── data.rs          # Core data structures (messages, conversations)
    ├── error.rs         # Custom error types
    └── llm_client.rs    # OpenAI API client with streaming
```

## Key Dependencies

- `tokio` - Async runtime
- `clap` - Command-line argument parsing
- `reqwest` - HTTP client for API calls
- `serde` - JSON serialization/deserialization
- `futures` - Stream processing utilities
- `thiserror` - Custom error types
- `config` & `toml` - Configuration file handling
- `dotenvy` - Environment variable loading

## Technical Features

- **Async Architecture**: Built with Tokio for efficient concurrent operations
- **Streaming Responses**: Real-time token-by-token display using OpenAI's streaming API
- **Modular Design**: Clean separation of concerns across well-organized modules
- **Configuration-Driven**: Easy agent customization through TOML configuration
- **Robust Error Handling**: Comprehensive error management with custom types
- **Secure Configuration**: Environment-based API key management

## License

MIT License - see LICENSE file for details.