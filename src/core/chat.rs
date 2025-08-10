use crate::core::agent::{AgentManager};
use crate::core::data::{ChatConversation, ChatMessage};
use crate::core::error::Result;
use crate::core::llm_client::LlmClient;
use futures::stream::StreamExt;
use std::io::{self, Write};

pub struct ChatSession {
    llm_client: LlmClient,
    agent_manager: AgentManager,
}

impl ChatSession {
    pub fn new() -> Result<Self> {
        let llm_client = LlmClient::new()?;
        let agent_manager = AgentManager::new()?;
        
        Ok(Self {
            llm_client,
            agent_manager,
        })
    }

    pub async fn start_chat(&self, agent_name: Option<&str>) -> Result<()> {
        let agent_name = agent_name.unwrap_or("default");
        let agent = self.agent_manager.get_agent(agent_name)?;
        
        println!("Starting chat with agent: {}", agent.name());
        println!("Type 'quit' or 'exit' to end the conversation.\n");

        let mut conversation = ChatConversation::new();
        conversation.add_message(ChatMessage::from_system(agent.system_prompt()));

        self.chat_loop(&mut conversation).await
    }

    async fn chat_loop(&self, conversation: &mut ChatConversation) -> Result<()> {
        loop {
            let user_input = self.get_user_input()?;

            if self.is_exit_command(&user_input) {
                println!("Goodbye!");
                break;
            }

            if user_input.is_empty() {
                continue;
            }

            conversation.add_message(ChatMessage::from_user(user_input));

            print!("Assistant: ");
            io::stdout().flush()?;
            
            match self.llm_client.send_conversation_stream(conversation, None, None).await {
                Ok(mut stream) => {
                    let mut full_response = String::new();
                    
                    while let Some(chunk_result) = stream.next().await {
                        match chunk_result {
                            Ok(chunk) => {
                                if !chunk.is_empty() {
                                    print!("{}", chunk);
                                    io::stdout().flush()?;
                                    full_response.push_str(&chunk);
                                }
                            }
                            Err(e) => {
                                eprintln!("\nStreaming error: {}", e);
                                break;
                            }
                        }
                    }
                    
                    if !full_response.is_empty() {
                        conversation.add_message(ChatMessage::from_assistant(full_response));
                    }
                    println!(); // New line after streaming response
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    println!("Please try again.");
                }
            }
            
            println!(); // Add blank line for readability
        }

        Ok(())
    }

    fn get_user_input(&self) -> Result<String> {
        print!("You: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn is_exit_command(&self, input: &str) -> bool {
        input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit")
    }

    pub fn list_agents(&self) -> Vec<String> {
        self.agent_manager.list_agents()
    }
}

impl Default for ChatSession {
    fn default() -> Self {
        Self::new().expect("Failed to create default chat session")
    }
}