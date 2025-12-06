//! Prompt argument types for fs_read_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for fs_read_file tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsReadFilePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple file reading
    /// - "large_files": Handling big files with offset/length
    /// - "code_files": Reading source code
    /// - "config_files": Reading configuration
    /// - "urls": Reading from URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
