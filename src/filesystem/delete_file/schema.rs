//! Schema types for fs_delete_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_FILESYSTEM, FS_DELETE_FILE};

use crate::{ToolArgs, tool_metadata};
use super::prompts::DeleteFilePrompts;

// ============================================================================
// DELETE FILE ARGS
// ============================================================================

/// Arguments for `fs_delete_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsDeleteFileArgs {
    /// Path to the file to delete
    pub path: String,
}

// ============================================================================
// DELETE FILE OUTPUT
// ============================================================================

/// Output from `fs_delete_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsDeleteFileOutput {
    pub success: bool,
    pub path: String,
    pub message: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Delete a file from the filesystem. This operation is permanent and cannot be undone"
)]
impl ToolArgs for FsDeleteFileArgs {
    type Output = FsDeleteFileOutput;
    type Prompts = DeleteFilePrompts;

    const NAME: &'static str = FS_DELETE_FILE;
    const CATEGORY: &'static str = CATEGORY_FILESYSTEM;
    const DESCRIPTION: &'static str = "Delete a file from the filesystem. This operation is permanent and cannot be undone";
}
