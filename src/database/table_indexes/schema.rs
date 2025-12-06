//! Schema types for db_table_indexes tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::super::types::IndexInfo;
use super::prompts::TableIndexesPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Tool name: Get table index information
pub const DB_TABLE_INDEXES: &str = "db_table_indexes";

// ============================================================================
// TOOL ARGUMENTS
// ============================================================================

/// Arguments for db_table_indexes tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetTableIndexesArgs {
    /// Table name to inspect
    pub table: String,

    /// Schema name (optional, uses default if not provided)
    #[serde(default)]
    pub schema: Option<String>,
}

// ============================================================================
// TOOL OUTPUT
// ============================================================================

/// Output from `db_table_indexes` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetTableIndexesOutput {
    pub schema: String,
    pub table: String,
    pub indexes: Vec<IndexInfo>,
    pub count: usize,
}

// ============================================================================
// TOOL ARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "db_table_indexes",
    category = "database",
    description = "View indexes on a table including types (btree, hash, gin), columns, and uniqueness constraints for query optimization."
)]
impl ToolArgs for GetTableIndexesArgs {
    type Output = GetTableIndexesOutput;
    type Prompts = TableIndexesPrompts;

    const NAME: &'static str = DB_TABLE_INDEXES;
    const CATEGORY: &'static str = "database";
    const DESCRIPTION: &'static str = "View indexes on a table including types (btree, hash, gin), columns, and uniqueness constraints for query optimization.";
}
