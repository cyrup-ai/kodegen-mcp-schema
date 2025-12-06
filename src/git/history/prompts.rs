//! Prompt messages for git_history tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitHistoryPromptArgs;

/// Prompt provider for git_history tool
pub struct HistoryPrompts;

impl PromptProvider for HistoryPrompts {
    type PromptArgs = GitHistoryPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("filtering") => prompt_filtering(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: basic, filtering".to_string()),
                required: Some(false),
            }
        ]
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I view commit history?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "View commit history:\\n\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"max_count\\\": 10}\\n\\\
                 ```\\n\\n\\\
                 This shows the last 10 commits with:\\n\\\
                 - Commit hash\\n\\\
                 - Author and date\\n\\\
                 - Commit message\\n\\n\\\
                 For more history, increase max_count:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"max_count\\\": 50}\\n\\\
                 ```"
            ),
        },
    ]
}

fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I filter commit history?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Filter commit history:\\n\\n\\\
                 By author:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"author\\\": \\\"john@example.com\\\", \\\"max_count\\\": 20}\\n\\\
                 ```\\n\\n\\\
                 By date range:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"since\\\": \\\"2024-01-01\\\", \\\"until\\\": \\\"2024-12-31\\\"}\\n\\\
                 ```\\n\\n\\\
                 Specific file:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"file_path\\\": \\\"src/main.rs\\\", \\\"max_count\\\": 10}\\n\\\
                 ```\\n\\n\\\
                 Specific branch:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"branch\\\": \\\"develop\\\", \\\"max_count\\\": 20}\\n\\\
                 ```"
            ),
        },
    ]
}
