//! Shared types for database tools

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

/// Typed SQL value - covers all SQL types without serde_json::Value
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum SqlValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Blob(Vec<u8>),  // base64 encoded in JSON serialization
}

/// A typed column value with name and typed value
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SqlColumnValue {
    pub name: String,
    pub value: SqlValue,
}

/// A single SQL row with typed column access
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SqlRow {
    /// Column values with names and typed values
    pub columns: Vec<SqlColumnValue>,
}

/// Error information for a failed SQL statement
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SqlStatementError {
    pub statement_index: usize,
    pub statement: String,
    pub error: String,
}
