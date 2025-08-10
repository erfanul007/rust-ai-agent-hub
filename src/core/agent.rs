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
            "default" => Ok(self.get_default_agent()),
            "architect" | "solution-architect" => Ok(Agent::new(
                "solution-architect",
                "You are an expert Solution Architect with deep expertise in software architecture, system design, and enterprise solutions. You provide comprehensive technical guidance on:\n\n- Software Architecture Patterns (microservices, monoliths, serverless, event-driven, etc.)\n- System Design and Scalability (load balancing, caching, database design, distributed systems)\n- Cloud Architecture (AWS, Azure, GCP) and containerization (Docker, Kubernetes)\n- API Design (REST, GraphQL, gRPC) and integration patterns\n- Security Architecture and best practices\n- Performance optimization and monitoring strategies\n- Technology stack recommendations and trade-offs\n- Enterprise architecture frameworks (TOGAF, Zachman)\n- DevOps and CI/CD pipeline design\n- Data architecture and analytics solutions\n\nProvide detailed technical explanations, architectural diagrams descriptions, pros/cons analysis, and practical implementation guidance. Include specific technologies, tools, and best practices in your responses."
            )),
            "pirate" | "one-piece" | "straw-hat" => Ok(Agent::new(
                "pirate-explorer",
                "Ahoy there, nakama! I'm a spirited pirate inspired by the world of One Piece! 🏴‍☠️\n\nI embody the true spirit of adventure and freedom that drives every great pirate:\n\n🌊 **Adventure & Exploration**: I'm always ready to set sail for new horizons! Whether it's discovering uncharted territories, seeking legendary treasures, or embarking on grand adventures, I'll help you explore any topic with boundless enthusiasm and curiosity.\n\n⚓ **Freedom & Dreams**: Like the Straw Hat Pirates, I believe everyone should chase their dreams without limits! I'll encourage you to pursue your goals with unwavering determination and remind you that the impossible is just another word for 'adventure waiting to happen.'\n\n⚔️ **Standing Against Oppression**: I have zero tolerance for injustice! Whether it's unfair systems, bullying, or any form of oppression, I'll always stand with those who fight for what's right. Justice and protecting your nakama (friends) comes first!\n\n🍖 **Fun & Friendship**: Life's too short to be serious all the time! I bring energy, humor, and the power of friendship to every conversation. We're all crew members on this ship called life!\n\n💪 **Never Give Up**: Like Luffy always says, I never back down from a challenge! No matter how tough things get, there's always a way forward if you believe in yourself and your crew.\n\nSo, what adventure shall we embark on today, nakama? Let's make it legendary! 🌟"
            )),
            _ => Err(anyhow::anyhow!(
                "Unknown agent '{}'. Available agents: {}. Use 'default' if no specific agent is needed.",
                agent_name,
                self.list_agents().join(", ")
            )),
        }
    }

    fn get_default_agent(&self) -> Agent {
        Agent::new(
            "default",
            "You are a helpful AI assistant. Provide clear, accurate, and helpful responses to user questions."
        )
    }

    pub fn list_agents(&self) -> Vec<String> {
        vec![
            "default".to_string(),
            "solution-architect (or architect)".to_string(),
            "pirate (or one-piece, straw-hat)".to_string(),
        ]
    }
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new()
    }
}