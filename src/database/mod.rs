//! Database category module

// Re-export all database tool name constants from kodegen_config
pub use kodegen_config::{
    DB_EXECUTE_SQL, DB_LIST_SCHEMAS, DB_LIST_TABLES, DB_POOL_STATS,
    DB_STORED_PROCEDURES, DB_TABLE_INDEXES, DB_TABLE_SCHEMA,
};

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
    ExecuteSQLArgs,
    ExecuteSQLOutput,
    DbExecuteSqlPromptArgs,
    DbExecuteSqlPrompts,
};

// Re-export list_schemas tool
pub use list_schemas::{
    ListSchemasArgs,
    ListSchemasOutput,
    ListSchemasPromptArgs,
    ListSchemasPrompts,
};

// Re-export list_tables tool
pub use list_tables::{
    ListTablesArgs,
    ListTablesOutput,
    ListTablesPromptArgs,
    ListTablesPrompts,
};

// Re-export table_schema tool
pub use table_schema::{
    GetTableSchemaArgs,
    GetTableSchemaOutput,
    GetTableSchemaPromptArgs,
    TableSchemaPrompts,
};

// Re-export table_indexes tool
pub use table_indexes::{
    GetTableIndexesArgs,
    GetTableIndexesOutput,
    GetTableIndexesPromptArgs,
    TableIndexesPrompts,
};

// Re-export stored_procedures tool
pub use stored_procedures::{
    GetStoredProceduresArgs,
    GetStoredProceduresOutput,
    GetStoredProceduresPromptArgs,
    StoredProceduresPrompts,
};

// Re-export pool_stats tool
pub use pool_stats::{
    GetPoolStatsArgs,
    GetPoolStatsOutput,
    GetPoolStatsPromptArgs,
    PoolStatsPrompts,
};
