//! Prompt argument types for config_get tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for config_get tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConfigGetPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple config retrieval
    /// - "security": Understanding security configuration
    /// - "troubleshooting": Debugging permission issues
    /// - "inspection": Reviewing before modification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
