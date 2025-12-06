//! Prompt messages for git_pull tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitPullPromptArgs;

/// Prompt provider for git_pull tool
///
/// This is the ONLY way to provide prompts for git_pull - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct PullPrompts;

impl PromptProvider for PullPrompts {
    type PromptArgs = GitPullPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("rebase") => prompt_rebase(),
            Some("conflicts") => prompt_conflicts(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, rebase, conflicts)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT PULL
// ============================================================================

/// Basic pulling from remote repositories
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I pull changes from a remote repository using git_pull?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_pull tool fetches and integrates remote changes in one operation. Here's how to use it effectively:\n\n\
                 PULLING CHANGES:\n\n\
                 1. Pull from tracking branch:\n\
                    git_pull({\n\
                        \"path\": \"/project\"\n\
                    })\n\
                    // Pulls from upstream of current branch\n\n\
                 2. Pull specific remote/branch:\n\
                    git_pull({\n\
                        \"path\": \"/project\",\n\
                        \"remote\": \"origin\",\n\
                        \"branch\": \"main\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"commits_fetched\": 5,\n\
                   \"merge_type\": \"fast-forward\",\n\
                   \"updated_files\": [\"src/main.rs\", \"Cargo.toml\"],\n\
                   \"success\": true\n\
                 }\n\n\
                 PULL = FETCH + MERGE:\n\
                 - Downloads new commits from remote\n\
                 - Merges them into current branch\n\
                 - Updates working directory automatically\n\n\
                 BEFORE PULLING:\n\
                 - Commit or stash local changes\n\
                 - Check git_status for clean state\n\
                 - Uncommitted changes may cause conflicts\n\n\
                 COMMON PATTERNS:\n\n\
                 Pull default branch:\n\
                 git_pull({\"path\": \"/project\"})\n\n\
                 Pull main from origin:\n\
                 git_pull({\n\
                     \"path\": \"/project\",\n\
                     \"remote\": \"origin\",\n\
                     \"branch\": \"main\"\n\
                 })\n\n\
                 Pull develop branch:\n\
                 git_pull({\n\
                     \"path\": \"/project\",\n\
                     \"remote\": \"origin\",\n\
                     \"branch\": \"develop\"\n\
                 })\n\n\
                 PULLING FROM DIFFERENT REMOTES:\n\n\
                 From origin:\n\
                 git_pull({\"path\": \"/project\", \"remote\": \"origin\"})\n\n\
                 From upstream (common in forks):\n\
                 git_pull({\n\
                     \"path\": \"/project\",\n\
                     \"remote\": \"upstream\",\n\
                     \"branch\": \"main\"\n\
                 })\n\n\
                 Colleague's feature branch:\n\
                 git_pull({\n\
                     \"path\": \"/project\",\n\
                     \"remote\": \"origin\",\n\
                     \"branch\": \"colleague-feature\"\n\
                 })\n\n\
                 FORK SYNC WORKFLOW:\n\
                 Keeping your fork up-to-date:\n\n\
                 Step 1: Checkout main branch\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\n\
                 Step 2: Pull upstream changes\n\
                 git_pull({\n\
                     \"path\": \"/project\",\n\
                     \"remote\": \"upstream\",\n\
                     \"branch\": \"main\"\n\
                 })\n\n\
                 Step 3: Push to your fork\n\
                 git_push({\"path\": \"/project\"})\n\n\
                 AFTER PULLING:\n\
                 - Local branch includes remote changes\n\
                 - Working directory is updated\n\
                 - May need to resolve conflicts (see conflicts scenario)\n\
                 - Build/test to ensure everything works\n\n\
                 MERGE TYPES:\n\
                 - fast-forward: Your branch was behind, simply moved forward\n\
                 - merge: Created merge commit to combine changes\n\
                 - conflict: Requires manual resolution\n\n\
                 PARAMETERS:\n\
                 - path (required): Repository path\n\
                 - remote (optional): Remote name (default: origin)\n\
                 - branch (optional): Branch to pull (default: current upstream)\n\
                 - rebase (optional): Use rebase instead of merge (default: false)\n\
                 - ff_only (optional): Only allow fast-forward (default: false)",
            ),
        },
    ]
}

