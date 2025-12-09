//! Schema types for db_execute_sql tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_DATABASE, DB_EXECUTE_SQL};
use crate::{ToolArgs, tool_metadata};
use super::super::types::{SqlRow, SqlStatementError};
use super::prompts::DbExecuteSqlPrompts;

// ============================================================================
// TOOL ARGUMENTS
// ============================================================================

/// Arguments for `db_execute_sql` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteSQLArgs {
    /// SQL query or multiple SQL statements (separated by semicolons)
    /// Multi-statement queries are executed within a transaction for consistency.
    pub sql: String,
}

// ============================================================================
// TOOL OUTPUT
// ============================================================================

/// Output from `db_execute_sql` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteSQLOutput {
    pub columns: Vec<String>,
    pub rows: Vec<SqlRow>,  // ✅ TYPED - was Vec<serde_json::Value>
    pub row_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_rows: Option<u64>,
    pub execution_time_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_statements: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_statements: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<SqlStatementError>>,  // ✅ TYPED - was Vec<serde_json::Value>
}

// ============================================================================
// TOOL ARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Execute SQL query with connection pooling and timeout support. Prefer read-only SELECT queries over modifications."
)]
impl ToolArgs for ExecuteSQLArgs {
    type Output = ExecuteSQLOutput;
    type Prompts = DbExecuteSqlPrompts;

    const NAME: &'static str = DB_EXECUTE_SQL;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_DATABASE;
    const DESCRIPTION: &'static str = "Execute SQL query with connection pooling and timeout support. Prefer read-only SELECT queries over modifications.";
}
