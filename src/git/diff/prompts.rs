//! Prompt messages for git_diff tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitDiffPromptArgs;

/// Prompt provider for git_diff tool
///
/// This is the ONLY way to provide prompts for git_diff - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct DiffPrompts;

impl PromptProvider for DiffPrompts {
    type PromptArgs = GitDiffPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("commits") => prompt_commits(),
            _ => prompt_working(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (working, commits). Default: working directory changes".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT_DIFF
// ============================================================================

/// Working directory changes
fn prompt_working() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I see what changed in my working directory?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_diff to view working directory changes - what you've modified but haven't committed yet.\n\n\
                 BASIC EXAMPLES:\n\
                 1. All changes:\n\
                    git_diff({\"path\": \"/project\"})\n\n\
                 2. Specific file:\n\
                    git_diff({\"path\": \"/project\", \"files\": [\"src/main.rs\"]})\n\n\
                 3. Directory:\n\
                    git_diff({\"path\": \"/project\", \"files\": [\"src/\"]})\n\n\
                 4. Staged changes only:\n\
                    git_diff({\"path\": \"/project\", \"staged\": true})\n\n\
                 5. Statistics only:\n\
                    git_diff({\"path\": \"/project\", \"stat_only\": true})\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"files_changed\": 3,\n\
                   \"insertions\": 25,\n\
                   \"deletions\": 10,\n\
                   \"diff\": \"...\"\n\
                 }\n\
                 - files_changed: Number of modified files\n\
                 - insertions: Total lines added\n\
                 - deletions: Total lines removed\n\
                 - diff: Line-by-line changes in unified diff format\n\n\
                 READING DIFF OUTPUT:\n\
                 Line prefixes:\n\
                 - '+' = ADDITIONS (new lines)\n\
                 - '-' = DELETIONS (removed lines)\n\
                 - ' ' = CONTEXT (unchanged reference lines)\n\
                 - '@@' = Line number ranges\n\n\
                 Example:\n\
                 @@ -42,6 +42,7 @@\n\
                  fn main() {\n\
                 -    println!(\"Hello\");\n\
                 +    println!(\"Hello, world!\");\n\
                 +    println!(\"Welcome\");\n\
                  }\n\n\
                 WHEN TO CHECK:\n\
                 - Before staging (git_add) - Review what you're about to stage\n\
                 - Before committing - Verify all changes are present\n\
                 - Before switching branches - Check for uncommitted work\n\
                 - After modifying files - Confirm changes are expected\n\n\
                 WORKFLOW EXAMPLE:\n\
                 1. Make changes to files\n\
                 2. git_diff({\"path\": \"/project\"}) - Review all changes\n\
                 3. git_add({\"path\": \"/project\", \"files\": [...]}) - Stage changes\n\
                 4. git_diff({\"path\": \"/project\", \"staged\": true}) - Verify staged\n\
                 5. git_commit({\"path\": \"/project\", \"message\": \"...\"}) - Commit\n\n\
                 FILE FILTERING:\n\
                 Single file:\n\
                 git_diff({\"path\": \"/project\", \"files\": [\"src/main.rs\"]})\n\n\
                 Multiple files:\n\
                 git_diff({\"path\": \"/project\", \"files\": [\"src/a.rs\", \"src/b.rs\"]})\n\n\
                 Directory:\n\
                 git_diff({\"path\": \"/project\", \"files\": [\"src/\"]})\n\n\
                 PARAMETERS:\n\
                 - path: Repository path (required)\n\
                 - files: Array of file paths (optional)\n\
                 - staged: true/false (optional) - Show staged vs unstaged\n\
                 - stat_only: Show statistics only (faster)\n\
                 - name_only: Show file names only (fastest)\n\n\
                 BEST PRACTICES:\n\
                 - Review diffs before staging to catch unintended changes\n\
                 - Look for debugging code (console.log, println!) before committing\n\
                 - Verify no sensitive data (passwords, keys) in changes\n\
                 - Check that tests were updated if code changed\n\
                 - Ensure documentation matches code changes",
            ),
        },
    ]
}

