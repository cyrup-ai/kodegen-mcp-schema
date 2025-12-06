//! Prompt messages for db_stored_procedures tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetStoredProceduresPromptArgs;

/// Prompt provider for db_stored_procedures tool
///
/// This is the ONLY way to provide prompts for db_stored_procedures - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct StoredProceduresPrompts;

impl PromptProvider for StoredProceduresPrompts {
    type PromptArgs = GetStoredProceduresPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("signatures") => prompt_signatures(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, signatures)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE STORED PROCEDURES
// ============================================================================

/// Basic listing and discovering stored procedures
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list stored procedures in a database?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The db_stored_procedures tool lists stored procedures, functions, and triggers in your database. Here's how to discover available procedures:\n\n\
                 LIST STORED PROCEDURES:\n\
                 db_stored_procedures({\"connection\": \"main\"})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"procedures\": [\n\
                     {\n\
                       \"name\": \"create_user\",\n\
                       \"type\": \"procedure\",\n\
                       \"schema\": \"public\",\n\
                       \"parameters\": [\"name VARCHAR\", \"email VARCHAR\"]\n\
                     },\n\
                     {\n\
                       \"name\": \"calculate_total\",\n\
                       \"type\": \"function\",\n\
                       \"schema\": \"public\",\n\
                       \"parameters\": [\"order_id INT\"],\n\
                       \"returns\": \"DECIMAL\"\n\
                     },\n\
                     {\n\
                       \"name\": \"update_timestamp\",\n\
                       \"type\": \"trigger\",\n\
                       \"schema\": \"public\",\n\
                       \"table\": \"users\",\n\
                       \"event\": \"BEFORE UPDATE\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 TYPES:\n\
                 - procedure: Executes actions, may not return value\n\
                 - function: Returns a value (used in SELECT/WHERE clauses)\n\
                 - trigger: Automatic action on table events (INSERT/UPDATE/DELETE)\n\n\
                 FILTER BY SCHEMA:\n\
                 db_stored_procedures({\"connection\": \"main\", \"schema\": \"public\"})\n\
                 Returns only procedures in the specified schema\n\n\
                 FILTER BY NAME PATTERN:\n\
                 db_stored_procedures({\"connection\": \"main\", \"pattern\": \"user%\"})\n\
                 Returns procedures matching the pattern (SQL LIKE syntax)\n\n\
                 WHAT YOU'LL SEE:\n\
                 - name: Procedure identifier for calling\n\
                 - type: procedure, function, or trigger\n\
                 - schema: Database schema containing the procedure\n\
                 - parameters: Input arguments with types\n\
                 - returns: Return type (for functions)\n\
                 - language: Implementation language (SQL, PL/pgSQL, PL/Python, etc.)\n\n\
                 DATABASE SUPPORT:\n\
                 - PostgreSQL: Full support for functions, procedures, triggers\n\
                 - MySQL: Stored procedures and functions\n\
                 - SQLite: Limited (triggers only, no stored procedures)\n\
                 - SQL Server: Stored procedures, functions, triggers\n\n\
                 DISCOVERY WORKFLOW:\n\
                 1. List all procedures:\n\
                    db_stored_procedures({\"connection\": \"main\"})\n\
                 2. Review names and parameters\n\
                 3. Use db_execute_sql to call them\n\n\
                 COMMON USE CASES:\n\
                 - Discover available business logic\n\
                 - Understand existing database API\n\
                 - Find procedures for specific operations\n\
                 - Review trigger definitions\n\
                 - Check function signatures before calling",
            ),
        },
    ]
}

