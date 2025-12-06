//! Prompt messages for github_create_branch tool

use super::prompt_args::CreateBranchPromptArgs;
use crate::tool::PromptProvider;
use rmcp::model::{PromptArgument, PromptMessage, PromptMessageContent, PromptMessageRole};

/// Prompt provider for create_branch tool
///
/// This is the ONLY way to provide prompts for create_branch - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct CreateBranchPrompts;

impl PromptProvider for CreateBranchPrompts {
    type PromptArgs = CreateBranchPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("from_ref") => prompt_from_ref(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some(
                "Scenario to show (basic, from_ref)".to_string(),
            ),
            required: Some(false),
        }]
    }
}

/// Basic branch creation from commit SHA
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create branches on GitHub using github_create_branch?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_create_branch tool creates a new branch in a GitHub repository directly via the GitHub API. No local clone needed.\n\n\
                 BASIC BRANCH CREATION:\n\
                 1. Create from default branch:\n\
                    github_create_branch({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"branch_name\": \"feature/new-feature\",\n\
                      \"sha\": \"abc1234567890abcdef1234567890abcdef1234\"\n\
                    })\n\n\
                 2. Feature branch:\n\
                    github_create_branch({\n\
                      \"owner\": \"company\",\n\
                      \"repo\": \"backend\",\n\
                      \"branch_name\": \"feature/user-auth\",\n\
                      \"sha\": \"def4567890abcdef1234567890abcdef12345678\"\n\
                    })\n\n\
                 3. Bug fix branch:\n\
                    github_create_branch({\n\
                      \"owner\": \"team\",\n\
                      \"repo\": \"frontend\",\n\
                      \"branch_name\": \"fix/login-error\",\n\
                      \"sha\": \"1234567890abcdef1234567890abcdef12345678\"\n\
                    })\n\n\
                 4. Hotfix branch:\n\
                    github_create_branch({\n\
                      \"owner\": \"org\",\n\
                      \"repo\": \"api\",\n\
                      \"branch_name\": \"hotfix/security-patch\",\n\
                      \"sha\": \"9876543210fedcba9876543210fedcba98765432\"\n\
                    })\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - branch_name (required): Name for the new branch\n\
                 - sha (required): 40-character commit SHA to branch from\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"branch_name\": \"feature/new-feature\",\n\
                   \"sha\": \"abc1234...\",\n\
                   \"message\": \"Branch created successfully\"\n\
                 }\n\n\
                 KEY ADVANTAGES:\n\
                 - Creates branch directly on GitHub (no local clone required)\n\
                 - Fast and lightweight operation\n\
                 - Useful for automation and CI/CD pipelines\n\
                 - Works with any repository you have write access to\n\
                 - Can create branches from any commit in history\n\n\
                 BRANCH NAMING CONVENTIONS:\n\
                 - feature/* - New features (feature/user-profile)\n\
                 - fix/* - Bug fixes (fix/memory-leak)\n\
                 - hotfix/* - Urgent production fixes (hotfix/critical-bug)\n\
                 - release/* - Release branches (release/v2.0)\n\
                 - docs/* - Documentation updates (docs/api-guide)\n\
                 - refactor/* - Code refactoring (refactor/cleanup)\n\
                 - test/* - Test additions (test/integration)\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with permissions:\n\
                 - repo scope (for private repositories)\n\
                 - public_repo scope (for public repositories)\n\n\
                 COMMON ERRORS:\n\
                 1. Branch already exists:\n\
                    Error: \"Reference already exists\"\n\
                    Solution: Use a different branch name or delete existing branch\n\n\
                 2. Invalid SHA:\n\
                    Error: \"Object does not exist\"\n\
                    Solution: Verify the SHA exists in the repository\n\n\
                 3. No write access:\n\
                    Error: \"Resource not accessible by integration\"\n\
                    Solution: Check GITHUB_TOKEN has write permissions\n\n\
                 4. Repository not found:\n\
                    Error: \"Not Found\"\n\
                    Solution: Verify owner/repo names and token has access",
            ),
        },
    ]
}

