//! Schema types for git_commit tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::CommitPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_commit
pub const GIT_COMMIT: &str = "git_commit";

// ============================================================================
// GIT_COMMIT TOOL
// ============================================================================

/// Arguments for `git_commit` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitCommitArgs {
    /// Path to repository
    pub path: String,

    /// Commit message
    pub message: String,

    /// Author name (optional, uses git config if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,

    /// Author email (optional, uses git config if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,

    /// Stage all modified tracked files before committing
    #[serde(default)]
    pub all: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_commit` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitCommitOutput {
    pub success: bool,
    pub commit_id: String,
    pub message: String,
    pub file_count: usize,
}

#[tool_metadata(
    name = "git_commit",
    category = "git",
    description = "Create a commit with staged changes"
)]
impl ToolArgs for GitCommitArgs {
    type Output = GitCommitOutput;
    type Prompts = CommitPrompts;

    const NAME: &'static str = GIT_COMMIT;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Create a commit with staged changes";
}