/// Understanding procedure signatures and parameter modes
fn prompt_signatures() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I understand stored procedure parameters and signatures?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Stored procedures have parameters that define how to call them. Understanding parameter modes is crucial for correct usage.\n\n\
                 UNDERSTANDING PARAMETERS:\n\
                 db_stored_procedures({\"connection\": \"main\", \"name\": \"create_user\"})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"name\": \"create_user\",\n\
                   \"parameters\": [\n\
                     {\"name\": \"p_name\", \"type\": \"VARCHAR(255)\", \"mode\": \"IN\"},\n\
                     {\"name\": \"p_email\", \"type\": \"VARCHAR(255)\", \"mode\": \"IN\"},\n\
                     {\"name\": \"p_user_id\", \"type\": \"INT\", \"mode\": \"OUT\"}\n\
                   ],\n\
                   \"returns\": \"INT\"\n\
                 }\n\n\
                 PARAMETER MODES:\n\n\
                 IN (Input parameter):\n\
                 - Value provided by caller\n\
                 - Used as input to procedure\n\
                 - Cannot be modified\n\
                 - Example: p_name VARCHAR(255) IN\n\n\
                 OUT (Output parameter):\n\
                 - Value returned by procedure\n\
                 - Not provided by caller\n\
                 - Receives value from procedure\n\
                 - Example: p_user_id INT OUT\n\n\
                 INOUT (Input/Output parameter):\n\
                 - Value provided by caller AND modified\n\
                 - Can be read and written\n\
                 - Both input and output\n\
                 - Example: p_counter INT INOUT\n\n\
                 DEFAULT (Optional parameter):\n\
                 - Has a default value\n\
                 - Can be omitted when calling\n\
                 - Example: p_limit INT DEFAULT 10\n\n\
                 PARAMETER EXAMPLES:\n\n\
                 Simple function (all IN parameters):\n\
                 {\n\
                   \"name\": \"calculate_discount\",\n\
                   \"parameters\": [\n\
                     {\"name\": \"p_price\", \"type\": \"DECIMAL\", \"mode\": \"IN\"},\n\
                     {\"name\": \"p_rate\", \"type\": \"DECIMAL\", \"mode\": \"IN\"}\n\
                   ],\n\
                   \"returns\": \"DECIMAL\"\n\
                 }\n\
                 Call: SELECT calculate_discount(100.00, 0.15)\n\n\
                 Procedure with OUT parameter:\n\
                 {\n\
                   \"name\": \"insert_order\",\n\
                   \"parameters\": [\n\
                     {\"name\": \"p_customer_id\", \"type\": \"INT\", \"mode\": \"IN\"},\n\
                     {\"name\": \"p_total\", \"type\": \"DECIMAL\", \"mode\": \"IN\"},\n\
                     {\"name\": \"p_order_id\", \"type\": \"INT\", \"mode\": \"OUT\"}\n\
                   ]\n\
                 }\n\
                 Call: CALL insert_order(123, 99.99, @order_id); SELECT @order_id;\n\n\
                 Function with DEFAULT parameters:\n\
                 {\n\
                   \"name\": \"get_recent_orders\",\n\
                   \"parameters\": [\n\
                     {\"name\": \"p_customer_id\", \"type\": \"INT\", \"mode\": \"IN\"},\n\
                     {\"name\": \"p_limit\", \"type\": \"INT\", \"mode\": \"IN\", \"default\": \"10\"},\n\
                     {\"name\": \"p_offset\", \"type\": \"INT\", \"mode\": \"IN\", \"default\": \"0\"}\n\
                   ],\n\
                   \"returns\": \"TABLE\"\n\
                 }\n\
                 Call: SELECT * FROM get_recent_orders(123)  -- Uses defaults\n\
                 Call: SELECT * FROM get_recent_orders(123, 20)  -- Custom limit\n\
                 Call: SELECT * FROM get_recent_orders(123, 20, 10)  -- Custom limit + offset\n\n\
                 PROCEDURE WITH INOUT:\n\
                 {\n\
                   \"name\": \"increment_counter\",\n\
                   \"parameters\": [\n\
                     {\"name\": \"p_counter\", \"type\": \"INT\", \"mode\": \"INOUT\"},\n\
                     {\"name\": \"p_amount\", \"type\": \"INT\", \"mode\": \"IN\"}\n\
                   ]\n\
                 }\n\
                 Usage:\n\
                 SET @counter = 10;\n\
                 CALL increment_counter(@counter, 5);\n\
                 SELECT @counter;  -- Returns 15\n\n\
                 READING SIGNATURES:\n\
                 When you see parameters:\n\
                 - Count IN parameters: How many values to provide\n\
                 - Check for OUT parameters: Expect return values\n\
                 - Look for DEFAULT: These are optional\n\
                 - Note INOUT: Provide value and receive modified value\n\n\
                 POSTGRESQL SPECIFICS:\n\
                 - Named parameters: Use => syntax\n\
                   SELECT create_user(p_name => 'John', p_email => 'john@example.com')\n\
                 - Positional parameters: Order matters\n\
                   SELECT create_user('John', 'john@example.com')\n\n\
                 MYSQL SPECIFICS:\n\
                 - Use @ for OUT parameters\n\
                   CALL insert_order(123, 99.99, @order_id)\n\
                 - Retrieve with SELECT\n\
                   SELECT @order_id\n\n\
                 BEST PRACTICES:\n\
                 1. Check parameter count before calling\n\
                 2. Match parameter types (INT, VARCHAR, DECIMAL, etc.)\n\
                 3. Respect parameter order for positional calls\n\
                 4. Use named parameters for clarity (PostgreSQL)\n\
                 5. Handle OUT parameters appropriately\n\
                 6. Leverage DEFAULT parameters to simplify calls",
            ),
        },
    ]
}

