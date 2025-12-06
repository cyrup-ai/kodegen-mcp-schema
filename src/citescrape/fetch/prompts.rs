//! Prompt messages for fetch tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::FetchPromptArgs;

/// Prompt provider for fetch tool
pub struct FetchPrompts;

impl PromptProvider for FetchPrompts {
    type PromptArgs = FetchPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        vec![
            PromptMessage {
                role: PromptMessageRole::User,
                content: PromptMessageContent::text(
                    "How do I fetch a single web page?",
                ),
            },
            PromptMessage {
                role: PromptMessageRole::Assistant,
                content: PromptMessageContent::text(
                    "FETCH - Single Page Retrieval:\n\n\
                     fetch({ \"url\": \"https://docs.rs/tokio/latest/tokio/\" })\n\n\
                     RESPONSE:\n\
                     {\n\
                       \"display\": \"<ANSI-highlighted markdown>\",\n\
                       \"path\": \"/path/to/saved/file.md\",\n\
                       \"search_helper\": \"scrape_url({ action: 'SEARCH', crawl_id: 0, query: 'your query' })\",\n\
                       \"url\": \"https://docs.rs/tokio/latest/tokio/\",\n\
                       \"title\": \"Tokio - Docs.rs\",\n\
                       \"content_length\": 12345\n\
                     }\n\n\
                     USE CASES:\n\
                     - Quick page preview with syntax highlighting\n\
                     - Retrieve documentation for reading\n\
                     - Fetch API reference pages\n\
                     - Get content path for follow-up search",
                ),
            },
        ]
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Example scenario".to_string()),
                required: Some(false),
            }
        ]
    }
}
