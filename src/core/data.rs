use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl fmt::Display for MessageRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_name = match self {
            MessageRole::System => "system",
            MessageRole::User => "user",
            MessageRole::Assistant => "assistant",
        };
        write!(f, "{}", role_name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    role: MessageRole,
    content: String,
}

impl ChatMessage {
    pub fn new(role: MessageRole, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn from_user(content: impl Into<String>) -> Self {
        Self::new(MessageRole::User, content)
    }

    pub fn from_system(content: impl Into<String>) -> Self {
        Self::new(MessageRole::System, content)
    }

    pub fn from_assistant(content: impl Into<String>) -> Self {
        Self::new(MessageRole::Assistant, content)
    }

    pub fn role(&self) -> &MessageRole {
        &self.role
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatConversation {
    messages: Vec<ChatMessage>,
}

impl ChatConversation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }
}

impl Deref for ChatConversation {
    type Target = Vec<ChatMessage>;

    fn deref(&self) -> &Self::Target {
        &self.messages
    }
}

impl DerefMut for ChatConversation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.messages
    }
}