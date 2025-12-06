//! Prompt messages for git_branch_create tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitBranchCreatePromptArgs;

/// Prompt provider for git_branch_create tool
///
/// This is the ONLY way to provide prompts for git_branch_create - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct BranchCreatePrompts;

impl PromptProvider for BranchCreatePrompts {
    type PromptArgs = GitBranchCreatePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("from_commit") => prompt_from_commit(),
            Some("tracking") => prompt_tracking(),
            _ => prompt_feature(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (feature, from_commit, tracking)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO CREATE GIT BRANCHES
// ============================================================================

/// Creating feature branches
fn prompt_feature() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create feature branches for development work?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_branch_create to create feature branches for new functionality, bug fixes, and experimental work.\n\n\
                 CREATING FEATURE BRANCHES:\n\n\
                 1. Simple feature branch:\n\
                    git_branch_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/user-auth\"\n\
                    })\n\
                    Creates branch from current HEAD, stays on current branch\n\n\
                 2. Create and checkout:\n\
                    git_branch_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/api-v2\",\n\
                        \"checkout\": true\n\
                    })\n\
                    Creates branch and switches to it immediately\n\
                    Now on feature/api-v2 branch, ready to work\n\n\
                 3. Bugfix branch:\n\
                    git_branch_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"fix/login-error\",\n\
                        \"checkout\": true\n\
                    })\n\
                    Creates fix/ branch for bug fixes\n\
                    checkout: true switches to it automatically\n\n\
                 4. Experiment branch:\n\
                    git_branch_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"experiment/new-algorithm\"\n\
                    })\n\
                    Creates experimental branch without switching\n\
                    Can continue work on current branch\n\n\
                 PARAMETERS:\n\
                 - path (required): Repository directory path\n\
                 - name (required): Branch name (use prefix like feature/, fix/)\n\
                 - checkout (optional, default: false): Switch to branch after creation\n\
                 - start_point (optional): Create from specific commit/branch/tag (defaults to HEAD)\n\
                 - track (optional): Set up tracking relationship with remote branch\n\n\
                 WHEN TO USE checkout: true:\n\
                 - When you want to start working on the branch immediately\n\
                 - Creating a feature branch and switching to it\n\
                 - Starting a bugfix that needs immediate attention\n\
                 - Beginning work on a new task\n\n\
                 WHEN TO USE checkout: false (or omit):\n\
                 - Creating branch for future work\n\
                 - Creating multiple branches at once\n\
                 - Setting up branch structure without switching\n\
                 - Staying on current branch to finish current work\n\n\
                 BRANCH NAMING:\n\
                 Use category prefixes for clarity:\n\
                 - feature/*: New functionality (feature/user-auth, feature/payment-gateway)\n\
                 - fix/*: Bug fixes (fix/login-error, fix/memory-leak)\n\
                 - hotfix/*: Urgent production fixes (hotfix/security-patch)\n\
                 - release/*: Release preparation (release/v2.0.0)\n\
                 - docs/*: Documentation (docs/api-guide)\n\
                 - refactor/*: Code restructuring (refactor/database-layer)\n\n\
                 Naming rules:\n\
                 - Lowercase with hyphens: feature/user-login (not Feature_User_Login)\n\
                 - Descriptive but concise: fix/null-pointer (not fix/bug)\n\
                 - Include issue number if applicable: fix/gh-123-validation-error\n\n\
                 Good examples: feature/oauth2-integration, fix/issue-456-memory-leak\n\
                 Bad examples: my-branch, test, Feature_New_Thing\n\n\
                 COMMON WORKFLOW:\n\
                 1. Create and checkout branch:\n\
                    git_branch_create({\"path\": \"/project\", \"name\": \"feature/new-feature\", \"checkout\": true})\n\
                 2. Make changes and commit:\n\
                    git_add({\"path\": \"/project\", \"files\": [\"src/file.rs\"]})\n\
                    git_commit({\"path\": \"/project\", \"message\": \"Add new feature\"})\n\
                 3. Push to remote:\n\
                    git_push({\"path\": \"/project\", \"set_upstream\": true})\n\n\
                 ERROR HANDLING:\n\
                 - If branch exists: Use git_branch_list to check existing branches first\n\
                 - If start_point invalid: Verify commit/branch/tag exists\n\
                 - If path invalid: Check repository path is correct\n\
                 - If checkout fails: Ensure working directory is clean (commit or stash changes)\n\n\
                 RETURNS:\n\
                 {\n\
                     \"success\": true,\n\
                     \"branch\": \"feature/user-auth\",\n\
                     \"checked_out\": false,\n\
                     \"message\": \"Branch created successfully\"\n\
                 }\n\n\
                 TIP: Use checkout: true for convenience when starting new work immediately. This saves an extra git_checkout call and gets you into the branch faster!",
            ),
        },
    ]
}

