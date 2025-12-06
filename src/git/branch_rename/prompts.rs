//! Prompt messages for git_branch_rename tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitBranchRenamePromptArgs;

/// Prompt provider for git_branch_rename tool
pub struct BranchRenamePrompts;

impl PromptProvider for BranchRenamePrompts {
    type PromptArgs = GitBranchRenamePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("current") => prompt_current(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: basic, current".to_string()),
                required: Some(false),
            }
        ]
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I rename a branch?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Rename a branch with git_branch_rename:\n\n\
                 ```json\n\
                 {\"path\": \"/repo\", \"old_name\": \"feature/old-name\", \"new_name\": \"feature/new-name\"}\n\
                 ```\n\n\
                 This renames the branch while preserving all commits and history.\n\n\
                 PARAMETERS:\n\n\
                 path (required): Repository location\n\
                 - Absolute or relative path to git repository\n\
                 - Example: \"/home/user/project\"\n\n\
                 old_name (required): Current branch name to rename\n\
                 - Must be an existing branch in the repository\n\
                 - Can be the current branch or any other branch\n\
                 - Example: \"feature/old-name\"\n\n\
                 new_name (required): New branch name\n\
                 - Must not already exist (unless using force)\n\
                 - Follow your team's naming conventions\n\
                 - Example: \"feature/improved-name\"\n\n\
                 force (optional, default: false): Overwrite if new_name exists\n\
                 - Set to true to force rename even if target exists\n\
                 - WARNING: This deletes the existing branch at new_name\n\
                 - Use with caution\n\n\
                 EXAMPLES:\n\n\
                 Rename a feature branch to be more descriptive:\n\
                 ```json\n\
                 {\"path\": \"/repo\", \"old_name\": \"feature/temp\", \"new_name\": \"feature/user-authentication\"}\n\
                 ```\n\n\
                 Fix a typo in branch name:\n\
                 ```json\n\
                 {\"path\": \"/repo\", \"old_name\": \"fix/validaton\", \"new_name\": \"fix/validation\"}\n\
                 ```\n\n\
                 Force rename (overwrites existing branch):\n\
                 ```json\n\
                 {\"path\": \"/repo\", \"old_name\": \"feature/a\", \"new_name\": \"feature/b\", \"force\": true}\n\
                 ```\n\
                 WARNING: Deletes existing feature/b branch\n\n\
                 SAFE RENAME (default behavior):\n\
                 - Fails if new_name already exists\n\
                 - Protects against accidental overwrites\n\
                 - Prevents data loss from typos\n\
                 - Recommended for normal use\n\n\
                 RESPONSE STRUCTURE:\n\
                 The tool returns:\n\
                 - success: true if rename succeeded\n\
                 - old_name: The previous branch name\n\
                 - new_name: The new branch name\n\
                 - message: Confirmation message\n\n\
                 COMMON PATTERNS:\n\n\
                 Fix typos in branch names:\n\
                 Rename branches with spelling errors to correct versions\n\n\
                 Make names more descriptive:\n\
                 Change generic names like \"feature/temp\" to specific names like \"feature/oauth-integration\"\n\n\
                 Adopt team conventions:\n\
                 Standardize branch names to match your team's naming patterns (e.g., feature/, fix/, hotfix/)\n\n\
                 WHEN TO USE THIS TOOL:\n\n\
                 Early in branch lifecycle:\n\
                 Best to rename before sharing the branch with others\n\n\
                 Before pushing to remote:\n\
                 Rename local branches before first push to avoid remote cleanup\n\n\
                 Before creating pull request:\n\
                 Ensure branch name is clear and follows conventions before PR\n\n\
                 To improve code clarity:\n\
                 Better branch names make project history more readable\n\n\
                 IMPORTANT BEHAVIORS:\n\n\
                 Preserves all commits:\n\
                 Every commit on the branch remains intact with same hashes\n\n\
                 Preserves branch history:\n\
                 Branch history and relationships are maintained\n\n\
                 Can rename current or other branches:\n\
                 Works whether you're on the branch or on a different branch\n\n\
                 Local operation only:\n\
                 Does not automatically update remote repositories\n\n\
                 Does not affect remote:\n\
                 Remote branches keep their old names until you push changes\n\n\
                 Does not update pull requests:\n\
                 Existing PRs continue referencing the old branch name"
            ),
        },
    ]
}

