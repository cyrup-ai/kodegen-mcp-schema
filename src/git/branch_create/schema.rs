//! Schema types for git_branch_create tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::BranchCreatePrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_branch_create
pub const GIT_BRANCH_CREATE: &str = "git_branch_create";

// ============================================================================
// GIT_BRANCH_CREATE TOOL
// ============================================================================

/// Arguments for `git_branch_create` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitBranchCreateArgs {
    /// Path to repository
    pub path: String,

    /// Name for new branch
    pub branch: String,

    /// Starting point (defaults to HEAD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_branch: Option<String>,

    /// Force creation (overwrite if exists)
    #[serde(default)]
    pub force: bool,

    /// Checkout the branch after creation
    #[serde(default)]
    pub checkout: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_branch_create` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitBranchCreateOutput {
    pub success: bool,
    pub branch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_branch: Option<String>,
    pub message: String,
}

#[tool_metadata(
    name = "git_branch_create",
    category = "git",
    description = "Create a new branch from a starting point"
)]
impl ToolArgs for GitBranchCreateArgs {
    type Output = GitBranchCreateOutput;
    type Prompts = BranchCreatePrompts;

    const NAME: &'static str = GIT_BRANCH_CREATE;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Create a new branch from a starting point";
}
