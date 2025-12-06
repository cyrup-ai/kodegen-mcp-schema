//! Schema types for git_remote_remove tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::RemoteRemovePrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_remote_remove
pub const GIT_REMOTE_REMOVE: &str = "git_remote_remove";

// ============================================================================
// GIT_REMOTE_REMOVE TOOL
// ============================================================================

/// Arguments for `git_remote_remove` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitRemoteRemoveArgs {
    /// Path to repository
    pub path: String,

    /// Remote name to remove (e.g., "origin", "upstream")
    pub name: String,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_remote_remove` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRemoteRemoveOutput {
    pub success: bool,
    pub name: String,
    pub message: String,
}

#[tool_metadata(
    name = "git_remote_remove",
    category = "git",
    description = "Remove a remote repository connection"
)]
impl ToolArgs for GitRemoteRemoveArgs {
    type Output = GitRemoteRemoveOutput;
    type Prompts = RemoteRemovePrompts;

    const NAME: &'static str = GIT_REMOTE_REMOVE;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Remove a remote repository connection";
}
