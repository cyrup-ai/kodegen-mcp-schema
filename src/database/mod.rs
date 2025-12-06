//! Database category module

// Shared types
pub mod types;

// Tool modules
pub mod execute_sql;
pub mod list_schemas;
pub mod list_tables;
pub mod table_schema;
pub mod table_indexes;
pub mod stored_procedures;
pub mod pool_stats;

// Re-export shared types at database level
pub use types::*;

// Re-export execute_sql tool
pub use execute_sql::{
    DB_EXECUTE_SQL,
    ExecuteSQLArgs,
    ExecuteSQLOutput,
    DbExecuteSqlPromptArgs,
    DbExecuteSqlPrompts,
};

// Re-export list_schemas tool
pub use list_schemas::{
    DB_LIST_SCHEMAS,
    ListSchemasArgs,
    ListSchemasOutput,
    ListSchemasPromptArgs,
    ListSchemasPrompts,
};

// Re-export list_tables tool
pub use list_tables::{
    DB_LIST_TABLES,
    ListTablesArgs,
    ListTablesOutput,
    ListTablesPromptArgs,
    ListTablesPrompts,
};

// Re-export table_schema tool
pub use table_schema::{
    DB_TABLE_SCHEMA,
    GetTableSchemaArgs,
    GetTableSchemaOutput,
    GetTableSchemaPromptArgs,
    TableSchemaPrompts,
};

// Re-export table_indexes tool
pub use table_indexes::{
    DB_TABLE_INDEXES,
    GetTableIndexesArgs,
    GetTableIndexesOutput,
    GetTableIndexesPromptArgs,
    TableIndexesPrompts,
};

// Re-export stored_procedures tool
pub use stored_procedures::{
    DB_STORED_PROCEDURES,
    GetStoredProceduresArgs,
    GetStoredProceduresOutput,
    GetStoredProceduresPromptArgs,
    StoredProceduresPrompts,
};

// Re-export pool_stats tool
pub use pool_stats::{
    DB_POOL_STATS,
    GetPoolStatsArgs,
    GetPoolStatsOutput,
    GetPoolStatsPromptArgs,
    PoolStatsPrompts,
};
