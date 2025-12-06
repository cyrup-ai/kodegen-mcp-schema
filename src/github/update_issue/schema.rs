//! Schema types for update_issue tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::UpdateIssuePrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for updating an issue
pub const GITHUB_UPDATE_ISSUE: &str = "github_update_issue";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for `update_issue` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateIssueArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Issue number to update
    pub issue_number: u64,
    /// New title (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// New body/description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// New state: "open" or "closed" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Replace labels (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Replace assignees (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_update_issue` / `github_close_issue` tools
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubUpdateIssueOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_update_issue",
    category = "github",
    description = "Update an existing GitHub issue"
)]
impl ToolArgs for UpdateIssueArgs {
    type Output = GitHubUpdateIssueOutput;
    type Prompts = UpdateIssuePrompts;

    const NAME: &'static str = GITHUB_UPDATE_ISSUE;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Update an existing GitHub issue";
}
