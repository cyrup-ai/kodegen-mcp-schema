//! Prompt messages for git_status tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitStatusPromptArgs;

/// Prompt provider for git_status tool
///
/// This is the ONLY way to provide prompts for git_status - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct StatusPrompts;

impl PromptProvider for StatusPrompts {
    type PromptArgs = GitStatusPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("interpreting") => prompt_interpreting(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, interpreting)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT STATUS
// ============================================================================

/// Basic status checking and understanding output
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I check the status of my Git repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_status tool shows your working tree status. Here's how to use it for basic status checking:\n\n\
                 CHECKING STATUS:\n\n\
                 1. Get status:\n\
                    git_status({\n\
                        \"path\": \"/project\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"branch\": \"feature/auth\",\n\
                   \"tracking\": \"origin/feature/auth\",\n\
                   \"ahead\": 2,\n\
                   \"behind\": 0,\n\
                   \"staged\": [\n\
                     {\"file\": \"src/auth.rs\", \"status\": \"modified\"}\n\
                   ],\n\
                   \"unstaged\": [\n\
                     {\"file\": \"src/main.rs\", \"status\": \"modified\"}\n\
                   ],\n\
                   \"untracked\": [\n\
                     \"src/new_file.rs\"\n\
                   ]\n\
                 }\n\n\
                 STATUS COMPONENTS:\n\
                 - branch: Current branch\n\
                 - tracking: Remote tracking branch\n\
                 - ahead/behind: Commits vs remote\n\
                 - staged: Ready to commit\n\
                 - unstaged: Changed but not staged\n\
                 - untracked: New files not in git\n\n\
                 BASIC EXAMPLES:\n\n\
                 1. Check status before committing:\n\
                    git_status({\"path\": \"/home/user/project\"})\n\
                    // See what will be committed\n\n\
                 2. Quick branch check:\n\
                    git_status({\"path\": \"/repo\"})\n\
                    // Verify you're on the right branch\n\n\
                 3. After making changes:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // See which files changed\n\n\
                 WHEN TO USE:\n\
                 - Before committing changes\n\
                 - After switching branches\n\
                 - Before pushing to remote\n\
                 - When unsure of current state\n\
                 - To check if working tree is clean\n\n\
                 READING THE OUTPUT:\n\
                 - Empty arrays mean nothing in that category\n\
                 - ahead: 0, behind: 0 means synchronized with remote\n\
                 - tracking: null means no remote branch set up\n\
                 - staged files will be in next commit\n\
                 - unstaged files need git add first\n\
                 - untracked files are new, not tracked by git",
            ),
        },
    ]
}

/// Understanding and interpreting status output
fn prompt_interpreting() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I interpret the git_status output?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Understanding git_status output helps you know exactly what's happening in your repository:\n\n\
                 FILE STATES:\n\
                 - staged: Added to index, will be in next commit\n\
                 - unstaged: Modified but not added\n\
                 - untracked: New file, not tracked by git\n\n\
                 STATUS CODES:\n\
                 - M: Modified\n\
                 - A: Added (new file staged)\n\
                 - D: Deleted\n\
                 - R: Renamed\n\
                 - C: Copied\n\
                 - U: Unmerged (conflict)\n\n\
                 AHEAD/BEHIND:\n\
                 - ahead 2: You have 2 commits to push\n\
                 - behind 3: Remote has 3 commits to pull\n\
                 - Both: Need pull then push (or rebase)\n\n\
                 CLEAN STATUS:\n\
                 {\n\
                   \"branch\": \"main\",\n\
                   \"staged\": [],\n\
                   \"unstaged\": [],\n\
                   \"untracked\": []\n\
                 }\n\
                 // Nothing to commit, working tree clean\n\n\
                 DETAILED FIELD MEANINGS:\n\n\
                 BRANCH:\n\
                 - Current branch name (e.g., \"main\", \"feature/login\")\n\
                 - Tells you which branch you're on\n\
                 - Important before making commits\n\n\
                 TRACKING:\n\
                 - Remote branch this branch tracks (e.g., \"origin/main\")\n\
                 - null if no upstream configured\n\
                 - Determines where pushes go\n\n\
                 AHEAD:\n\
                 - Number of commits ahead of remote\n\
                 - ahead: 3 means 3 local commits not pushed\n\
                 - null if no tracking branch\n\
                 - Action needed: git push\n\n\
                 BEHIND:\n\
                 - Number of commits behind remote\n\
                 - behind: 2 means remote has 2 new commits\n\
                 - null if no tracking branch\n\
                 - Action needed: git pull or git fetch\n\n\
                 STAGED:\n\
                 - Files added to staging area (git add)\n\
                 - Will be included in next commit\n\
                 - Each entry shows file path and status\n\
                 - Empty array means nothing staged\n\n\
                 UNSTAGED:\n\
                 - Modified files not yet staged\n\
                 - Changes exist but not added\n\
                 - Need git add to stage them\n\
                 - Won't be in next commit unless staged\n\n\
                 UNTRACKED:\n\
                 - New files git doesn't know about\n\
                 - Never been added to repository\n\
                 - Need git add to start tracking\n\
                 - Usually new source files or configs\n\n\
                 COMMON SCENARIOS:\n\n\
                 1. Ready to commit:\n\
                 {\n\
                   \"staged\": [{\"file\": \"src/main.rs\", \"status\": \"modified\"}],\n\
                   \"unstaged\": [],\n\
                   \"untracked\": []\n\
                 }\n\
                 // Changes staged, ready for git commit\n\n\
                 2. Changes not staged:\n\
                 {\n\
                   \"staged\": [],\n\
                   \"unstaged\": [{\"file\": \"README.md\", \"status\": \"modified\"}],\n\
                   \"untracked\": []\n\
                 }\n\
                 // Need to run git add first\n\n\
                 3. New files to add:\n\
                 {\n\
                   \"staged\": [],\n\
                   \"unstaged\": [],\n\
                   \"untracked\": [\"new_feature.rs\"]\n\
                 }\n\
                 // New file, needs git add to track\n\n\
                 4. Need to push:\n\
                 {\n\
                   \"ahead\": 3,\n\
                   \"behind\": 0,\n\
                   \"staged\": [],\n\
                   \"unstaged\": []\n\
                 }\n\
                 // 3 commits ready to push\n\n\
                 5. Need to pull:\n\
                 {\n\
                   \"ahead\": 0,\n\
                   \"behind\": 2,\n\
                   \"staged\": [],\n\
                   \"unstaged\": []\n\
                 }\n\
                 // Remote has 2 new commits\n\n\
                 6. Diverged branches:\n\
                 {\n\
                   \"ahead\": 2,\n\
                   \"behind\": 3\n\
                 }\n\
                 // Both local and remote have unique commits\n\
                 // Need to pull (may have conflicts) then push\n\n\
                 DECISION TREE:\n\n\
                 Clean status (all arrays empty)?\n\
                 → Safe to switch branches, nothing to commit\n\n\
                 Has staged changes?\n\
                 → Ready to commit with git commit\n\n\
                 Has unstaged changes?\n\
                 → Need git add before committing\n\n\
                 Has untracked files?\n\
                 → New files, decide if should track with git add\n\n\
                 ahead > 0?\n\
                 → Local commits to push to remote\n\n\
                 behind > 0?\n\
                 → Remote commits to pull\n\n\
                 ahead > 0 AND behind > 0?\n\
                 → Branches diverged, need to sync (pull + push or rebase)",
            ),
        },
    ]
}
