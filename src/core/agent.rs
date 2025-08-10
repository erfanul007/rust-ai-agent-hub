use anyhow::Result;
use config::{Config, ConfigError};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub system_prompt: String,
    pub aliases: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AgentsConfig {
    pub agents: HashMap<String, AgentConfig>,
}

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

pub struct AgentManager {
    agents_config: AgentsConfig,
}

impl AgentManager {
    pub fn new() -> Result<Self> {
        let config = Self::load_config()?;
        Ok(Self {
            agents_config: config,
        })
    }

    fn load_config() -> Result<AgentsConfig, ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::with_name("agents"))
            .build()?;
        
        settings.try_deserialize::<AgentsConfig>()
    }

    pub fn get_agent(&self, agent_name: &str) -> Result<Agent> {
        // First, try to find by exact name match
        if let Some(agent_config) = self.agents_config.agents.get(agent_name) {
            return Ok(Agent::new(&agent_config.name, &agent_config.system_prompt));
        }

        // Then, try to find by alias
        for (_, agent_config) in &self.agents_config.agents {
            if agent_config.aliases.contains(&agent_name.to_string()) {
                return Ok(Agent::new(&agent_config.name, &agent_config.system_prompt));
            }
        }

        // If not found, return error
        Err(anyhow::anyhow!(
            "Unknown agent '{}'. Available agents: {}. Use 'default' if no specific agent is needed.",
            agent_name,
            self.list_agents().join(", ")
        ))
    }

    pub fn list_agents(&self) -> Vec<String> {
        let mut agents = Vec::new();
        
        // Add default agent first if it exists
        if let Some(default_config) = self.agents_config.agents.get("default") {
            agents.push(Self::format_agent_display(&default_config.name, &default_config.aliases));
        }
        
        // Add remaining agents
        for (_, agent_config) in &self.agents_config.agents {
            if agent_config.name != "default" {
                agents.push(Self::format_agent_display(&agent_config.name, &agent_config.aliases));
            }
        }
        
        agents
    }

    fn format_agent_display(name: &str, aliases: &[String]) -> String {
        if aliases.is_empty() {
            name.to_string()
        } else {
            let aliases_str = aliases.join(", ");
            format!("{} (or {})", name, aliases_str)
        }
    }
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new().expect("Failed to load agent configuration")
    }
}