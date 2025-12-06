//! Schema types for fs_move_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::MoveFilePrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for fs_move_file
pub const FS_MOVE_FILE: &str = "fs_move_file";

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
    name = "fs_move_file",
    category = "filesystem",
    description = "Move or rename files and directories. Can move files between directories and rename them in a single operation"
)]
impl ToolArgs for FsMoveFileArgs {
    type Output = FsMoveFileOutput;
    type Prompts = MoveFilePrompts;

    const NAME: &'static str = FS_MOVE_FILE;
    const CATEGORY: &'static str = "filesystem";
    const DESCRIPTION: &'static str = "Move or rename files and directories. Can move files between directories and rename them in a single operation";
}
