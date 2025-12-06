//! Prompt messages for git_remote_add tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitRemoteAddPromptArgs;

/// Prompt provider for git_remote_add tool
///
/// This is the ONLY way to provide prompts for git_remote_add - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct RemoteAddPrompts;

impl PromptProvider for RemoteAddPrompts {
    type PromptArgs = GitRemoteAddPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// Single scenario: basic git_remote_add usage

/// Basic remote addition
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I add a remote repository using git_remote_add?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_remote_add tool adds a new remote repository reference to your local Git repository. Here's how to use it for basic remote addition:\n\n\
                 ADDING A REMOTE:\n\n\
                 1. Add origin with HTTPS:\n\
                    git_remote_add({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"origin\",\n\
                        \"url\": \"https://github.com/user/repo.git\"\n\
                    })\n\n\
                 2. Add with SSH:\n\
                    git_remote_add({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"origin\",\n\
                        \"url\": \"git@github.com:user/repo.git\"\n\
                    })\n\n\
                 3. Verify remote was added:\n\
                    git_remote_list({\"path\": \"/project\"})\n\
                    // Shows: origin -> https://github.com/user/repo.git\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"name\": \"origin\",\n\
                   \"url\": \"https://github.com/user/repo.git\",\n\
                   \"success\": true\n\
                 }\n\n\
                 PARAMETERS:\n\
                 - path (required): Local repository path\n\
                 - name (required): Remote name (e.g., \"origin\", \"upstream\")\n\
                 - url (required): Remote URL (HTTPS or SSH format)\n\
                 - fetch (optional): Set up fetch refspec automatically\n\n\
                 URL FORMATS:\n\
                 HTTPS: https://github.com/user/repo.git\n\
                 - Prompts for credentials if needed\n\
                 - Works behind firewalls\n\
                 - Easier for read-only access\n\n\
                 SSH: git@github.com:user/repo.git\n\
                 - Requires SSH key setup\n\
                 - No password prompts\n\
                 - Better for frequent pushing\n\n\
                 Git protocol: git://github.com/user/repo.git\n\
                 - Read-only access\n\
                 - Fast but less common\n\n\
                 COMMON REMOTE NAMES:\n\
                 - origin: Primary remote (default convention)\n\
                 - upstream: Original repository (for forks)\n\
                 - production: Deployment server\n\
                 - staging: Staging environment\n\
                 - backup: Backup mirror\n\n\
                 TYPICAL WORKFLOW:\n\
                 1. Initialize repository:\n\
                    git_init({\"path\": \"/project\"})\n\n\
                 2. Add files and commit:\n\
                    git_add({\"path\": \"/project\", \"files\": [\".\"],})\n\
                    git_commit({\"path\": \"/project\", \"message\": \"Initial commit\"})\n\n\
                 3. Add remote:\n\
                    git_remote_add({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"origin\",\n\
                        \"url\": \"https://github.com/user/repo.git\"\n\
                    })\n\n\
                 4. Push to remote:\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"remote\": \"origin\",\n\
                        \"branch\": \"main\"\n\
                    })\n\n\
                 NAMING CONVENTIONS:\n\
                 - origin: The default name for your primary remote\n\
                 - upstream: Conventional name for the original repo when working with forks\n\
                 - Custom names: Use descriptive names like \"personal\", \"work\", \"deploy\"\n\n\
                 ERROR HANDLING:\n\
                 - Remote already exists: Use git_remote_remove first or check with git_remote_list\n\
                 - Invalid URL: Verify URL format and repository access\n\
                 - Path not a repository: Run git_init or git_clone first\n\n\
                 AFTER ADDING REMOTE:\n\
                 Use git_fetch to retrieve branches:\n\
                 git_fetch({\"path\": \"/project\", \"remote\": \"origin\"})\n\n\
                 Use git_push to send changes:\n\
                 git_push({\"path\": \"/project\", \"remote\": \"origin\"})\n\n\
                 Use git_pull to fetch and merge:\n\
                 git_pull({\"path\": \"/project\", \"remote\": \"origin\"})",
            ),
        },
    ]
}
