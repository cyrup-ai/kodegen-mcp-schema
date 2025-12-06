//! Prompt messages for db_table_indexes tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetTableIndexesPromptArgs;

/// Prompt provider for db_table_indexes tool
pub struct TableIndexesPrompts;

impl PromptProvider for TableIndexesPrompts {
    type PromptArgs = GetTableIndexesPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("usage") => prompt_usage_scenarios(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Index inspection scenario: usage (optional) for practical patterns".to_string()),
                required: Some(false),
            }
        ]
    }
}

fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I view indexes on a table?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "```json\n\
                 {\"schema\": \"public\", \"table\": \"users\"}\n\
                 ```\n\n\
                 Returns all indexes on the table:\n\
                 ```json\n\
                 {\n\
                   \"schema\": \"public\",\n\
                   \"table\": \"users\",\n\
                   \"indexes\": [\n\
                     {\n\
                       \"name\": \"users_pkey\",\n\
                       \"columns\": [\"id\"],\n\
                       \"index_type\": \"btree\",\n\
                       \"is_unique\": true,\n\
                       \"is_primary\": true\n\
                     },\n\
                     {\n\
                       \"name\": \"users_email_idx\",\n\
                       \"columns\": [\"email\"],\n\
                       \"index_type\": \"btree\",\n\
                       \"is_unique\": true,\n\
                       \"is_primary\": false\n\
                     },\n\
                     {\n\
                       \"name\": \"users_created_at_idx\",\n\
                       \"columns\": [\"created_at\"],\n\
                       \"index_type\": \"btree\",\n\
                       \"is_unique\": false,\n\
                       \"is_primary\": false\n\
                     }\n\
                   ],\n\
                   \"count\": 3\n\
                 }\n\
                 ```\n\n\
                 **Key index properties:**\n\
                 - **name** - Index identifier\n\
                 - **columns** - Columns covered by index (order matters!)\n\
                 - **index_type** - Storage structure (btree, hash, gin, gist)\n\
                 - **is_unique** - Whether duplicate values are allowed\n\
                 - **is_primary** - Whether this is the primary key index\n\n\
                 **Why indexes matter:**\n\
                 - Speed up queries with WHERE, JOIN, ORDER BY on indexed columns\n\
                 - Enforce uniqueness constraints (UNIQUE indexes)\n\
                 - Slow down writes (INSERT/UPDATE must update indexes)\n\
                 - Consume disk space"
            ),
        },
    ]
}

