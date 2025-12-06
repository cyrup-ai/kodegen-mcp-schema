//! Prompt messages for db_execute_sql tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::DbExecuteSqlPromptArgs;

/// Prompt provider for db_execute_sql tool
///
/// This is the ONLY way to provide prompts for db_execute_sql - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct DbExecuteSqlPrompts;

impl PromptProvider for DbExecuteSqlPrompts {
    type PromptArgs = DbExecuteSqlPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("safety") => prompt_safety(),
            Some("patterns") => prompt_patterns(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (default: basic SQL queries, safety: parameterized queries and transactions, patterns: common patterns and troubleshooting)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO EXECUTE SQL SAFELY
// ============================================================================

/// Basic SQL queries (preferred)
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I execute basic SQL queries using db_execute_sql?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "BASIC SQL QUERIES:\n\n\
                 Read operations are the safest database operations. Always prefer SELECT over write operations.\n\n\
                 1. Basic SELECT:\n\
                    db_execute_sql({\n\
                        \"connection\": \"main\",\n\
                        \"query\": \"SELECT * FROM users LIMIT 10\"\n\
                    })\n\
                    - Always use LIMIT unless you need all rows\n\
                    - Safe for exploration and debugging\n\n\
                 2. Filtered query:\n\
                    db_execute_sql({\n\
                        \"connection\": \"main\",\n\
                        \"query\": \"SELECT id, name, email FROM users WHERE active = true\"\n\
                    })\n\
                    - Select only needed columns\n\
                    - Use WHERE clauses to filter data\n\n\
                 3. With parameters (ALWAYS USE FOR USER INPUT):\n\
                    db_execute_sql({\n\
                        \"connection\": \"main\",\n\
                        \"query\": \"SELECT * FROM users WHERE id = ?\",\n\
                        \"params\": [123]\n\
                    })\n\
                    - ? for SQLite/MySQL, $1,$2 for PostgreSQL\n\
                    - Prevents SQL injection attacks\n\n\
                 4. Aggregation:\n\
                    db_execute_sql({\n\
                        \"connection\": \"main\",\n\
                        \"query\": \"SELECT COUNT(*) as total, status FROM orders GROUP BY status\"\n\
                    })\n\
                    - COUNT, SUM, AVG, MIN, MAX for summaries\n\
                    - GROUP BY for grouping results\n\n\
                 5. Joins:\n\
                    db_execute_sql({\n\
                        \"connection\": \"main\",\n\
                        \"query\": \"SELECT u.name, o.total FROM users u JOIN orders o ON u.id = o.user_id LIMIT 100\"\n\
                    })\n\
                    - Combine data from multiple tables\n\
                    - Use table aliases (u, o) for readability\n\
                    - Always include LIMIT with joins\n\n\
                 6. Advanced filtering:\n\
                    db_execute_sql({\n\
                        \"connection\": \"main\",\n\
                        \"query\": \"SELECT * FROM products WHERE price BETWEEN ? AND ? ORDER BY price DESC LIMIT 50\",\n\
                        \"params\": [10, 100]\n\
                    })\n\
                    - BETWEEN for range queries\n\
                    - ORDER BY for sorting\n\n\
                 7. Pattern matching:\n\
                    db_execute_sql({\n\
                        \"connection\": \"main\",\n\
                        \"query\": \"SELECT * FROM users WHERE email LIKE ? LIMIT 50\",\n\
                        \"params\": [\"%@gmail.com\"]\n\
                    })\n\
                    - LIKE for pattern matching (% matches any characters)\n\
                    - Use parameters to prevent injection\n\n\
                 BEST PRACTICES:\n\
                 - Always use LIMIT unless you need all rows\n\
                 - Select only needed columns when possible\n\
                 - Use WHERE clauses to filter early\n\
                 - Use parameters for any variable values\n\
                 - Test complex queries on small datasets first\n\
                 - Use EXPLAIN to analyze query performance",
            ),
        },
    ]
}


