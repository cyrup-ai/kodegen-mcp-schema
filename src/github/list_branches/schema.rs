//! Schema types for list_branches tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_LIST_BRANCHES};

use crate::{ToolArgs, tool_metadata};
use super::prompts::ListBranchesPrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for listing branches
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListBranchesArgs {
    /// Repository owner
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Page number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page (optional, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u8>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_list_branches` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubListBranchesOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub count: usize,
    pub branches: Vec<GitHubBranch>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubBranch {
    pub name: String,
    pub sha: String,
    pub protected: bool,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "List branches in a GitHub repository"
)]
impl ToolArgs for ListBranchesArgs {
    type Output = GitHubListBranchesOutput;
    type Prompts = ListBranchesPrompts;

    const NAME: &'static str = GITHUB_LIST_BRANCHES;
    const CATEGORY: &'static str = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "List branches in a GitHub repository";
}
