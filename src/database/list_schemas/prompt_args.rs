//! Prompt argument types for db_list_schemas tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for db_list_schemas tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListSchemasPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple schema listing
    /// - "exploration": Database discovery workflow
    /// - "multi_schema": Working with multiple schemas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
