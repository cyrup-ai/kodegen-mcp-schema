//! Schema types for git_open tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::OpenPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_open
pub const GIT_OPEN: &str = "git_open";

// ============================================================================
// GIT_OPEN TOOL
// ============================================================================

/// Arguments for `git_open` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitOpenArgs {
    /// Path to the existing repository
    pub path: String,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_open` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitOpenOutput {
    pub success: bool,
    pub path: String,
    pub branch: String,
    pub is_clean: bool,
    pub message: String,
}

#[tool_metadata(
    name = "git_open",
    category = "git",
    description = "Open an existing Git repository and get its current state"
)]
impl ToolArgs for GitOpenArgs {
    type Output = GitOpenOutput;
    type Prompts = OpenPrompts;

    const NAME: &'static str = GIT_OPEN;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Open an existing Git repository and get its current state";
}
