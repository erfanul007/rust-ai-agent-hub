mod core;

use anyhow::Result;
use clap::Parser;
use core::cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.get_command() {
        Commands::Chat { agent } => handle_chat(agent.as_deref()).await?,
        Commands::ListAgents => handle_list_agents().await?,
    }

    Ok(())
}

async fn handle_chat(agent: Option<&str>) -> Result<()> {
    match agent {
        Some(name) => println!("Starting chat with agent: {}", name),
        None => println!("Starting chat with default agent"),
    }
    Ok(())
}

async fn handle_list_agents() -> Result<()> {
    println!("Available agents:");
    println!("  - default (built-in agent)");
    Ok(())
}
