//! Schema types for fs_delete_directory tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::DeleteDirectoryPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for fs_delete_directory
pub const FS_DELETE_DIRECTORY: &str = "fs_delete_directory";

// ============================================================================
// DELETE DIRECTORY ARGS
// ============================================================================

/// Arguments for `fs_delete_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsDeleteDirectoryArgs {
    /// Path to the directory to delete
    pub path: String,

    /// Confirm recursive deletion (must be true)
    #[serde(default)]
    pub recursive: bool,
}

// ============================================================================
// DELETE DIRECTORY OUTPUT
// ============================================================================

/// Output from `fs_delete_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsDeleteDirectoryOutput {
    pub success: bool,
    pub path: String,
    pub message: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "fs_delete_directory",
    category = "filesystem",
    description = "Delete a directory and all its contents recursively. This operation is permanent and cannot be undone"
)]
impl ToolArgs for FsDeleteDirectoryArgs {
    type Output = FsDeleteDirectoryOutput;
    type Prompts = DeleteDirectoryPrompts;

    const NAME: &'static str = FS_DELETE_DIRECTORY;
    const CATEGORY: &'static str = "filesystem";
    const DESCRIPTION: &'static str = "Delete a directory and all its contents recursively. This operation is permanent and cannot be undone";
}
