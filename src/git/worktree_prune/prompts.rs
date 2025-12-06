//! Prompt messages for git_worktree_prune tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitWorktreePrunePromptArgs;

/// Prompt provider for git_worktree_prune tool
pub struct WorktreePrunePrompts;

impl PromptProvider for WorktreePrunePrompts {
    type PromptArgs = GitWorktreePrunePromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I clean up stale git worktrees?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Remove stale worktree administrative files:\\n\\n\\\
                 WHAT IT DOES:\\n\\\
                 Prunes orphaned worktree metadata when actual directories have been manually \\\
                 deleted. Git maintains .git/worktrees/ references. This tool removes those stale \\\
                 references without deleting actual directories.\\n\\n\\\
                 BASIC USAGE:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\"}\\n\\\
                 ```\\n\\n\\\
                 RESPONSE STRUCTURE:\\n\\\
                 The tool returns three fields:\\n\\\
                 - success (bool): Whether pruning completed\\n\\\
                 - pruned_count (usize): Number of stale references removed\\n\\\
                 - message (String): Status description\\n\\n\\\
                 Example response: {\\\"success\\\": true, \\\"pruned_count\\\": 2, \\\
                 \\\"message\\\": \\\"Pruned 2 stale worktrees\\\"}\\n\\n\\\
                 WHEN TO USE:\\n\\\
                 - After manually deleting a worktree directory with rm -rf\\n\\\
                 - Before git_worktree_list to get clean, accurate listings\\n\\\
                 - When 'git worktree list' shows missing or broken entries\\n\\\
                 - After disk cleanup that removed worktree directories\\n\\\
                 - Before adding a new worktree with a previously used name\\n\\n\\\
                 COMMON WORKFLOW:\\n\\\
                 1. Check current worktrees: git_worktree_list\\n\\\
                 2. Manually delete directory: rm -rf /repo-feature\\n\\\
                 3. Run prune to clean metadata: git_worktree_prune {\\\"path\\\": \\\"./repo\\\"}\\n\\\
                 4. Verify it's gone: git_worktree_list (should be removed)\\n\\\
                 5. Now safe to reuse that worktree name\\n\\n\\\
                 IMPORTANT NOTES:\\n\\\
                 - Locked worktrees are NOT pruned (they're protected)\\n\\\
                 - To force-remove locked worktrees, use git_worktree_remove first\\n\\\
                 - Safe to run even if no stale worktrees exist (pruned_count will be 0)\\n\\n\\\
                 RELATED TOOLS:\\n\\\
                 - git_worktree_list: View all worktrees and their status\\n\\\
                 - git_worktree_add: Create new worktree\\n\\\
                 - git_worktree_lock: Protect worktree from pruning\\n\\\
                 - git_worktree_remove: Safely remove worktree and its metadata"
            ),
        },
    ]
}

