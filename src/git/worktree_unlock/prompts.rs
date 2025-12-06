//! Prompt messages for git_worktree_unlock tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitWorktreeUnlockPromptArgs;

/// Prompt provider for git_worktree_unlock tool
pub struct WorktreeUnlockPrompts;

impl PromptProvider for WorktreeUnlockPrompts {
    type PromptArgs = GitWorktreeUnlockPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("cleanup") => prompt_cleanup(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: basic, cleanup".to_string()),
                required: Some(false),
            }
        ]
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I unlock a worktree?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Unlock a worktree to enable automatic cleanup:\\n\\n\\\
                 What it does:\\n\\\
                 Removes the lock flag from a Git worktree, allowing it to be\\n\\\
                 automatically pruned via 'git worktree prune' command. When a\\n\\\
                 worktree is locked, Git protects it from automatic cleanup. Unlocking\\n\\\
                 removes this protection.\\n\\n\\\
                 Parameters:\\n\\\
                 - path (string, required): Main repository directory\\n\\\
                   The path to the Git repository that contains the worktree\\n\\\
                   Examples:\\n\\\
                   • /home/user/project (absolute path)\\n\\\
                   • ./project (relative path)\\n\\\
                   • ~/repos/myproject (home-relative path)\\n\\n\\\
                 - worktree_path (string, required): Path to the worktree to unlock\\n\\\
                   The directory of the specific worktree you want to unlock\\n\\\
                   Examples:\\n\\\
                   • /home/user/project-feature (absolute path)\\n\\\
                   • ./project-old (relative path)\\n\\\
                   • ~/repos/myproject-hotfix (home-relative path)\\n\\n\\\
                 Examples:\\n\\n\\\
                 UNLOCK WITH ABSOLUTE PATHS:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/home/user/repo\\\", \\\"worktree_path\\\": \\\"/home/user/repo-feature\\\"}\\n\\\
                 ```\\n\\\
                 This unlocks the worktree at /home/user/repo-feature that belongs\\n\\\
                 to the repository at /home/user/repo.\\n\\n\\\
                 UNLOCK WITH RELATIVE PATHS:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"worktree_path\\\": \\\"./repo-temp\\\"}\\n\\\
                 ```\\n\\\
                 This unlocks ./repo-temp relative to your current directory.\\n\\n\\\
                 UNLOCK BEFORE CLEANUP:\\n\\\
                 When you're done with a worktree and want to clean it up:\\n\\\
                 1. Finish work in worktree: /repo-old\\n\\\
                 2. Unlock it using this tool:\\n\\\
                    ```json\\n\\\
                    {\\\"path\\\": \\\"/repo\\\", \\\"worktree_path\\\": \\\"/repo-old\\\"}\\n\\\
                    ```\\n\\\
                 3. Manually delete the directory if desired:\\n\\\
                    $ rm -rf /repo-old\\n\\\
                 4. Later run: git worktree prune (safely removes the entry)\\n\\n\\\
                 UNLOCK MULTIPLE WORKTREES:\\n\\\
                 For each worktree you want to clean up:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./myproject\\\", \\\"worktree_path\\\": \\\"./myproject-feature1\\\"}\\n\\\
                 ```\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./myproject\\\", \\\"worktree_path\\\": \\\"./myproject-feature2\\\"}\\n\\\
                 ```\\n\\\
                 Then run git worktree prune to clean all unlocked entries.\\n\\n\\\
                 When to use:\\n\\\
                 - Before running 'git worktree prune' to allow cleanup\\n\\\
                 - When you no longer need a worktree and want to remove it\\n\\\
                 - After manually deleting a worktree directory\\n\\\
                 - In cleanup scripts or automated maintenance tasks\\n\\\
                 - When reorganizing your worktree structure\\n\\n\\\
                 Common patterns:\\n\\\
                 • Temporary worktrees: Create for short-term work, unlock when done\\n\\\
                 • Feature branches: Unlock after merging the feature\\n\\\
                 • Stale worktrees: Unlock old worktrees that are no longer active\\n\\\
                 • Automation: Include unlock in CI/CD cleanup pipelines\\n\\n\\\
                 Important notes:\\n\\\
                 - Unlocking does NOT delete the worktree directory\\n\\\
                 - You must manually delete the directory if desired\\n\\\
                 - After unlocking, 'git worktree prune' can remove the entry\\n\\\
                 - Always unlock before pruning to avoid leaving stale entries\\n\\\
                 - Safe to unlock worktrees you no longer need\\n\\n\\\
                 Relationship to other Git commands:\\n\\\
                 - git worktree lock: The opposite operation, protects from pruning\\n\\\
                 - git worktree prune: Removes entries for missing worktrees (only works on unlocked)\\n\\\
                 - git worktree list: Shows which worktrees are locked/unlocked\\n\\\
                 - git worktree remove: Alternative to manual deletion + prune"
            ),
        },
    ]
}

