//! Schema types for fs_read_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_FILESYSTEM, FS_READ_FILE};
use crate::{ToolArgs, tool_metadata};
use super::prompts::ReadFilePrompts;

// ============================================================================
// READ FILE ARGS
// ============================================================================

/// Arguments for `fs_read_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsReadFileArgs {
    /// Path to the file to read (or URL if `is_url` is true)
    pub path: String,

    /// Line offset to start reading from (0-based)
    /// Positive: Start from line N (0-based indexing)
    /// Negative: Read last N lines from end (tail behavior)
    #[serde(default)]
    pub offset: i64,

    /// Maximum number of lines to read (None = use tool's default)
    /// Ignored when offset is negative
    #[serde(default)]
    pub length: Option<usize>,

    /// Whether the path is a URL (auto-detected if not specified)
    #[serde(default)]
    pub is_url: bool,
}

// ============================================================================
// READ FILE OUTPUT
// ============================================================================

/// Output from `fs_read_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsReadFileOutput {
    pub success: bool,
    pub path: String,
    pub mime_type: String,
    pub is_image: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_lines: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_read: Option<u64>,
    pub is_partial: bool,
    pub content: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Read the contents of a file from the filesystem or a URL. Supports text files (returned as text) and image files (returned as base64)"
)]
impl ToolArgs for FsReadFileArgs {
    type Output = FsReadFileOutput;
    type Prompts = ReadFilePrompts;

    const NAME: &'static str = FS_READ_FILE;
    const CATEGORY: &'static str = CATEGORY_FILESYSTEM;
    const DESCRIPTION: &'static str = "Read the contents of a file from the filesystem or a URL. Supports text files (returned as text) and image files (returned as base64)";
}
