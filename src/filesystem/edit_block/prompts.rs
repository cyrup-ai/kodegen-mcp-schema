//! Prompt messages for fs_edit_block tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::FsEditBlockPromptArgs;

/// Prompt provider for fs_edit_block tool
///
/// This is the ONLY way to provide prompts for fs_edit_block - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct EditBlockPrompts;

impl PromptProvider for EditBlockPrompts {
    type PromptArgs = FsEditBlockPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("precision") => prompt_precision(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, precision, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE FS_EDIT_BLOCK
// ============================================================================

/// Basic string replacement examples
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use fs_edit_block for basic file editing?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "EDITING FILES:\n\n\
                 1. Simple replacement:\n\
                    fs_edit_block({\n\
                        \"path\": \"src/main.rs\",\n\
                        \"old_string\": \"println!(\\\"Hello\\\")\",\n\
                        \"new_string\": \"println!(\\\"Hello, World!\\\")\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"path\": \"src/main.rs\",\n\
                   \"replacements\": 1,\n\
                   \"success\": true\n\
                 }\n\n\
                 2. Replace variable name:\n\
                    fs_edit_block({\n\
                        \"path\": \"src/lib.rs\",\n\
                        \"old_string\": \"let x = 5\",\n\
                        \"new_string\": \"let count = 5\"\n\
                    })\n\n\
                 3. Replace multiple occurrences:\n\
                    fs_edit_block({\n\
                        \"path\": \"src/config.rs\",\n\
                        \"old_string\": \"localhost\",\n\
                        \"new_string\": \"127.0.0.1\",\n\
                        \"expected_replacements\": 3\n\
                    })\n\n\
                 KEY PARAMETERS:\n\
                 - path: File to edit\n\
                 - old_string: Exact text to find\n\
                 - new_string: Replacement text\n\
                 - expected_replacements: Validate count (default: 1)",
            ),
        },
    ]
}

/// Precise editing patterns with context and validation
fn prompt_precision() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I make precise edits and validate replacement counts?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "PRECISE EDITING & VALIDATION:\n\n\
                 1. Include context for uniqueness:\n\
                    // If \"x = 5\" appears multiple times, include more context\n\
                    fs_edit_block({\n\
                        \"path\": \"src/main.rs\",\n\
                        \"old_string\": \"fn calculate() {\\n    let x = 5\",\n\
                        \"new_string\": \"fn calculate() {\\n    let x = 10\"\n\
                    })\n\n\
                 2. Edit specific function:\n\
                    fs_edit_block({\n\
                        \"path\": \"src/lib.rs\",\n\
                        \"old_string\": \"pub fn process(input: &str) {\\n    // old implementation\\n}\",\n\
                        \"new_string\": \"pub fn process(input: &str) {\\n    // new implementation\\n}\"\n\
                    })\n\n\
                 3. Unique markers:\n\
                    // Use surrounding code as context\n\
                    fs_edit_block({\n\
                        \"path\": \"src/config.rs\",\n\
                        \"old_string\": \"// DATABASE CONFIG\\nhost: \\\"localhost\\\"\",\n\
                        \"new_string\": \"// DATABASE CONFIG\\nhost: \\\"production.db.com\\\"\"\n\
                    })\n\n\
                 4. Replace entire function:\n\
                    fs_edit_block({\n\
                        \"path\": \"src/handlers.rs\",\n\
                        \"old_string\": \"fn handle_error(e: Error) {\\n    eprintln!(\\\"Error: {}\\\", e);\\n}\",\n\
                        \"new_string\": \"fn handle_error(e: Error) {\\n    log::error!(\\\"Error occurred: {:?}\\\", e);\\n    metrics::increment(\\\"errors\\\");\\n}\"\n\
                    })\n\n\
                 5. Replace struct definition:\n\
                    fs_edit_block({\n\
                        \"path\": \"src/types.rs\",\n\
                        \"old_string\": \"struct Config {\\n    host: String,\\n}\",\n\
                        \"new_string\": \"struct Config {\\n    host: String,\\n    port: u16,\\n    timeout: Duration,\\n}\"\n\
                    })\n\n\
                 6. Replace import block:\n\
                    fs_edit_block({\n\
                        \"path\": \"src/main.rs\",\n\
                        \"old_string\": \"use std::io;\",\n\
                        \"new_string\": \"use std::io;\\nuse std::fs;\\nuse std::path::Path;\"\n\
                    })\n\n\
                 7. Validate replacement count:\n\
                    fs_edit_block({\n\
                        \"path\": \"src/lib.rs\",\n\
                        \"old_string\": \"old_value\",\n\
                        \"new_string\": \"new_value\",\n\
                        \"expected_replacements\": 1\n\
                    })\n\
                    // Fails if not exactly 1 match\n\n\
                 8. Search before edit:\n\
                    // Find occurrences first\n\
                    fs_search({\n\
                        \"pattern\": \"deprecated_function\",\n\
                        \"path\": \"./src\",\n\
                        \"return_only\": \"counts\"\n\
                    })\n\n\
                    // Edit with known count\n\
                    fs_edit_block({\n\
                        \"path\": \"src/main.rs\",\n\
                        \"old_string\": \"deprecated_function\",\n\
                        \"new_string\": \"new_function\",\n\
                        \"expected_replacements\": 3\n\
                    })\n\n\
                 PRECISION & SAFETY:\n\
                 - Include enough context for uniqueness\n\
                 - Use expected_replacements to validate count\n\
                 - Match exact whitespace and indentation\n\
                 - Use function/block boundaries\n\
                 - Include comments or markers\n\
                 - Combine with fs_search for confidence\n\
                 - Always read file first to understand state",
            ),
        },
    ]
}

