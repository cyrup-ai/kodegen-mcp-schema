//! Prompt argument types for reasoner tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for reasoner tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReasonerPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Basic reasoning steps
    /// - "beam_search": Beam search strategy
    /// - "mcts": Monte Carlo Tree Search
    /// - "branching": Branching and revision
    /// - "strategies": Comparing strategies
    /// - "comprehensive": All scenarios combined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
