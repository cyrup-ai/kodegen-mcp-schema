//! Prompt messages for git_open tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitOpenPromptArgs;

/// Prompt provider for git_open tool
pub struct OpenPrompts;

impl PromptProvider for OpenPrompts {
    type PromptArgs = GitOpenPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I open an existing Git repository?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Open an existing Git repository with git_open:\n\n\
                 ```json\n\
                 {\"path\": \"/path/to/repository\"}\n\
                 ```\n\n\
                 This connects to a repository that already exists on disk (has a .git directory).\n\n\
                 Example:\n\
                 ```json\n\
                 {\"path\": \"/home/user/myproject\"}\n\
                 ```\n\n\
                 WHAT HAPPENS:\n\
                 - Verifies .git directory exists\n\
                 - Loads repository configuration\n\
                 - Returns current branch information\n\
                 - Checks if working directory is clean\n\n\
                 USE git_open WHEN:\n\
                 - You know the exact repository path\n\
                 - Repository was previously cloned or initialized\n\
                 - Need to perform git operations on existing repo"
            ),
        },
    ]
}

