//! Prompt argument types for claude_agent tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for claude_agent tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClaudeAgentPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple delegation
    /// - "specialized": Tool-constrained agents
    /// - "parallel": Multi-agent coordination
    /// - "research": Research-focused with add_dirs
    /// - "monitoring": Progress checking patterns
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
