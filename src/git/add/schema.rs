//! Schema types for git_add tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::AddPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_add
pub const GIT_ADD: &str = "git_add";

// ============================================================================
// GIT_ADD TOOL
// ============================================================================

/// Arguments for `git_add` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitAddArgs {
    /// Path to repository
    pub path: String,

    /// Specific file paths to stage
    /// 
    /// Accepts both single string and array: `paths: "file.rs"` or `paths: ["file1.rs", "file2.rs"]`
    #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub paths: Vec<String>,

    /// Stage all modified files
    #[serde(default)]
    pub all: bool,

    /// Force add files even if in .gitignore
    #[serde(default)]
    pub force: bool,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_add` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitAddOutput {
    pub success: bool,
    pub all: bool,
    pub paths: Vec<String>,
    pub count: usize,
}

#[tool_metadata(
    name = "git_add",
    category = "git",
    description = "Stage file changes for the next commit"
)]
impl ToolArgs for GitAddArgs {
    type Output = GitAddOutput;
    type Prompts = AddPrompts;

    const NAME: &'static str = GIT_ADD;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Stage file changes for the next commit";
}
