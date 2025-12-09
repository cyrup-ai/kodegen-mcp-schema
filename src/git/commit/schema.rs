//! Schema types for git_commit tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_COMMIT};
use crate::{ToolArgs, tool_metadata};
use super::prompts::CommitPrompts;

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
    description = "Create a commit with staged changes"
)]
impl ToolArgs for GitCommitArgs {
    type Output = GitCommitOutput;
    type Prompts = CommitPrompts;

    const NAME: &'static str = GIT_COMMIT;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Create a commit with staged changes";
}
