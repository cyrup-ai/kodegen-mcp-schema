//! Schema types for delete_branch tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_DELETE_BRANCH};

use crate::{ToolArgs, tool_metadata};
use super::prompts::DeleteBranchPrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for deleting a branch
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeleteBranchArgs {
    /// Repository owner
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Branch name to delete
    pub branch_name: String,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_delete_branch` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubDeleteBranchOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub branch_name: String,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Delete a branch from a repository"
)]
impl ToolArgs for DeleteBranchArgs {
    type Output = GitHubDeleteBranchOutput;
    type Prompts = DeleteBranchPrompts;

    const NAME: &'static str = GITHUB_DELETE_BRANCH;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "Delete a branch from a repository";
}