fn prompt_cleanup() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("Why would I unlock a worktree?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Unlock a worktree to enable cleanup and pruning:\\n\\n\\\
                 Why unlock?\\n\\\
                 By default, Git locks worktrees to protect them from being accidentally\\n\\\
                 deleted during cleanup operations. When a worktree is locked, 'git worktree prune'\\n\\\
                 will skip over it, even if the directory has been manually removed. Unlocking\\n\\\
                 removes this protection, allowing 'git worktree prune' to safely remove the\\n\\\
                 worktree entry if its directory is gone.\\n\\n\\\
                 The lock mechanism prevents automated cleanup tools from removing worktrees\\n\\\
                 that are still in use or important. You unlock when you're certain the worktree\\n\\\
                 is no longer needed.\\n\\n\\\
                 Cleanup workflow:\\n\\n\\\
                 1. FINISH WORK: Complete and commit all changes in the worktree\\n\\\
                    Navigate to the worktree directory:\\n\\\
                    $ cd /home/user/repo-feature\\n\\\
                    Commit your final changes:\\n\\\
                    $ git commit -am 'Final changes for feature'\\n\\\
                    Or push your work if needed:\\n\\\
                    $ git push origin feature-branch\\n\\n\\\
                 2. UNLOCK: Remove the lock using this tool\\n\\\
                    ```json\\n\\\
                    {\\\"path\\\": \\\"/home/user/repo\\\", \\\"worktree_path\\\": \\\"/home/user/repo-feature\\\"}\\n\\\
                    ```\\n\\\
                    This marks the worktree as eligible for cleanup.\\n\\n\\\
                 3. VERIFY: Optionally check the unlock succeeded\\n\\\
                    $ cd /home/user/repo\\n\\\
                    $ git worktree list\\n\\\
                    Look for the worktree - it should no longer show as locked.\\n\\n\\\
                 4. DELETE: Manually remove the worktree directory if desired\\n\\\
                    $ rm -rf /home/user/repo-feature\\n\\\
                    Or use Git's built-in remove command:\\n\\\
                    $ git worktree remove /home/user/repo-feature\\n\\n\\\
                 5. PRUNE: Clean up stale entries in the repository\\n\\\
                    $ cd /home/user/repo\\n\\\
                    $ git worktree prune\\n\\\
                    This removes metadata for worktrees whose directories are gone.\\n\\n\\\
                 Example - Cleanup abandoned worktree:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./myproject\\\", \\\"worktree_path\\\": \\\"./myproject-old-branch\\\"}\\n\\\
                 ```\\n\\\
                 Use this when you have an old worktree (like myproject-old-branch) that\\n\\\
                 you created for temporary work and no longer need. After unlocking,\\n\\\
                 you can safely delete the directory and prune.\\n\\n\\\
                 Example - Cleanup multiple worktrees:\\n\\\
                 When you have several worktrees to clean up, unlock each one first:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"worktree_path\\\": \\\"/repo-feature1\\\"}\\n\\\
                 ```\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"worktree_path\\\": \\\"/repo-feature2\\\"}\\n\\\
                 ```\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"worktree_path\\\": \\\"/repo-experimental\\\"}\\n\\\
                 ```\\n\\\
                 Then delete all the directories and run a single prune:\\n\\\
                 $ rm -rf /repo-feature1 /repo-feature2 /repo-experimental\\n\\\
                 $ cd /repo && git worktree prune\\n\\n\\\
                 Example - Automated cleanup in CI/CD:\\n\\\
                 In build scripts, you might create temporary worktrees for testing.\\n\\\
                 Always unlock them before cleanup:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./main-repo\\\", \\\"worktree_path\\\": \\\"./test-worktree-temp\\\"}\\n\\\
                 ```\\n\\\
                 Then your cleanup script can safely remove and prune.\\n\\n\\\
                 Safety considerations:\\n\\n\\\
                 • Lock purpose: Locked worktrees are protected from automatic pruning\\n\\\
                   This prevents accidental data loss during cleanup operations\\n\\n\\\
                 • Unlock safety: Unlocking does NOT delete anything immediately\\n\\\
                   It only marks the worktree as eligible for cleanup\\n\\\
                   The directory remains until you manually delete it\\n\\n\\\
                 • Directory deletion: Always safe to delete a worktree directory\\n\\\
                   Git stores all important data in the main repository\\n\\\
                   Worktrees are just working copies\\n\\n\\\
                 • Prune safety: 'git worktree prune' only removes metadata\\n\\\
                   It doesn't delete directories, only cleans up bookkeeping\\n\\\
                   It only affects unlocked worktrees with missing directories\\n\\n\\\
                 • Safe workflow: Unlock → Delete → Prune is the safest approach\\n\\\
                   This ensures proper cleanup without data loss\\n\\n\\\
                 Relationship to git worktree prune:\\n\\n\\\
                 Without unlock (worktree is locked):\\n\\\
                 - 'git worktree prune' will skip the locked worktree\\n\\\
                 - Even if the directory is deleted, the entry remains\\n\\\
                 - You'll see stale entries in 'git worktree list'\\n\\\
                 - Metadata accumulates over time\\n\\n\\\
                 With unlock (worktree is unlocked):\\n\\\
                 - 'git worktree prune' can process the worktree\\n\\\
                 - If directory is missing, entry is removed\\n\\\
                 - 'git worktree list' stays clean\\n\\\
                 - No stale metadata\\n\\n\\\
                 State transitions:\\n\\\
                 LOCKED → (unlock tool) → UNLOCKED → (prune) → REMOVED\\n\\n\\\
                 Best practices:\\n\\\
                 • Always unlock worktrees you no longer need\\n\\\
                 • Run 'git worktree prune' regularly to clean up stale entries\\n\\\
                 • In automation, unlock as part of teardown scripts\\n\\\
                 • Keep only active worktrees locked for protection\\n\\\
                 • Use 'git worktree list' to audit locked vs unlocked worktrees"
            ),
        },
    ]
}

