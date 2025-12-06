//! Prompt argument types for git_diff tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for git_diff tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitDiffPromptArgs {
    /// Scenario to show examples for
    /// - "working": Working directory changes
    /// - "staged": Staged changes review
    /// - "commits": Comparing commits
    /// - "branches": Comparing branches
    /// - "files": Specific file diffs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
