//! Prompt messages for git_worktree_remove tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitWorktreeRemovePromptArgs;

/// Prompt provider for git_worktree_remove tool
pub struct WorktreeRemovePrompts;

impl PromptProvider for WorktreeRemovePrompts {
    type PromptArgs = GitWorktreeRemovePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("force") => prompt_force(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: basic, force".to_string()),
                required: Some(false),
            }
        ]
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I remove a worktree?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Remove a Git worktree cleanly and safely.\\n\\n\\\
                 WHAT IT DOES:\\n\\\
                 - Removes the worktree directory from the filesystem\\n\\\
                 - Removes Git's administrative files tracking the worktree\\n\\\
                 - Verifies the worktree is safe to remove (no uncommitted changes)\\n\\n\\\
                 PARAMETERS:\\n\\\
                 - path (required): Path to the main repository\\n\\\
                 - worktree_path (required): Path to the worktree you want to remove\\n\\\
                 - force (optional): Set to true to skip safety checks (dangerous)\\n\\n\\\
                 BASIC EXAMPLE:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"worktree_path\\\": \\\"/repo-feature\\\"}\\n\\\
                 ```\\n\\n\\\
                 This removes the worktree at /repo-feature that was created from the main repository at /repo.\\n\\n\\\
                 WHEN TO USE:\\n\\\
                 - After finishing work in a worktree and merging changes\\n\\\
                 - When cleaning up temporary worktrees for feature branches\\n\\\
                 - After changes have been committed and pushed\\n\\\
                 - When you no longer need parallel development environments\\n\\n\\\
                 TYPICAL WORKFLOW:\\n\\\
                 1. Check worktree status first:\\n\\\
                    Use git_worktree_list to see all worktrees and their state\\n\\n\\\
                 2. Ensure work is saved:\\n\\\
                    - Commit any changes in the worktree\\n\\\
                    - Or merge the worktree's branch into main\\n\\\
                    - Or verify you don't need uncommitted changes\\n\\n\\\
                 3. Remove the worktree:\\n\\\
                    ```json\\n\\\
                    {\\\"path\\\": \\\"./repo\\\", \\\"worktree_path\\\": \\\"./repo-feature\\\"}\\n\\\
                    ```\\n\\n\\\
                 4. Optional cleanup:\\n\\\
                    Delete the branch if no longer needed (use git_branch_delete)\\n\\n\\\
                 COMMON PATTERN:\\n\\\
                 After creating a worktree with git_worktree_add:\\n\\\
                 - Create worktree: git_worktree_add → work on feature\\n\\\
                 - Develop: make changes, commit, test in isolation\\n\\\
                 - Integrate: merge branch or cherry-pick commits\\n\\\
                 - Clean up: git_worktree_remove → remove the worktree\\n\\n\\\
                 RESPONSE FORMAT:\\n\\\
                 On success, returns confirmation that:\\n\\\
                 - The worktree directory has been removed\\n\\\
                 - Git's administrative tracking has been cleaned up\\n\\\
                 - The main repository is unaffected\\n\\\
                 Note: The branch associated with the worktree is NOT deleted - only the worktree itself.\\n\\n\\\
                 SAFETY NOTES:\\n\\\
                 - Removal fails if worktree has uncommitted changes (use force parameter to override)\\n\\\
                 - Cannot remove the main worktree (the original repository)\\n\\\
                 - Removal is permanent - the worktree directory is deleted from disk\\n\\\
                 - Always verify with git_worktree_list before removing"
            ),
        },
    ]
}

fn prompt_force() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("What if the worktree has uncommitted changes?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Force remove a worktree that has uncommitted changes or is locked.\\n\\n\\\
                 WHEN FORCE IS NEEDED:\\n\\\
                 - Worktree has uncommitted changes you want to discard\\n\\\
                 - Worktree directory was manually deleted but Git still tracks it\\n\\\
                 - Temporary worktrees created for quick experiments\\n\\\
                 - Locked worktrees that need emergency cleanup\\n\\\
                 - Abandoned development branches with incomplete work\\n\\n\\\
                 FORCE PARAMETER:\\n\\\
                 The force parameter (force: true) tells Git to:\\n\\\
                 - Skip the safety check for uncommitted changes\\n\\\
                 - Remove the worktree regardless of its current state\\n\\\
                 - Delete Git's administrative tracking immediately\\n\\\
                 - Proceed even if the worktree appears to be in use\\n\\n\\\
                 FORCE EXAMPLE:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"worktree_path\\\": \\\"./repo-temp\\\", \\\"force\\\": true}\\n\\\
                 ```\\n\\n\\\
                 ⚠️  DANGER WARNINGS:\\n\\\
                 - ALL uncommitted changes in the worktree will be PERMANENTLY LOST\\n\\\
                 - There is NO recovery mechanism - files are deleted from disk\\n\\\
                 - Uncommitted work cannot be retrieved after force removal\\n\\\
                 - Any stashes in that worktree location will also be lost\\n\\\
                 - Even if the branch still exists, the local uncommitted edits are gone\\n\\\
                 - Force removal bypasses all safety checks - use with extreme caution\\n\\n\\\
                 SAFER ALTERNATIVES:\\n\\\
                 Before using force, consider these safer options:\\n\\n\\\
                 1. CHECK STATUS FIRST:\\n\\\
                    Use git_worktree_list to see the worktree's current state\\n\\\
                    Verify what changes exist before deciding to force remove\\n\\n\\\
                 2. COMMIT CHANGES:\\n\\\
                    If there's any valuable work, commit it first:\\n\\\
                    - Switch to the worktree and create a commit\\n\\\
                    - Push the commit to preserve it remotely\\n\\\
                    - Then remove the worktree normally (without force)\\n\\n\\\
                 3. STASH CHANGES:\\n\\\
                    Use git_stash to temporarily save uncommitted work\\n\\\
                    - Stash the changes in the worktree\\n\\\
                    - Remove the worktree normally\\n\\\
                    - Apply the stash later in the main repo if needed\\n\\n\\\
                 4. NORMAL REMOVAL:\\n\\\
                    If you've saved the work, remove without force:\\n\\\
                    ```json\\n\\\
                    {\\\"path\\\": \\\"./repo\\\", \\\"worktree_path\\\": \\\"./repo-temp\\\"}\\n\\\
                    ```\\n\\n\\\
                 DECISION TREE:\\n\\\
                 Ask yourself before using force:\\n\\n\\\
                 - Is the work already committed on another branch?\\n\\\
                   → YES: Safe to use force=true\\n\\n\\\
                 - Are there uncommitted changes I might need?\\n\\\
                   → YES: Commit or stash first, then remove without force\\n\\n\\\
                 - Is this truly temporary work (experiments, quick tests)?\\n\\\
                   → YES: Force=true is acceptable\\n\\n\\\
                 - Is the worktree locked but I verified it's safe to remove?\\n\\\
                   → YES: Check git_worktree_list, confirm lock reason, then force if necessary\\n\\n\\\
                 WHEN FORCE IS APPROPRIATE:\\n\\\
                 - Cleaning up experimental worktrees with throwaway code\\n\\\
                 - Removing worktrees for abandoned feature branches\\n\\\
                 - Emergency cleanup when disk space is critical\\n\\\
                 - Removing orphaned worktrees after manual directory deletion\\n\\\
                 - Confirmed that all important work is already committed elsewhere"
            ),
        },
    ]
}

