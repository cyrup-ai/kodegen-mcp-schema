//! Schema types for git_history tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_HISTORY};
use crate::{ToolArgs, tool_metadata};
use super::super::GitHistoryCommit;
use super::prompts::HistoryPrompts;


// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn default_history_limit() -> usize {
    20
}

// ============================================================================
// GIT_HISTORY TOOL
// ============================================================================

/// Arguments for `git_history` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitHistoryArgs {
    /// Path to repository
    pub path: String,

    /// File path to investigate
    pub file: String,

    /// Maximum number of commits to return (default: 20)
    #[serde(default = "default_history_limit")]
    pub limit: usize,

    /// Optional regex pattern to filter diffs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Start revision (default: HEAD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,

    /// End revision (for range mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_history` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHistoryOutput {
    pub success: bool,
    /// File path that was analyzed
    pub file: String,
    /// Mode: "commits" or "range"
    pub mode: String,
    /// Total commits examined (commits mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_examined: Option<usize>,
    /// List of commits with diffs (commits mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<GitHistoryCommit>>,
    /// Start revision (range mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// End revision (range mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    /// Total additions (range mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions: Option<u32>,
    /// Total deletions (range mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletions: Option<u32>,
    /// Cumulative diff (range mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
}

#[tool_metadata(
    description = "View commit history and changes for a specific file"
)]
impl ToolArgs for GitHistoryArgs {
    type Output = GitHistoryOutput;
    type Prompts = HistoryPrompts;

    const NAME: &'static str = GIT_HISTORY;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "View commit history and changes for a specific file";
}
