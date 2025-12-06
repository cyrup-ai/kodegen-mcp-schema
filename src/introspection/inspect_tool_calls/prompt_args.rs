//! Prompt argument types for inspect_tool_calls tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for inspect_tool_calls tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InspectToolCallsPromptArgs {
    /// Scenario focus: "debugging", "onboarding", "pagination", "filtering", "recent_activity"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario_type: Option<String>,

    /// Show detailed usage examples with TypeScript
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_examples: Option<bool>,
}
