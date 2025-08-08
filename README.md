# Chatbot LLM

A Rust application for creating intelligent chatbot agents using LangChain. This project supports multiple specialized agents, each optimized for specific contexts and response patterns.

## Description

This is a learning project focused on building a multi-agent chatbot system in Rust. The application will feature:

- Multiple specialized AI agents with distinct personalities and capabilities
- LangChain integration for advanced language model interactions
- Modular architecture for easy agent management and deployment
- Step-by-step development approach for educational purposes

## Prerequisites

- Windows 10/11 (other platforms may work but are not tested)
- Internet connection for downloading dependencies
- At least 2GB of free disk space

## Installation

### Step 1: Install Rust

This project requires Rust and Cargo. Follow these steps to install them:

1. **Download and install Rust:**
   ```powershell
   Invoke-WebRequest -Uri https://win.rustup.rs/ -OutFile rustup-init.exe
   .\rustup-init.exe -y
   ```

2. **Add Cargo to your PATH (for current session):**
   ```powershell
   $env:PATH += ";$env:USERPROFILE\.cargo\bin"
   ```

3. **Install GNU toolchain (recommended for Windows):**
   ```powershell
   rustup target add x86_64-pc-windows-gnu
   rustup default stable-x86_64-pc-windows-gnu
   ```

   > **Note:** We use the GNU toolchain to avoid requiring Visual Studio Build Tools. This provides a simpler setup process.

4. **Verify installation:**
   ```powershell
   cargo --version
   rustc --version
   ```

### Step 2: Clone and Build

1. **Navigate to the project directory:**
   ```powershell
   cd path\to\chatbot-llm
   ```

2. **Build the project:**
   ```powershell
   cargo build
   ```

3. **Run the application:**
   ```powershell
   cargo run
   ```

## Usage

Currently, the application displays a "Hello, world!" message. As development progresses, this section will be updated with:

- Agent configuration examples
- API usage instructions
- Command-line interface documentation
- Integration examples

## Development

### Project Structure

```
chatbot-llm/
├── src/
│   └── main.rs          # Application entry point
├── Cargo.toml           # Project configuration and dependencies
├── Cargo.lock           # Dependency lock file
├── .gitignore           # Git ignore rules
└── README.md            # This file
```

### Building for Development

```powershell
# Build in debug mode (faster compilation)
cargo build

# Build in release mode (optimized)
cargo build --release

# Run with automatic rebuilding on file changes
cargo watch -x run  # Requires: cargo install cargo-watch
```

### Testing

```powershell
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

## Troubleshooting

### Common Issues

**"cargo: command not found" or similar errors:**
- Ensure Rust is properly installed
- Restart your terminal or add Cargo to PATH manually
- Verify installation with `cargo --version`

**"linker `link.exe` not found" errors:**
- This occurs when using the MSVC toolchain without Visual Studio
- Switch to GNU toolchain as described in installation steps
- Alternative: Install Visual Studio Build Tools with C++ support

**Permission errors during installation:**
- Run PowerShell as Administrator
- Ensure antivirus software isn't blocking the installation

### Getting Help

- Check the [Rust Book](https://doc.rust-lang.org/book/) for Rust fundamentals <mcreference link="https://doc.rust-lang.org/book/" index="1">1</mcreference>
- Visit [Rust Users Forum](https://users.rust-lang.org/) for community support
- Review [Cargo documentation](https://doc.rust-lang.org/cargo/) for build system help

## Contributing

This is a learning project, but contributions and suggestions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Roadmap

- [ ] Basic project structure and setup ✅
- [ ] LangChain integration
- [ ] First agent implementation
- [ ] Multi-agent architecture
- [ ] Configuration management
- [ ] API endpoints
- [ ] Documentation and examples

---

*This project is part of a step-by-step learning journey in Rust and AI development.*