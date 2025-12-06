//! Prompt messages for db_list_schemas tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ListSchemasPromptArgs;

/// Prompt provider for db_list_schemas tool
///
/// This is the ONLY way to provide prompts for db_list_schemas - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ListSchemasPrompts;

impl PromptProvider for ListSchemasPrompts {
    type PromptArgs = ListSchemasPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

// ============================================================================
// PROMPT FUNCTIONS - BASIC SCHEMA LISTING GUIDANCE
// ============================================================================

/// Basic schema listing
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list all database schemas?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "LIST DATABASE SCHEMAS:\n\n\
                 1. List all schemas:\n\
                    db_list_schemas({\"connection\": \"main\"})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"schemas\": [\"public\", \"app\", \"analytics\", \"audit\"]\n\
                 }\n\n\
                 WHEN TO LIST SCHEMAS:\n\
                 - First step when connecting to unfamiliar database\n\
                 - Finding which schema contains target tables\n\
                 - Understanding database organization\n\
                 - Planning data access strategy\n\n\
                 WHAT ARE SCHEMAS:\n\
                 - Schemas are namespaces within a database\n\
                 - Group related tables, views, functions together\n\
                 - Provide logical separation of data\n\
                 - Enable security boundaries and permissions\n\n\
                 COMMON SCHEMA TYPES:\n\
                 - public: Default PostgreSQL schema\n\
                 - dbo: Default SQL Server schema\n\
                 - information_schema: System metadata (read-only)\n\
                 - pg_catalog: PostgreSQL system catalog (read-only)\n\
                 - Custom schemas: User-created namespaces\n\n\
                 DATABASE-SPECIFIC BEHAVIOR:\n\
                 - PostgreSQL: Schemas are true namespaces within a database\n\
                 - MySQL: Uses databases instead of schemas (similar concept)\n\
                 - SQL Server: Schemas provide security boundaries\n\
                 - SQLite: Does not support schemas (single namespace)\n\n\
                 SCHEMA DISCOVERY IS SAFE:\n\
                 - Read-only operation\n\
                 - No data modification\n\
                 - Cannot harm database\n\
                 - Always first step in exploration\n\n\
                 EXAMPLE OUTPUT:\n\
                 {\n\
                   \"schemas\": [\n\
                     \"public\",        // Default user schema\n\
                     \"app\",           // Application data\n\
                     \"analytics\",     // Analytics tables\n\
                     \"audit\",         // Audit logs\n\
                     \"information_schema\",  // System metadata\n\
                     \"pg_catalog\"     // PostgreSQL system\n\
                   ],\n\
                   \"count\": 6\n\
                 }\n\n\
                 FILTERING SYSTEM SCHEMAS:\n\
                 Most workflows ignore system schemas:\n\
                 - Skip: information_schema, pg_catalog, pg_toast\n\
                 - Skip: pg_temp_*, pg_toast_temp_*\n\
                 - Focus on: User-created schemas\n\n\
                 NEXT STEPS:\n\
                 After listing schemas:\n\
                 1. Choose relevant schema (e.g., \"app\")\n\
                 2. List tables: db_list_tables({\"connection\": \"main\", \"schema\": \"app\"})\n\
                 3. Explore table structure\n\
                 4. Query data safely",
            ),
        },
    ]
}

