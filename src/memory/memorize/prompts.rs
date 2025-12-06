//! Prompt messages for memory_memorize tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::MemorizePromptArgs;

/// Prompt provider for memory_memorize tool
///
/// This is the ONLY way to provide prompts for memory_memorize - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct MemorizePrompts;

impl PromptProvider for MemorizePrompts {
    type PromptArgs = MemorizePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("organization") => prompt_organization(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, organization)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic memory storage
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I store content in a memory library?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The memory_memorize tool stores content in named libraries with automatic embedding generation for semantic search.\n\n\
                 STORING MEMORIES:\n\n\
                 Store content:\n\
                 memory_memorize({\n\
                     \"library\": \"project-notes\",\n\
                     \"content\": \"The API uses JWT tokens for authentication.\"\n\
                 })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"library\": \"project-notes\",\n\
                   \"stored\": true\n\
                 }\n\n\
                 WHAT HAPPENS:\n\
                 - Content stored in library\n\
                 - Embedding generated automatically\n\
                 - Available for semantic recall\n\
                 - Library created if it doesn't exist\n\n\
                 PARAMETERS:\n\
                 - library (required): Name of memory library\n\
                 - content (required): Text content to store\n\n\
                 USAGE EXAMPLES:\n\n\
                 // Example 1: Store configuration knowledge\n\
                 memory_memorize({\n\
                     \"library\": \"project-notes\",\n\
                     \"content\": \"The API uses JWT tokens with 1-hour expiry for authentication\"\n\
                 })\n\n\
                 // Example 2: Store implementation details\n\
                 memory_memorize({\n\
                     \"library\": \"codebase\",\n\
                     \"content\": \"Config loader in src/config/loader.rs uses TOML format with env var overrides\"\n\
                 })\n\n\
                 LIBRARY NAMING:\n\
                 - Use descriptive names: project-auth, rust-patterns, api-docs\n\
                 - Lowercase with hyphens: my-project-notes\n\n\
                 CONTENT GUIDELINES:\n\
                 - Be specific and descriptive\n\
                 - Include context for better retrieval\n\
                 - Can include code snippets\n\
                 - Aim for 1-3 paragraphs per memory\n\n\
                 LATER RETRIEVAL:\n\
                 Use memory_recall to find related content:\n\
                 memory_recall({\n\
                     \"library\": \"project-notes\",\n\
                     \"context\": \"authentication\",\n\
                     \"limit\": 5\n\
                 })\n\n\
                 BEST PRACTICES:\n\
                 - Store insights as you discover them\n\
                 - Use consistent library naming\n\
                 - Include enough context for future recall",
            ),
        },
    ]
}

/// Library organization strategies
fn prompt_organization() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How should I organize my memory libraries?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Organize memories using different library namespaces. Each library is independent and can be recalled separately.\n\n\
                 LIBRARY ORGANIZATION STRATEGIES:\n\n\
                 Strategy A - By Project:\n\
                 Store project-specific knowledge in dedicated libraries:\n\
                 memory_memorize({\n\
                     \"library\": \"project-auth\",\n\
                     \"content\": \"Auth uses OAuth2 with refresh tokens, configured in auth.toml\"\n\
                 })\n\
                 memory_memorize({\n\
                     \"library\": \"project-api\",\n\
                     \"content\": \"API endpoints defined in src/routes/, use Axum framework\"\n\
                 })\n\
                 Best for: Working on multiple distinct projects\n\n\
                 Strategy B - By Type:\n\
                 Organize by type of information:\n\
                 memory_memorize({\n\
                     \"library\": \"rust-patterns\",\n\
                     \"content\": \"Use Result<T, E> for error handling, avoid unwrap() in production\"\n\
                 })\n\
                 memory_memorize({\n\
                     \"library\": \"decisions\",\n\
                     \"content\": \"ADR-001: Use PostgreSQL for persistence - strong consistency needed\"\n\
                 })\n\
                 Best for: Cross-project knowledge accumulation\n\n\
                 NAMING CONVENTIONS:\n\
                 - Use lowercase with hyphens: project-auth, rust-patterns\n\
                 - Be descriptive: auth-service not auth\n\n\
                 LISTING LIBRARIES:\n\
                 memory_list_libraries({})\n\
                 Returns:\n\
                 {\n\
                   \"libraries\": [\"project-auth\", \"rust-patterns\", \"decisions\"]\n\
                 }\n\n\
                 RECALL FROM SPECIFIC LIBRARY:\n\
                 memory_recall({\n\
                     \"library\": \"rust-patterns\",\n\
                     \"context\": \"error handling\",\n\
                     \"limit\": 5\n\
                 })\n\n\
                 BEST PRACTICES:\n\
                 - Use library names that will make sense in 6 months\n\
                 - Keep related content in the same library\n\
                 - Don't create too many micro-libraries",
            ),
        },
    ]
}