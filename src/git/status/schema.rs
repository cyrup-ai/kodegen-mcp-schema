//! Schema types for git_status tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::StatusPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_status
pub const GIT_STATUS: &str = "git_status";

// ============================================================================
// GIT_STATUS TOOL
// ============================================================================

/// Arguments for `git_status` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitStatusArgs {
    /// Path to repository
    pub path: String,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_status` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitStatusOutput {
    pub success: bool,
    pub branch: String,
    pub commit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ahead: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behind: Option<u32>,
    pub is_clean: bool,
    pub is_detached: bool,
}

#[tool_metadata(
    name = "git_status",
    category = "git",
    description = "Check working tree status and branch information"
)]
impl ToolArgs for GitStatusArgs {
    type Output = GitStatusOutput;
    type Prompts = StatusPrompts;

    const NAME: &'static str = GIT_STATUS;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Check working tree status and branch information";
}
