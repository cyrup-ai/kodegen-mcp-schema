//! Prompt argument types for fs_write_file tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for fs_write_file tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsWriteFilePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple file writing
    /// - "append": Adding to existing files
    /// - "code_files": Writing source code
    /// - "config_files": Writing configuration
    /// - "workflows": Complete write workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
