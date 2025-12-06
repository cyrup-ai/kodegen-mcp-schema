//! Schema types for git_init tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::InitPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_init
pub const GIT_INIT: &str = "git_init";

// ============================================================================
// GIT_INIT TOOL
// ============================================================================

/// Arguments for `git_init` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitInitArgs {
    /// Path where to initialize the repository
    pub path: String,

    /// Create a bare repository (no working directory)
    #[serde(default)]
    pub bare: bool,

    /// Name of the initial branch (informational only, gix uses default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_branch: Option<String>,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_init` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitInitOutput {
    pub success: bool,
    pub path: String,
    pub bare: bool,
    pub message: String,
}

#[tool_metadata(
    name = "git_init",
    category = "git",
    description = "Initialize a new Git repository at the specified path"
)]
impl ToolArgs for GitInitArgs {
    type Output = GitInitOutput;
    type Prompts = InitPrompts;

    const NAME: &'static str = GIT_INIT;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Initialize a new Git repository at the specified path";
}
