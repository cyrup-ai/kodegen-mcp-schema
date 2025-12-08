//! Schema types for git_branch_list tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_BRANCH_LIST};
use crate::{ToolArgs, tool_metadata};
use super::prompts::BranchListPrompts;

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
    description = "List all branches in the repository"
)]
impl ToolArgs for GitBranchListArgs {
    type Output = GitBranchListOutput;
    type Prompts = BranchListPrompts;

    const NAME: &'static str = GIT_BRANCH_LIST;
    const CATEGORY: &'static str = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "List all branches in the repository";
}
