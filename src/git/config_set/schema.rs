//! Schema types for git_config_set tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_CONFIG_SET};
use crate::{ToolArgs, tool_metadata};
use super::prompts::ConfigSetPrompts;

// ============================================================================
// GIT_CONFIG_SET TOOL
// ============================================================================

/// Arguments for `git_config_set` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitConfigSetArgs {
    /// Configuration key to set (e.g., "user.name", "core.editor")
    pub key: String,

    /// Value to set for the configuration key
    pub value: String,

    /// Path to repository (optional, not needed for global scope)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Configuration scope: "local", "global", or "system"
    /// - "local": Repository-specific (.git/config)
    /// - "global": User-wide (~/.gitconfig)
    /// - "system": Machine-wide (/etc/gitconfig)
    ///   Default: "local" if path provided, otherwise "global"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_config_set` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitConfigSetOutput {
    /// Whether the operation succeeded
    pub success: bool,

    /// Configuration key that was set
    pub key: String,

    /// Value that was set
    pub value: String,

    /// Scope where the value was set
    pub scope: String,

    /// Optional message (e.g., error details if success is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[tool_metadata(
    description = "Set Git configuration values"
)]
impl ToolArgs for GitConfigSetArgs {
    type Output = GitConfigSetOutput;
    type Prompts = ConfigSetPrompts;

    const NAME: &'static str = GIT_CONFIG_SET;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Set Git configuration values";
}
