//! Schema types for merge_pull_request tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::MergePullRequestPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for merging a pull request
pub const GITHUB_MERGE_PULL_REQUEST: &str = "github_merge_pull_request";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for merging a pull request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MergePullRequestArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pr_number: u64,
    /// Title for the merge commit (optional)
    #[serde(default)]
    pub commit_title: Option<String>,
    /// Extra detail for the merge commit message (optional)
    #[serde(default)]
    pub commit_message: Option<String>,
    /// Merge method: "merge", "squash", or "rebase" (optional, defaults to repository setting)
    #[serde(default)]
    pub merge_method: Option<String>,
    /// SHA that pull request head must match to allow merge (optional, for safety)
    #[serde(default)]
    pub sha: Option<String>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_merge_pull_request` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubMergePrOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub pr_number: u64,
    pub merged: bool,
    pub sha: Option<String>,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_merge_pull_request",
    category = "github",
    description = "Merge a pull request"
)]
impl ToolArgs for MergePullRequestArgs {
    type Output = GitHubMergePrOutput;
    type Prompts = MergePullRequestPrompts;

    const NAME: &'static str = GITHUB_MERGE_PULL_REQUEST;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Merge a pull request";
}
