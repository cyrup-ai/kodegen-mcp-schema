//! Schema types for git_config_get tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_CONFIG_GET};
use crate::{ToolArgs, tool_metadata};
use super::prompts::ConfigGetPrompts;

// ============================================================================
// GIT_CONFIG_GET TOOL
// ============================================================================

/// Arguments for `git_config_get` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitConfigGetArgs {
    /// Path to repository (or any path within the repository)
    pub path: String,

    /// Config key to read (e.g., "user.name", "remote.origin.url")
    /// Either provide 'key' for single value OR 'list: true' for all values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Config scope to read from
    /// - "local": Repository-specific (.git/config)
    /// - "global": User-wide (~/.gitconfig)
    /// - "system": Machine-wide (/etc/gitconfig)
    /// - None: Effective config (merged hierarchy, local overrides global overrides system)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// List all config values instead of reading a single key
    /// Set to true to get all configuration values
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<bool>,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Single config key-value pair
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConfigValue {
    /// Config key
    pub key: String,

    /// Config value (null if not set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Output from `git_config_get` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum GitConfigGetOutput {
    /// Single key-value response
    Single {
        /// The config key that was requested
        key: String,

        /// The value of the key (null if not set)
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<String>,

        /// The scope where this value was found (if scope was specified in request)
        #[serde(skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
    },

    /// List of all config values
    List {
        /// Array of all config key-value pairs
        values: Vec<ConfigValue>,
    },
}

#[tool_metadata(
    description = "Read Git configuration values"
)]
impl ToolArgs for GitConfigGetArgs {
    type Output = GitConfigGetOutput;
    type Prompts = ConfigGetPrompts;

    const NAME: &'static str = GIT_CONFIG_GET;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Read Git configuration values";
}
