//! Schema types for git_remote_list tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::super::GitRemoteInfo;
use super::prompts::RemoteListPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_remote_list
pub const GIT_REMOTE_LIST: &str = "git_remote_list";

// ============================================================================
// GIT_REMOTE_LIST TOOL
// ============================================================================

/// Arguments for `git_remote_list` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitRemoteListArgs {
    /// Path to repository
    pub path: String,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_remote_list` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRemoteListOutput {
    pub success: bool,
    pub count: usize,
    pub remotes: Vec<GitRemoteInfo>,
}

#[tool_metadata(
    name = "git_remote_list",
    category = "git",
    description = "List all remote repository connections"
)]
impl ToolArgs for GitRemoteListArgs {
    type Output = GitRemoteListOutput;
    type Prompts = RemoteListPrompts;

    const NAME: &'static str = GIT_REMOTE_LIST;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "List all remote repository connections";
}
