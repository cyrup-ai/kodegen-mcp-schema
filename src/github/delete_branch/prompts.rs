//! Prompt messages for github_delete_branch tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::DeleteBranchPromptArgs;

/// Prompt provider for delete_branch tool
///
/// This is the ONLY way to provide prompts for delete_branch - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct DeleteBranchPrompts;

impl PromptProvider for DeleteBranchPrompts {
    type PromptArgs = DeleteBranchPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("safety") => prompt_safety(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, safety)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic branch deletion
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I delete branches from GitHub repositories?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_delete_branch tool removes a branch reference from a GitHub repository via the API.\n\n\
                 BASIC DELETION:\n\
                 1. Delete merged feature branch:\n\
                    github_delete_branch({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"branch_name\": \"feature/new-api\"\n\
                    })\n\n\
                 2. Delete stale branch:\n\
                    github_delete_branch({\n\
                      \"owner\": \"company\",\n\
                      \"repo\": \"backend\",\n\
                      \"branch_name\": \"fix/old-bug\"\n\
                    })\n\n\
                 3. Clean up after PR:\n\
                    github_delete_branch({\n\
                      \"owner\": \"team\",\n\
                      \"repo\": \"frontend\",\n\
                      \"branch_name\": \"feature/login\"\n\
                    })\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - branch_name (required): Exact name of branch to delete\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"branch_name\": \"feature/new-api\",\n\
                   \"message\": \"Branch deleted successfully\"\n\
                 }\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with:\n\
                 - repo scope (for private repositories)\n\
                 - public_repo scope (for public repositories)\n\n\
                 COMMON ERRORS:\n\
                 1. 404 Not Found: Branch doesn't exist\n\
                    Fix: Verify branch name is correct using github_list_branches\n\n\
                 2. 422 Unprocessable: Branch is protected or default\n\
                    Fix: Cannot delete protected branches via API; remove protection in GitHub UI\n\n\
                 3. 403 Forbidden: Token lacks write access\n\
                    Fix: Generate new token with 'repo' scope\n\n\
                 BASIC WORKFLOW:\n\
                 1. Merge pull request on GitHub\n\
                 2. Delete source branch using github_delete_branch\n\
                 3. Keep repository clean and organized\n\n\
                 KEY POINTS:\n\
                 - Branches are remote references; local copies must be deleted separately\n\
                 - Deletion is immediate and cannot be undone from API\n\
                 - Use github_list_branches to verify branch exists first\n\
                 - Document retention policy in CONTRIBUTING.md",
            ),
        },
    ]
}

/// Safe branch deletion with protection and verification
fn prompt_safety() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I safely delete branches without breaking the repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Safe branch deletion requires verification and understanding protection rules.\n\n\
                 PROTECTED BRANCHES:\n\
                 Cannot delete these via API:\n\
                 - Default branch (usually main/master)\n\
                 - Branches with protection rules enabled\n\
                 - Branches with active status checks\n\n\
                 Error response (422 Unprocessable):\n\
                 {\n\
                   \"success\": false,\n\
                   \"message\": \"Reference cannot be deleted\"\n\
                 }\n\n\
                 Solution: Remove protection rules in GitHub UI (Settings > Branches > Branch protection rules)\n\n\
                 VERIFICATION BEFORE DELETION:\n\
                 Always verify branch exists and merged before deleting:\n\
                 1. List all branches:\n\
                    github_list_branches({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\"\n\
                    })\n\n\
                 2. Check branch details:\n\
                    github_get_pull_request_status({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"branch\": \"feature/xyz\"\n\
                    })\n\n\
                 3. Only delete if merged:\n\
                    github_delete_branch({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"branch_name\": \"feature/xyz\"\n\
                    })\n\n\
                 BRANCHES NEVER TO DELETE:\n\
                 - main (primary production branch)\n\
                 - master (legacy default branch)\n\
                 - develop (integration branch in Git Flow)\n\
                 - release/* (unless specifically archiving)\n\
                 - Any branch with active pull requests\n\n\
                 RECOVERY AFTER ACCIDENTAL DELETION:\n\
                 - Branch references can be recovered if commits still exist\n\
                 - Use github_create_branch to recreate from same commit SHA\n\
                 - GitHub retains commit history (GitHub Support can recover)\n\
                 - Best protection: Require PR reviews and protection rules\n\n\
                 BEST PRACTICES:\n\
                 1. Always verify with github_list_branches first\n\
                 2. Confirm branch is merged before deletion\n\
                 3. Use branch protection for critical branches\n\
                 4. Document retention policy for release branches\n\
                 5. Delete only feature/hotfix branches after merge\n\
                 6. Require approval before deleting any branch\n\
                 7. Maintain branch naming convention to identify protection needs\n\
                 8. Regular cleanup after sprint/release cycles\n\n\
                 PERMISSION REQUIREMENTS:\n\
                 - GITHUB_TOKEN must have write access to repository\n\
                 - For organizations: ensure token has access to org\n\
                 - For private repos: token needs 'repo' scope\n\
                 - For public repos: token needs 'public_repo' scope\n\n\
                 CLEANUP WORKFLOW:\n\
                 1. Merge pull request (GitHub UI)\n\
                 2. Delete source branch (GitHub PR option, OR)\n\
                 3. Delete with tool:\n\
                    github_delete_branch({\"owner\": \"...\", \"repo\": \"...\", \"branch_name\": \"...\"})\n\
                 4. Verify deletion:\n\
                    github_list_branches({\"owner\": \"...\", \"repo\": \"...\"})\n\
                 5. Keep repository uncluttered",
            ),
        },
    ]
}
