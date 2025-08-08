mod core;

use anyhow::Result;
use core::error::{ErrorContext, ErrorType};

#[tokio::main]
async fn main() -> Result<()> {
    run().await.with_context_type(ErrorType::Cli, "application startup")?;
    Ok(())
}

async fn run() -> Result<()> {
    println!("Chatbot LLM Application Starting...");
    Ok(())
}
