//! Prompt messages for git_push tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitPushPromptArgs;

/// Prompt provider for git_push tool
///
/// This is the ONLY way to provide prompts for git_push - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct PushPrompts;

impl PromptProvider for PushPrompts {
    type PromptArgs = GitPushPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("upstream") => prompt_upstream(),
            Some("force") => prompt_force(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, upstream, force)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO PUSH TO REMOTES
// ============================================================================

/// Basic pushing operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I push commits to a remote repository using git_push?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_push tool uploads local commits to a remote repository. Here's how to use it for basic pushing:\n\n\
                 PUSHING COMMITS:\n\n\
                 1. Push current branch:\n\
                    git_push({\n\
                        \"path\": \"/project\"\n\
                    })\n\
                    // Pushes to tracking remote\n\n\
                 2. Push to specific remote:\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"remote\": \"origin\"\n\
                    })\n\n\
                 3. Push specific branch:\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"remote\": \"origin\",\n\
                        \"refspecs\": [\"feature/x\"]\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"remote\": \"origin\",\n\
                   \"pushed_refs\": [\n\
                     {\n\
                       \"ref_name\": \"refs/heads/main\",\n\
                       \"old_oid\": \"abc123...\",\n\
                       \"new_oid\": \"def456...\"\n\
                     }\n\
                   ],\n\
                   \"updates\": 3\n\
                 }\n\n\
                 PUSH REQUIREMENTS:\n\
                 - Local commits to push\n\
                 - Write access to remote\n\
                 - Up to date with remote (or force)\n\
                 - Proper authentication (SSH keys or credentials)\n\n\
                 PARAMETERS:\n\
                 - path (required): Repository path\n\
                 - remote (optional): Remote name (default: \"origin\")\n\
                 - refspecs (optional): Branches/tags to push (empty = current branch)\n\
                 - force (optional): Force push - DANGEROUS! (default: false)\n\
                 - tags (optional): Also push all tags (default: false)\n\
                 - timeout_secs (optional): Network timeout (default: 300)\n\n\
                 COMMON PATTERNS:\n\n\
                 Push after commit:\n\
                 git_commit({\"path\": \"/project\", \"message\": \"Fix bug\"})\n\
                 git_push({\"path\": \"/project\"})\n\n\
                 Push multiple branches:\n\
                 git_push({\n\
                     \"path\": \"/project\",\n\
                     \"refspecs\": [\"main\", \"develop\"]\n\
                 })\n\n\
                 Push to different remote:\n\
                 git_push({\n\
                     \"path\": \"/project\",\n\
                     \"remote\": \"backup\"\n\
                 })\n\n\
                 AUTHENTICATION:\n\
                 - SSH: Requires SSH keys configured (~/.ssh/id_rsa)\n\
                 - HTTPS: Requires credential helper or PAT\n\
                 - Test SSH: ssh -T git@github.com\n\n\
                 COMMON ERRORS:\n\n\
                 \"rejected - non-fast-forward\":\n\
                 - Remote has commits you don't have\n\
                 - Solution: git_pull first, then push\n\
                 - Or use force (DANGEROUS on shared branches)\n\n\
                 \"authentication failed\":\n\
                 - SSH keys not configured\n\
                 - Credentials expired\n\
                 - Solution: Set up SSH keys or update credentials\n\n\
                 \"protected branch\":\n\
                 - Branch requires pull request\n\
                 - Solution: Use PR workflow instead of direct push\n\n\
                 AFTER PUSHING:\n\
                 - Remote branch is updated with your commits\n\
                 - Local tracking info synced\n\
                 - Commits visible to collaborators\n\
                 - CI/CD pipelines may trigger\n\n\
                 BEST PRACTICES:\n\
                 - Always pull before pushing to avoid conflicts\n\
                 - Push frequently to back up work\n\
                 - Use meaningful commit messages\n\
                 - Never force push to shared branches",
            ),
        },
    ]
}

