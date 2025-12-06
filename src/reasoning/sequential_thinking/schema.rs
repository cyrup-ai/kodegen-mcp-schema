//! Sequential thinking tool schema types

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::SequentialThinkingPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for sequential thinking tool
pub const SEQUENTIAL_THINKING: &str = "sequential_thinking";

// ============================================================================
// SEQUENTIAL THINKING TOOL
// ============================================================================

/// Arguments for sequential thinking tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SequentialThinkingArgs {
    /// Optional session ID for maintaining state across calls
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// Your current thinking step
    pub thought: String,

    /// Current thought number (1-based, minimum: 1)
    #[schemars(range(min = 1))]
    pub thought_number: u32,

    /// Estimated total thoughts needed (minimum: 1)
    #[schemars(range(min = 1))]
    pub total_thoughts: u32,

    /// Whether another thought step is needed
    pub next_thought_needed: bool,

    /// Whether this revises previous thinking
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_revision: Option<bool>,

    /// Which thought is being reconsidered (minimum: 1)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 1))]
    pub revises_thought: Option<u32>,

    /// Branching point thought number (minimum: 1)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 1))]
    pub branch_from_thought: Option<u32>,

    /// Branch identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,

    /// If more thoughts are needed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub needs_more_thoughts: Option<bool>,
}

/// Output from `sequential_thinking` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SequentialThinkingOutput {
    /// Session identifier for maintaining state
    pub session_id: String,
    /// Current thought number
    pub thought_number: u32,
    /// Total thoughts expected
    pub total_thoughts: u32,
    /// The thought content
    pub thought: String,
    /// Whether another thought step is needed
    pub next_thought_needed: bool,
    /// Whether this was a revision of a previous thought
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_revision: Option<bool>,
    /// Which thought was revised (if is_revision is true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revises_thought: Option<u32>,
    /// Branch identifier (if branching)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Which thought this branches from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_from_thought: Option<u32>,
    /// List of active branches
    pub branches: Vec<String>,
    /// Total thoughts recorded in session history
    pub thought_history_length: usize,
}

// ============================================================================
// TOOL ARGS TRAIT IMPL
// ============================================================================

#[tool_metadata(
    name = "sequential_thinking",
    category = "reasoning",
    description = "A detailed tool for dynamic and reflective problem-solving through thoughts. This tool helps analyze problems through a flexible thinking process that can adapt and evolve. Each thought can build on, question, or revise previous insights as understanding deepens."
)]
impl ToolArgs for SequentialThinkingArgs {
    type Output = SequentialThinkingOutput;
    type Prompts = SequentialThinkingPrompts;

    const NAME: &'static str = SEQUENTIAL_THINKING;
    const CATEGORY: &'static str = "reasoning";
    const DESCRIPTION: &'static str = "A detailed tool for dynamic and reflective problem-solving through thoughts. This tool helps analyze problems through a flexible thinking process that can adapt and evolve. Each thought can build on, question, or revise previous insights as understanding deepens.";
}
