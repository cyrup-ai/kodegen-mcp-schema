//! Prompt messages for github_get_commit tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetCommitPromptArgs;

/// Prompt provider for get_commit tool
///
/// This is the ONLY way to provide prompts for get_commit - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GetCommitPrompts;

impl PromptProvider for GetCommitPrompts {
    type PromptArgs = GetCommitPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_get_commit to retrieve commit details?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_commit tool retrieves detailed information about a specific commit including changes, statistics, and metadata.\n\n\
                 BASIC USAGE:\n\
                 1. Get commit by SHA:\n\
                    github_get_commit({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"sha\": \"abc123def456789012345678901234567890abcd\"})\n\n\
                 2. Get commit with short SHA:\n\
                    github_get_commit({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"sha\": \"abc123d\"})\n\n\
                 3. Get HEAD commit:\n\
                    github_get_commit({\"owner\": \"actix\", \"repo\": \"actix-web\", \"sha\": \"HEAD\"})\n\n\
                 4. Get commit by branch name:\n\
                    github_get_commit({\"owner\": \"serde-rs\", \"repo\": \"serde\", \"sha\": \"main\"})\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - sha (required): Commit SHA (full/short), branch name, or tag\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo (for private repositories)\n\
                 - public_repo (for public repositories only)\n\n\
                 RESPONSE:\n\
                 Returns JSON with:\n\
                 - success: true/false\n\
                 - owner, repo: Repository identifiers\n\
                 - commit: Object containing:\n\
                   - sha: Full commit SHA\n\
                   - message: Commit message\n\
                   - author: Name, email, date\n\
                   - committer: Name, email, date\n\
                   - parents: Array of parent commit SHAs\n\
                   - stats: additions, deletions, total changes\n\
                   - files: Array of changed files with patches\n\
                   - html_url: Link to commit on GitHub\n\n\
                 COMMON WORKFLOWS:\n\
                 1. Code review analysis:\n\
                    - Get commit details\n\
                    - Analyze changed files\n\
                    - Review diff patches\n\
                    - Provide feedback on changes\n\
                 2. Change tracking:\n\
                    - Retrieve commit for specific feature\n\
                    - Extract file changes\n\
                    - Generate changelog entry\n\
                    - Document breaking changes\n\
                 3. Verification:\n\
                    - Get commit by SHA\n\
                    - Verify author and timestamp\n\
                    - Check GPG signature status\n\
                    - Validate commit in CI pipeline\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Unauthenticated: 60 requests/hour\n\
                 - Check X-RateLimit-Remaining header\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found: Commit SHA doesn't exist in repository\n\
                    Fix: Verify SHA is correct and exists in repo history\n\
                 2. 422 Unprocessable: Invalid SHA format\n\
                    Fix: Use valid 40-character hex SHA or short SHA (7+ chars)\n\
                 3. 403 Forbidden: No access to private repository\n\
                    Fix: Verify GITHUB_TOKEN has repo access\n\n\
                 BEST PRACTICES:\n\
                 - Use full 40-character SHA for accuracy\n\
                 - Short SHAs (7+ chars) work but may be ambiguous\n\
                 - Branch names resolve to latest commit on that branch\n\
                 - Tags resolve to tagged commit\n\
                 - Check stats for impact analysis (lines changed)\n\
                 - Parse files array for detailed diff information\n\
                 - Use commit data for automated changelog generation\n\
                 - Verify author/committer for security audits\n\
                 - Check parent commits for merge analysis\n\
                 - Combine with github_list_commits for commit history",
            ),
        },
    ]
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}
