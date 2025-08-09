use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "chatbot-llm")]
#[command(about = "A Rust-based chatbot application that interfaces with Large Language Models")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Start an interactive chat session
    Chat {
        /// Specify which agent to use for the conversation
        #[arg(short, long)]
        agent: Option<String>,
    },
    /// Display all available agents
    ListAgents,
}

impl Default for Commands {
    fn default() -> Self {
        Self::Chat { agent: None }
    }
}

impl Cli {
    pub fn command(&self) -> Commands {
        self.command.clone().unwrap_or_default()
    }
}