/// Setting upstream tracking
fn prompt_upstream() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I set up upstream tracking when pushing new branches?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_push to set up upstream tracking for new branches. This establishes the relationship between local and remote branches.\n\n\
                 SETTING UPSTREAM:\n\n\
                 1. First push with upstream:\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"refspecs\": [\"feature/new-auth\"]\n\
                    })\n\
                    // Creates remote branch and sets tracking\n\n\
                 2. New branch first push:\n\
                    git_checkout({\n\
                        \"path\": \"/project\",\n\
                        \"branch\": \"feature/x\",\n\
                        \"create\": true\n\
                    })\n\
                    // Make commits\n\
                    git_commit({\"path\": \"/project\", \"message\": \"Add feature\"})\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"refspecs\": [\"feature/x\"]\n\
                    })\n\
                    // Creates origin/feature/x and sets tracking\n\n\
                 3. Push to different remote name:\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"remote\": \"upstream\",\n\
                        \"refspecs\": [\"main\"]\n\
                    })\n\n\
                 UPSTREAM BENEFITS:\n\
                 - git_push works without specifying branch\n\
                 - git_pull knows where to pull from\n\
                 - git_status shows ahead/behind counts\n\
                 - Simplified workflow for team collaboration\n\
                 - Remote tracking info stored in .git/config\n\n\
                 TYPICAL WORKFLOW:\n\n\
                 1. Create local branch:\n\
                    git_branch_create({\"path\": \"/project\", \"name\": \"feature/api\"})\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature/api\"})\n\n\
                 2. Make some commits:\n\
                    git_commit({\"path\": \"/project\", \"message\": \"Add API endpoint\"})\n\
                    git_commit({\"path\": \"/project\", \"message\": \"Add tests\"})\n\n\
                 3. First push (sets upstream):\n\
                    git_push({\"path\": \"/project\", \"refspecs\": [\"feature/api\"]})\n\n\
                 4. Subsequent pushes (upstream already set):\n\
                    git_push({\"path\": \"/project\"})\n\
                    // No refspecs needed - uses tracking info\n\n\
                 CHECKING UPSTREAM:\n\
                 Use git_status to see tracking info:\n\
                 git_status({\"path\": \"/project\"})\n\
                 Returns:\n\
                 {\n\
                   \"branch\": \"feature/api\",\n\
                   \"upstream\": \"origin/feature/api\",\n\
                   \"ahead\": 2,\n\
                   \"behind\": 0\n\
                 }\n\n\
                 MULTIPLE REMOTES:\n\
                 Useful for fork workflows:\n\n\
                 1. Add upstream remote:\n\
                    git_remote_add({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"upstream\",\n\
                        \"url\": \"https://github.com/original/repo.git\"\n\
                    })\n\n\
                 2. Push to your fork:\n\
                    git_push({\"path\": \"/project\", \"remote\": \"origin\"})\n\n\
                 3. Push to upstream (if you have permission):\n\
                    git_push({\"path\": \"/project\", \"remote\": \"upstream\"})\n\n\
                 BRANCH NAMING:\n\
                 Good branch names for tracking:\n\
                 - feature/user-authentication\n\
                 - bugfix/login-error\n\
                 - hotfix/security-patch\n\
                 - release/v1.2.0\n\n\
                 UPSTREAM CONFIGURATION:\n\
                 Stored in .git/config:\n\
                 [branch \"feature/api\"]\n\
                     remote = origin\n\
                     merge = refs/heads/feature/api\n\n\
                 This happens automatically on first push with branch name.\n\n\
                 BEST PRACTICES:\n\
                 - Always push new branches with explicit refspec first time\n\
                 - Use descriptive branch names\n\
                 - Set upstream immediately after creating branch\n\
                 - Check git_status to verify upstream is set\n\
                 - Use same branch name locally and remotely",
            ),
        },
    ]
}

