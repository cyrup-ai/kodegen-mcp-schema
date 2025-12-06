//! Prompt messages for git_branch_delete tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitBranchDeletePromptArgs;

/// Prompt provider for git_branch_delete tool
///
/// This is the ONLY way to provide prompts for git_branch_delete - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct BranchDeletePrompts;

impl PromptProvider for BranchDeletePrompts {
    type PromptArgs = GitBranchDeletePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("local") => prompt_local(),
            Some("remote") => prompt_remote(),
            _ => prompt_local(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (local, remote)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Deleting local branches with safe vs force modes
fn prompt_local() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I safely delete local Git branches?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_branch_delete tool deletes local branches with built-in safety checks. Here's how to use it:\n\n\
                 DELETING LOCAL BRANCHES:\n\n\
                 1. Delete merged branch (SAFE):\n\
                    git_branch_delete({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/completed\"\n\
                    })\n\
                    // Safe: only works if merged into current branch\n\
                    // Protects against losing unmerged commits\n\n\
                 2. Force delete unmerged branch (DANGEROUS):\n\
                    git_branch_delete({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"experiment/abandoned\",\n\
                        \"force\": true\n\
                    })\n\
                    // WARNING: Loses unmerged commits permanently\n\
                    // Use only when you're absolutely certain\n\n\
                 3. Delete multiple branches:\n\
                    git_branch_delete({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/old-1\"\n\
                    })\n\
                    git_branch_delete({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/old-2\"\n\
                    })\n\
                    // Delete one at a time for safety\n\
                    // Each deletion is verified before proceeding\n\n\
                 SAFE DELETE vs FORCE DELETE:\n\n\
                 Default (force: false or omitted):\n\
                 - Only deletes if branch is fully merged\n\
                 - Git checks merge status automatically\n\
                 - Errors if unmerged commits exist\n\
                 - Recommended for normal workflow\n\
                 - Prevents accidental data loss\n\n\
                 Force (force: true):\n\
                 - Deletes regardless of merge status\n\
                 - Bypasses all safety checks\n\
                 - Permanently loses unmerged commits\n\
                 - Difficult to recover (requires reflog)\n\
                 - Use ONLY when abandoning work\n\n\
                 WHAT YOU CANNOT DELETE:\n\n\
                 Currently checked out branch:\n\
                 - Error: Cannot delete the branch you're on\n\
                 - Solution: Switch to another branch first\n\
                 - Use git_checkout to switch branches\n\
                 - Example:\n\
                   git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\
                   git_branch_delete({\"path\": \"/project\", \"name\": \"feature/old\"})\n\n\
                 PARAMETERS:\n\
                 - path (required): Repository directory path\n\
                 - name (required): Branch name to delete\n\
                 - force (optional): Set true to force delete unmerged branches\n\
                 - remote (optional): Remote name for remote branch deletion\n\n\
                 RETURN VALUES:\n\
                 Success:\n\
                 - deleted: true\n\
                 - branch: Name of deleted branch\n\
                 - was_forced: Whether force deletion was used\n\n\
                 Error cases:\n\
                 - Branch not found\n\
                 - Currently checked out\n\
                 - Has unmerged commits (when force: false)\n\
                 - Invalid repository path\n\n\
                 WHEN TO USE SAFE DELETE:\n\
                 - After PR is merged\n\
                 - Feature branch completed and merged\n\
                 - Hotfix branch merged to main\n\
                 - Release branch fully integrated\n\
                 - Any branch you want to keep commits from\n\n\
                 WHEN TO USE FORCE DELETE:\n\
                 - Experimental branch no longer needed\n\
                 - Duplicate branch created by mistake\n\
                 - Test branch with throwaway commits\n\
                 - Branch created for learning/testing\n\
                 - You're ABSOLUTELY CERTAIN work is not needed\n\n\
                 BEST PRACTICES:\n\
                 1. Always try safe delete first\n\
                 2. Check merge status with git_status before deleting\n\
                 3. Review commits with git_log if unsure\n\
                 4. Use git_diff to compare branches\n\
                 5. Switch away from branch before deleting\n\
                 6. Keep main/develop branches always\n\
                 7. Document why you're force deleting\n\n\
                 VERIFICATION WORKFLOW:\n\
                 // Step 1: Check current branch\n\
                 git_status({\"path\": \"/project\"})\n\
                 // Ensure not on branch you want to delete\n\n\
                 // Step 2: Verify merge status\n\
                 git_log({\"path\": \"/project\", \"branch\": \"feature/to-delete\", \"limit\": 5})\n\
                 // Review recent commits\n\n\
                 // Step 3: Safe delete\n\
                 git_branch_delete({\"path\": \"/project\", \"name\": \"feature/to-delete\"})\n\
                 // If error about unmerged commits, decide:\n\
                 // - Merge it first, OR\n\
                 // - Force delete if abandoning\n\n\
                 // Step 4: Confirm deletion\n\
                 git_branch_list({\"path\": \"/project\"})\n\
                 // Verify branch no longer in list",
            ),
        },
    ]
}

