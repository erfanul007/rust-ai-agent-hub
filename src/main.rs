mod core;

use anyhow::Result;
use clap::Parser;
use core::chat::ChatSession;
use core::cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();
    execute_command(cli.command()).await
}

async fn execute_command(command: Commands) -> Result<()> {
    let chat_session = ChatSession::new()?;

    match command {
        Commands::Chat { agent } => chat_session.start_chat(agent.as_deref()).await,
        Commands::ListAgents => display_available_agents(&chat_session).await,
    }
}

async fn display_available_agents(chat_session: &ChatSession) -> Result<()> {
    println!("Available agents:");
    for agent in chat_session.list_agents() {
        println!("  - {} (built-in agent)", agent);
    }
    Ok(())
}
