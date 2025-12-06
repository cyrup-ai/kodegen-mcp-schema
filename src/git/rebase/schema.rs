//! Schema types for git_rebase tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::RebasePrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_rebase
pub const GIT_REBASE: &str = "git_rebase";

// ============================================================================
// GIT_REBASE TOOL
// ============================================================================

/// Arguments for `git_rebase` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitRebaseArgs {
    /// Path to repository
    pub path: String,

    /// Branch or commit to rebase onto (the new base)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream: Option<String>,

    /// Alternative base to place commits onto (used with upstream)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onto: Option<String>,

    /// Enable interactive rebase mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,

    /// Continue rebase after resolving conflicts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#continue: Option<bool>,

    /// Skip current commit and continue rebase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<bool>,

    /// Abort rebase and return to original state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort: Option<bool>,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_rebase` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRebaseOutput {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits_rebased: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<String>>,
    pub message: String,
}

#[tool_metadata(
    name = "git_rebase",
    category = "git",
    description = "Reapply commits on top of another base"
)]
impl ToolArgs for GitRebaseArgs {
    type Output = GitRebaseOutput;
    type Prompts = RebasePrompts;

    const NAME: &'static str = GIT_REBASE;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Reapply commits on top of another base";
}
