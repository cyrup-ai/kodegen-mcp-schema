//! Prompt messages for git_reset tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitResetPromptArgs;

/// Prompt provider for git_reset tool
///
/// This is the ONLY way to provide prompts for git_reset - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ResetPrompts;

impl PromptProvider for ResetPrompts {
    type PromptArgs = GitResetPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("soft") => prompt_soft(),
            Some("mixed") => prompt_mixed(),
            _ => prompt_mixed(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (soft, mixed)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// Prompt scenarios

/// Soft reset - keep changes staged
fn prompt_soft() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use soft reset to undo commits while keeping changes staged?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Soft reset moves HEAD to a previous commit while keeping all changes staged.\n\n\
                 SOFT RESET:\n\n\
                 1. Undo last commit (keep staged):\n\
                 git_reset({\n\
                     \"path\": \"/project\",\n\
                     \"target\": \"HEAD~1\",\n\
                     \"mode\": \"soft\"\n\
                 })\n\
                 // Commit undone, changes stay staged\n\n\
                 2. Squash preparation:\n\
                 // Have 3 commits to squash\n\
                 git_reset({\n\
                     \"path\": \"/project\",\n\
                     \"target\": \"HEAD~3\",\n\
                     \"mode\": \"soft\"\n\
                 })\n\
                 git_commit({\n\
                     \"path\": \"/project\",\n\
                     \"message\": \"Combined commit\"\n\
                 })\n\
                 // All 3 commits now combined into one\n\n\
                 3. Reset to specific commit:\n\
                 git_reset({\n\
                     \"path\": \"/project\",\n\
                     \"target\": \"abc1234\",\n\
                     \"mode\": \"soft\"\n\
                 })\n\
                 // HEAD now at abc1234, all changes since then are staged\n\n\
                 SOFT RESET EFFECT:\n\
                 - HEAD moves to target commit\n\
                 - Working directory unchanged\n\
                 - Changes stay STAGED\n\
                 - Ready to commit differently\n\
                 - Branch pointer updated\n\
                 - No files modified\n\n\
                 USE CASES:\n\
                 - Fix commit message (undo, recommit with new message)\n\
                 - Combine multiple commits into one (squashing)\n\
                 - Reorganize commit history before pushing\n\
                 - Amend multiple commits at once\n\
                 - Change what goes in which commit\n\n\
                 WORKFLOW EXAMPLE:\n\
                 1. Made 3 small commits\n\
                 2. Realize they should be one commit\n\
                 3. git_reset({\"path\": \".\", \"target\": \"HEAD~3\", \"mode\": \"soft\"})\n\
                 4. All changes now staged\n\
                 5. git_commit({\"path\": \".\", \"message\": \"One comprehensive commit\"})\n\
                 6. History now cleaner\n\n\
                 SAFETY:\n\
                 - SAFE: No data loss\n\
                 - Changes remain staged\n\
                 - Can easily recommit\n\
                 - Reversible via reflog\n\n\
                 Remember: Soft reset is the safest reset mode - nothing is lost!",
            ),
        },
    ]
}

/// Mixed reset - keep changes unstaged (default)
fn prompt_mixed() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use mixed reset to keep changes but unstage them?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Mixed reset is the default and safest way to undo commits while keeping changes.\n\n\
                 MIXED RESET (DEFAULT):\n\n\
                 1. Undo commit, keep changes:\n\
                 git_reset({\n\
                     \"path\": \"/project\",\n\
                     \"target\": \"HEAD~1\"\n\
                 })\n\
                 // mode: \"mixed\" is default\n\
                 // Commit undone, changes are unstaged\n\n\
                 2. Reset to specific commit:\n\
                 git_reset({\n\
                     \"path\": \"/project\",\n\
                     \"target\": \"abc1234\",\n\
                     \"mode\": \"mixed\"\n\
                 })\n\
                 // HEAD at abc1234, all changes since then are unstaged\n\n\
                 3. Reorganize commits:\n\
                 git_reset({\n\
                     \"path\": \"/project\",\n\
                     \"target\": \"HEAD~2\"\n\
                 })\n\
                 // Now selectively stage and commit\n\
                 git_add({\"path\": \"/project\", \"files\": [\"important.rs\"]})\n\
                 git_commit({\"path\": \"/project\", \"message\": \"First commit\"})\n\
                 git_add({\"path\": \"/project\", \"files\": [\"other.rs\"]})\n\
                 git_commit({\"path\": \"/project\", \"message\": \"Second commit\"})\n\n\
                 MIXED RESET EFFECT:\n\
                 - HEAD moves to target\n\
                 - Working directory unchanged\n\
                 - Changes become UNSTAGED\n\
                 - Can re-stage selectively\n\
                 - Index/staging area cleared\n\
                 - Files keep modifications\n\n\
                 USE CASES:\n\
                 - Undo commits but keep the work\n\
                 - Reorganize changes into different commits\n\
                 - Split one large commit into multiple smaller ones\n\
                 - Unstage everything to start fresh\n\
                 - Change commit organization before pushing\n\n\
                 WHY IT'S DEFAULT:\n\
                 - Safe: No data loss\n\
                 - Flexible: Can re-stage differently\n\
                 - Common: Most frequent use case\n\
                 - Reversible: Changes still in working directory\n\n\
                 WORKFLOW EXAMPLE:\n\
                 1. Made commit with mixed concerns\n\
                 2. git_reset({\"path\": \".\", \"target\": \"HEAD~1\"})\n\
                 3. Changes now unstaged\n\
                 4. git_add features separately\n\
                 5. Commit feature A\n\
                 6. Commit feature B\n\
                 7. Better commit history\n\n\
                 COMPARISON:\n\
                 - soft: Changes stay staged (ready to commit)\n\
                 - mixed: Changes become unstaged (default, flexible)\n\
                 - hard: Changes discarded (DANGEROUS)\n\n\
                 Remember: Mixed is the default for a reason - it's safe and flexible!",
            ),
        },
    ]
}
