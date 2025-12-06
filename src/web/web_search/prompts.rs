//! Prompt messages for web_search tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::WebSearchPromptArgs;

/// Prompt provider for web_search tool
///
/// This is the ONLY way to provide prompts for web_search - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct WebSearchPrompts;

impl PromptProvider for WebSearchPrompts {
    type PromptArgs = WebSearchPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("queries") => prompt_queries(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, queries)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// Scenario functions for web_search prompts

/// Basic web searching
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I perform basic web searches using the web_search tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "WEB SEARCHING:\n\n\
                 1. Basic search:\n\
                    web_search({\n\
                        \"query\": \"rust async await tutorial\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"results\": [\n\
                     {\n\
                       \"title\": \"Async in Rust - The Book\",\n\
                       \"url\": \"https://rust-lang.github.io/async-book/\",\n\
                       \"snippet\": \"Learn async programming in Rust with async/await syntax...\"\n\
                     },\n\
                     {\n\
                       \"title\": \"Tokio Tutorial\",\n\
                       \"url\": \"https://tokio.rs/tokio/tutorial\",\n\
                       \"snippet\": \"Getting started with async Rust using Tokio runtime...\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 RESULT FIELDS:\n\
                 - title: Page title\n\
                 - url: Full URL\n\
                 - snippet: Text excerpt",
            ),
        },
    ]
}

/// Effective query patterns
fn prompt_queries() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are the most effective query patterns for web searching?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "EFFECTIVE QUERY PATTERNS:\n\n\
                 1. Specific technology:\n\
                    web_search({\n\
                        \"query\": \"actix-web middleware authentication example\"\n\
                    })\n\n\
                 2. Error messages:\n\
                    web_search({\n\
                        \"query\": \"rust borrow checker cannot borrow as mutable\"\n\
                    })\n\n\
                 3. Version-specific:\n\
                    web_search({\n\
                        \"query\": \"tokio 1.0 migration guide\"\n\
                    })\n\n\
                 4. Comparisons:\n\
                    web_search({\n\
                        \"query\": \"rust diesel vs sqlx comparison 2024\"\n\
                    })\n\n\
                 5. Best practices:\n\
                    web_search({\n\
                        \"query\": \"rust error handling best practices anyhow thiserror\"\n\
                    })\n\n\
                 6. Official docs:\n\
                    web_search({\n\
                        \"query\": \"site:docs.rs serde json\"\n\
                    })\n\n\
                 QUERY TIPS:\n\
                 - Be specific with library names\n\
                 - Include version numbers\n\
                 - Add \"example\" or \"tutorial\"\n\
                 - Use \"site:\" for specific domains\n\
                 - Include error text for debugging",
            ),
        },
    ]
}


