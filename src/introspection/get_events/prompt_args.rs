//! Prompt argument types for introspection_get_events tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for introspection_get_events tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IntrospectionGetEventsPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Getting events
    /// - "filtering": Filter by tool, time
    /// - "analysis": Event analysis patterns
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
