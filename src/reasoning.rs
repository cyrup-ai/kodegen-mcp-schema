//! Reasoning tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool names for reasoning tools
pub const SEQUENTIAL_THINKING: &str = "sequential_thinking";
pub const REASONER: &str = "reasoner";

// ============================================================================
// SEQUENTIAL THINKING
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

/// Prompt arguments for sequential thinking tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SequentialThinkingPromptArgs {
    /// Optional domain for customized examples (e.g., "software engineering", "mathematics", "creative writing", "data analysis")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example_domain: Option<String>,

    /// Optional feature to focus teaching on (e.g., "branching", "revision", "basic", "advanced", "all")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus_feature: Option<String>,
}

// ============================================================================
// REASONER
// ============================================================================

/// Arguments for the reasoner tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReasonerArgs {
    /// Current reasoning step
    pub thought: String,

    /// Current step number (1-based)
    #[schemars(range(min = 1))]
    pub thought_number: usize,

    /// Total expected steps
    #[schemars(range(min = 1))]
    pub total_thoughts: usize,

    /// Whether another step is needed
    pub next_thought_needed: bool,

    /// Optional parent thought ID for branching
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,

    /// Strategy: beam_search, mcts, mcts_002_alpha, mcts_002alt_alpha
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<String>,

    /// Number of top paths to maintain (beam search)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 1, max = 10))]
    pub beam_width: Option<usize>,

    /// Number of MCTS simulations to run
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 1, max = 150))]
    pub num_simulations: Option<usize>,
}

/// Prompt arguments for reasoner tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReasonerPromptArgs {
    /// Optional strategy to focus teaching on: "beam_search", "mcts", "mcts_002_alpha", "mcts_002alt_alpha", or "all" (default)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy_focus: Option<String>,

    /// Optional explanation depth: "basic" (quick overview), "advanced" (detailed mechanics), or "all" (comprehensive, default)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation_depth: Option<String>,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `reasoner` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReasonerOutput {
    /// Session/node identifier for this reasoning step
    pub session_id: String,
    /// Current thought number (echoed from input)
    pub thought_number: usize,
    /// Total thoughts expected (echoed from input)
    pub total_thoughts: usize,
    /// The thought content processed
    pub thought: String,
    /// Strategy used for this reasoning step
    pub strategy: String,
    /// Whether another thought step is needed
    pub next_thought_needed: bool,
    /// Best path score (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_path_score: Option<f64>,
    /// Number of possible paths/branches from this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<usize>,
    /// Number of nodes in reasoning history
    pub history_length: usize,
    /// Quality score for this node (0.0-1.0)
    pub score: f64,
    /// Depth in reasoning tree
    pub depth: usize,
    /// Whether this thought completes a reasoning path
    pub is_complete: bool,
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
// TOOL ARGS TRAIT IMPLEMENTATIONS
// ============================================================================

impl crate::ToolArgs for ReasonerArgs {
    type Output = ReasonerOutput;
}

impl crate::ToolArgs for SequentialThinkingArgs {
    type Output = SequentialThinkingOutput;
}
