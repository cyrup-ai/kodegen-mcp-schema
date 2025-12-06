//! Schema types for git_remote_add tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::RemoteAddPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_remote_add
pub const GIT_REMOTE_ADD: &str = "git_remote_add";


// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn default_remote() -> String {
    "origin".to_string()
}

// ============================================================================
// GIT_REMOTE_ADD TOOL
// ============================================================================

/// Arguments for `git_remote_add` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitRemoteAddArgs {
    /// Path to repository
    pub path: String,

    /// Remote name (e.g., "origin", "upstream")
    pub name: String,

    /// Remote URL (https, git, ssh, or file URL)
    pub url: String,

    /// Force add (overwrite existing remote with same name)
    #[serde(default)]
    pub force: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_remote_add` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRemoteAddOutput {
    pub success: bool,
    pub name: String,
    pub url: String,
    pub message: String,
}

#[tool_metadata(
    name = "git_remote_add",
    category = "git",
    description = "Add a new remote repository connection"
)]
impl ToolArgs for GitRemoteAddArgs {
    type Output = GitRemoteAddOutput;
    type Prompts = RemoteAddPrompts;

    const NAME: &'static str = GIT_REMOTE_ADD;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Add a new remote repository connection";
}
