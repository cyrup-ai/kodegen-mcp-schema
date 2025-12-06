//! Schema types for fs_get_file_info tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::GetFileInfoPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for fs_get_file_info
pub const FS_GET_FILE_INFO: &str = "fs_get_file_info";

// ============================================================================
// GET FILE INFO ARGS
// ============================================================================

/// Arguments for `fs_get_file_info` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsGetFileInfoArgs {
    /// Path to the file or directory
    pub path: String,
}

// ============================================================================
// GET FILE INFO OUTPUT
// ============================================================================

/// Output from `fs_get_file_info` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsGetFileInfoOutput {
    pub success: bool,
    pub path: String,
    pub exists: bool,
    pub is_file: bool,
    pub is_directory: bool,
    pub is_symlink: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_count: Option<u64>,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "fs_get_file_info",
    category = "filesystem",
    description = "Retrieve detailed metadata about a file or directory including size, creation time, permissions, and type"
)]
impl ToolArgs for FsGetFileInfoArgs {
    type Output = FsGetFileInfoOutput;
    type Prompts = GetFileInfoPrompts;

    const NAME: &'static str = FS_GET_FILE_INFO;
    const CATEGORY: &'static str = "filesystem";
    const DESCRIPTION: &'static str = "Retrieve detailed metadata about a file or directory including size, creation time, permissions, and type";
}
