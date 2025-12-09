//! Schema types for fs_move_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_FILESYSTEM, FS_MOVE_FILE};

use crate::{ToolArgs, tool_metadata};
use super::prompts::MoveFilePrompts;

// ============================================================================
// MOVE FILE ARGS
// ============================================================================

/// Arguments for `fs_move_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsMoveFileArgs {
    /// Source path (file or directory to move)
    pub source: String,

    /// Destination path (where to move it)
    pub destination: String,
}

// ============================================================================
// MOVE FILE OUTPUT
// ============================================================================

/// Output from `fs_move_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsMoveFileOutput {
    pub success: bool,
    pub source: String,
    pub destination: String,
    pub message: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Move or rename files and directories. Can move files between directories and rename them in a single operation"
)]
impl ToolArgs for FsMoveFileArgs {
    type Output = FsMoveFileOutput;
    type Prompts = MoveFilePrompts;

    const NAME: &'static str = FS_MOVE_FILE;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_FILESYSTEM;
    const DESCRIPTION: &'static str = "Move or rename files and directories. Can move files between directories and rename them in a single operation";
}
