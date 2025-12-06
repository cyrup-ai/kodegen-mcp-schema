//! Prompt argument types for db_table_schema tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for db_table_schema tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetTableSchemaPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Getting table structure
    /// - "relationships": Understanding foreign keys
    /// - "query_building": Using schema for queries
    /// - "data_types": Understanding column types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
