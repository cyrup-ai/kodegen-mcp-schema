//! Schema types for db_stored_procedures tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::super::types::ProcedureInfo;
use super::prompts::StoredProceduresPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Tool name: List stored procedures
pub const DB_STORED_PROCEDURES: &str = "db_stored_procedures";

// ============================================================================
// TOOL ARGUMENTS
// ============================================================================

/// Arguments for db_stored_procedures tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetStoredProceduresArgs {
    /// Schema name (optional, uses default if not provided)
    #[serde(default)]
    pub schema: Option<String>,

    /// Include detailed information (parameters, return type, definition)
    /// Warning: definition can be large for complex procedures
    #[serde(default)]
    pub include_details: bool,
}

// ============================================================================
// TOOL OUTPUT
// ============================================================================

/// Output from `db_stored_procedures` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetStoredProceduresOutput {
    pub schema: String,
    pub procedures: Vec<ProcedureInfo>,
    pub count: usize,
}

// ============================================================================
// TOOL ARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "db_stored_procedures",
    category = "database",
    description = "List stored procedures (functions) in a schema with parameters and return types."
)]
impl ToolArgs for GetStoredProceduresArgs {
    type Output = GetStoredProceduresOutput;
    type Prompts = StoredProceduresPrompts;

    const NAME: &'static str = DB_STORED_PROCEDURES;
    const CATEGORY: &'static str = "database";
    const DESCRIPTION: &'static str = "List stored procedures (functions) in a schema with parameters and return types.";
}
