//! Schema types for git_open tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_OPEN};
use crate::{ToolArgs, tool_metadata};
use super::prompts::OpenPrompts;

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
    description = "Open an existing Git repository and get its current state"
)]
impl ToolArgs for GitOpenArgs {
    type Output = GitOpenOutput;
    type Prompts = OpenPrompts;

    const NAME: &'static str = GIT_OPEN;
    const CATEGORY: &'static str = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Open an existing Git repository and get its current state";
}
