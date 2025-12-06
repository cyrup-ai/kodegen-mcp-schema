//! Prompt messages for git_commit tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitCommitPromptArgs;

/// Prompt provider for git_commit tool
///
/// This is the ONLY way to provide prompts for git_commit - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct CommitPrompts;

impl PromptProvider for CommitPrompts {
    type PromptArgs = GitCommitPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("amend") => prompt_amend(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, amend)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic commit creation patterns
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create basic commits using the git_commit tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_commit tool records changes to the repository. Use after staging files with git_add.\n\n\
                 BASIC USAGE:\n\n\
                 1. Simple commit:\n\
                    git_commit({\"path\": \"/project\", \"message\": \"Add user authentication\"})\n\n\
                 2. Multi-line message:\n\
                    git_commit({\n\
                        \"path\": \"/project\",\n\
                        \"message\": \"Add user auth\\n\\nImplements JWT-based authentication.\"\n\
                    })\n\n\
                 3. With body parameter:\n\
                    git_commit({\n\
                        \"path\": \"/project\",\n\
                        \"message\": \"Fix null pointer in user service\",\n\
                        \"body\": \"Handles missing users with 404 response.\"\n\
                    })\n\n\
                 4. Auto-stage all tracked files:\n\
                    git_commit({\"path\": \"/project\", \"message\": \"Update dependencies\", \"all\": true})\n\n\
                 REQUIRED PARAMETERS:\n\
                 - path: Repository directory path\n\
                 - message: Commit message (non-empty)\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - body: Extended description\n\
                 - all: Auto-stage all tracked files (default: false)\n\
                 - amend: Modify last commit (see amend scenario)\n\
                 - no_edit: Keep message when amending (default: false)\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"commit\": \"abc1234\",\n\
                   \"message\": \"Add user authentication\",\n\
                   \"files_changed\": 5,\n\
                   \"insertions\": 120,\n\
                   \"deletions\": 15\n\
                 }\n\n\
                 TYPICAL WORKFLOW:\n\
                 1. git_status - Check what changed\n\
                 2. git_add - Stage files\n\
                 3. git_commit - Create commit\n\
                 4. git_push - Upload to remote\n\n\
                 ERRORS:\n\
                 - \"nothing to commit\": No staged changes (use git_add first)\n\
                 - \"commit message cannot be empty\": Message required\n\
                 - \"not a git repository\": Invalid path or not initialized",
            ),
        },
    ]
}


/// Amending commits safely
fn prompt_amend() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I amend or fix the last commit?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use amend parameter to modify the most recent commit. Rewrites history - use carefully.\n\n\
                 AMENDING EXAMPLES:\n\n\
                 1. Fix commit message only:\n\
                    git_commit({\"path\": \"/project\", \"amend\": true, \"message\": \"Corrected message\"})\n\n\
                 2. Add forgotten file (keep same message):\n\
                    git_add({\"path\": \"/project\", \"files\": [\"forgotten.rs\"]})\n\
                    git_commit({\"path\": \"/project\", \"amend\": true, \"no_edit\": true})\n\n\
                 3. Fix last commit entirely (new files + new message):\n\
                    git_add({\"path\": \"/project\", \"all\": true})\n\
                    git_commit({\"path\": \"/project\", \"amend\": true, \"message\": \"Updated message\"})\n\n\
                 SAFE TO AMEND:\n\
                 - Local commits not yet pushed\n\
                 - Private feature branches\n\
                 - Before code review\n\
                 - Fixing mistakes immediately\n\n\
                 UNSAFE TO AMEND (creates conflicts):\n\
                 - Commits already pushed to shared branches\n\
                 - Commits on main/master\n\
                 - Commits others based work on\n\
                 - Merged commits\n\n\
                 AMEND WARNINGS:\n\
                 - Changes commit hash (history rewrite)\n\
                 - Breaks others' work if already pushed\n\
                 - Requires force push if already pushed\n\n\
                 RECOMMENDED WORKFLOW:\n\
                 1. Verify last commit: git_log({\"path\": \"/project\", \"max_count\": 1})\n\
                 2. Stage new files (if needed): git_add({\"files\": [\"file.rs\"]})\n\
                 3. Amend: git_commit({\"amend\": true, \"message\": \"...\"})\n\
                 4. Force push only if necessary: git_push({\"force\": true})\n\n\
                 ALTERNATIVES:\n\
                 - New commit: Safer for shared branches\n\
                 - git_revert: Undo with new commit\n\
                 - git_reset: Remove commits (local only)\n\n\
                 BEST PRACTICES:\n\
                 - Only amend unpushed commits\n\
                 - Use no_edit: true to keep message unchanged\n\
                 - Check git_status before amending\n\
                 - Avoid force push on shared branches",
            ),
        },
    ]
}
