//! Prompt messages for db_table_schema tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetTableSchemaPromptArgs;

/// Prompt provider for db_table_schema tool
///
/// This is the ONLY way to provide prompts for db_table_schema - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct TableSchemaPrompts;

impl PromptProvider for TableSchemaPrompts {
    type PromptArgs = GetTableSchemaPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") | None => prompt_basic(),
            _ => prompt_basic(), // Default to basic for any unknown scenario
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE DB_TABLE_SCHEMA
// ============================================================================

/// Basic table structure inspection
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I get the structure of a database table?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use db_table_schema to retrieve complete table structure including columns, types, and constraints.\n\n\
                 GET TABLE SCHEMA:\n\
                 db_table_schema({\"connection\": \"main\", \"table\": \"users\"})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"table\": \"users\",\n\
                   \"schema\": \"public\",\n\
                   \"columns\": [\n\
                     {\n\
                       \"name\": \"id\",\n\
                       \"type\": \"INT\",\n\
                       \"nullable\": false,\n\
                       \"primary_key\": true,\n\
                       \"auto_increment\": true\n\
                     },\n\
                     {\n\
                       \"name\": \"email\",\n\
                       \"type\": \"VARCHAR(255)\",\n\
                       \"nullable\": false,\n\
                       \"unique\": true\n\
                     },\n\
                     {\n\
                       \"name\": \"name\",\n\
                       \"type\": \"VARCHAR(100)\",\n\
                       \"nullable\": true,\n\
                       \"default\": null\n\
                     },\n\
                     {\n\
                       \"name\": \"created_at\",\n\
                       \"type\": \"TIMESTAMP\",\n\
                       \"nullable\": false,\n\
                       \"default\": \"CURRENT_TIMESTAMP\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 KEY INFORMATION:\n\
                 - type: Data type with size (VARCHAR(255), INT, etc.)\n\
                 - nullable: Can column be NULL? (true/false)\n\
                 - primary_key: Is this the primary key? (true/false)\n\
                 - unique: Must values be unique? (true/false)\n\
                 - default: Value if not provided (or null if required)\n\
                 - auto_increment: Does database generate value automatically?\n\n\
                 UNDERSTANDING COLUMN PROPERTIES:\n\n\
                 TYPE - Data storage format:\n\
                 - VARCHAR(N): Variable-length string, max N characters\n\
                 - INT: 32-bit integer\n\
                 - TIMESTAMP: Date and time\n\
                 - DECIMAL(P,S): Decimal number, P total digits, S after decimal\n\n\
                 NULLABLE - Can field be empty?\n\
                 - false: Required field (NOT NULL constraint)\n\
                 - true: Optional field (can be NULL)\n\n\
                 PRIMARY_KEY - Unique identifier:\n\
                 - Automatically indexed for fast lookups\n\
                 - Used for foreign key references\n\
                 - Cannot be NULL\n\n\
                 UNIQUE - No duplicates allowed:\n\
                 - Common for email, username fields\n\
                 - NULL values can occur multiple times\n\n\
                 DEFAULT - Automatic value:\n\
                 - CURRENT_TIMESTAMP: Current date/time\n\
                 - nextval(...): Auto-incrementing sequence\n\
                 - Literal values: 0, '', 'active'\n\
                 - null: Must be provided in INSERT\n\n\
                 WHY INSPECT SCHEMA BEFORE QUERIES:\n\
                 1. Avoid type errors - Know what data types to use\n\
                 2. Avoid NULL errors - Know which fields are required\n\
                 3. Understand constraints - Know validation rules\n\
                 4. Find primary keys - Know unique identifiers\n\
                 5. Discover defaults - Know what's auto-generated",
            ),
        },
    ]
}
