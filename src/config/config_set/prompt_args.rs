//! Prompt argument types for config_set tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for config_set tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SetConfigValuePromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple value changes
    /// - "security_aware": Understanding implications
    /// - "value_types": Different value formats
    /// - "workflow": Proper inspect-modify-verify
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
