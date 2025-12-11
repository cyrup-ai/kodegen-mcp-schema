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
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO GET GITHUB COMMITS
// ============================================================================

/// Basic commit retrieval and analysis
fn prompt_basic() -> Vec<PromptMessage> {
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
                "The github_get_commit tool retrieves detailed information about a specific commit \
                 including changes, statistics, and metadata.\n\n\
                 BASIC USAGE:\n\
                 1. Get commit by SHA:\n\
                    github_get_commit({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \
                    \"sha\": \"abc123def456789012345678901234567890abcd\"})\n\n\
                 2. Get commit with short SHA:\n\
                    github_get_commit({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"sha\": \"abc123d\"})\n\n\
                 3. Get HEAD commit:\n\
                    github_get_commit({\"owner\": \"actix\", \"repo\": \"actix-web\", \"sha\": \"HEAD\"})\n\n\
                 4. Get commit by branch name:\n\
                    github_get_commit({\"owner\": \"serde-rs\", \"repo\": \"serde\", \"sha\": \"main\"})\n\n\
                 5. Get commit by tag:\n\
                    github_get_commit({\"owner\": \"kubernetes\", \"repo\": \"kubernetes\", \
                    \"sha\": \"v1.28.0\"})\n\n\
                 6. Get merge commit details:\n\
                    github_get_commit({\"owner\": \"golang\", \"repo\": \"go\", \"sha\": \"8a1e5f3abc\"})\n\n\
                 7. Retrieve release commit:\n\
                    github_get_commit({\"owner\": \"torvalds\", \"repo\": \"linux\", \
                    \"sha\": \"abc123def456\"})\n\n\
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
                    - Validate commit in CI pipeline\n\
                 4. Release validation:\n\
                    - Get release tag commit\n\
                    - Verify author is release maintainer\n\
                    - Check timestamp aligns with schedule\n\
                    - Review files for unexpected changes\n\
                    - Analyze stats for impact\n\
                 5. Performance impact analysis:\n\
                    - Get commit stats (additions, deletions)\n\
                    - Categorize impact (high > 500, medium 100-500, low < 100)\n\
                    - Review files array for critical changes\n\
                    - Generate impact report for review\n\n\
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
                    Fix: Verify GITHUB_TOKEN has repo access\n\
                 4. 401 Unauthorized: Invalid or expired token\n\
                    Fix: Regenerate GITHUB_TOKEN with correct scopes\n\n\
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
                 - Monitor stats growth for feature detection\n\
                 - Analyze files for sensitive file modifications\n\
                 - Use parents array to detect merge commits (length > 1)\n\
                 - Cache commit data for reporting to avoid rate limits\n\
                 - Combine with github_get_branch to get latest commit SHA\n\
                 - Validate commits belong to target branch before integration\n\
                 - Extract message format for Conventional Commits compliance\n\
                 - Analyze diff patches for automated code pattern detection\n\
                 - Monitor file count in changes for scope estimation\n\n\
                 ADVANCED PATTERNS:\n\
                 Pattern 1: Build complete commit history\n\
                 - Use github_list_commits to get recent commits\n\
                 - For each commit SHA, call github_get_commit for full details\n\
                 - Collect stats and file changes across all commits\n\
                 - Aggregate metrics for comprehensive release analysis\n\
                 - Track trends in code churn and contributor activity\n\n\
                 Pattern 2: Track file evolution\n\
                 - Get commit and examine files array for specific file\n\
                 - See patches showing exact line-by-line changes\n\
                 - Repeat for multiple commits to track complete file history\n\
                 - Identify when specific changes were introduced\n\
                 - Build timeline of modifications for debugging\n\n\
                 Pattern 3: Merge analysis\n\
                 - Check parents array: length > 1 means merge commit\n\
                 - Retrieve each parent commit separately for comparison\n\
                 - Compare changes from each branch to understand merge\n\
                 - Identify conflicts and resolution patterns\n\
                 - Validate merge strategy and integration approach\n\n\
                 Pattern 4: Security audit\n\
                 - For each commit on critical branch\n\
                 - Check files array for sensitive paths (secrets, keys, credentials)\n\
                 - Verify author is authorized contributor\n\
                 - Ensure commit message references ticket/issue for traceability\n\
                 - Validate commit signing status if required by policy\n\
                 - Combine with github_get_branch for full context",
            ),
        },
    ]
}
