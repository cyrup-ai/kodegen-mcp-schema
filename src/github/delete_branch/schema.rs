//! Schema types for delete_branch tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::DeleteBranchPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for deleting a branch
pub const GITHUB_DELETE_BRANCH: &str = "github_delete_branch";

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
    name = "github_delete_branch",
    category = "github",
    description = "Delete a branch from a repository"
)]
impl ToolArgs for DeleteBranchArgs {
    type Output = GitHubDeleteBranchOutput;
    type Prompts = DeleteBranchPrompts;

    const NAME: &'static str = GITHUB_DELETE_BRANCH;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Delete a branch from a repository";
}
