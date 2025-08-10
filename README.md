# Rust AI Agent Hub 🤖

Chat with different AI personalities right from your terminal! This Rust CLI app lets you have conversations with specialized agents, each with their own expertise and personality - from helpful assistants to quirky pirates.

## What's Cool About It ✨

- **Multiple AI Personalities**: Switch between different agents - each one has their own vibe and expertise
- **Live Streaming**: Watch responses appear in real-time, just like ChatGPT
- **Easy to Customize**: Add new agents by just editing a simple config file
- **Simple CLI**: No complex setup - just type and chat
- **Fast & Efficient**: Built with async Rust for smooth performance

## Meet the Agents 👥

- **default** - Your friendly neighborhood AI assistant
- **solution-architect** (or just `architect`) - The tech guru who knows all about software design
- **pirate-explorer** (`pirate`, `one-piece`, `straw-hat`) - Ahoy! Adventure awaits with this One Piece-inspired crew member
- **detective-l** (`investigator`, `l`, `detective`) - Sharp analytical mind for solving mysteries and problems

## What You'll Need 📋

- Rust installed on your machine (grab it from [rustup.rs](https://rustup.rs/))
- An OpenAI API key (don't worry, it's free to get started)

## Getting Started 🚀

1. **Grab the code:**
   ```bash
   git clone <repository-url>
   cd rust-ai-agent-hub
   ```

2. **Set up your API key:**
   
   Copy the example env file and add your OpenAI key:
   ```bash
   cp .env.example .env
   ```
   
   Then edit `.env` and add your key:
   ```
   OPENAI_API_KEY=your_actual_api_key_here
   OPENAI_BASE_URL=https://api.openai.com/v1
   OPENAI_MODEL=gpt-3.5-turbo
   OPENAI_TIMEOUT_SECONDS=120
   ```

3. **Build it:**
   ```bash
   cargo build --release
   ```

## How to Use It 💡

**See who's available to chat:**
```bash
cargo run -- list-agents
```

**Start chatting with the default agent:**
```bash
cargo run -- chat
```

**Pick a specific agent to talk to:**
```bash
cargo run -- chat --agent solution-architect
# or use their nicknames:
cargo run -- chat --agent architect
cargo run -- chat --agent pirate
cargo run -- chat --agent detective
```

**While chatting:**
- Just type your message and hit Enter
- Say `exit`, `quit`, or `bye` when you're done
- Type `agents` to see who else you can talk to

## How It's Organized 📁

```
src/
├── main.rs              # Where it all starts
└── core/
    ├── mod.rs           # Module exports
    ├── agent.rs         # Agent management
    ├── chat.rs          # Chat handling
    ├── cli.rs           # Command-line stuff
    ├── data.rs          # Message structures
    ├── error.rs         # Error handling
    └── llm_client.rs    # Talks to OpenAI
```

## What's Under the Hood ⚙️

- `tokio` - Makes everything async and fast
- `clap` - Handles command-line arguments nicely
- `reqwest` - Talks to OpenAI's servers
- `serde` - Converts between Rust and JSON
- `futures` - Handles streaming responses
- `thiserror` - Makes error messages actually helpful
- `config` & `toml` - Reads the agent config file
- `dotenvy` - Loads your API key safely

## The Technical Bits 🔧

- **Async Everything**: Uses Tokio so it doesn't block while waiting for responses
- **Live Streaming**: Responses show up as they're generated, not all at once
- **Clean Code**: Well-organized modules that each do one thing well
- **Easy Config**: Add new agents without touching any code
- **Good Error Messages**: When things go wrong, you'll know why
- **Secure Setup**: Your API key stays in environment variables, not in code

## License 📄

MIT - See [LICENSE](LICENSE) for details.