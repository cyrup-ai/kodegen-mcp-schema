//! Schema types for create_branch tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::CreateBranchPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for creating a branch
pub const GITHUB_CREATE_BRANCH: &str = "github_create_branch";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for creating a branch
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateBranchArgs {
    /// Repository owner
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// New branch name
    pub branch_name: String,
    /// SHA to create branch from
    pub sha: String,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_create_branch` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCreateBranchOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub branch_name: String,
    pub sha: String,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_create_branch",
    category = "github",
    description = "Create a new branch in a repository"
)]
impl ToolArgs for CreateBranchArgs {
    type Output = GitHubCreateBranchOutput;
    type Prompts = CreateBranchPrompts;

    const NAME: &'static str = GITHUB_CREATE_BRANCH;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Create a new branch in a repository";
}
