//! Prompt messages for git_remote_list tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitRemoteListPromptArgs;

/// Prompt provider for git_remote_list tool
///
/// This is the ONLY way to provide prompts for git_remote_list - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct RemoteListPrompts;

impl PromptProvider for RemoteListPrompts {
    type PromptArgs = GitRemoteListPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario parameter (basic only, parameter is ignored)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic remote listing
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list all configured remotes in a Git repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_remote_list tool shows all configured remote repositories. Here's how to use it for basic remote listing:\n\n\
                 LISTING REMOTES:\n\n\
                 1. List all remotes:\n\
                    git_remote_list({\"path\": \"/project\"})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"remotes\": [\n\
                     {\n\
                       \"name\": \"origin\",\n\
                       \"url\": \"https://github.com/user/repo.git\"\n\
                     },\n\
                     {\n\
                       \"name\": \"upstream\",\n\
                       \"url\": \"https://github.com/original/repo.git\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 2. After clone:\n\
                    git_clone({\"url\": \"https://github.com/user/repo.git\", \"path\": \"/project\"})\n\
                    git_remote_list({\"path\": \"/project\"})\n\
                    // Shows origin pointing to cloned URL\n\n\
                 COMMON REMOTES:\n\
                 - origin: Main remote (typically from git clone)\n\
                 - upstream: Original repository (common in fork workflows)\n\
                 - personal: Personal fork or backup remote\n\
                 - production: Production deployment remote\n\n\
                 PARAMETERS:\n\
                 - path (required): Path to Git repository\n\
                 - verbose (optional): Show detailed URL information (default: false)\n\n\
                 WHEN TO USE:\n\
                 - Verify remote configuration after cloning\n\
                 - Check which remotes are available before push/pull\n\
                 - Confirm remote setup in fork workflows\n\
                 - Troubleshoot remote-related issues\n\n\
                 TYPICAL WORKFLOW:\n\
                 1. Clone a repository\n\
                 2. List remotes to verify origin is correct\n\
                 3. Add additional remotes if needed (upstream, etc.)\n\
                 4. List again to confirm all remotes are configured\n\n\
                 RESPONSE STRUCTURE:\n\
                 - remotes: Array of remote objects\n\
                 - name: Remote name (e.g., \"origin\", \"upstream\")\n\
                 - url: Remote URL (fetch URL by default)\n\n\
                 NO REMOTES:\n\
                 If repository has no remotes configured:\n\
                 {\"remotes\": []}\n\
                 This is normal for:\n\
                 - Newly initialized repositories (git init)\n\
                 - Local-only repositories\n\
                 - Repositories where remotes were removed",
            ),
        },
    ]
}
