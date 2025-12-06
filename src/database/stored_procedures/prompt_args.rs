//! Prompt argument types for db_stored_procedures tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for db_stored_procedures tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetStoredProceduresPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Listing procedures
    /// - "signatures": Understanding parameters
    /// - "usage": Calling stored procedures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
