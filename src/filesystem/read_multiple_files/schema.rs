//! Schema types for fs_read_multiple_files tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_FILESYSTEM, FS_READ_MULTIPLE_FILES};
use crate::{ToolArgs, tool_metadata};
use super::prompts::ReadMultipleFilesPrompts;

// ============================================================================
// READ MULTIPLE FILES ARGS
// ============================================================================

/// Arguments for `fs_read_multiple_files` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsReadMultipleFilesArgs {
    /// List of file paths to read
    ///
    /// Accepts both single string and array: `paths: "file.txt"` or `paths: ["file1.txt", "file2.txt"]`
    #[serde(deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub paths: Vec<String>,

    /// Line offset for all files (optional)
    /// Positive: Start from line N (0-based indexing)
    /// Negative: Read last N lines from end (tail behavior)
    #[serde(default)]
    pub offset: i64,

    /// Max lines to read per file (optional)
    /// Ignored when offset is negative
    #[serde(default)]
    pub length: Option<usize>,
}

// ============================================================================
// READ MULTIPLE FILES OUTPUT
// ============================================================================

/// Output from `fs_read_multiple_files` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsReadMultipleFilesOutput {
    pub success: bool,
    pub files_requested: usize,
    pub files_read: usize,
    pub files_failed: usize,
    pub results: Vec<FileReadResult>,
}

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileReadResult {
    pub path: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Read multiple files in parallel. Returns results for all files, including errors for individual files that fail"
)]
impl ToolArgs for FsReadMultipleFilesArgs {
    type Output = FsReadMultipleFilesOutput;
    type Prompts = ReadMultipleFilesPrompts;

    const NAME: &'static str = FS_READ_MULTIPLE_FILES;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_FILESYSTEM;
    const DESCRIPTION: &'static str = "Read multiple files in parallel. Returns results for all files, including errors for individual files that fail";
}