/// Comparing commits
fn prompt_commits() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I compare different commits?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Compare any two commits, branches, or tags to see what changed between them.\n\n\
                 COMMIT REFERENCE FORMATS:\n\
                 Commit hashes:\n\
                 - \"abc1234\" - Abbreviated hash (7 chars)\n\
                 - \"a1b2c3d4e5f6...\" - Full SHA-1 hash\n\n\
                 Branch and tag references:\n\
                 - \"HEAD\" - Current commit\n\
                 - \"main\" - Latest commit on main branch\n\
                 - \"origin/main\" - Remote branch tip\n\
                 - \"feature/auth\" - Feature branch tip\n\
                 - \"v1.0.0\" - Version tag\n\n\
                 Relative references:\n\
                 - \"HEAD~1\" or \"HEAD^\" - Parent of HEAD\n\
                 - \"HEAD~5\" - Five commits before HEAD\n\
                 - \"abc1234^\" - Parent of commit abc1234\n\
                 - \"main~3\" - Three commits before main\n\n\
                 Merge commit parents:\n\
                 - \"HEAD^1\" - First parent\n\
                 - \"HEAD^2\" - Second parent\n\n\
                 BASIC COMPARISON EXAMPLES:\n\
                 Two commits:\n\
                 git_diff({\"path\": \"/project\", \"from\": \"abc1234\", \"to\": \"def5678\"})\n\n\
                 Since commit to now:\n\
                 git_diff({\"path\": \"/project\", \"from\": \"abc1234\"})\n\n\
                 Last N commits:\n\
                 git_diff({\"path\": \"/project\", \"from\": \"HEAD~5\", \"to\": \"HEAD\"})\n\n\
                 Between versions:\n\
                 git_diff({\"path\": \"/project\", \"from\": \"v1.0.0\", \"to\": \"v2.0.0\"})\n\n\
                 Single commit changes:\n\
                 git_diff({\"path\": \"/project\", \"from\": \"HEAD^\", \"to\": \"HEAD\"})\n\n\
                 Branch comparison:\n\
                 git_diff({\"path\": \"/project\", \"from\": \"main\", \"to\": \"feature/x\"})\n\n\
                 Specific file across commits:\n\
                 git_diff({\"path\": \"/project\", \"from\": \"v1.0.0\", \"to\": \"HEAD\", \"files\": [\"src/api.rs\"]})\n\n\
                 USE CASES:\n\
                 Release comparison:\n\
                 git_diff({\"path\": \"/project\", \"from\": \"v1.0.0\", \"to\": \"v2.0.0\"})\n\
                 Generate changelogs or verify release scope\n\n\
                 Bug investigation:\n\
                 git_diff({\"path\": \"/project\", \"from\": \"good_commit\", \"to\": \"bad_commit\"})\n\
                 Find changes that introduced issue\n\n\
                 Single commit review:\n\
                 git_diff({\"path\": \"/project\", \"from\": \"abc1234^\", \"to\": \"abc1234\"})\n\
                 See what a specific commit changed\n\n\
                 DEBUGGING WORKFLOW:\n\
                 When investigating bug introduction:\n\n\
                 1. Find last known good commit:\n\
                    git_log({\"path\": \"/project\"})\n\n\
                 2. Compare good to current:\n\
                    git_diff({\"path\": \"/project\", \"from\": \"last_good_commit\"})\n\n\
                 3. Narrow to specific file:\n\
                    git_diff({\"path\": \"/project\", \"from\": \"last_good_commit\", \"files\": [\"src/buggy.rs\"]})\n\n\
                 4. Binary search commits:\n\
                    git_diff({\"path\": \"/project\", \"from\": \"commit1\", \"to\": \"commit2\"})\n\n\
                 5. Identify problematic change\n\n\
                 RESPONSE INTERPRETATION:\n\
                 {\n\
                   \"from\": \"abc1234\",\n\
                   \"to\": \"def5678\",\n\
                   \"files_changed\": 15,\n\
                   \"insertions\": 234,\n\
                   \"deletions\": 89,\n\
                   \"diff\": \"...\"\n\
                 }\n\
                 - from/to: Commits compared\n\
                 - files_changed: Number of modified files\n\
                 - insertions/deletions: Code growth or reduction\n\
                 - diff: Line-by-line changes\n\n\
                 FINDING COMMIT HASHES:\n\
                 Use git_log to see history:\n\
                 git_log({\"path\": \"/project\", \"max_count\": 10})\n\
                 Returns recent commits with hashes\n\n\
                 PARAMETERS:\n\
                 - path: Repository path (required)\n\
                 - from: Starting revision (required for comparison)\n\
                 - to: Ending revision (optional, defaults to working directory)\n\
                 - files: Optional file filtering\n\
                 - stat_only: Show statistics only\n\
                 - name_only: Show file names only\n\n\
                 BEST PRACTICES:\n\
                 - Use abbreviated hashes (7 chars) for readability\n\
                 - Compare tags for release documentation\n\
                 - Use HEAD~N for recent history\n\
                 - Focus on specific files for large diffs\n\
                 - Combine with git_log for context",
            ),
        },
    ]
}
