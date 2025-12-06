//! Prompt messages for git_remote_remove tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitRemoteRemovePromptArgs;

/// Prompt provider for git_remote_remove tool
///
/// This is the ONLY way to provide prompts for git_remote_remove - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct RemoteRemovePrompts;

impl PromptProvider for RemoteRemovePrompts {
    type PromptArgs = GitRemoteRemovePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("cleanup") => prompt_cleanup(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show examples for (basic, cleanup)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic remote removal - simple removal of a remote reference
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I remove a remote repository reference?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_remote_remove tool removes a remote repository reference from your local git configuration. Here's how to use it for basic remote removal:\n\n\
                 REMOVING A REMOTE:\n\n\
                 1. Remove remote:\n\
                    git_remote_remove({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"upstream\"\n\
                    })\n\n\
                 2. Verify removed:\n\
                    git_remote_list({\"path\": \"/project\"})\n\
                    // upstream no longer listed\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"name\": \"upstream\",\n\
                   \"success\": true\n\
                 }\n\n\
                 WHAT CHANGES:\n\
                 Removed: Remote reference from .git/config, remote tracking branches, configuration settings, URL and fetch/push specs\n\
                 Preserved: Remote server (unchanged), local branches, commit history, working directory files\n\n\
                 COMMON USE CASES:\n\
                 Remove unused upstream:\n\
                 git_remote_remove({\"path\": \"./repo\", \"name\": \"upstream\"})\n\n\
                 Remove old fork reference:\n\
                 git_remote_remove({\"path\": \"./repo\", \"name\": \"fork\"})\n\n\
                 PARAMETERS:\n\
                 - path (required): Path to the Git repository\n\
                 - name (required): Name of the remote to remove\n\n\
                 SAFETY NOTES:\n\
                 - This operation only affects local configuration\n\
                 - Remote server and its data remain unchanged\n\
                 - Can be undone by re-adding the remote with git_remote_add\n\
                 - No data loss risk - only removes reference\n\
                 - Local commits and branches are preserved\n\n\
                 VERIFICATION:\n\
                 After removing, verify with:\n\
                 git_remote_list({\"path\": \"/project\"})\n\
                 The removed remote should no longer appear in the list.",
            ),
        },
    ]
}

/// Cleanup patterns - removing old, unused, or invalid remotes
fn prompt_cleanup() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "When should I remove remotes and how do I clean them up?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Remote cleanup is essential for maintaining a clean git configuration. Here are common cleanup scenarios and best practices:\n\n\
                 CLEANING UP REMOTES:\n\n\
                 1. Remove old remote:\n\
                    git_remote_list({\"path\": \"/project\"})\n\
                    // Shows: origin, old-remote, upstream\n\
                    git_remote_remove({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"old-remote\"\n\
                    })\n\n\
                 2. Remove after URL change:\n\
                    // Old remote no longer valid\n\
                    git_remote_remove({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"origin\"\n\
                    })\n\
                    git_remote_add({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"origin\",\n\
                        \"url\": \"https://new-url/repo.git\"\n\
                    })\n\n\
                 WHEN TO REMOVE REMOTES:\n\n\
                 Remote URL Changed:\n\
                 - Repository migrated to new hosting provider\n\
                 - Organization renamed on GitHub/GitLab\n\
                 - Personal username changed\n\
                 - Server hostname updated\n\n\
                 Service Migrated:\n\
                 - Moving from GitHub to GitLab\n\
                 - Switching from GitLab to Bitbucket\n\
                 - Migrating to self-hosted solution\n\
                 - Consolidating repositories\n\n\
                 Unused Remotes:\n\
                 - Fork no longer needed\n\
                 - Upstream project archived\n\
                 - Temporary collaboration ended\n\
                 - Experimental remote no longer used\n\n\
                 CLEANUP WORKFLOW:\n\
                 1. List: git_remote_list({\"path\": \"/project\"})\n\
                 2. Remove: git_remote_remove({\"path\": \"/project\", \"name\": \"old-fork\"})\n\
                 3. Verify: git_remote_list({\"path\": \"/project\"})\n\n\
                 COMMON CLEANUP PATTERNS:\n\n\
                 Remove multiple old remotes:\n\
                 git_remote_remove({\"path\": \"./repo\", \"name\": \"old-origin\"})\n\
                 git_remote_remove({\"path\": \"./repo\", \"name\": \"backup\"})\n\
                 git_remote_remove({\"path\": \"./repo\", \"name\": \"test\"})\n\n\
                 Clean up after organization rename:\n\
                 git_remote_remove({\"path\": \"./repo\", \"name\": \"origin\"})\n\
                 git_remote_add({\n\
                     \"path\": \"./repo\",\n\
                     \"name\": \"origin\",\n\
                     \"url\": \"https://github.com/newname/repo.git\"\n\
                 })\n\n\
                 BEST PRACTICES:\n\
                 - Audit remotes regularly and document their purpose\n\
                 - Remove when no longer needed, verify before removal\n\
                 - Use git_remote_list to review before cleanup\n\n\
                 SAFETY CONSIDERATIONS:\n\
                 - Verify you're removing the correct remote\n\
                 - Ensure no active work depends on the remote\n\
                 - Check with team before removing shared remotes\n\
                 - Document removal in team communications\n\
                 - Can always re-add if removed by mistake\n\n\
                 ERROR SCENARIOS:\n\
                 Remote doesn't exist:\n\
                 - Error will indicate remote not found\n\
                 - Use git_remote_list to see available remotes\n\n\
                 Wrong repository path:\n\
                 - Verify path points to valid git repository\n\
                 - Use absolute or correct relative paths",
            ),
        },
    ]
}

