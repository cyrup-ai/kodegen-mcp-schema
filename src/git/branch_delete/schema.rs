//! Schema types for git_branch_delete tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::BranchDeletePrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_branch_delete
pub const GIT_BRANCH_DELETE: &str = "git_branch_delete";

// ============================================================================
// GIT_BRANCH_DELETE TOOL
// ============================================================================

/// Arguments for `git_branch_delete` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitBranchDeleteArgs {
    /// Path to repository
    pub path: String,

    /// Name of branch to delete
    pub branch: String,

    /// Force deletion
    #[serde(default)]
    pub force: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_branch_delete` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitBranchDeleteOutput {
    pub success: bool,
    pub branch: String,
    pub message: String,
}

#[tool_metadata(
    name = "git_branch_delete",
    category = "git",
    description = "Delete a branch from the repository"
)]
impl ToolArgs for GitBranchDeleteArgs {
    type Output = GitBranchDeleteOutput;
    type Prompts = BranchDeletePrompts;

    const NAME: &'static str = GIT_BRANCH_DELETE;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Delete a branch from the repository";
}
