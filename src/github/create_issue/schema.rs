//! Schema types for create_issue tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_CREATE_ISSUE};

use crate::{ToolArgs, tool_metadata};
use super::prompts::CreateIssuePrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for `create_issue` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateIssueArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Issue title
    pub title: String,
    /// Issue body/description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Labels to apply (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Assignees (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_create_issue` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCreateIssueOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    pub html_url: String,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Create a new issue in a repository"
)]
impl ToolArgs for CreateIssueArgs {
    type Output = GitHubCreateIssueOutput;
    type Prompts = CreateIssuePrompts;

    const NAME: &'static str = GITHUB_CREATE_ISSUE;
    const CATEGORY: &'static str = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Create a new issue in a repository";
}