/// Deleting remote branches and cleanup
fn prompt_remote() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I delete branches from remote repositories?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Remote branch deletion removes branches from remote servers (like GitHub, GitLab). Use with caution.\n\n\
                 DELETING REMOTE BRANCHES:\n\n\
                 1. Delete from remote:\n\
                    git_branch_delete({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/merged\",\n\
                        \"remote\": \"origin\"\n\
                    })\n\
                    // Deletes origin/feature/merged from remote server\n\
                    // Other team members won't see this branch anymore\n\
                    // Use after PR is merged and closed\n\n\
                 2. Delete both local and remote:\n\
                    // First delete from remote\n\
                    git_branch_delete({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/pr-123\",\n\
                        \"remote\": \"origin\"\n\
                    })\n\
                    // Then delete local copy\n\
                    git_branch_delete({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/pr-123\"\n\
                    })\n\
                    // Complete cleanup of branch\n\n\
                 3. After PR merge on GitHub:\n\
                    // PR was merged on GitHub web interface\n\
                    // Delete remote branch\n\
                    git_branch_delete({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/pr-456\",\n\
                        \"remote\": \"origin\"\n\
                    })\n\
                    // Delete local branch\n\
                    git_branch_delete({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/pr-456\"\n\
                    })\n\
                    // Both local and remote cleaned up\n\n\
                 CLEANUP TRACKING REFERENCES:\n\n\
                 After deleting remote branches, prune stale references:\n\
                 git_fetch({\n\
                     \"path\": \"/project\",\n\
                     \"prune\": true\n\
                 })\n\
                 // Removes local references to deleted remote branches\n\
                 // Keeps your branch list clean\n\
                 // Run periodically to sync with remote state\n\n\
                 REMOTE DELETION BEHAVIOR:\n\n\
                 What happens:\n\
                 - Branch removed from remote server immediately\n\
                 - Other developers won't see it on next fetch\n\
                 - Cannot be undone easily (requires force push)\n\
                 - Remote tracking references become stale\n\
                 - Local copies on other machines remain until pruned\n\n\
                 Safety considerations:\n\
                 - Ensure PR is merged before deleting remote branch\n\
                 - Verify no one else is actively using the branch\n\
                 - Check team communication/conventions\n\
                 - Some teams auto-delete on PR merge\n\
                 - Others keep branches for audit trail\n\n\
                 REMOTE DELETION WORKFLOW:\n\n\
                 Typical PR merge and cleanup:\n\
                 // 1. Create and push feature branch\n\
                 // (already done)\n\n\
                 // 2. PR merged on GitHub/GitLab\n\
                 // (done via web interface)\n\n\
                 // 3. Delete remote branch\n\
                 git_branch_delete({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"feature/done\",\n\
                     \"remote\": \"origin\"\n\
                 })\n\n\
                 // 4. Update main branch\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\
                 git_pull({\"path\": \"/project\"})\n\n\
                 // 5. Delete local branch\n\
                 git_branch_delete({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"feature/done\"\n\
                 })\n\n\
                 // 6. Prune stale references\n\
                 git_fetch({\"path\": \"/project\", \"prune\": true})\n\n\
                 CANNOT UNDO EASILY:\n\n\
                 Remote deletion is permanent:\n\
                 - Branch removed from server\n\
                 - Other developers lose access\n\
                 - Recovery requires force push\n\
                 - May violate team policies\n\
                 - Always verify before deleting\n\n\
                 If you delete by mistake:\n\
                 - Check local copy still exists\n\
                 - Push branch back to remote\n\
                 - Or restore from another developer's local copy\n\
                 - Or use git reflog if pushed recently\n\n\
                 PARAMETERS FOR REMOTE DELETION:\n\
                 - path (required): Repository directory\n\
                 - name (required): Branch name (without remote/ prefix)\n\
                 - remote (required): Remote name (usually \"origin\")\n\
                 - force: Not applicable for remote deletion\n\n\
                 COMMON REMOTES:\n\
                 - origin: Default remote (your fork or main repo)\n\
                 - upstream: Original repository (for forks)\n\
                 - Use git_remote_list to see all remotes\n\n\
                 TEAM WORKFLOWS:\n\n\
                 Auto-delete on merge:\n\
                 - GitHub/GitLab can auto-delete after PR merge\n\
                 - Configure in repository settings\n\
                 - Then only delete local copy\n\n\
                 Manual deletion:\n\
                 - Delete remote after confirming PR merged\n\
                 - Keeps branch until explicitly removed\n\
                 - Gives time for verification\n\n\
                 Keep branches:\n\
                 - Some teams keep all branches for audit\n\
                 - Periodic cleanup by maintainers\n\
                 - Or never delete (if space permits)\n\n\
                 BEST PRACTICES:\n\
                 1. Ensure PR is merged before deleting remote\n\
                 2. Communicate with team about branch deletion\n\
                 3. Delete both remote and local for complete cleanup\n\
                 4. Run git_fetch with prune regularly\n\
                 5. Keep main/develop branches on remote always\n\
                 6. Document deletion in PR comments if needed\n\
                 7. Follow your team's branch lifecycle conventions",
            ),
        },
    ]
}


