//! Prompt messages for git_worktree_add tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitWorktreeAddPromptArgs;

/// Prompt provider for git_worktree_add tool
pub struct WorktreeAddPrompts;

impl PromptProvider for WorktreeAddPrompts {
    type PromptArgs = GitWorktreeAddPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("branch") => prompt_branch(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: basic, branch".to_string()),
                required: Some(false),
            }
        ]
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("What is a Git worktree and how do I create one?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Git worktrees let you check out multiple branches from the same repository simultaneously. \\
                 Each worktree is a separate working directory that shares the same .git repository.\\n\\n\\\
                 BASIC USAGE:\\n\\\
                 Create a worktree for an existing branch:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"worktree_path\\\": \\\"/repo-hotfix\\\", \\\"branch\\\": \\\"hotfix\\\"}\\n\\\
                 ```\\n\\n\\\
                 REQUIRED PARAMETERS:\\n\\\
                 - path: Path to the main repository\\n\\\
                 - worktree_path: Where to create the new worktree directory\\n\\\
                 - branch: Which branch to check out in the worktree\\n\\n\\\
                 WHAT HAPPENS:\\n\\\
                 1. Creates a new directory at worktree_path\\n\\\
                 2. Links it to your existing .git repository\\n\\\
                 3. Checks out the specified branch there\\n\\\
                 4. The branch becomes active in that worktree only\\n\\n\\\
                 CORE CONCEPTS:\\n\\n\\\
                 Separate Working Directory:\\n\\\
                 - Each worktree has its own files and folders\\n\\\
                 - Changes in one worktree don't affect others\\n\\\
                 - You can edit files in both simultaneously\\n\\n\\\
                 Shared Repository (.git):\\n\\\
                 - All worktrees share the same commit history\\n\\\
                 - Commits made in any worktree are visible to all\\n\\\
                 - Same remotes, same configuration\\n\\\
                 - Disk space efficient (no duplication of .git)\\n\\n\\\
                 Different Branches Checked Out:\\n\\\
                 - Main repo might be on 'main' branch\\n\\\
                 - Worktree A might be on 'feature/auth'\\n\\\
                 - Worktree B might be on 'hotfix/bug-123'\\n\\\
                 - Each worktree is independent\\n\\n\\\
                 KEY BENEFITS:\\n\\n\\\
                 Work on Multiple Branches Simultaneously:\\n\\\
                 - No need to switch branches back and forth\\n\\\
                 - Keep your work-in-progress intact\\n\\\
                 - Review code in one worktree while developing in another\\n\\n\\\
                 No Stashing Required:\\n\\\
                 - Uncommitted changes stay in their worktree\\n\\\
                 - Switch between tasks instantly\\n\\\
                 - No risk of forgetting stashed changes\\n\\n\\\
                 Separate Build Environments:\\n\\\
                 - Compile main branch in one worktree\\n\\\
                 - Test feature branch in another\\n\\\
                 - Compare outputs side-by-side\\n\\\
                 - Run different versions simultaneously\\n\\n\\\
                 Complete Isolation:\\n\\\
                 - Dependency changes in one worktree don't affect others\\n\\\
                 - Different node_modules, different builds\\n\\\
                 - Safe experimentation without affecting main work\\n\\n\\\
                 COMMON EXAMPLES:\\n\\n\\\
                 Create worktree for hotfix:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./myproject\\\", \\\"worktree_path\\\": \\\"./myproject-hotfix\\\", \\\"branch\\\": \\\"hotfix/urgent-bug\\\"}\\n\\\
                 ```\\n\\n\\\
                 Create worktree for feature development:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/home/dev/app\\\", \\\"worktree_path\\\": \\\"/home/dev/app-feature\\\", \\\"branch\\\": \\\"feature/new-ui\\\"}\\n\\\
                 ```\\n\\n\\\
                 Create worktree for code review:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"~/project\\\", \\\"worktree_path\\\": \\\"~/project-review\\\", \\\"branch\\\": \\\"pr/456\\\"}\\n\\\
                 ```\\n\\n\\\
                 TYPICAL WORKFLOW:\\n\\\
                 1. You're working on a feature in your main repo\\n\\\
                 2. Urgent bug needs fixing on production branch\\n\\\
                 3. Create worktree for the hotfix branch\\n\\\
                 4. Fix the bug in the worktree\\n\\\
                 5. Commit and push from the worktree\\n\\\
                 6. Return to your main repo, feature work untouched\\n\\\
                 7. Remove the worktree when done\\n\\n\\\
                 NOTES:\\n\\\
                 - The branch must already exist in the repository\\n\\\
                 - Use 'new_branch' parameter to create a new branch (see branch scenario)\\n\\\
                 - Worktree directories can be anywhere on your filesystem\\n\\\
                 - Multiple worktrees can exist for the same repository\\n\\\
                 - Each branch can only be checked out in one worktree at a time"
            ),
        },
    ]
}

