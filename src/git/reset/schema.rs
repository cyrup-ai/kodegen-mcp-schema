//! Schema types for git_reset tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_RESET};
use crate::{ToolArgs, tool_metadata};
use super::super::ResetMode;
use super::prompts::ResetPrompts;


// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn default_reset_mode() -> ResetMode {
    ResetMode::Mixed
}

// ============================================================================
// GIT_RESET TOOL
// ============================================================================

/// Arguments for `git_reset` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitResetArgs {
    /// Path to repository
    pub path: String,

    /// Target commit (hash, ref, or symbolic name like "HEAD~1")
    pub target: String,

    /// Reset mode: soft, mixed, or hard (default: mixed)
    #[serde(default = "default_reset_mode")]
    pub mode: ResetMode,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_reset` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitResetOutput {
    pub success: bool,
    pub mode: String,
    pub target: String,
}

#[tool_metadata(
    description = "Reset current HEAD to a specified state (soft/mixed/hard)"
)]
impl ToolArgs for GitResetArgs {
    type Output = GitResetOutput;
    type Prompts = ResetPrompts;

    const NAME: &'static str = GIT_RESET;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Reset current HEAD to a specified state (soft/mixed/hard)";
}
