//! Prompt messages for git_tag tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitTagPromptArgs;

/// Prompt provider for git_tag tool
pub struct TagPrompts;

impl PromptProvider for TagPrompts {
    type PromptArgs = GitTagPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("annotated") => prompt_annotated(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: annotated (optional)".to_string()),
                required: Some(false),
            }
        ]
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I create and manage tags?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Create and manage tags:\\n\\n\\\
                 Create lightweight tag:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"name\\\": \\\"v1.0.0\\\", \\\"action\\\": \\\"create\\\"}\\n\\\
                 ```\\n\\n\\\
                 Create annotated tag with message:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"name\\\": \\\"v1.0.0\\\", \\\"message\\\": \\\"Release 1.0\\\", \\\"action\\\": \\\"create\\\"}\\n\\\
                 ```\\n\\n\\\
                 Tag specific commit:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"name\\\": \\\"v0.9.0\\\", \\\"ref\\\": \\\"abc123\\\", \\\"action\\\": \\\"create\\\"}\\n\\\
                 ```\\n\\n\\\
                 List all tags:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"action\\\": \\\"list\\\"}\\n\\\
                 ```\\n\\n\\\
                 List tags matching pattern:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"pattern\\\": \\\"v1.*\\\", \\\"action\\\": \\\"list\\\"}\\n\\\
                 ```\\n\\n\\\
                 Delete tag:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"name\\\": \\\"v1.0.0-beta\\\", \\\"action\\\": \\\"delete\\\"}\\n\\\
                 ```\\n\\n\\\
                 Push tags to remote:\\n\\\
                 ```json\\n\\\
                 git_push({\\\"path\\\": \\\"./repo\\\", \\\"tags\\\": true})\\n\\\
                 ```"
            ),
        },
    ]
}

fn prompt_annotated() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("What's the difference between lightweight and annotated tags?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Lightweight vs Annotated tags:\\n\\n\\\
                 LIGHTWEIGHT (just a pointer):\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"name\\\": \\\"v1.0.0\\\", \\\"action\\\": \\\"create\\\"}\\n\\\
                 ```\\n\\\
                 - Simple pointer to commit\\n\\\
                 - No metadata stored\\n\\\
                 - Good for temporary markers\\n\\n\\\
                 ANNOTATED (full object):\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"name\\\": \\\"v1.0.0\\\", \\\"message\\\": \\\"Release 1.0.0\\\", \\\"action\\\": \\\"create\\\"}\\n\\\
                 ```\\n\\\
                 - Stores tagger name, email, date\\n\\\
                 - Includes tag message\\n\\\
                 - Can be GPG signed\\n\\\
                 - Recommended for releases\\n\\n\\\
                 BEST PRACTICE:\\n\\\
                 Use annotated tags for all releases and important milestones."
            ),
        },
    ]
}
