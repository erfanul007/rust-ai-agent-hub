use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "chatbot-llm")]
#[command(about = "A Rust-based chatbot application that interfaces with Large Language Models")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    Chat {
        #[arg(short, long)]
        agent: Option<String>,
    },
    ListAgents,
}

impl Default for Commands {
    fn default() -> Self {
        Self::Chat { agent: None }
    }
}

impl Cli {
    pub fn get_command(&self) -> Commands {
        self.command.clone().unwrap_or_default()
    }
}