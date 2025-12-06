//! Schema types for db_table_schema tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::super::types::ColumnInfo;
use super::prompts::TableSchemaPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Tool name: Get table schema (column information)
pub const DB_TABLE_SCHEMA: &str = "db_table_schema";

// ============================================================================
// TOOL ARGUMENTS
// ============================================================================

/// Arguments for db_table_schema tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetTableSchemaArgs {
    /// Table name to inspect
    pub table: String,

    /// Schema name (optional, uses default if not provided)
    /// PostgreSQL: defaults to "public"
    /// MySQL/MariaDB: defaults to current DATABASE()
    /// SQLite: defaults to "main"
    /// SQL Server: defaults to "dbo"
    #[serde(default)]
    pub schema: Option<String>,
}

// ============================================================================
// TOOL OUTPUT
// ============================================================================

/// Output from `db_table_schema` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetTableSchemaOutput {
    pub schema: String,
    pub table: String,
    pub columns: Vec<ColumnInfo>,
    pub column_count: usize,
}

// ============================================================================
// TOOL ARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "db_table_schema",
    category = "database",
    description = "Get complete table structure including columns, data types, constraints, primary keys, and foreign key relationships."
)]
impl ToolArgs for GetTableSchemaArgs {
    type Output = GetTableSchemaOutput;
    type Prompts = TableSchemaPrompts;

    const NAME: &'static str = DB_TABLE_SCHEMA;
    const CATEGORY: &'static str = "database";
    const DESCRIPTION: &'static str = "Get complete table structure including columns, data types, constraints, primary keys, and foreign key relationships.";
}
