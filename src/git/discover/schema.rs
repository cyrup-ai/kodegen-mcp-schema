//! Schema types for git_discover tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::DiscoverPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_discover
pub const GIT_DISCOVER: &str = "git_discover";

// ============================================================================
// GIT_DISCOVER TOOL
// ============================================================================

/// Arguments for `git_discover` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitDiscoverArgs {
    /// Path to search from (can be subdirectory within a repo)
    pub path: String,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_discover` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitDiscoverOutput {
    pub success: bool,
    pub searched_from: String,
    pub repo_root: String,
    pub message: String,
}

#[tool_metadata(
    name = "git_discover",
    category = "git",
    description = "Find the Git repository root from any path inside it"
)]
impl ToolArgs for GitDiscoverArgs {
    type Output = GitDiscoverOutput;
    type Prompts = DiscoverPrompts;

    const NAME: &'static str = GIT_DISCOVER;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Find the Git repository root from any path inside it";
}
