//! Schema types for config_set tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_CONFIG, CONFIG_SET};
use crate::{ToolArgs, tool_metadata};
use super::prompts::SetConfigValuePrompts;

// ============================================================================
// CONFIG_SET TOOL
// ============================================================================

/// Configuration value that can be string, number, boolean, or array of strings
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ConfigValue {
    String(String),
    Number(i64),
    Boolean(bool),
    Array(Vec<String>),
}

/// Arguments for `config_set` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SetConfigValueArgs {
    /// Configuration key to update
    pub key: String,

    /// New value (string, number, boolean, or array)
    pub value: ConfigValue,
}

/// Output from `config_set` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConfigSetOutput {
    /// Whether the operation succeeded
    pub success: bool,
    /// The key that was set
    pub key: String,
    /// Human-readable result message
    pub message: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPL
// ============================================================================

#[tool_metadata(
    description = "Set a specific configuration value by key. WARNING: Should be used in a separate chat from file operations and command execution to prevent security violations. Always read config_get first before making changes"
)]
impl ToolArgs for SetConfigValueArgs {
    type Output = ConfigSetOutput;
    type Prompts = SetConfigValuePrompts;

    const NAME: &'static str = CONFIG_SET;
    const CATEGORY: &'static str = CATEGORY_CONFIG;
    const DESCRIPTION: &'static str = "Set a specific configuration value by key. WARNING: Should be used in a separate chat from file operations and command execution to prevent security violations. Always read config_get first before making changes";
}
