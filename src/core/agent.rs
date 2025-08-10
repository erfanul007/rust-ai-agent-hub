use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Agent {
    name: String,
    system_prompt: String,
}

impl Agent {
    pub fn new(name: impl Into<String>, system_prompt: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            system_prompt: system_prompt.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn system_prompt(&self) -> &str {
        &self.system_prompt
    }
}

pub struct AgentManager;

impl AgentManager {
    pub fn new() -> Self {
        Self
    }

    pub fn get_agent(&self, agent_name: &str) -> Result<Agent> {
        match agent_name {
            "default" => Ok(Agent::new(
                "default",
                "You are a helpful AI assistant. Provide clear, accurate, and helpful responses to user questions."
            )),
            _ => Ok(Agent::new(
                agent_name,
                "You are a helpful AI assistant. Provide clear, accurate, and helpful responses to user questions."
            )),
        }
    }

    pub fn list_agents(&self) -> Vec<String> {
        vec!["default".to_string()]
    }
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new()
    }
}