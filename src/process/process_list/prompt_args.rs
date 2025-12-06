//! Prompt argument types for process_list tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for process_list tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessListPromptArgs {
    /// Scenario to show examples for
    /// - "basic": List all processes
    /// - "filtering": Filter by name
    /// - "monitoring": System monitoring patterns
    /// - "workflows": Process management workflows
    /// - "comprehensive": All scenarios combined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
