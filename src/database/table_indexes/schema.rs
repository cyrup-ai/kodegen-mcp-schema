//! Schema types for db_table_indexes tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_DATABASE, DB_TABLE_INDEXES};
use crate::{ToolArgs, tool_metadata};
use super::super::types::IndexInfo;
use super::prompts::TableIndexesPrompts;

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
    description = "View indexes on a table including types (btree, hash, gin), columns, and uniqueness constraints for query optimization."
)]
impl ToolArgs for GetTableIndexesArgs {
    type Output = GetTableIndexesOutput;
    type Prompts = TableIndexesPrompts;

    const NAME: &'static str = DB_TABLE_INDEXES;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_DATABASE;
    const DESCRIPTION: &'static str = "View indexes on a table including types (btree, hash, gin), columns, and uniqueness constraints for query optimization.";
}
