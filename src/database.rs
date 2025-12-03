//! Database tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ToolArgs;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================
// These constants are the single source of truth for database tool names.
// Reference these constants in tool implementations and metadata registration
// to ensure consistency and enable compile-time verification.

/// Tool name: Execute SQL queries
pub const DB_EXECUTE_SQL: &str = "db_execute_sql";

/// Tool name: Get connection pool statistics
pub const DB_POOL_STATS: &str = "db_pool_stats";

/// Tool name: List stored procedures
pub const DB_STORED_PROCEDURES: &str = "db_stored_procedures";

/// Tool name: Get table index information
pub const DB_TABLE_INDEXES: &str = "db_table_indexes";

/// Tool name: Get table schema (column information)
pub const DB_TABLE_SCHEMA: &str = "db_table_schema";

/// Tool name: List database schemas
pub const DB_LIST_SCHEMAS: &str = "db_list_schemas";

/// Tool name: List tables in a schema
pub const DB_LIST_TABLES: &str = "db_list_tables";

// ============================================================================
// DB_STORED_PROCEDURES
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

/// Prompt arguments for db_stored_procedures tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetStoredProceduresPromptArgs {}

// ============================================================================
// DB_TABLE_INDEXES
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

/// Prompt arguments for db_table_indexes tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetTableIndexesPromptArgs {}

// ============================================================================
// DB_LIST_SCHEMAS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListSchemasArgs {
    // Empty - no parameters needed
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListSchemasPromptArgs {
    /// Optional: Focus teaching examples on a specific database type
    /// Helps agents learn patterns relevant to their actual database system
    /// Examples: "postgresql", "mysql", "sqlite", "mariadb", "sql_server"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    
    /// Optional: Include the full schema discovery workflow in teaching
    /// When true: Shows list_schemas → list_tables → describe_table → query chain
    /// When false: Focuses only on list_schemas capabilities and behavior
    /// Default: true (shows full workflow for better context)
    #[serde(default = "default_include_workflow")]
    pub include_workflow: bool,
}

fn default_include_workflow() -> bool {
    true
}

// ============================================================================
// DB_LIST_TABLES
// ============================================================================

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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListTablesPromptArgs {}

// ============================================================================
// DB_POOL_STATS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPoolStatsArgs {}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPoolStatsPromptArgs {}

// ============================================================================
// DB_TABLE_SCHEMA
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

/// Prompt arguments for db_table_schema tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetTableSchemaPromptArgs {
    /// Optional: database type to focus examples on (postgres, mysql, sqlite, sql_server)
    /// Helpful for agents working with specific database systems to see relevant schema syntax
    #[serde(default)]
    pub database_type: Option<String>,

    /// Optional: focus area for learning (constraints, indexes, data_types, defaults, nullability, workflow)
    /// Customizes the teaching conversation to emphasize specific aspects of schema inspection
    #[serde(default)]
    pub focus_area: Option<String>,
}

// ============================================================================
// DB_EXECUTE_SQL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteSQLArgs {
    /// SQL query or multiple SQL statements (separated by semicolons)
    /// Multi-statement queries are executed within a transaction for consistency.
    pub sql: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteSQLPromptArgs {
    /// Optional: database type to focus examples on (postgres, mysql, sqlite, etc.)
    #[serde(default)]
    pub database_type: Option<String>,
}

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

/// Column information for table schema
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    pub is_primary_key: bool,
}

/// Foreign key constraint information
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ForeignKeyInfo {
    pub column: String,
    pub references_table: String,
    pub references_column: String,
}

/// Index information
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IndexInfo {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
    pub is_primary: bool,
}

/// Table summary information
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TableInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_type: Option<String>,
}

/// Stored procedure information
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcedureInfo {
    pub name: String,
    pub procedure_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

/// Connection pool statistics
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConnectionStats {
    pub total: u32,
    pub active: u32,
    pub idle: usize,
}

/// Pool configuration details
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PoolConfiguration {
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_secs: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_secs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lifetime_secs: Option<u64>,
    pub test_before_acquire: bool,
}

/// Pool health status
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PoolHealth {
    pub status: String,
    pub utilization_pct: u32,
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `db_list_schemas` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListSchemasOutput {
    pub schemas: Vec<String>,
    pub count: usize,
}

/// Output from `db_list_tables` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListTablesOutput {
    pub schema: String,
    pub tables: Vec<TableInfo>,
    pub count: usize,
}

/// Output from `db_execute_sql` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteSQLOutput {
    pub columns: Vec<String>,
    pub rows: Vec<serde_json::Value>,
    pub row_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_rows: Option<u64>,
    pub execution_time_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_statements: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_statements: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<serde_json::Value>>,
}

/// Output from `db_table_schema` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetTableSchemaOutput {
    pub schema: String,
    pub table: String,
    pub columns: Vec<ColumnInfo>,
    pub column_count: usize,
}

/// Output from `db_table_indexes` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetTableIndexesOutput {
    pub schema: String,
    pub table: String,
    pub indexes: Vec<IndexInfo>,
    pub count: usize,
}

/// Output from `db_stored_procedures` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetStoredProceduresOutput {
    pub schema: String,
    pub procedures: Vec<ProcedureInfo>,
    pub count: usize,
}

/// Output from `db_pool_stats` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPoolStatsOutput {
    pub database_type: String,
    pub connections: ConnectionStats,
    pub configuration: PoolConfiguration,
    pub health: PoolHealth,
}

// ============================================================================
// TOOL ARGS IMPLEMENTATIONS (Args→Output Binding)
// ============================================================================

impl ToolArgs for ListSchemasArgs {
    type Output = ListSchemasOutput;
}

impl ToolArgs for ListTablesArgs {
    type Output = ListTablesOutput;
}

impl ToolArgs for ExecuteSQLArgs {
    type Output = ExecuteSQLOutput;
}

impl ToolArgs for GetTableSchemaArgs {
    type Output = GetTableSchemaOutput;
}

impl ToolArgs for GetTableIndexesArgs {
    type Output = GetTableIndexesOutput;
}

impl ToolArgs for GetStoredProceduresArgs {
    type Output = GetStoredProceduresOutput;
}

impl ToolArgs for GetPoolStatsArgs {
    type Output = GetPoolStatsOutput;
}
