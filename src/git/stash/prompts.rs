//! Prompt messages for git_stash_save tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitStashSavePromptArgs;

/// Prompt provider for git_stash tool
///
/// This is the ONLY way to provide prompts for git_stash - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct StashPrompts;

impl PromptProvider for StashPrompts {
    type PromptArgs = GitStashSavePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("operations") => prompt_operations(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, operations)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// Helper functions for prompt scenarios

/// Basic stash save operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I stash my uncommitted changes?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_stash_save tool saves your uncommitted work and cleans your working directory. Here's how to use it:\n\n\
                 BASIC STASHING:\n\
                 1. Stash all changes:\n\
                    git_stash_save({\n\
                        \"path\": \"/project\"\n\
                    })\n\
                    // Stashes modified tracked files\n\n\
                 2. Stash with message:\n\
                    git_stash_save({\n\
                        \"path\": \"/project\",\n\
                        \"message\": \"WIP: feature work\"\n\
                    })\n\
                    // Descriptive message helps identify stash later\n\n\
                 3. Verify working tree is clean:\n\
                    git_stash_save({\"path\": \"/project\"})\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Working tree should be clean after stash\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"stash\": \"stash@{0}\",\n\
                   \"files_stashed\": [\"src/main.rs\", \"src/lib.rs\"],\n\
                   \"success\": true\n\
                 }\n\n\
                 WHAT GETS STASHED:\n\
                 - Modified tracked files (files already in git)\n\
                 - Staged changes (files added with git add)\n\
                 - NOT untracked files by default (new files not in git)\n\
                 - NOT ignored files (.gitignore entries)\n\n\
                 PARAMETERS:\n\
                 - path (required): Path to git repository\n\
                 - message (optional): Description of stashed changes\n\
                 - include_untracked (optional): Also stash new files (default: false)\n\
                 - keep_index (optional): Keep staged changes staged (default: false)\n\
                 - all (optional): Stash everything including ignored (default: false)\n\n\
                 WHEN TO STASH:\n\
                 - Need to switch branches without committing\n\
                 - Want to pull changes but have local modifications\n\
                 - Need to test something on a clean state\n\
                 - Backup work before risky operations\n\
                 - Temporarily set aside incomplete work\n\n\
                 COMMON PATTERNS:\n\
                 1. Quick stash:\n\
                    git_stash_save({\"path\": \"/project\"})\n\
                 2. Stash with context:\n\
                    git_stash_save({\n\
                        \"path\": \"/project\",\n\
                        \"message\": \"Debugging session work\"\n\
                    })\n\
                 3. Stash everything:\n\
                    git_stash_save({\n\
                        \"path\": \"/project\",\n\
                        \"include_untracked\": true\n\
                    })\n\n\
                 VERIFYING STASH:\n\
                 After stashing, check status:\n\
                 git_status({\"path\": \"/project\"})\n\
                 Should show: \"nothing to commit, working tree clean\"\n\n\
                 List your stashes:\n\
                 git_stash_list({\"path\": \"/project\"})\n\
                 Shows all saved stashes with messages\n\n\
                 RETRIEVING STASHED WORK:\n\
                 Apply latest stash:\n\
                 git_stash_pop({\"path\": \"/project\"})\n\
                 // Restores changes and removes from stash list\n\n\
                 Apply without removing:\n\
                 git_stash_apply({\"path\": \"/project\"})\n\
                 // Restores changes but keeps in stash list\n\n\
                 ERROR HANDLING:\n\
                 - If nothing to stash: Returns success but no changes saved\n\
                 - If path not a git repo: Error indicating invalid repository\n\
                 - If conflicts exist: Error with conflict details\n\n\
                 BEST PRACTICES:\n\
                 1. Use descriptive messages for multiple stashes\n\
                 2. Don't keep stashes long-term (prefer commits)\n\
                 3. Pop/apply stashes promptly\n\
                 4. Clean up old stashes regularly\n\
                 5. Check git_stash_list before creating many stashes",
            ),
        },
    ]
}

/// Stash save operations
fn prompt_operations() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What options can I use with git_stash_save?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "git_stash_save supports several options to control what gets stashed:\n\n\
                 OPTION 1: include_untracked\n\
                 Stash new untracked files:\n\
                 git_stash_save({\"path\": \"/project\", \"include_untracked\": true})\n\
                 - Stashes: Modified tracked files + Staged changes + New untracked files\n\
                 - IGNORES: Files in .gitignore\n\
                 Use when you have new files to stash.\n\n\
                 OPTION 2: keep_index\n\
                 Keep staged changes staged after stash:\n\
                 git_stash_save({\"path\": \"/project\", \"keep_index\": true})\n\
                 - Staged files remain staged\n\
                 - Unstaged files are stashed and removed\n\
                 Use for partial commits: stage what you want to commit, stash the rest.\n\n\
                 OPTION 3: all\n\
                 Stash everything including ignored files:\n\
                 git_stash_save({\"path\": \"/project\", \"all\": true})\n\
                 - Stashes: Tracked + Untracked + Ignored files\n\
                 WARNING: May create very large stash with build artifacts.\n\
                 Usually prefer include_untracked instead.\n\n\
                 COMBINING OPTIONS:\n\
                 git_stash_save({\n\
                     \"path\": \"/project\",\n\
                     \"message\": \"Complete stash\",\n\
                     \"include_untracked\": true,\n\
                     \"keep_index\": true\n\
                 })\n\
                 Combines multiple options as needed.\n\n\
                 WHICH OPTIONS TO USE:\n\
                 - Have new files? → include_untracked: true\n\
                 - Want to commit staged? → keep_index: true\n\
                 - Need complete clean? → all: true\n\
                 - Just tracked files? → No options needed\n\n\
                 OPTION COMPARISON:\n\n\
                 Scenario: You have:\n\
                 - Modified: src/lib.rs (tracked)\n\
                 - Modified: src/main.rs (tracked, staged)\n\
                 - New: src/helper.rs (untracked)\n\
                 - Build: target/ (ignored)\n\n\
                 No options:\n\
                 Stashes: lib.rs, main.rs\n\
                 Leaves: helper.rs, target/\n\n\
                 include_untracked: true\n\
                 Stashes: lib.rs, main.rs, helper.rs\n\
                 Leaves: target/\n\n\
                 keep_index: true\n\
                 Stashes: lib.rs, main.rs\n\
                 Keeps staged: main.rs\n\
                 Leaves: helper.rs, target/\n\n\
                 all: true\n\
                 Stashes: lib.rs, main.rs, helper.rs, target/\n\
                 Leaves: nothing\n\n\
                 BEST PRACTICES:\n\
                 1. Use include_untracked when you have new files\n\
                 2. Use keep_index for partial commits\n\
                 3. Avoid 'all' unless you really need it\n\
                 4. Always use a message with options\n\
                 5. Verify with git_status after stashing",
            ),
        },
    ]
}
