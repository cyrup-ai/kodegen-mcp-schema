//! Schema types for browser_agent tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_BROWSER, BROWSER_AGENT};
use crate::{ToolArgs, tool_metadata};
use super::prompts::AgentPrompts;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

const fn zero() -> u32 {
    0
}

fn default_agent_timeout_ms() -> u64 {
    600000 // 10 minutes
}

fn default_max_steps() -> u32 {
    10
}

fn default_max_actions() -> u32 {
    3
}

fn default_agent_temperature() -> f64 {
    0.7
}

fn default_agent_max_tokens() -> u64 {
    2048
}

fn default_vision_timeout_secs() -> u64 {
    60
}

fn default_llm_timeout_secs() -> u64 {
    120
}

// ============================================================================
// ACTION ENUM
// ============================================================================

/// Actions for browser_agent tool
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BrowserAgentAction {
    /// Prompt the agent with a new task (spawn background work)
    Prompt,
    /// Read current progress from an active agent
    Read,
    /// Kill a running agent (destroys slot permanently)
    Kill,
}

// ============================================================================
// INPUT ARGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserAgentArgs {
    /// Action to perform on the agent session
    pub action: BrowserAgentAction,

    /// Agent number (0-based, default: 0) - unique per connection_id
    #[serde(default = "zero")]
    pub agent: u32,

    /// Maximum time in milliseconds to wait for completion (default: 600000ms = 10 minutes)
    /// - On timeout: returns current progress, agent continues in background
    /// - Special value 0: fire-and-forget (returns immediately)
    #[serde(default = "default_agent_timeout_ms")]
    pub await_completion_ms: u64,

    /// Task description for the agent to accomplish (required for EXEC, ignored for READ/KILL)
    #[serde(default)]
    pub task: Option<String>,

    /// Optional additional context or hints
    #[serde(default)]
    pub additional_info: Option<String>,

    /// Optional initial URL to navigate to before starting
    #[serde(default)]
    pub start_url: Option<String>,

    /// Maximum steps agent can take (default: 10)
    #[serde(default = "default_max_steps")]
    pub max_steps: u32,

    /// Maximum actions per step (default: 3)
    #[serde(default = "default_max_actions")]
    pub max_actions_per_step: u32,

    /// LLM temperature for action generation (default: 0.7)
    #[serde(default = "default_agent_temperature")]
    pub temperature: f64,

    /// Max tokens per LLM call (default: 2048)
    #[serde(default = "default_agent_max_tokens")]
    pub max_tokens: u64,

    /// Vision model timeout in seconds (default: 60s)
    /// Vision analysis is typically fast, but allow time for model loading
    #[serde(default = "default_vision_timeout_secs")]
    pub vision_timeout_secs: u64,

    /// LLM generation timeout in seconds (default: 120s)
    /// Allow time for complex reasoning and high token generation
    #[serde(default = "default_llm_timeout_secs")]
    pub llm_timeout_secs: u64,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `browser_agent` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserAgentOutput {
    /// Agent number
    pub agent: u32,
    /// Task being executed
    pub task: String,
    /// Current step count
    pub steps_taken: usize,
    /// Whether agent is complete
    pub completed: bool,
    /// Error message if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Progress summary
    pub summary: String,
    /// Detailed history
    pub history: Vec<BrowserAgentStepInfo>,
}

/// Step information from browser agent execution
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserAgentStepInfo {
    pub step: usize,
    pub timestamp: String,
    pub actions: Vec<String>,
    pub summary: String,
    pub complete: bool,
}

/// Output from agent kill action
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserAgentKillOutput {
    /// Agent number that was killed
    pub agent: u32,
    /// Success message
    pub message: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPL
// ============================================================================

#[tool_metadata(
    description = "Autonomous AI agent that accomplishes complex web tasks through multi-step reasoning and adaptive decision-making"
)]
impl ToolArgs for BrowserAgentArgs {
    type Output = BrowserAgentOutput;
    type Prompts = AgentPrompts;

    const NAME: &'static str = BROWSER_AGENT;
    const CATEGORY: &'static str = CATEGORY_BROWSER;
    const DESCRIPTION: &'static str = "Autonomous AI agent that accomplishes complex web tasks through multi-step reasoning and adaptive decision-making";
}