/// Branches from specific points
fn prompt_from_commit() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create branches from specific commits, tags, or other branches?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use the start_point parameter to create branches from specific points in Git history.\n\n\
                 BRANCHES FROM SPECIFIC POINTS:\n\n\
                 1. From specific commit:\n\
                    git_branch_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"fix/regression\",\n\
                        \"start_point\": \"abc1234\"\n\
                    })\n\
                    Creates branch starting at commit abc1234\n\
                    Useful for fixing bugs discovered at specific points\n\n\
                 2. From tag:\n\
                    git_branch_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"hotfix/v1.0.1\",\n\
                        \"start_point\": \"v1.0.0\"\n\
                    })\n\
                    Creates hotfix branch from release tag v1.0.0\n\
                    Perfect for patching production releases\n\n\
                 3. From another branch:\n\
                    git_branch_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/derived\",\n\
                        \"start_point\": \"develop\"\n\
                    })\n\
                    Creates feature branch from develop branch\n\
                    Ensures you start from the right base\n\n\
                 4. From remote branch:\n\
                    git_branch_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"local-feature\",\n\
                        \"start_point\": \"origin/feature/remote-work\"\n\
                    })\n\
                    Creates local branch from remote branch\n\
                    Useful for working on colleague's code\n\n\
                 WHEN TO USE start_point:\n\n\
                 Hotfix from release tag:\n\
                 git_branch_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"hotfix/critical-bug\",\n\
                     \"start_point\": \"v2.1.0\",\n\
                     \"checkout\": true\n\
                 })\n\
                 - Critical production bug needs immediate fix\n\
                 - Must be based on exact release version\n\
                 - Tag ensures consistency with deployed code\n\n\
                 START_POINT FORMATS:\n\
                 - Commit SHA: \"a1b2c3d\" or full \"a1b2c3d4e5f6...\"\n\
                 - Tag: \"v1.2.3\" or \"release-2023-12\"\n\
                 - Local branch: \"develop\" or \"main\"\n\
                 - Remote branch: \"origin/feature/remote\"\n\n\
                 HOTFIX WORKFLOW EXAMPLE:\n\
                 git_branch_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"hotfix/security-issue\",\n\
                     \"start_point\": \"v1.5.2\",\n\
                     \"checkout\": true\n\
                 })\n\
                 // Fix bug, commit, merge to main and develop\n\n\
                 TROUBLESHOOTING:\n\
                 - Invalid start_point: Use git_log, git_tag, or git_branch_list to verify it exists\n\
                 - Fetch first if referencing remote: git_fetch({\"path\": \"/project\"})\n\n\
                 RETURNS:\n\
                 {\n\
                     \"success\": true,\n\
                     \"branch\": \"hotfix/v1.0.1\",\n\
                     \"start_point\": \"v1.0.0\",\n\
                     \"checked_out\": false,\n\
                     \"message\": \"Branch created from v1.0.0\"\n\
                 }\n\n\
                 TIP: Always use start_point for hotfixes from tags and features from develop. This ensures you're starting from the correct base and documents your intent!",
            ),
        },
    ]
}

