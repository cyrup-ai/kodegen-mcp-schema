//! Schema types for fs_create_directory tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_FILESYSTEM, FS_CREATE_DIRECTORY};

use crate::{ToolArgs, tool_metadata};
use super::prompts::CreateDirectoryPrompts;

// ============================================================================
// CREATE DIRECTORY ARGS
// ============================================================================

/// Arguments for `fs_create_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsCreateDirectoryArgs {
    /// Path to the directory to create
    pub path: String,
}

// ============================================================================
// CREATE DIRECTORY OUTPUT
// ============================================================================

/// Output from `fs_create_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsCreateDirectoryOutput {
    pub success: bool,
    pub path: String,
    pub created: bool,
    pub message: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Create a new directory or ensure a directory exists. Automatically creates parent directories (like mkdir -p)"
)]
impl ToolArgs for FsCreateDirectoryArgs {
    type Output = FsCreateDirectoryOutput;
    type Prompts = CreateDirectoryPrompts;

    const NAME: &'static str = FS_CREATE_DIRECTORY;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_FILESYSTEM;
    const DESCRIPTION: &'static str = "Create a new directory or ensure a directory exists. Automatically creates parent directories (like mkdir -p)";
}
