//! Prompt argument types for inspect_usage_stats tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for inspect_usage_stats tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InspectUsageStatsPromptArgs {
    /// Focus area: "performance", "failures", "overview", "optimization"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus_area: Option<String>,

    /// Show detailed usage examples
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_examples: Option<bool>,
}