/// Remote tracking branches
fn prompt_tracking() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I set up tracking relationships between local and remote branches?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Set up tracking relationships so your local branch knows which remote branch to push to and pull from.\n\n\
                 TRACKING BRANCHES:\n\n\
                 1. Track remote branch:\n\
                    git_branch_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/x\",\n\
                        \"track\": \"origin/feature/x\"\n\
                    })\n\
                    Creates local branch that tracks remote branch\n\
                    git_pull and git_push know which remote branch to use\n\n\
                 2. Create with upstream:\n\
                    git_branch_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"my-feature\",\n\
                        \"checkout\": true\n\
                    })\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"set_upstream\": true\n\
                    })\n\
                    First push sets up tracking automatically\n\
                    Now tracks origin/my-feature\n\n\
                 3. Work on colleague's branch:\n\
                    git_fetch({\"path\": \"/project\"})\n\
                    git_branch_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"review/colleague-feature\",\n\
                        \"start_point\": \"origin/colleague-feature\",\n\
                        \"track\": \"origin/colleague-feature\"\n\
                    })\n\
                    Local branch tracks colleague's remote branch\n\
                    Pull to get their updates, push to contribute\n\n\
                 TRACKING BENEFITS:\n\
                 - git_pull knows which branch to pull from\n\
                 - git_push knows where to push to\n\
                 - git_status shows ahead/behind information\n\
                 - Simplifies collaboration workflow\n\
                 - Prevents mistakes\n\n\
                 SETTING UP TRACKING:\n\n\
                 METHOD 1: During branch creation\n\
                 git_branch_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"feature/api-v2\",\n\
                     \"start_point\": \"origin/feature/api-v2\",\n\
                     \"track\": \"origin/feature/api-v2\",\n\
                     \"checkout\": true\n\
                 })\n\
                 Creates local branch tracking remote immediately\n\n\
                 METHOD 2: During first push\n\
                 git_branch_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"feature/new-work\",\n\
                     \"checkout\": true\n\
                 })\n\
                 // Make commits...\n\
                 git_push({\n\
                     \"path\": \"/project\",\n\
                     \"set_upstream\": true\n\
                 })\n\
                 First push with set_upstream establishes tracking\n\n\
                 COMMON PATTERNS:\n\n\
                 PATTERN 1: New feature branch\n\
                 Step 1: Create local branch\n\
                 git_branch_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"feature/payment-system\",\n\
                     \"checkout\": true\n\
                 })\n\n\
                 Step 2: Make initial commit\n\
                 git_add({\"path\": \"/project\", \"files\": [\".\"]})\n\
                 git_commit({\n\
                     \"path\": \"/project\",\n\
                     \"message\": \"Initial payment system structure\"\n\
                 })\n\n\
                 Step 3: Push and set upstream\n\
                 git_push({\n\
                     \"path\": \"/project\",\n\
                     \"set_upstream\": true\n\
                 })\n\
                 Now tracks origin/feature/payment-system automatically\n\n\
                 Step 4: Future pushes are simple\n\
                 git_push({\"path\": \"/project\"})  // No need to specify branch!\n\n\
                 TROUBLESHOOTING:\n\
                 - Remote branch doesn't exist: Fetch first with git_fetch({\"path\": \"/project\"})\n\
                 - Wrong remote name: Check with git_remote_list({\"path\": \"/project\"})\n\
                 - Not tracking after creation: Set upstream on next push with git_push({\"set_upstream\": true})\n\n\
                 RETURNS WITH TRACKING:\n\
                 {\n\
                     \"success\": true,\n\
                     \"branch\": \"feature/x\",\n\
                     \"tracking\": \"origin/feature/x\",\n\
                     \"checked_out\": false,\n\
                     \"message\": \"Branch created and set to track origin/feature/x\"\n\
                 }\n\n\
                 TIP: Set up tracking immediately when working with remote branches. It simplifies your workflow and prevents mistakes by ensuring git_push and git_pull always use the correct remote branch!",
            ),
        },
    ]
}

