//! Schema types for fs_edit_block tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::EditBlockPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for fs_edit_block
pub const FS_EDIT_BLOCK: &str = "fs_edit_block";

// ============================================================================
// EDIT BLOCK ARGS
// ============================================================================

fn default_expected_replacements() -> usize {
    1
}

/// Arguments for `fs_edit_block` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsEditBlockArgs {
    /// Path to the file to edit
    pub path: String,

    /// The exact string to search for and replace
    pub old_string: String,

    /// The replacement string
    pub new_string: String,

    /// Expected number of replacements (defaults to 1)
    #[serde(default = "default_expected_replacements")]
    pub expected_replacements: usize,
}

// ============================================================================
// EDIT BLOCK OUTPUT
// ============================================================================

/// Output from `fs_edit_block` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsEditBlockOutput {
    pub success: bool,
    pub path: String,
    pub replacements_made: u32,
    pub message: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "fs_edit_block",
    category = "filesystem",
    description = "Apply surgical text replacements to files. Takes old_string and new_string, performs exact string replacement"
)]
impl ToolArgs for FsEditBlockArgs {
    type Output = FsEditBlockOutput;
    type Prompts = EditBlockPrompts;

    const NAME: &'static str = FS_EDIT_BLOCK;
    const CATEGORY: &'static str = "filesystem";
    const DESCRIPTION: &'static str = "Apply surgical text replacements to files. Takes old_string and new_string, performs exact string replacement";
}
