//! Schema types for git_diff tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::super::GitDiffFile;
use super::prompts::DiffPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_diff
pub const GIT_DIFF: &str = "git_diff";

// ============================================================================
// GIT_DIFF TOOL
// ============================================================================

/// Arguments for `git_diff` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitDiffArgs {
    /// Path to repository
    pub path: String,

    /// Source revision (commit hash, branch name, or 'HEAD')
    pub from: String,

    /// Target revision (optional, defaults to working directory)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_diff` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitDiffOutput {
    pub success: bool,
    pub from: String,
    pub to: String,
    pub files_changed: u32,
    pub insertions: u32,
    pub deletions: u32,
    pub files: Vec<GitDiffFile>,
}

#[tool_metadata(
    name = "git_diff",
    category = "git",
    description = "Show differences between commits, branches, or working directory"
)]
impl ToolArgs for GitDiffArgs {
    type Output = GitDiffOutput;
    type Prompts = DiffPrompts;

    const NAME: &'static str = GIT_DIFF;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Show differences between commits, branches, or working directory";
}
