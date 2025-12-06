//! Prompt messages for git_worktree_list tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitWorktreeListPromptArgs;

/// Prompt provider for git_worktree_list tool
pub struct WorktreeListPrompts;

impl PromptProvider for WorktreeListPrompts {
    type PromptArgs = GitWorktreeListPromptArgs;

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
            content: PromptMessageContent::text("How do I list and manage worktrees in a git repository?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "List git worktrees in your repository.

TOOL DESCRIPTION:
git_worktree_list displays all worktrees attached to a git repository, including the main 
worktree and all linked worktrees. Each worktree can have a different branch checked out,
allowing you to work on multiple branches simultaneously without constant switching.

REQUIRED PARAMETERS:
- path: Repository path (can be absolute or relative)

OPTIONAL PARAMETERS:
- verbose: Boolean (default: false). Shows detailed information when true.

BASIC USAGE:
```json
{\"path\": \"/path/to/repo\"}
```

RESPONSE STRUCTURE:
Shows for each worktree:
- path: Filesystem location of the worktree
- branch: Branch name checked out (or detached state)
- commit: Current HEAD commit (if needed via verbose)
- locked: Boolean - whether worktree is locked
- prunable: Boolean - whether worktree can be cleaned up

RESPONSE EXAMPLE:
```json
{
  \"worktrees\": [
    {
      \"path\": \"/home/user/repo\",
      \"branch\": \"main\",
      \"commit\": \"abc123def456\",
      \"locked\": false,
      \"prunable\": false
    },
    {
      \"path\": \"/home/user/repo-feature\",
      \"branch\": \"feature/new-api\",
      \"commit\": \"xyz789uvw012\",
      \"locked\": false,
      \"prunable\": false
    }
  ]
}
```

WHEN TO USE THIS TOOL:
- Before adding a new worktree: Verify existing worktrees and plan branch assignments
- Managing active work: See which branches are in use across all worktrees
- Before removing a worktree: Check which worktrees exist and their status
- Cleanup and maintenance: Identify prunable worktrees (incomplete operations)
- Debugging branch state: Verify which branch is checked out in each worktree
- Checking lock status: See if any worktrees are locked (protected from removal)

COMMON PATTERN - List and Check Before Creating New Worktree:
```
1. List current worktrees: {\"path\": \"/path/to/repo\"}
   → See all existing worktrees and their branches

2. Check if feature branch exists: 
   → Review which branches are already in use

3. If branch not in use, create new worktree:
   {\"path\": \"/path/to/repo\", \"branch\": \"feature/new-work\", \"track\": true}

4. Verify it was created: {\"path\": \"/path/to/repo\"}
   → Confirm new worktree appears in list
```

QUICK REFERENCE:
- Command: git_worktree_list
- Parameters: path (required), verbose (optional bool)
- Returns: Array of worktree objects with path, branch, commit, locked, prunable
- Related tools: git_worktree_add, git_worktree_remove, git_worktree_lock, git_worktree_unlock, git_worktree_prune"
            ),
        },
    ]
}
