//! Prompt argument types for git_add tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_add tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitAddPromptArgs {
    /// Scenario to show examples for
    /// - "specific_files": Adding individual files
    /// - "patterns": Adding by glob patterns
    /// - "all_changes": Staging everything
    /// - "partial": Selective staging strategies
    /// - "workflows": Complete staging workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