/// Force push scenarios and safety
fn prompt_force() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "When should I use force push and what are the dangers?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Force push is DANGEROUS and should be used with extreme caution. It overwrites remote history.\n\n\
                 FORCE PUSH:\n\n\
                 1. Force push (DANGEROUS):\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"force\": true\n\
                    })\n\
                    // Overwrites remote history!\n\n\
                 2. Force push specific branch:\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"refspecs\": [\"feature/my-branch\"],\n\
                        \"force\": true\n\
                    })\n\n\
                 WHEN FORCE IS NEEDED:\n\n\
                 After rebase:\n\
                 git_rebase({\"path\": \"/project\", \"upstream\": \"origin/main\"})\n\
                 git_push({\"path\": \"/project\", \"force\": true})\n\
                 - History rewritten locally\n\
                 - Must force push to update remote\n\n\
                 After amend:\n\
                 git_commit({\"path\": \"/project\", \"amend\": true})\n\
                 git_push({\"path\": \"/project\", \"force\": true})\n\
                 - Last commit changed\n\
                 - Remote still has old version\n\n\
                 After history rewrite:\n\
                 git_reset({\"path\": \"/project\", \"mode\": \"hard\", \"commit\": \"HEAD~3\"})\n\
                 git_push({\"path\": \"/project\", \"force\": true})\n\
                 - Removed commits from history\n\
                 - Remote needs to match\n\n\
                 Resetting branch:\n\
                 git_reset({\"path\": \"/project\", \"mode\": \"hard\", \"commit\": \"abc123\"})\n\
                 git_push({\"path\": \"/project\", \"force\": true})\n\
                 - Moving branch pointer backward\n\
                 - Discarding commits\n\n\
                 FORCE RISKS:\n\n\
                 Overwrites others' work:\n\
                 - Teammate pushed commits\n\
                 - You force push\n\
                 - Their commits are LOST from remote\n\
                 - They get conflicts on next pull\n\n\
                 Loses remote commits:\n\
                 - Remote has commits you don't\n\
                 - Force push replaces them\n\
                 - Data loss - may be permanent\n\
                 - Reflog only helps temporarily\n\n\
                 Breaks collaborators' repos:\n\
                 - Their local branches become invalid\n\
                 - They must reset to match remote\n\
                 - Work in progress conflicts\n\
                 - Wastes team time fixing\n\n\
                 CI/CD issues:\n\
                 - Pipeline tracking commits\n\
                 - Force push changes history\n\
                 - Deployments may fail\n\
                 - Rollback becomes difficult\n\n\
                 SAFE PRACTICES:\n\n\
                 1. Only force push YOUR OWN branches:\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"refspecs\": [\"feature/my-personal-work\"],\n\
                        \"force\": true\n\
                    })\n\
                    // OK - it's your feature branch\n\n\
                 2. Never force push shared branches:\n\
                    NEVER: git_push({\"path\": \"/project\", \"refspecs\": [\"main\"], \"force\": true})\n\
                    NEVER: git_push({\"path\": \"/project\", \"refspecs\": [\"develop\"], \"force\": true})\n\
                    NEVER: git_push({\"path\": \"/project\", \"refspecs\": [\"master\"], \"force\": true})\n\n\
                 3. Communicate with team:\n\
                    - Warn teammates before force push\n\
                    - Ensure no one else working on branch\n\
                    - Use team chat to coordinate\n\n\
                 4. Use branch protection:\n\
                    - Protect main/master/release branches\n\
                    - Prevent force push via GitHub/GitLab settings\n\
                    - Require pull requests\n\n\
                 SAFER ALTERNATIVES:\n\n\
                 Instead of force push, consider:\n\n\
                 1. Create new branch:\n\
                    git_branch_create({\"path\": \"/project\", \"name\": \"feature/v2\"})\n\
                    git_push({\"path\": \"/project\", \"refspecs\": [\"feature/v2\"]})\n\
                    // Keeps old branch intact\n\n\
                 2. Merge instead of rebase:\n\
                    git_merge({\"path\": \"/project\", \"branch\": \"origin/main\"})\n\
                    git_push({\"path\": \"/project\"})\n\
                    // No force needed\n\n\
                 3. Revert commits:\n\
                    git_revert({\"path\": \"/project\", \"commit\": \"abc123\"})\n\
                    git_push({\"path\": \"/project\"})\n\
                    // Adds new commit, no history rewrite\n\n\
                 FORCE PUSH CHECKLIST:\n\n\
                 Before force pushing, verify:\n\
                 [ ] This is MY personal branch, not shared\n\
                 [ ] No teammates are working on this branch\n\
                 [ ] I've communicated with the team\n\
                 [ ] This is not main/master/production\n\
                 [ ] I understand I may lose remote commits\n\
                 [ ] I have backups of important work\n\
                 [ ] Branch protection allows force push\n\n\
                 REMEMBER:\n\
                 - Force push is a LAST RESORT\n\
                 - Prefer alternatives when possible\n\
                 - NEVER on shared/protected branches\n\
                 - Always warn your team\n\
                 - Use branch protection to prevent accidents",
            ),
        },
    ]
}

