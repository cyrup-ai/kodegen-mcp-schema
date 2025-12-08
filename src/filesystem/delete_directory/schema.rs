//! Schema types for fs_delete_directory tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_FILESYSTEM, FS_DELETE_DIRECTORY};

use crate::{ToolArgs, tool_metadata};
use super::prompts::DeleteDirectoryPrompts;

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
    description = "Delete a directory and all its contents recursively. This operation is permanent and cannot be undone"
)]
impl ToolArgs for FsDeleteDirectoryArgs {
    type Output = FsDeleteDirectoryOutput;
    type Prompts = DeleteDirectoryPrompts;

    const NAME: &'static str = FS_DELETE_DIRECTORY;
    const CATEGORY: &'static str = CATEGORY_FILESYSTEM;
    const DESCRIPTION: &'static str = "Delete a directory and all its contents recursively. This operation is permanent and cannot be undone";
}
