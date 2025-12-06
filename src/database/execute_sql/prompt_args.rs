//! Prompt argument types for db_execute_sql tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for db_execute_sql tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DbExecuteSqlPromptArgs {
    /// Scenario to show examples for
    /// - "select": Safe read queries
    /// - "modification": INSERT/UPDATE/DELETE (dangerous)
    /// - "safety": SQL injection prevention
    /// - "transactions": Transaction handling
    /// - "troubleshooting": Query debugging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
