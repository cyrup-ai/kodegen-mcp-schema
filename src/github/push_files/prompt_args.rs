//! Prompt argument types for github_push_files tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for push_files tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PushFilesPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Basic multi-file push to existing branch
    /// - "feature_branch": Feature branch workflow with code generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