fn prompt_branch() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("Can I create a new branch when adding a worktree?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Yes! Use the 'new_branch' parameter to create a new branch simultaneously with the worktree. \\
                 This is useful when starting fresh work that doesn't have an existing branch yet.\\n\\n\\\
                 THE NEW_BRANCH PARAMETER:\\n\\n\\\
                 When new_branch is true:\\n\\\
                 - Creates a new branch with the specified name\\n\\\
                 - Creates the worktree directory\\n\\\
                 - Checks out the new branch in the worktree\\n\\\
                 - The new branch starts from your current HEAD\\n\\n\\\
                 When new_branch is false or omitted:\\n\\\
                 - Uses an existing branch\\n\\\
                 - Branch must already exist in the repository\\n\\\
                 - Creates worktree and checks out that branch\\n\\n\\\
                 CREATING A NEW BRANCH:\\n\\n\\\
                 Start a new feature from scratch:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"worktree_path\\\": \\\"./repo-experiment\\\", \\\"branch\\\": \\\"experiment\\\", \\\"new_branch\\\": true}\\n\\\
                 ```\\n\\n\\\
                 What this does:\\n\\\
                 1. Creates new branch named 'experiment'\\n\\\
                 2. Creates worktree directory at ./repo-experiment\\n\\\
                 3. Checks out the new branch in that worktree\\n\\\
                 4. Ready to start working immediately\\n\\n\\\
                 Create new feature branch:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/project\\\", \\\"worktree_path\\\": \\\"/project-new-feature\\\", \\\"branch\\\": \\\"feature/dashboard\\\", \\\"new_branch\\\": true}\\n\\\
                 ```\\n\\n\\\
                 Create new bugfix branch:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"~/app\\\", \\\"worktree_path\\\": \\\"~/app-bugfix\\\", \\\"branch\\\": \\\"bugfix/login-error\\\", \\\"new_branch\\\": true}\\n\\\
                 ```\\n\\n\\\
                 USING AN EXISTING BRANCH:\\n\\n\\\
                 Checkout existing develop branch:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"worktree_path\\\": \\\"./repo-develop\\\", \\\"branch\\\": \\\"develop\\\"}\\n\\\
                 ```\\n\\n\\\
                 Checkout existing release branch:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/app\\\", \\\"worktree_path\\\": \\\"/app-release\\\", \\\"branch\\\": \\\"release/v2.0\\\"}\\n\\\
                 ```\\n\\n\\\
                 Checkout someone else's feature branch:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"~/project\\\", \\\"worktree_path\\\": \\\"~/project-review\\\", \\\"branch\\\": \\\"feature/auth-refactor\\\"}\\n\\\
                 ```\\n\\n\\\
                 WHEN TO USE EACH APPROACH:\\n\\n\\\
                 Use new_branch=true when:\\n\\\
                 - Starting a new feature from scratch\\n\\\
                 - Creating an experimental branch\\n\\\
                 - Beginning work that doesn't have a branch yet\\n\\\
                 - You want the branch to start from current HEAD\\n\\n\\\
                 Use existing branch (new_branch=false or omitted) when:\\n\\\
                 - Continuing work on an existing feature\\n\\\
                 - Reviewing someone else's code\\n\\\
                 - Working on a shared branch\\n\\\
                 - Checking out a release or stable branch\\n\\n\\\
                 ERROR CONDITIONS:\\n\\n\\\
                 If new_branch=true but branch already exists:\\n\\\
                 - Git will return an error\\n\\\
                 - Use a different branch name\\n\\\
                 - Or omit new_branch to use the existing branch\\n\\n\\\
                 If new_branch=false but branch doesn't exist:\\n\\\
                 - Git will return an error\\n\\\
                 - Create the branch first\\n\\\
                 - Or set new_branch=true\\n\\n\\\
                 PRACTICAL WORKFLOW EXAMPLES:\\n\\n\\\
                 Starting new feature development:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./main-repo\\\", \\\"worktree_path\\\": \\\"./feature-work\\\", \\\"branch\\\": \\\"feature/new-api\\\", \\\"new_branch\\\": true}\\n\\\
                 ```\\n\\\
                 Result: New branch created, ready for fresh development\\n\\n\\\
                 Reviewing a pull request:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./main-repo\\\", \\\"worktree_path\\\": \\\"./pr-review\\\", \\\"branch\\\": \\\"feature/submitted-pr\\\"}\\n\\\
                 ```\\n\\\
                 Result: Existing PR branch checked out for review\\n\\n\\\
                 Experimenting with different approaches:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"worktree_path\\\": \\\"./experiment-1\\\", \\\"branch\\\": \\\"experiment/approach-a\\\", \\\"new_branch\\\": true}\\n\\\
                 ```\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"./repo\\\", \\\"worktree_path\\\": \\\"./experiment-2\\\", \\\"branch\\\": \\\"experiment/approach-b\\\", \\\"new_branch\\\": true}\\n\\\
                 ```\\n\\\
                 Result: Two new branches for parallel experimentation\\n\\n\\\
                 NOTES:\\n\\\
                 - The new_branch parameter is optional (defaults to false)\\n\\\
                 - Branch names follow Git conventions (no spaces, special chars)\\n\\\
                 - New branches start from the current HEAD of the repository\\n\\\
                 - You can have multiple worktrees with different branches\\n\\\
                 - Each branch can only be checked out in one worktree at a time"
            ),
        },
    ]
}
