use crate::core::data::{ChatConversation, ChatMessage};
use crate::core::error::{AppError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct LlmClient {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ApiMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Serialize, Deserialize)]
struct ApiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ApiMessage,
}

impl LlmClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| AppError::missing_env_var("OPENAI_API_KEY"))?;

        let base_url =
            env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

        let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());

        let timeout_seconds = env::var("OPENAI_TIMEOUT_SECONDS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(120);

        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            .map_err(|e| AppError::http_client_error(e.to_string()))?;

        Ok(Self {
            client,
            api_key,
            base_url,
            model,
        })
    }

    pub async fn send_conversation(
        &self,
        conversation: &ChatConversation,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> Result<ChatMessage> {
        let messages: Vec<ApiMessage> = conversation
            .iter()
            .map(|msg| ApiMessage {
                role: msg.role().to_string(),
                content: msg.content().to_string(),
            })
            .collect();

        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            max_tokens: Some(max_tokens.unwrap_or(1000)),
            temperature: Some(temperature.unwrap_or(0.7)),
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error response".to_string());
            return Err(AppError::api_response_error(status, error_body));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| AppError::response_parsing_error(e.to_string()))?;

        let assistant_message = chat_response
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| AppError::no_llm_response())?
            .message;

        Ok(ChatMessage::from_assistant(assistant_message.content))
    }
}

impl Default for LlmClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default LLM client")
    }
}
