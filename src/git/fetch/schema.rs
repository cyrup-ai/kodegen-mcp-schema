//! Schema types for git_fetch tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::FetchPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_fetch
pub const GIT_FETCH: &str = "git_fetch";


// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn default_remote() -> String {
    "origin".to_string()
}

// ============================================================================
// GIT_FETCH TOOL
// ============================================================================

/// Arguments for `git_fetch` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitFetchArgs {
    /// Path to repository
    pub path: String,

    /// Remote name (defaults to "origin")
    #[serde(default = "default_remote")]
    pub remote: String,

    /// Refspecs to fetch (e.g., ["refs/heads/main:refs/remotes/origin/main"]).
    /// If empty, uses repository's configured refspecs for the remote.
    /// 
    /// Accepts both single string and array: `refspecs: "main"` or `refspecs: ["main", "develop"]`
    #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub refspecs: Vec<String>,

    /// Prune remote-tracking branches that no longer exist on remote (default: false)
    #[serde(default)]
    pub prune: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_fetch` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitFetchOutput {
    pub success: bool,
    pub remote: String,
    pub pruned: bool,
}

#[tool_metadata(
    name = "git_fetch",
    category = "git",
    description = "Download changes from a remote repository without merging"
)]
impl ToolArgs for GitFetchArgs {
    type Output = GitFetchOutput;
    type Prompts = FetchPrompts;

    const NAME: &'static str = GIT_FETCH;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Download changes from a remote repository without merging";
}
