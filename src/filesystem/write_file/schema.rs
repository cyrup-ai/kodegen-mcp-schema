//! Schema types for fs_write_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::WriteFilePrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for fs_write_file
pub const FS_WRITE_FILE: &str = "fs_write_file";

// ============================================================================
// WRITE FILE ARGS
// ============================================================================

fn default_mode() -> String {
    "rewrite".to_string()
}

/// Arguments for `fs_write_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsWriteFileArgs {
    /// Path to the file to write
    pub path: String,

    /// Content to write to the file
    pub content: String,

    /// Write mode: "rewrite" (default) or "append"
    #[serde(default = "default_mode")]
    pub mode: String,
}

// ============================================================================
// WRITE FILE OUTPUT
// ============================================================================

/// Output from `fs_write_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsWriteFileOutput {
    pub success: bool,
    pub path: String,
    pub bytes_written: u64,
    pub lines_written: u64,
    pub mode: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "fs_write_file",
    category = "filesystem",
    description = "Write or append to file contents. Supports two modes: 'rewrite' (overwrite entire file) and 'append' (add to end of file)"
)]
impl ToolArgs for FsWriteFileArgs {
    type Output = FsWriteFileOutput;
    type Prompts = WriteFilePrompts;

    const NAME: &'static str = FS_WRITE_FILE;
    const CATEGORY: &'static str = "filesystem";
    const DESCRIPTION: &'static str = "Write or append to file contents. Supports two modes: 'rewrite' (overwrite entire file) and 'append' (add to end of file)";
}
