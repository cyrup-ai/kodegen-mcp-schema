//! Schema types for fs_list_directory tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::ListDirectoryPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool name for fs_list_directory
pub const FS_LIST_DIRECTORY: &str = "fs_list_directory";

// ============================================================================
// LIST DIRECTORY ARGS
// ============================================================================

/// Arguments for `fs_list_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsListDirectoryArgs {
    /// Path to the directory to list
    pub path: String,

    /// Include hidden files (starting with .)
    #[serde(default)]
    pub include_hidden: bool,
}

// ============================================================================
// LIST DIRECTORY OUTPUT
// ============================================================================

/// Output from `fs_list_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsListDirectoryOutput {
    pub success: bool,
    pub path: String,
    pub total_entries: usize,
    pub directories: usize,
    pub files: usize,
    pub entries: Vec<DirectoryEntry>,
}

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DirectoryEntry {
    pub name: String,
    pub is_directory: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "fs_list_directory",
    category = "filesystem",
    description = "List all files and directories in a specified path. Returns entries prefixed with [DIR] or [FILE]"
)]
impl ToolArgs for FsListDirectoryArgs {
    type Output = FsListDirectoryOutput;
    type Prompts = ListDirectoryPrompts;

    const NAME: &'static str = FS_LIST_DIRECTORY;
    const CATEGORY: &'static str = "filesystem";
    const DESCRIPTION: &'static str = "List all files and directories in a specified path. Returns entries prefixed with [DIR] or [FILE]";
}
