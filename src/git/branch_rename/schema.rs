//! Schema types for git_branch_rename tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::BranchRenamePrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_branch_rename
pub const GIT_BRANCH_RENAME: &str = "git_branch_rename";

// ============================================================================
// GIT_BRANCH_RENAME TOOL
// ============================================================================

/// Arguments for `git_branch_rename` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitBranchRenameArgs {
    /// Path to repository
    pub path: String,

    /// Current branch name
    pub old_name: String,

    /// New branch name
    pub new_name: String,

    /// Force rename (overwrite if new name exists)
    #[serde(default)]
    pub force: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_branch_rename` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitBranchRenameOutput {
    pub success: bool,
    pub old_name: String,
    pub new_name: String,
    pub message: String,
}

#[tool_metadata(
    name = "git_branch_rename",
    category = "git",
    description = "Rename an existing branch"
)]
impl ToolArgs for GitBranchRenameArgs {
    type Output = GitBranchRenameOutput;
    type Prompts = BranchRenamePrompts;

    const NAME: &'static str = GIT_BRANCH_RENAME;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Rename an existing branch";
}
