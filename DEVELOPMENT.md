# Development Plan

This document outlines the milestone-driven development plan for the Rust Chatbot LLM application. Each milestone is designed to deliver a functional, testable version of the application, allowing for iterative progress and early feedback.

---

### **Milestone 1: The "Vertical Slice" - A Single, Simple Agent**

**Goal:** To get a minimal, end-to-end version of the application running as quickly as possible. This creates the core skeleton that all future development will build upon.

1.  **Add Foundational Dependencies:**
    *   `tokio`: For the asynchronous runtime, which is essential for non-blocking I/O (like network requests).
    *   `anyhow`: For simple and flexible error handling in the early stages.
    *   `clap`: A powerful and popular library for building the command-line interface (CLI).
    *   `serde`: For serializing Rust structs into JSON for API requests and deserializing responses.
    *   `reqwest`: A high-level, ergonomic HTTP client for making API calls to the LLM.
    *   `dotenvy`: To manage secrets like API keys from a `.env` file, keeping them out of source code.

2.  **Define Core Data Structures:**
    *   Create `src/main.rs` with a basic async `main` function.
    *   Define initial structs: `Message { role: String, content: String }` and `Conversation { messages: Vec<Message> }`.

3.  **Implement a Basic LLM API Client:**
    *   Create a new module, `src/llm_client.rs`.
    *   Implement a function that takes a conversation history and sends it to an LLM provider's API (e.g., OpenAI). This function will handle authentication using the API key from the `.env` file.

4.  **Build the CLI Interaction Loop:**
    *   In `main.rs`, create a simple loop that reads user input from the command line, adds it to the `Conversation`, calls the `llm_client`, and prints the response.

**Outcome of Milestone 1:** A working command-line application where you can have a basic, single-turn conversation with a hardcoded agent.

---

### **Milestone 2: The Agent Framework**

**Goal:** To build the infrastructure for defining, configuring, and switching between multiple, specialized agents.

1.  **Formalize the "Agent" Concept:**
    *   Define a `struct Agent { name: String, system_prompt: String, ... }`. The `system_prompt` is key to defining an agent's persona and expertise.
    *   Create an `AgentManager` struct responsible for managing the available agents.

2.  **Configuration-Driven Agents:**
    *   Add the `config` and `toml` crates to `Cargo.toml`.
    *   Create a configuration file, `agents.toml`, to define the properties of each agent. This makes adding or modifying agents possible without changing Rust code.
    *   Implement logic in the `AgentManager` to load and parse `agents.toml` at startup.

3.  **Enhance the CLI:**
    *   Use `clap`'s more advanced features to add subcommands to the application:
        *   `chatbot-llm chat`: The default command to start a conversation.
        *   `chatbot-llm list-agents`: To show all available agents from the configuration.
        *   `chatbot-llm chat --agent <AGENT_NAME>`: To start a chat with a specific agent.

**Outcome of Milestone 2:** The application now supports multiple, distinctly defined agents. You can list them and choose which one to talk to, and the core logic is driven by a clean configuration file.

---

### **Milestone 3: Polish and Production-Readiness**

**Goal:** To transform the functional prototype into a robust and user-friendly tool.

1.  **Advanced Error Handling:**
    *   Introduce the `thiserror` crate to create specific, custom error types. This makes debugging far easier and provides clearer error messages to the user.

2.  **Structured Logging:**
    *   Integrate the `tracing` crate. This will allow for structured logs that can show the flow of a conversation, which agent is being used, and the exact API calls being made.

3.  **Contextual Conversations:**
    *   Modify the chat loop to maintain conversation history. On each turn, the *entire* history (up to a certain token limit) is sent to the LLM, allowing for contextually aware, multi-turn conversations.

4.  **Streaming Responses:**
    *   Update the `llm_client` and the CLI to handle streaming API responses. This dramatically improves the user experience, as the response appears token-by-token, just like in ChatGPT.

5.  **Comprehensive Testing:**
    *   Write unit tests for the `AgentManager` and configuration loading.
    *   Write integration tests for the chat loop and API client, using mock servers to avoid making real API calls during tests.

**Outcome of Milestone 3:** A polished, reliable, and observable application that is easy to debug and provides a professional user experience.

---

### **Consistency and Improvements**

**Iterative Development:**
The plan now emphasizes iterative progress, allowing milestones to overlap where feasible. For example, foundational dependencies can be added while defining core data structures.

**Concrete Definitions:**
Abstract goals have been replaced with specific, actionable tasks. For instance, "LangChain Integration" has been clarified into "Implement a Basic LLM API Client" and "Formalize the Agent Concept."

**Early Wins:**
Milestone 1 ensures a "first victory" by delivering a minimal, functional prototype early in the development cycle.

**Integrated Essentials:**
Critical features like configuration management, error handling, and testing are introduced early and refined throughout the milestones.