//! Prompt argument types for db_table_indexes tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for db_table_indexes tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetTableIndexesPromptArgs {
    /// Scenario: types, composite, performance, usage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
