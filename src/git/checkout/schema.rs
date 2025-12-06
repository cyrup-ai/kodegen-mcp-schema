//! Schema types for git_checkout tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::GitCheckoutPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_checkout
pub const GIT_CHECKOUT: &str = "git_checkout";

// ============================================================================
// GIT_CHECKOUT TOOL
// ============================================================================

/// Arguments for `git_checkout` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitCheckoutArgs {
    /// Path to repository
    pub path: String,

    /// Target reference (branch, tag, or commit)
    pub target: String,

    /// Specific file paths to restore from the target reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,

    /// Create new branch before checking out
    #[serde(default)]
    pub create: bool,

    /// Force checkout (discard local changes)
    #[serde(default)]
    pub force: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_checkout` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitCheckoutOutput {
    pub success: bool,
    pub target: String,
    pub created: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    pub message: String,
}

#[tool_metadata(
    name = "git_checkout",
    category = "git",
    description = "Switch branches or restore files from a specific revision"
)]
impl ToolArgs for GitCheckoutArgs {
    type Output = GitCheckoutOutput;
    type Prompts = GitCheckoutPrompts;

    const NAME: &'static str = GIT_CHECKOUT;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Switch branches or restore files from a specific revision";
}
