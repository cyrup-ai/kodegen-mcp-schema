//! Prompt argument types for db_list_tables tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for db_list_tables tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListTablesPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple table listing
    /// - "filtering": Finding specific tables
    /// - "exploration": Table discovery workflow
    /// - "views": Tables vs views
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
