//! Schema types for git_branch_list tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::BranchListPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_branch_list
pub const GIT_BRANCH_LIST: &str = "git_branch_list";

// ============================================================================
// GIT_BRANCH_LIST TOOL
// ============================================================================

/// Arguments for `git_branch_list` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitBranchListArgs {
    /// Path to repository
    pub path: String,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_branch_list` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitBranchListOutput {
    pub success: bool,
    pub branches: Vec<String>,
    pub count: usize,
}

#[tool_metadata(
    name = "git_branch_list",
    category = "git",
    description = "List all branches in the repository"
)]
impl ToolArgs for GitBranchListArgs {
    type Output = GitBranchListOutput;
    type Prompts = BranchListPrompts;

    const NAME: &'static str = GIT_BRANCH_LIST;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "List all branches in the repository";
}
