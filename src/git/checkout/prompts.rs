//! Prompt messages for git_checkout tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitCheckoutPromptArgs;

/// Prompt provider for git_checkout tool
///
/// This is the ONLY way to provide prompts for git_checkout - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GitCheckoutPrompts;

impl PromptProvider for GitCheckoutPrompts {
    type PromptArgs = GitCheckoutPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("switch_branch") => prompt_switch_branch(),
            Some("detached") => prompt_detached(),
            _ => prompt_switch_branch(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (switch_branch, detached)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT CHECKOUT
// ============================================================================

/// Switching between existing branches
fn prompt_switch_branch() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I switch between branches using git_checkout?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_checkout tool switches branches or restores files. Here's how to switch between branches:\n\n\
                 SWITCHING BRANCHES:\n\n\
                 1. Switch to existing branch:\n\
                    git_checkout({\n\
                        \"path\": \"/project\",\n\
                        \"branch\": \"develop\"\n\
                    })\n\n\
                 2. Switch to main:\n\
                    git_checkout({\n\
                        \"path\": \"/project\",\n\
                        \"branch\": \"main\"\n\
                    })\n\n\
                 3. Switch to feature branch:\n\
                    git_checkout({\n\
                        \"path\": \"/project\",\n\
                        \"branch\": \"feature/user-auth\"\n\
                    })\n\n\
                 4. Switch to remote branch:\n\
                    git_fetch({\"path\": \"/project\"})\n\
                    git_checkout({\n\
                        \"path\": \"/project\",\n\
                        \"branch\": \"feature/remote-work\"\n\
                    })\n\
                    // Creates local branch from origin/feature/remote-work\n\n\
                 BEFORE SWITCHING:\n\
                 - Commit or stash uncommitted changes\n\
                 - Check git_status for clean working tree\n\
                 - Ensure branch exists (use git_branch_list)\n\n\
                 WHAT HAPPENS:\n\
                 - Working directory files updated to match target branch\n\
                 - HEAD pointer moved to target branch\n\
                 - Current branch changes\n\
                 - Uncommitted changes may cause conflicts\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"previous_branch\": \"main\",\n\
                   \"current_branch\": \"develop\",\n\
                   \"success\": true\n\
                 }\n\n\
                 CHECKING AVAILABLE BRANCHES:\n\
                 git_branch_list({\"path\": \"/project\"})\n\
                 // Shows all local branches\n\
                 git_branch_list({\"path\": \"/project\", \"remote\": true})\n\
                 // Shows all remote branches\n\n\
                 COMMON ERRORS:\n\
                 1. Uncommitted changes conflict:\n\
                    ERROR: \"Your local changes would be overwritten\"\n\
                    FIX: Commit or stash changes first\n\n\
                 2. Branch doesn't exist:\n\
                    ERROR: \"Branch 'xyz' not found\"\n\
                    FIX: Create branch first with create: true\n\n\
                 3. Already on branch:\n\
                    ERROR: \"Already on branch 'main'\"\n\
                    FIX: Normal, no action needed\n\n\
                 SAFE WORKFLOW:\n\
                 1. Check status:\n\
                    git_status({\"path\": \"/project\"})\n\
                 2. Commit or stash if needed:\n\
                    git_stash({\"path\": \"/project\"})\n\
                 3. Switch branch:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"develop\"})\n\
                 4. Verify switch:\n\
                    git_status({\"path\": \"/project\"})\n\n\
                 WHEN TO USE:\n\
                 - Starting work on different feature\n\
                 - Reviewing code in another branch\n\
                 - Preparing for merge or rebase\n\
                 - Switching between parallel work streams\n\
                 - Testing different implementations",
            ),
        },
    ]
}

/// Understanding detached HEAD state
fn prompt_detached() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What is detached HEAD and how do I work with it?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Detached HEAD occurs when you checkout a specific commit instead of a branch:\n\n\
                 WHAT IS DETACHED HEAD?\n\
                 - HEAD points directly to commit, not to a branch\n\
                 - You are not \"on\" any branch\n\
                 - Commits made here won't belong to any branch\n\
                 - Easy to lose work if you switch away\n\n\
                 VISUAL REPRESENTATION:\n\
                 Normal state:\n\
                   main → commit3 → commit2 → commit1\n\
                   HEAD → main\n\n\
                 Detached HEAD:\n\
                   main → commit3 → commit2 → commit1\n\
                   HEAD → commit2 (directly)\n\n\
                 WHEN DETACHED HEAD HAPPENS:\n\
                 - Checkout specific commit SHA:\n\
                   git_checkout({\"path\": \"/project\", \"commit\": \"abc1234\"})\n\
                 - Checkout tag:\n\
                   git_checkout({\"path\": \"/project\", \"commit\": \"v1.0.0\"})\n\
                 - During interactive rebase or bisect operations\n\n\
                 SAFE USES (Read-Only):\n\
                 - Inspecting old code versions\n\
                 - Testing at specific historical point\n\
                 - Building old releases\n\
                 - No commits planned\n\n\
                 DANGEROUS USES (Making Changes):\n\
                 - Making commits in detached state\n\
                 - Commits become orphaned and may be garbage collected\n\
                 - Easy to lose work permanently\n\n\
                 SAVING WORK FROM DETACHED HEAD:\n\
                 If you made commits while detached:\n\
                 git_checkout({\n\
                     \"path\": \"/project\",\n\
                     \"branch\": \"recovered-work\",\n\
                     \"create\": true\n\
                 })\n\
                 // Creates branch at current position, saves all commits\n\n\
                 EXITING DETACHED HEAD:\n\
                 Simple exit (discards commits):\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\n\
                 Safe exit (saves work):\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"temp-work\", \"create\": true})\n\n\
                 CHECKING IF DETACHED:\n\
                 git_status({\"path\": \"/project\"})\n\
                 // Returns: {\"is_detached\": true, \"head_commit\": \"abc1234\", \"branch\": null}\n\n\
                 COMMON SCENARIO - Testing Historical Code:\n\
                 git_checkout({\"path\": \"/project\", \"commit\": \"abc1234\"})\n\
                 // Test if bug exists at this point\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\
                 // Return to branch when done\n\n\
                 BEST PRACTICES:\n\
                 - Use detached HEAD only for inspection\n\
                 - Create branch before making commits\n\
                 - Check git_status to verify detached state\n\
                 - Exit detached HEAD promptly\n\n\
                 WARNING:\n\
                 Git warns: \"You are in 'detached HEAD' state. Commits made without \n\
                 creating a branch will be lost when you checkout another branch.\"\n\
                 This is accurate - heed the warning!",
            ),
        },
    ]
}