/// SQL safety, modifications, and transactions
fn prompt_safety() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I safely execute SQL queries and modifications?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "SQL SAFETY & MODIFICATIONS:\n\n\
                 SQL injection is the most dangerous security vulnerability. ALWAYS use parameterized queries.\n\n\
                 SQL INJECTION PREVENTION:\n\n\
                 ❌ DANGEROUS - String concatenation:\n\
                 db_execute_sql({\"query\": \"SELECT * FROM users WHERE id = \" + user_input})\n\
                 // If user_input = \"1; DROP TABLE users;\" → table deleted!\n\n\
                 ✅ SAFE - Parameterized:\n\
                 db_execute_sql({\n\
                     \"query\": \"SELECT * FROM users WHERE id = ?\",\n\
                     \"params\": [user_input]\n\
                 })\n\
                 - Database treats parameter as data, not code\n\
                 - Automatically escapes special characters\n\
                 - Prevents all SQL injection attacks\n\n\
                 PARAMETER SYNTAX BY DATABASE:\n\
                 SQLite/MySQL: ? placeholders\n\
                 db_execute_sql({\"query\": \"SELECT * FROM users WHERE id = ? AND status = ?\", \"params\": [123, \"active\"]})\n\n\
                 PostgreSQL: $1, $2 placeholders\n\
                 db_execute_sql({\"query\": \"SELECT * FROM users WHERE id = $1 AND status = $2\", \"params\": [123, \"active\"]})\n\n\
                 REAL-WORLD EXAMPLE:\n\
                 db_execute_sql({\n\
                     \"query\": \"SELECT * FROM users WHERE name LIKE ? LIMIT 50\",\n\
                     \"params\": [\"%\" + search_term + \"%\"]\n\
                 })\n\n\
                 MODIFICATION WORKFLOW:\n\n\
                 ⚠️ MODIFICATIONS CAN CAUSE DATA LOSS - EXTREME CARE REQUIRED ⚠️\n\n\
                 BEFORE any UPDATE/DELETE:\n\
                 1. SELECT to preview affected rows:\n\
                    db_execute_sql({\"query\": \"SELECT * FROM users WHERE status = 'inactive' LIMIT 10\"})\n\n\
                 2. COUNT affected rows:\n\
                    db_execute_sql({\"query\": \"SELECT COUNT(*) FROM users WHERE status = 'inactive'\"})\n\n\
                 3. Execute modification:\n\
                    db_execute_sql({\n\
                        \"query\": \"UPDATE users SET status = ? WHERE status = 'inactive'\",\n\
                        \"params\": [\"archived\"]\n\
                    })\n\n\
                 4. Verify the change:\n\
                    db_execute_sql({\"query\": \"SELECT COUNT(*) FROM users WHERE status = 'archived'\"})\n\n\
                 CRITICAL WARNINGS:\n\
                 - NEVER execute UPDATE/DELETE without WHERE clause\n\
                 - ALWAYS use parameterized queries\n\
                 - Run SELECT first to verify what will be affected\n\n\
                 EXAMPLES:\n\
                 INSERT: db_execute_sql({\"query\": \"INSERT INTO logs (message) VALUES (?)\", \"params\": [\"Log entry\"]})\n\
                 UPDATE: db_execute_sql({\"query\": \"UPDATE users SET email = ? WHERE id = ?\", \"params\": [\"new@email.com\", 123]})\n\
                 DELETE: db_execute_sql({\"query\": \"DELETE FROM sessions WHERE expires_at < ?\", \"params\": [\"2024-01-01\"]})\n\n\
                 BASIC TRANSACTIONS:\n\n\
                 Use transactions when multiple related operations must all succeed or all fail.\n\n\
                 STRUCTURE:\n\
                 1. Start: db_execute_sql({\"query\": \"BEGIN TRANSACTION\"})\n\
                 2. Execute operations\n\
                 3. Commit: db_execute_sql({\"query\": \"COMMIT\"})\n\
                 4. Or rollback: db_execute_sql({\"query\": \"ROLLBACK\"})\n\n\
                 EXAMPLE - Money transfer:\n\
                 db_execute_sql({\"query\": \"BEGIN TRANSACTION\"})\n\
                 db_execute_sql({\"query\": \"UPDATE accounts SET balance = balance - ? WHERE id = ?\", \"params\": [100, 1]})\n\
                 db_execute_sql({\"query\": \"UPDATE accounts SET balance = balance + ? WHERE id = ?\", \"params\": [100, 2]})\n\
                 db_execute_sql({\"query\": \"COMMIT\"})\n\n\
                 Use transactions when all operations must succeed or all fail together.",
            ),
        },
    ]
}


/// Common patterns and troubleshooting
fn prompt_patterns() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common SQL patterns and how do I troubleshoot issues?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "COMMON SQL PATTERNS & TROUBLESHOOTING:\n\n\
                 COMMON SELECT PATTERNS:\n\
                 - Explore table: SELECT * FROM table LIMIT 10\n\
                 - Count records: SELECT COUNT(*) FROM table WHERE ...\n\
                 - Find specific record: SELECT * FROM table WHERE id = ?\n\
                 - Recent records: SELECT * FROM table ORDER BY created_at DESC LIMIT 20\n\
                 - Range query: SELECT * FROM table WHERE created_at BETWEEN ? AND ?\n\
                 - Pagination: SELECT * FROM table LIMIT 100 OFFSET 0\n\
                 - Distinct values: SELECT DISTINCT column FROM table\n\n\
                 COMMON MODIFICATION PATTERNS:\n\
                 - INSERT single: INSERT INTO table (col1, col2) VALUES (?, ?)\n\
                 - UPDATE single column: UPDATE table SET col = ? WHERE id = ?\n\
                 - UPDATE multiple: UPDATE table SET col1 = ?, col2 = ? WHERE id = ?\n\
                 - DELETE with condition: DELETE FROM table WHERE created_at < ?\n\
                 - Batch operations: Use transactions for multiple related changes\n\
                 - RETURNING (PostgreSQL): INSERT ... RETURNING id\n\n\
                 TROUBLESHOOTING:\n\n\
                 No results:\n\
                 1. Check table has data: SELECT COUNT(*) FROM table\n\
                 2. Remove WHERE clause temporarily\n\
                 3. Check case sensitivity (PostgreSQL is case-sensitive)\n\
                 4. Use WHERE column IS NULL for NULL checks, not = NULL\n\n\
                 Too many results:\n\
                 1. Add LIMIT immediately\n\
                 2. Use more specific WHERE conditions\n\
                 3. Add date ranges: WHERE created_at BETWEEN ? AND ?\n\n\
                 Syntax error:\n\
                 1. Check quotes around strings ('value' not value)\n\
                 2. Verify SQL keywords spelling\n\
                 3. Test simpler query first\n\n\
                 Performance:\n\
                 1. Use EXPLAIN to analyze query plan\n\
                 2. Check indexes exist with db_table_indexes\n\
                 3. Look for \"Seq Scan\" (slow) vs \"Index Scan\" (fast)\n\n\
                 QUICK DEBUG CHECKLIST:\n\
                 - Start with simple query, add complexity gradually\n\
                 - Test each JOIN separately\n\
                 - Use COUNT before fetching large result sets\n\
                 - Use EXPLAIN for slow queries\n\
                 - Check schema with db_table_schema tool\n\
                 - Test on small dataset first\n\n\
                 For detailed help on specific topics, use the safety scenario for SQL injection prevention and modification workflows.",
            ),
        },
    ]
}

