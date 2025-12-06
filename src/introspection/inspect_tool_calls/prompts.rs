//! Prompt messages for inspect_tool_calls tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::InspectToolCallsPromptArgs;

/// Prompt provider for inspect_tool_calls tool
pub struct InspectToolCallsPrompts;

impl PromptProvider for InspectToolCallsPrompts {
    type PromptArgs = InspectToolCallsPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario_type.as_deref() {
            Some("filtering") => prompt_filter_by_tool(),
            _ => prompt_context_recovery(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario_type".to_string(),
                title: None,
                description: Some("Inspection scenario: filtering or onboarding (default)".to_string()),
                required: Some(false),
            },
            PromptArgument {
                name: "show_examples".to_string(),
                title: None,
                description: Some("Show detailed usage examples with TypeScript".to_string()),
                required: Some(false),
            },
        ]
    }
}


fn prompt_context_recovery() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "I just joined this chat. What work has already been done? Show me the recent tool history so I can understand the context."
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "```typescript\n\
                 // Get first 50 tool calls (default behavior)\n\
                 {}\n\n\
                 // Or get last 30 for most recent work\n\
                 {\"offset\": -30}\n\n\
                 // Returns: InspectToolCallsOutput\n\
                 // - success: true\n\
                 // - count: Number of calls returned\n\
                 // - total_entries_in_memory: Total history size\n\
                 // - calls: Array with tool_name, timestamp, args_json, output_json for each call\n\
                 // - Perfect for understanding what files were read, what searches were run, etc.\n\
                 ```"
            ),
        },
    ]
}


fn prompt_filter_by_tool() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "I want to see only the fs_read_file calls to understand what files were accessed in this session."
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "```typescript\n\
                 // Filter by specific tool name\n\
                 {\"tool_name\": \"fs_read_file\"}\n\n\
                 // Combine with pagination\n\
                 {\"tool_name\": \"fs_read_file\", \"max_results\": 100}\n\n\
                 // Get most recent fs_read_file calls\n\
                 {\"tool_name\": \"fs_read_file\", \"offset\": -20}\n\n\
                 // Returns: InspectToolCallsOutput\n\
                 // - filter_tool_name: \"fs_read_file\"\n\
                 // - calls: Only fs_read_file records\n\
                 // - args_json contains file paths that were read\n\
                 // - output_json contains file contents or errors\n\
                 ```"
            ),
        },
    ]
}


