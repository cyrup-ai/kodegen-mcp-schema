//! Schema types for git_add tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_ADD};
use crate::{ToolArgs, tool_metadata};
use super::prompts::AddPrompts;

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
    description = "Stage file changes for the next commit"
)]
impl ToolArgs for GitAddArgs {
    type Output = GitAddOutput;
    type Prompts = AddPrompts;

    const NAME: &'static str = GIT_ADD;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Stage file changes for the next commit";
}
