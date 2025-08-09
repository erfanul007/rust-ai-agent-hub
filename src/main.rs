mod core;

use anyhow::Result;
use clap::Parser;
use core::cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    execute_command(cli.command()).await
}

async fn execute_command(command: Commands) -> Result<()> {
    match command {
        Commands::Chat { agent } => start_chat_session(agent.as_deref()).await,
        Commands::ListAgents => display_available_agents().await,
    }
}

async fn start_chat_session(agent_name: Option<&str>) -> Result<()> {
    let selected_agent = agent_name.unwrap_or("default");
    println!("Starting chat with agent: {}", selected_agent);
    Ok(())
}

async fn display_available_agents() -> Result<()> {
    println!("Available agents:");
    println!("  - default (built-in agent)");
    Ok(())
}
