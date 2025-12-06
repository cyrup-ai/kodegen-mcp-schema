//! Schema types for db_list_tables tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::super::types::TableInfo;
use super::prompts::ListTablesPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Tool name: List tables in a schema
pub const DB_LIST_TABLES: &str = "db_list_tables";

// ============================================================================
// TOOL ARGUMENTS
// ============================================================================

/// Arguments for `db_list_tables` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListTablesArgs {
    /// Optional schema name. If not provided, uses default schema:
    /// - PostgreSQL: 'public'
    /// - MySQL/MariaDB: Current database (from DATABASE())
    /// - SQLite: 'main'
    /// - SQL Server: 'dbo'
    #[serde(default)]
    pub schema: Option<String>,
}

// ============================================================================
// TOOL OUTPUT
// ============================================================================

/// Output from `db_list_tables` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListTablesOutput {
    pub schema: String,
    pub tables: Vec<TableInfo>,
    pub count: usize,
}

// ============================================================================
// TOOL ARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "db_list_tables",
    category = "database",
    description = "List all tables and views in a schema. Use this to discover available tables before querying."
)]
impl ToolArgs for ListTablesArgs {
    type Output = ListTablesOutput;
    type Prompts = ListTablesPrompts;

    const NAME: &'static str = DB_LIST_TABLES;
    const CATEGORY: &'static str = "database";
    const DESCRIPTION: &'static str = "List all tables and views in a schema. Use this to discover available tables before querying.";
}
