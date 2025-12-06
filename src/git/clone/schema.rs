//! Schema types for git_clone tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::ClonePrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_clone
pub const GIT_CLONE: &str = "git_clone";

// ============================================================================
// GIT_CLONE TOOL
// ============================================================================

/// Arguments for `git_clone` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitCloneArgs {
    /// Git URL to clone from (https:// or git://)
    pub url: String,

    /// Local path to clone into
    pub path: String,

    /// Specific branch to checkout (defaults to repository default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,

    /// Shallow clone depth (minimum: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_clone` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitCloneOutput {
    pub success: bool,
    pub url: String,
    pub path: String,
    pub branch: String,
    pub shallow: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
    pub message: String,
}

#[tool_metadata(
    name = "git_clone",
    category = "git",
    description = "Clone a remote Git repository to a local path"
)]
impl ToolArgs for GitCloneArgs {
    type Output = GitCloneOutput;
    type Prompts = ClonePrompts;

    const NAME: &'static str = GIT_CLONE;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Clone a remote Git repository to a local path";
}
