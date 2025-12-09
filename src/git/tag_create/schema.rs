//! Schema types for git_tag_create tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_TAG_CREATE};
use crate::{ToolArgs, tool_metadata};
use super::prompts::TagCreatePrompts;

// ============================================================================
// GIT_TAG_CREATE TOOL
// ============================================================================

/// Arguments for `git_tag_create` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitTagCreateArgs {
    /// Path to repository
    pub path: String,

    /// Tag name (e.g., "v1.0.0")
    pub name: String,

    /// Tag message for annotated tags (if provided, creates annotated tag; if omitted, creates lightweight tag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Target commit (defaults to HEAD if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Force create (overwrite if exists)
    #[serde(default)]
    pub force: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_tag_create` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitTagCreateOutput {
    pub success: bool,
    pub name: String,
    pub is_annotated: bool,
    pub target_commit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[tool_metadata(
    description = "Create annotated or lightweight tags to mark specific points in repository history"
)]
impl ToolArgs for GitTagCreateArgs {
    type Output = GitTagCreateOutput;
    type Prompts = TagCreatePrompts;

    const NAME: &'static str = GIT_TAG_CREATE;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Create annotated or lightweight tags to mark specific points in repository history";
}
