//! Schema types for git_tag tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::super::GitTagInfo;
use super::prompts::TagPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_tag
pub const GIT_TAG: &str = "git_tag";


// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn default_tag_operation() -> String {
    "list".to_string()
}

// ============================================================================
// GIT_TAG TOOL
// ============================================================================

/// Arguments for `git_tag` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitTagArgs {
    /// Path to repository
    pub path: String,

    /// Operation: "create", "delete", or "list" (default: "list")
    #[serde(default = "default_tag_operation")]
    pub operation: String,

    /// Tag name (required for create and delete operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Tag message for annotated tags (if provided, creates annotated tag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Target commit (defaults to HEAD if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Force create/delete (overwrite if exists)
    #[serde(default)]
    pub force: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_tag` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitTagOutput {
    pub success: bool,
    pub operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_annotated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GitTagInfo>>,
}

#[tool_metadata(
    name = "git_tag",
    category = "git",
    description = "Create, list, or delete tags in a repository"
)]
impl ToolArgs for GitTagArgs {
    type Output = GitTagOutput;
    type Prompts = TagPrompts;

    const NAME: &'static str = GIT_TAG;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Create, list, or delete tags in a repository";
}
