//! Schema types for git_branch_delete tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_BRANCH_DELETE};
use crate::{ToolArgs, tool_metadata};
use super::prompts::BranchDeletePrompts;

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
    description = "Delete a branch from the repository"
)]
impl ToolArgs for GitBranchDeleteArgs {
    type Output = GitBranchDeleteOutput;
    type Prompts = BranchDeletePrompts;

    const NAME: &'static str = GIT_BRANCH_DELETE;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Delete a branch from the repository";
}