fn prompt_current() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Can I rename the branch I'm currently on?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Yes! You can rename the current branch. Here's the recommended workflow:\n\n\
                 ```json\n\
                 {\"path\": \"/repo\", \"old_name\": \"feature/temp\", \"new_name\": \"feature/user-authentication\"}\n\
                 ```\n\n\
                 WORKFLOW FOR RENAMING CURRENT BRANCH:\n\n\
                 STEP 1: Check which branch you're on\n\n\
                 Use git_status to discover your current branch:\n\
                 ```json\n\
                 {\"path\": \"/repo\"}\n\
                 ```\n\
                 Returns: branch: \"feature/temp\"\n\n\
                 Why check first? Confirms the exact branch name before renaming, preventing typos in old_name parameter.\n\n\
                 STEP 2: Rename the current branch\n\n\
                 Use git_branch_rename with the current branch name:\n\
                 ```json\n\
                 {\"path\": \"/repo\", \"old_name\": \"feature/temp\", \"new_name\": \"feature/user-auth\"}\n\
                 ```\n\n\
                 What happens during rename:\n\
                 - Branch reference updated to new name\n\
                 - HEAD still points to same branch (now with new name)\n\
                 - All commits remain on the branch\n\
                 - Working directory unchanged\n\n\
                 STEP 3: Verify the rename\n\n\
                 Use git_status again to confirm:\n\
                 ```json\n\
                 {\"path\": \"/repo\"}\n\
                 ```\n\
                 Returns: branch: \"feature/user-auth\"\n\n\
                 This confirms you're still on the same branch, just renamed.\n\n\
                 WHAT HAPPENS WHEN YOU RENAME CURRENT BRANCH:\n\n\
                 You stay on the same branch:\n\
                 No checkout occurs, you remain on the branch throughout\n\n\
                 Branch is just renamed:\n\
                 Only the branch name changes, everything else stays the same\n\n\
                 All commits intact:\n\
                 Every commit on the branch is preserved with identical hashes\n\n\
                 Working directory unchanged:\n\
                 Your files, modifications, and staged changes remain exactly as they were\n\n\
                 No checkout needed:\n\
                 You don't need to switch branches before or after the rename\n\n\
                 ERROR HANDLING:\n\n\
                 What if old_name doesn't exist?\n\
                 The tool fails with an error. Always check current branch name first with git_status.\n\n\
                 What if new_name already exists?\n\
                 The tool fails unless you use force:true. Check branch list first to avoid conflicts.\n\n\
                 How to handle these errors:\n\
                 Use git_branch_list to see all branches before renaming. This prevents naming conflicts.\n\n\
                 REMOTE COORDINATION:\n\n\
                 If branch was pushed to remote:\n\
                 The old name remains on remote repositories. The rename is local only.\n\n\
                 May need separate operations:\n\
                 To clean up remote, you'll need to delete old remote branch and push new one.\n\n\
                 Important for team coordination:\n\
                 If others are using the branch, coordinate rename to avoid confusion.\n\n\
                 COMMON WORKFLOW PATTERNS:\n\n\
                 Local-only rename:\n\
                 Rename branch that was never pushed. No remote cleanup needed. Continue working normally.\n\n\
                 Before push rename:\n\
                 Rename branch, then push with new name for first time. Clean history from the start.\n\n\
                 Shared branch rename:\n\
                 Coordinate with team before renaming. May need to notify others and clean up remote refs.\n\n\
                 RENAMING OTHER BRANCHES:\n\n\
                 You can rename any branch, not just current:\n\
                 ```json\n\
                 // You're on main, renaming develop\n\
                 {\"path\": \"/repo\", \"old_name\": \"develop\", \"new_name\": \"development\"}\n\
                 ```\n\n\
                 Works the same way:\n\
                 Same tool, same parameters, same behavior. You don't need to checkout the branch first.\n\n\
                 WHEN TO USE THIS SCENARIO:\n\n\
                 When actively on a branch you want to rename:\n\
                 Perfect for improving branch names while working on them.\n\n\
                 Need to understand workflow implications:\n\
                 Learn what happens to your current state during rename.\n\n\
                 Want to know what happens to your current state:\n\
                 Understand that working directory and commits remain unchanged."
            ),
        },
    ]
}