fn prompt_usage_scenarios() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common scenarios where I should check table indexes?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "**Scenario 1: Query optimization**\n\
                 ```typescript\n\
                 // Step 1: Check existing indexes\n\
                 const indexes = await db_table_indexes({schema: \"public\", table: \"orders\"});\n\
                 console.log(indexes.indexes.map(i => ({name: i.name, columns: i.columns})));\n\
                 // Output: [{name: \"orders_pkey\", columns: [\"id\"]}]\n\
                 \n\
                 // Step 2: Notice slow query\n\
                 const slow = await db_execute_sql({\n\
                   sql: \"SELECT * FROM orders WHERE user_id = 123\"  // Slow!\n\
                 });\n\
                 // Takes 5 seconds (full table scan)\n\
                 \n\
                 // Step 3: Identify missing index\n\
                 // No index on user_id! Need to create one:\n\
                 // CREATE INDEX orders_user_id_idx ON orders(user_id);\n\
                 ```\n\n\
                 **Scenario 2: Foreign key relationships**\n\
                 ```typescript\n\
                 // Step 1: Get table schema to find foreign keys\n\
                 const schema = await db_table_schema({schema: \"public\", table: \"orders\"});\n\
                 const fks = schema.foreign_keys;\n\
                 console.log(fks.map(fk => ({columns: fk.columns, foreign_table: fk.foreign_table})));\n\
                 // Output: [{columns: [\"user_id\"], foreign_table: \"users\"}]\n\
                 \n\
                 // Step 2: Check if foreign key columns are indexed\n\
                 const indexes = await db_table_indexes({schema: \"public\", table: \"orders\"});\n\
                 const hasUserIdIndex = indexes.indexes.some(idx => \n\
                   idx.columns.includes(\"user_id\")\n\
                 );\n\
                 \n\
                 if (!hasUserIdIndex) {\n\
                   console.log(\"WARNING: Foreign key user_id is not indexed!\");\n\
                   console.log(\"JOINs on this column will be slow.\");\n\
                   console.log(\"Recommended: CREATE INDEX orders_user_id_idx ON orders(user_id);\");\n\
                 }\n\
                 ```\n\n\
                 **Scenario 3: Uniqueness constraints**\n\
                 ```typescript\n\
                 // Step 1: Check which columns have unique indexes\n\
                 const indexes = await db_table_indexes({schema: \"public\", table: \"users\"});\n\
                 const uniqueCols = indexes.indexes\n\
                   .filter(idx => idx.is_unique)\n\
                   .map(idx => idx.columns);\n\
                 console.log(\"Unique columns:\", uniqueCols);\n\
                 // Output: [[\"id\"], [\"email\"], [\"username\"]]\n\
                 \n\
                 // Step 2: Understand insertion constraints\n\
                 // Now I know that email and username must be unique\n\
                 // This INSERT will fail if email already exists:\n\
                 try {\n\
                   await db_execute_sql({\n\
                     sql: \"INSERT INTO users (email, username) VALUES ($1, $2)\",\n\
                     params: [\"existing@example.com\", \"newuser\"]\n\
                   });\n\
                 } catch (error) {\n\
                   // ERROR: duplicate key value violates unique constraint \"users_email_idx\"\n\
                 }\n\
                 ```\n\n\
                 **Scenario 4: Performance analysis**\n\
                 ```typescript\n\
                 // Step 1: List all indexes on a table\n\
                 const indexes = await db_table_indexes({schema: \"public\", table: \"events\"});\n\
                 console.log(`Table has ${indexes.count} indexes`);\n\
                 \n\
                 // Step 2: Identify potential issues\n\
                 const issues = [];\n\
                 \n\
                 // Too many indexes?\n\
                 if (indexes.count > 10) {\n\
                   issues.push(`Too many indexes (${indexes.count}). High write overhead.`);\n\
                 }\n\
                 \n\
                 // Redundant indexes?\n\
                 const singleCols = indexes.indexes.filter(i => i.columns.length === 1);\n\
                 const compositeCols = indexes.indexes.filter(i => i.columns.length > 1);\n\
                 for (const single of singleCols) {\n\
                   for (const composite of compositeCols) {\n\
                     if (composite.columns[0] === single.columns[0]) {\n\
                       issues.push(`Redundant: ${single.name} covered by ${composite.name}`);\n\
                     }\n\
                   }\n\
                 }\n\
                 \n\
                 if (issues.length > 0) {\n\
                   console.log(\"Index issues found:\");\n\
                   issues.forEach(issue => console.log(`- ${issue}`));\n\
                 }\n\
                 ```\n\n\
                 **Scenario 5: Query planning**\n\
                 ```typescript\n\
                 // Before writing a complex query, check available indexes\n\
                 \n\
                 // Step 1: Check indexes on all tables I'll JOIN\n\
                 const ordersIndexes = await db_table_indexes({schema: \"public\", table: \"orders\"});\n\
                 const usersIndexes = await db_table_indexes({schema: \"public\", table: \"users\"});\n\
                 const productsIndexes = await db_table_indexes({schema: \"public\", table: \"products\"});\n\
                 \n\
                 // Step 2: Verify foreign key columns are indexed\n\
                 const hasOrdersUserIdIdx = ordersIndexes.indexes.some(i => i.columns.includes(\"user_id\"));\n\
                 const hasOrdersProductIdIdx = ordersIndexes.indexes.some(i => i.columns.includes(\"product_id\"));\n\
                 \n\
                 // Step 3: Write optimized query knowing indexes exist\n\
                 if (hasOrdersUserIdIdx && hasOrdersProductIdIdx) {\n\
                   const result = await db_execute_sql({\n\
                     sql: `\n\
                       SELECT u.email, p.name, o.quantity\n\
                       FROM orders o\n\
                       INNER JOIN users u ON o.user_id = u.id       -- Uses orders_user_id_idx\n\
                       INNER JOIN products p ON o.product_id = p.id -- Uses orders_product_id_idx\n\
                       WHERE o.created_at > $1\n\
                     `,\n\
                     params: ['2024-01-01']\n\
                   });\n\
                   // Fast query! All joins use indexes.\n\
                 } else {\n\
                   console.log(\"WARNING: Missing indexes on foreign keys. Query will be slow.\");\n\
                 }\n\
                 ```\n\n\
                 **Scenario 6: Understanding primary keys**\n\
                 ```typescript\n\
                 // Primary key is always indexed\n\
                 const indexes = await db_table_indexes({schema: \"public\", table: \"users\"});\n\
                 const primaryKey = indexes.indexes.find(idx => idx.is_primary);\n\
                 \n\
                 if (primaryKey) {\n\
                   console.log(\"Primary key columns:\", primaryKey.columns);\n\
                   console.log(\"Index type:\", primaryKey.index_type);\n\
                   console.log(\"Index name:\", primaryKey.name);\n\
                 }\n\
                 \n\
                 // Common output:\n\
                 // Primary key columns: [\"id\"]\n\
                 // Index type: btree\n\
                 // Index name: users_pkey\n\
                 \n\
                 // Queries on primary key are always fast:\n\
                 SELECT * FROM users WHERE id = 123;  // ✅ Uses primary key index\n\
                 ```"
            ),
        },
    ]
}
