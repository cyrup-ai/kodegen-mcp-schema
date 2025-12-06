//! Schema types for fork_repository tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::ForkRepositoryPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for forking a repository
pub const GITHUB_FORK_REPOSITORY: &str = "github_fork_repository";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for forking a repository
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ForkRepositoryArgs {
    /// Repository owner to fork from
    pub owner: String,
    /// Repository name to fork
    pub repo: String,
    /// Organization to fork to (optional, defaults to user)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_fork_repository` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubForkRepoOutput {
    pub success: bool,
    pub source_owner: String,
    pub source_repo: String,
    pub forked_owner: String,
    pub forked_name: String,
    pub forked_full_name: String,
    pub html_url: String,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_fork_repository",
    category = "github",
    description = "Fork a repository to your account or organization"
)]
impl ToolArgs for ForkRepositoryArgs {
    type Output = GitHubForkRepoOutput;
    type Prompts = ForkRepositoryPrompts;

    const NAME: &'static str = GITHUB_FORK_REPOSITORY;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Fork a repository to your account or organization";
}
