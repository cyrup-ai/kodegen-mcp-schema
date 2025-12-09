//! Reasoner tool schema types

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_REASONER, REASONER};

use crate::{ToolArgs, tool_metadata};
use super::prompts::ReasonerPrompts;

// ============================================================================
// REASONER TOOL
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

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Advanced reasoning tool with multiple strategies (beam search, MCTS). Processes thoughts step-by-step, supports branching and revision, and tracks best reasoning paths. Use for complex problem-solving that requires exploration of multiple solution approaches."
)]
impl ToolArgs for ReasonerArgs {
    type Output = ReasonerOutput;
    type Prompts = ReasonerPrompts;

    const NAME: &'static str = REASONER;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_REASONER;
    const DESCRIPTION: &'static str = "Advanced reasoning tool with multiple strategies (beam search, MCTS). Processes thoughts step-by-step, supports branching and revision, and tracks best reasoning paths. Use for complex problem-solving that requires exploration of multiple solution approaches.";
}
