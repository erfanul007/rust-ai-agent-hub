use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Represents the role of a message sender.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role_str = match self {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        };
        write!(f, "{}", role_str)
    }
}

/// Represents a single message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message sender.
    pub role: Role,
    /// The content of the message.
    pub content: String,
}

impl Message {
    /// Creates a new message with the specified role and content.
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    /// Creates a user message.
    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }

    /// Creates an assistant message.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(Role::Assistant, content)
    }

    /// Creates a system message.
    pub fn system(content: impl Into<String>) -> Self {
        Self::new(Role::System, content)
    }
}

/// Represents a conversation containing multiple messages.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Conversation {
    messages: Vec<Message>,
}

impl Conversation {
    /// Adds a message to the conversation.
    pub fn add_message(&mut self, message: Message) {
        self.push(message);
    }
}

impl Deref for Conversation {
    type Target = Vec<Message>;

    fn deref(&self) -> &Self::Target {
        &self.messages
    }
}

impl DerefMut for Conversation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.messages
    }
}