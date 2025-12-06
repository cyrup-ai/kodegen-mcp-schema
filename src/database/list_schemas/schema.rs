//! Schema types for db_list_schemas tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::ListSchemasPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Tool name: List database schemas
pub const DB_LIST_SCHEMAS: &str = "db_list_schemas";

// ============================================================================
// TOOL ARGUMENTS
// ============================================================================

/// Arguments for `db_list_schemas` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListSchemasArgs {
    // Empty - no parameters needed
}

// ============================================================================
// TOOL OUTPUT
// ============================================================================

/// Output from `db_list_schemas` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListSchemasOutput {
    pub schemas: Vec<String>,
    pub count: usize,
}

// ============================================================================
// TOOL ARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "db_list_schemas",
    category = "database",
    description = "List all schemas (namespaces) in the database. Use this to discover database structure before querying."
)]
impl ToolArgs for ListSchemasArgs {
    type Output = ListSchemasOutput;
    type Prompts = ListSchemasPrompts;

    const NAME: &'static str = DB_LIST_SCHEMAS;
    const CATEGORY: &'static str = "database";
    const DESCRIPTION: &'static str = "List all schemas (namespaces) in the database. Use this to discover database structure before querying.";
}
