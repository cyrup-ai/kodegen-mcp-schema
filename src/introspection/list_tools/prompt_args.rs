//! Prompt argument types for introspection_list_tools tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for introspection_list_tools tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IntrospectionListToolsPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Listing all tools
    /// - "filtering": Filter by category
    /// - "discovery": Tool discovery patterns
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