/// Common editing workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows when using fs_edit_block?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "EDIT WORKFLOWS:\n\n\
                 1. Bug fix workflow:\n\
                    // Read to understand\n\
                    fs_read_file({ \"path\": \"src/bug.rs\" })\n\n\
                    // Make targeted fix\n\
                    fs_edit_block({\n\
                        \"path\": \"src/bug.rs\",\n\
                        \"old_string\": \"if x > 0\",\n\
                        \"new_string\": \"if x >= 0\"\n\
                    })\n\n\
                    // Verify change\n\
                    fs_read_file({ \"path\": \"src/bug.rs\" })\n\n\
                 2. Refactoring workflow:\n\
                    // Search for usages\n\
                    fs_search({\n\
                        \"pattern\": \"old_function_name\",\n\
                        \"path\": \"./src\"\n\
                    })\n\n\
                    // Edit each file\n\
                    fs_edit_block({\n\
                        \"path\": \"src/a.rs\",\n\
                        \"old_string\": \"old_function_name\",\n\
                        \"new_string\": \"new_function_name\",\n\
                        \"expected_replacements\": 2\n\
                    })\n\n\
                 3. Configuration change:\n\
                    fs_edit_block({\n\
                        \"path\": \"config.toml\",\n\
                        \"old_string\": \"port = 8080\",\n\
                        \"new_string\": \"port = 9090\"\n\
                    })\n\n\
                 4. Add import:\n\
                    fs_edit_block({\n\
                        \"path\": \"src/main.rs\",\n\
                        \"old_string\": \"use std::io;\",\n\
                        \"new_string\": \"use std::io;\\nuse std::collections::HashMap;\"\n\
                    })\n\n\
                 5. Batch rename:\n\
                    // Multiple files, same change\n\
                    fs_edit_block({ \"path\": \"src/a.rs\", \"old_string\": \"OldName\", \"new_string\": \"NewName\" })\n\
                    fs_edit_block({ \"path\": \"src/b.rs\", \"old_string\": \"OldName\", \"new_string\": \"NewName\" })\n\
                    fs_edit_block({ \"path\": \"src/c.rs\", \"old_string\": \"OldName\", \"new_string\": \"NewName\" })",
            ),
        },
    ]
}