/// Pull with rebase for cleaner history
fn prompt_rebase() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "When should I use pull with rebase instead of merge?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Pull with rebase creates cleaner, linear history by replaying your commits on top of remote changes:\n\n\
                 PULL WITH REBASE:\n\n\
                 1. Rebase instead of merge:\n\
                    git_pull({\n\
                        \"path\": \"/project\",\n\
                        \"rebase\": true\n\
                    })\n\n\
                 2. Always rebase (preferred for feature branches):\n\
                    git_pull({\n\
                        \"path\": \"/project\",\n\
                        \"remote\": \"origin\",\n\
                        \"branch\": \"main\",\n\
                        \"rebase\": true\n\
                    })\n\
                    // Your commits replayed on top of remote\n\n\
                 MERGE vs REBASE:\n\n\
                 MERGE (default):\n\
                 - Creates merge commit\n\
                 - Preserves complete history\n\
                 - Safe for shared branches\n\
                 - Shows when branches diverged\n\
                 git_pull({\"path\": \"/project\", \"rebase\": false})\n\n\
                 REBASE:\n\
                 - Linear history, no merge commits\n\
                 - Cleaner git log\n\
                 - Easier to review\n\
                 - Rewrites commit history\n\
                 git_pull({\"path\": \"/project\", \"rebase\": true})\n\n\
                 WHEN TO USE REBASE:\n\
                 ✓ Personal feature branches\n\
                 ✓ Before creating pull request\n\
                 ✓ Syncing with upstream main\n\
                 ✓ Want clean linear history\n\
                 ✓ Commits not yet pushed\n\n\
                 WHEN TO USE MERGE:\n\
                 ✓ Main/master branches\n\
                 ✓ Shared public branches\n\
                 ✓ Want to preserve history\n\
                 ✓ Multiple collaborators on branch\n\
                 ✓ Default safe option\n\n\
                 REBASE WARNING:\n\
                 - Rewrites commit history\n\
                 - Don't rebase commits already pushed to shared branches\n\
                 - Never rebase main/master branch\n\
                 - Can cause issues for collaborators if misused\n\n\
                 FEATURE BRANCH WORKFLOW:\n\
                 1. Working on feature-x branch\n\
                 2. Main has new commits\n\
                 3. Rebase to stay current:\n\
                    git_pull({\"path\": \"/project\", \"remote\": \"origin\", \"branch\": \"main\", \"rebase\": true})\n\
                 4. Your feature-x commits now on top of latest main\n\
                 5. Clean history when you create PR",
            ),
        },
    ]
}

/// Handling pull conflicts
fn prompt_conflicts() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I handle merge conflicts that occur when pulling?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Merge conflicts occur when the same code was changed in both local and remote branches. Here's how to resolve them:\n\n\
                 HANDLING PULL CONFLICTS:\n\n\
                 1. Conflict occurs:\n\
                    git_pull({\"path\": \"/project\"})\n\
                    // Response indicates conflict in src/config.rs\n\n\
                 2. View status:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Shows conflicted files in red\n\n\
                 3. Resolve conflicts:\n\
                    // Read the conflicted file\n\
                    fs_read_file({\"path\": \"/project/src/config.rs\"})\n\
                    // Look for <<<<<<< markers\n\n\
                 4. Edit to fix conflicts:\n\
                    // Remove conflict markers\n\
                    // Choose which changes to keep\n\
                    fs_edit_block({\n\
                        \"path\": \"/project/src/config.rs\",\n\
                        \"old_string\": \"<<<<<<< HEAD\\nlocal code\\n=======\\nremote code\\n>>>>>>>\",\n\
                        \"new_string\": \"resolved code\"\n\
                    })\n\n\
                 5. Mark as resolved:\n\
                    git_add({\"path\": \"/project\", \"files\": [\"src/config.rs\"]})\n\n\
                 6. Complete the merge:\n\
                    git_commit({\n\
                        \"path\": \"/project\",\n\
                        \"message\": \"Merge remote changes, resolve conflicts\"\n\
                    })\n\n\
                 CONFLICT MARKERS EXPLAINED:\n\n\
                 <<<<<<< HEAD\n\
                 Your local changes\n\
                 (current branch code)\n\
                 =======\n\
                 Remote changes\n\
                 (incoming code from pull)\n\
                 >>>>>>> origin/main\n\n\
                 RESOLUTION STRATEGIES:\n\n\
                 1. Keep local version:\n\
                    Remove markers, keep code between <<<<<<< and =======\n\n\
                 2. Keep remote version:\n\
                    Remove markers, keep code between ======= and >>>>>>>\n\n\
                 3. Combine both:\n\
                    Remove markers, manually merge both changes\n\n\
                 4. Write new solution:\n\
                    Remove all, write better code that incorporates both\n\n\
                 ABORTING A MERGE:\n\
                 If you want to cancel the pull/merge:\n\
                 git_merge({\n\
                     \"path\": \"/project\",\n\
                     \"abort\": true\n\
                 })\n\
                 // Cancels the merge, returns to pre-pull state\n\n\
                 CONFLICT RESOLUTION WORKFLOW:\n\n\
                 Step 1: Identify conflicts\n\
                 git_status({\"path\": \"/project\"})\n\n\
                 Step 2: For each conflicted file:\n\
                 - Read the file\n\
                 - Find conflict markers\n\
                 - Understand both changes\n\
                 - Edit to resolve\n\
                 - Remove all markers\n\n\
                 Step 3: Test the resolution\n\
                 - Build/compile the code\n\
                 - Run tests\n\n\
                 Step 4: Mark as resolved\n\
                 git_add({\"path\": \"/project\", \"files\": [\"file1.rs\", \"file2.rs\"]})\n\n\
                 Step 5: Complete the merge\n\
                 git_commit({\"path\": \"/project\", \"message\": \"Merge and resolve conflicts\"})\n\n\
                 COMMON CONFLICT SCENARIOS:\n\n\
                 1. Same line edited:\n\
                    Most common, both changed same code\n\
                    Resolution: Choose best version or combine\n\n\
                 2. File moved/deleted:\n\
                    One branch deleted, other modified\n\
                    Resolution: Decide if file should exist\n\n\
                 3. Binary files:\n\
                    Images, PDFs, compiled files\n\
                    Resolution: Choose one version (can't merge)\n\n\
                 4. Multiple files:\n\
                    Conflicts across many files\n\
                    Resolution: Resolve each systematically\n\n\
                 PREVENTING FUTURE CONFLICTS:\n\
                 - Pull frequently\n\
                 - Commit before pulling\n\
                 - Use feature branches\n\
                 - Communicate with team",
            ),
        },
    ]
}