/// Getting SHA from various refs (tags, branches, commits)
fn prompt_from_ref() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I get the SHA needed for creating a branch from tags, branches, or specific refs?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_create_branch tool requires a commit SHA. Use github_get_commit to resolve branch names, tags, or other refs to SHAs.\n\n\
                 GETTING SHA FROM BRANCH:\n\
                 1. Get SHA from main branch:\n\
                    github_get_commit({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"ref\": \"main\"\n\
                    })\n\
                    // Returns commit object with sha field\n\n\
                 2. Create branch from that SHA:\n\
                    github_create_branch({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"branch_name\": \"feature/new\",\n\
                      \"sha\": \"<sha from step 1>\"\n\
                    })\n\n\
                 GETTING SHA FROM TAG:\n\
                 1. Get SHA from release tag:\n\
                    github_get_commit({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"ref\": \"v1.0.0\"\n\
                    })\n\n\
                 2. Create hotfix branch from tag:\n\
                    github_create_branch({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"branch_name\": \"hotfix/v1.0.1\",\n\
                      \"sha\": \"<sha from tag>\"\n\
                    })\n\n\
                 GETTING SHA FROM ANOTHER BRANCH:\n\
                 1. Get SHA from develop branch:\n\
                    github_get_commit({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"ref\": \"develop\"\n\
                    })\n\n\
                 2. Create feature branch from develop:\n\
                    github_create_branch({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"branch_name\": \"feature/experimental\",\n\
                      \"sha\": \"<sha from develop>\"\n\
                    })\n\n\
                 GETTING SHA FROM SPECIFIC COMMIT:\n\
                 If you already know a partial SHA:\n\
                 1. Get full commit info:\n\
                    github_get_commit({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"ref\": \"abc1234\"  // Partial SHA works\n\
                    })\n\n\
                 2. Use full SHA for branch creation:\n\
                    github_create_branch({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"branch_name\": \"fix/from-commit\",\n\
                      \"sha\": \"<full sha>\"\n\
                    })\n\n\
                 USING LIST_COMMITS TO FIND SHA:\n\
                 1. List recent commits:\n\
                    github_list_commits({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"sha\": \"main\",  // Branch to list from\n\
                      \"per_page\": 10\n\
                    })\n\
                    // Returns array of commits with sha fields\n\n\
                 2. Pick a commit and create branch:\n\
                    github_create_branch({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"branch_name\": \"feature/from-history\",\n\
                      \"sha\": \"<chosen sha from list>\"\n\
                    })\n\n\
                 COMPLETE WORKFLOW - BRANCH FROM TAG:\n\
                 // Step 1: Get SHA from production tag\n\
                 github_get_commit({\n\
                   \"owner\": \"company\",\n\
                   \"repo\": \"api\",\n\
                   \"ref\": \"v2.3.4\"\n\
                 })\n\
                 // Response: { \"sha\": \"abc123...\", ... }\n\n\
                 // Step 2: Create hotfix branch from that tag\n\
                 github_create_branch({\n\
                   \"owner\": \"company\",\n\
                   \"repo\": \"api\",\n\
                   \"branch_name\": \"hotfix/v2.3.5\",\n\
                   \"sha\": \"abc123...\"\n\
                 })\n\n\
                 REF FORMATS ACCEPTED BY github_get_commit:\n\
                 - Branch names: \"main\", \"develop\", \"feature/xyz\"\n\
                 - Tag names: \"v1.0.0\", \"release-2024\"\n\
                 - Full SHA: \"abc1234567890abcdef1234567890abcdef1234\"\n\
                 - Short SHA: \"abc1234\" (7+ characters)\n\
                 - HEAD: \"HEAD\" (current default branch)\n\n\
                 BEST PRACTICES:\n\
                 - Always verify the ref exists before creating branch\n\
                 - Use full SHAs (40 characters) for branch creation\n\
                 - Document which ref you branched from in PR description\n\
                 - For releases, always branch from tagged commits\n\
                 - For hotfixes, branch from production tag\n\
                 - For features, branch from latest main/develop\n\n\
                 ERROR HANDLING:\n\
                 1. If github_get_commit fails with 404:\n\
                    - Ref doesn't exist in repository\n\
                    - Check branch/tag name spelling\n\
                    - Verify repository access\n\n\
                 2. If ref is ambiguous (branch and tag same name):\n\
                    - Use refs/heads/name for branches\n\
                    - Use refs/tags/name for tags\n\
                    - Example: \"refs/heads/v1.0\" vs \"refs/tags/v1.0\"",
            ),
        },
    ]
}
