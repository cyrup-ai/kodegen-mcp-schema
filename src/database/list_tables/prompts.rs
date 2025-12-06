//! Prompt messages for db_list_tables tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ListTablesPromptArgs;

/// Prompt provider for db_list_tables tool
///
/// This is the ONLY way to provide prompts for db_list_tables - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ListTablesPrompts;

impl PromptProvider for ListTablesPrompts {
    type PromptArgs = ListTablesPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("filtering") => prompt_filtering(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, filtering)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST DATABASE TABLES
// ============================================================================

/// Basic table listing operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list tables in a database using db_list_tables?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The db_list_tables tool lists all tables and views in a database schema. Here's how to use it:\n\n\
                 BASIC TABLE LISTING:\n\
                 1. All tables in default schema:\n\
                    db_list_tables({\"connection\": \"main\"})\n\n\
                 2. Tables in specific schema:\n\
                    db_list_tables({\"connection\": \"main\", \"schema\": \"app\"})\n\n\
                 RESPONSE FORMAT:\n\
                 {\n\
                   \"tables\": [\n\
                     {\"name\": \"users\", \"type\": \"table\", \"rows\": 15000},\n\
                     {\"name\": \"orders\", \"type\": \"table\", \"rows\": 50000},\n\
                     {\"name\": \"user_stats\", \"type\": \"view\", \"rows\": null}\n\
                   ]\n\
                 }\n\n\
                 TABLE TYPES:\n\
                 - table: Physical table with data stored on disk\n\
                 - view: Virtual table (stored query, no physical storage)\n\
                 - materialized_view: Cached view results (physical storage)\n\n\
                 UNDERSTANDING THE RESPONSE:\n\
                 - name: Table or view name\n\
                 - type: One of: table, view, materialized_view\n\
                 - rows: Row count (null for views, may be approximate for large tables)\n\n\
                 DEFAULT SCHEMAS BY DATABASE:\n\
                 - PostgreSQL: public\n\
                 - MySQL: Current database name\n\
                 - SQL Server: dbo\n\
                 - SQLite: main\n\n\
                 COMMON PATTERNS:\n\
                 1. List all tables:\n\
                    db_list_tables({\"connection\": \"main\"})\n\
                 2. List tables in production schema:\n\
                    db_list_tables({\"connection\": \"prod\", \"schema\": \"public\"})\n\
                 3. Check specific schema:\n\
                    db_list_tables({\"connection\": \"main\", \"schema\": \"analytics\"})\n\n\
                 WHEN TO USE:\n\
                 - Finding what tables exist in a database\n\
                 - Understanding database structure\n\
                 - Discovering available data sources\n\
                 - Planning queries and data analysis\n\
                 - Database documentation and exploration",
            ),
        },
    ]
}

/// Finding specific tables with patterns
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I find specific tables in a database?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use db_list_tables to discover tables, then filter results programmatically. Here's how:\n\n\
                 FINDING SPECIFIC TABLES:\n\
                 1. Filter by pattern (after getting results):\n\
                    db_list_tables({\"connection\": \"main\"})\n\
                    // Then filter: tables starting with \"user\"\n\
                    results.tables.filter(t => t.name.startsWith('user'))\n\n\
                 2. Filter by type:\n\
                    db_list_tables({\"connection\": \"main\"})\n\
                    // Only physical tables, no views\n\
                    results.tables.filter(t => t.type === 'table')\n\n\
                 3. Find tables by size:\n\
                    db_list_tables({\"connection\": \"main\"})\n\
                    // Large tables (over 100K rows)\n\
                    results.tables.filter(t => t.rows > 100000)\n\n\
                 COMMON PATTERNS:\n\
                 Pattern matching examples:\n\
                 - tables.filter(t => t.name.startsWith('user'))\n\
                   // Tables starting with \"user\": users, user_logs, user_prefs\n\
                 - tables.filter(t => t.name.endsWith('log'))\n\
                   // Tables ending with \"log\": access_log, error_log, audit_log\n\
                 - tables.filter(t => t.name.includes('order'))\n\
                   // Tables containing \"order\": orders, order_items, customer_orders\n\n\
                 FILTERING BY TYPE:\n\
                 1. Only physical tables:\n\
                    const physicalTables = results.tables.filter(t => t.type === 'table');\n\
                    // Excludes views and materialized views\n\n\
                 2. Only views:\n\
                    const views = results.tables.filter(t => t.type === 'view');\n\
                    // Virtual tables only\n\n\
                 3. Persistent objects (tables + materialized views):\n\
                    const persistent = results.tables.filter(t => \n\
                      t.type === 'table' || t.type === 'materialized_view'\n\
                    );\n\n\
                 FILTERING BY SIZE:\n\
                 1. Small tables (good for testing):\n\
                    const small = results.tables.filter(t => t.rows !== null && t.rows < 1000);\n\n\
                 2. Large tables (may need optimization):\n\
                    const large = results.tables.filter(t => t.rows !== null && t.rows > 1000000);\n\n\
                 3. Empty tables:\n\
                    const empty = results.tables.filter(t => t.rows === 0);\n\n\
                 FILTERING BY SCHEMA:\n\
                 1. List tables across multiple schemas:\n\
                    const schemas = ['public', 'analytics', 'staging'];\n\
                    const allTables = [];\n\
                    for (const schema of schemas) {\n\
                      const result = await db_list_tables({connection: 'main', schema});\n\
                      allTables.push(...result.tables.map(t => ({...t, schema})));\n\
                    }\n\n\
                 2. Find table in any schema:\n\
                    const targetTable = 'users';\n\
                    for (const schema of schemas) {\n\
                      const result = await db_list_tables({connection: 'main', schema});\n\
                      const found = result.tables.find(t => t.name === targetTable);\n\
                      if (found) {\n\
                        console.log(`Found ${targetTable} in schema ${schema}`);\n\
                        break;\n\
                      }\n\
                    }\n\n\
                 BEST PRACTICES:\n\
                 - Always check if rows is null before comparing (views return null)\n\
                 - Cache results if filtering multiple times",
            ),
        },
    ]
}
