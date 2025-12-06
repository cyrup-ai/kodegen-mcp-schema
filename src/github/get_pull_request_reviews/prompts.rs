//! Prompt messages for github_get_pull_request_reviews tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetPullRequestReviewsPromptArgs;

/// Prompt provider for get_pull_request_reviews tool
///
/// This is the ONLY way to provide prompts for get_pull_request_reviews - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GetPullRequestReviewsPrompts;

impl PromptProvider for GetPullRequestReviewsPrompts {
    type PromptArgs = GetPullRequestReviewsPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_get_pull_request_reviews to retrieve PR reviews?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_pull_request_reviews tool retrieves all reviews submitted on a pull request, including approvals, change requests, and comments.\n\n\
                 BASIC USAGE:\n\
                 1. Get all reviews:\n\
                    github_get_pull_request_reviews({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"pull_number\": 5678})\n\n\
                 2. Paginated reviews:\n\
                    github_get_pull_request_reviews({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"pull_number\": 123, \"per_page\": 50, \"page\": 1})\n\n\
                 3. Check approval status:\n\
                    github_get_pull_request_reviews({\"owner\": \"actix\", \"repo\": \"actix-web\", \"pull_number\": 999})\n\n\
                 4. Review history:\n\
                    github_get_pull_request_reviews({\"owner\": \"serde-rs\", \"repo\": \"serde\", \"pull_number\": 456})\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - pull_number (required): Pull request number\n\
                 - page (optional): Page number for pagination\n\
                 - per_page (optional): Results per page (max 100, default 30)\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo (for private repositories)\n\
                 - public_repo (for public repositories only)\n\n\
                 RESPONSE:\n\
                 Returns JSON with:\n\
                 - success: true/false\n\
                 - owner, repo: Repository identifiers\n\
                 - pr_number: Pull request number\n\
                 - count: Number of reviews returned\n\
                 - reviews: Array of review objects:\n\
                   - id: Review ID\n\
                   - user: Reviewer username\n\
                   - body: Review comment text\n\
                   - state: \"APPROVED\", \"CHANGES_REQUESTED\", \"COMMENTED\", \"DISMISSED\", \"PENDING\"\n\
                   - commit_id: Commit SHA reviewed\n\
                   - submitted_at: Review submission timestamp\n\
                   - html_url: Link to review on GitHub\n\n\
                 COMMON WORKFLOWS:\n\
                 1. Merge readiness check:\n\
                    - Get all reviews\n\
                    - Count approvals vs change requests\n\
                    - Check if required approvals met\n\
                    - Determine if PR ready to merge\n\
                 2. Review tracking:\n\
                    - Retrieve all reviews\n\
                    - Track reviewer participation\n\
                    - Identify outstanding feedback\n\
                    - Send reminders to pending reviewers\n\
                 3. Compliance verification:\n\
                    - Get review list\n\
                    - Verify required reviewers approved\n\
                    - Check for dismissed reviews\n\
                    - Ensure no blocking change requests\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Unauthenticated: 60 requests/hour\n\
                 - Check X-RateLimit-Remaining header\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found: PR or repository doesn't exist\n\
                    Fix: Verify owner/repo/pull_number are correct\n\
                 2. 403 Forbidden: No access to private repository\n\
                    Fix: Verify GITHUB_TOKEN has repo access\n\
                 3. 422 Unprocessable: Invalid pagination parameters\n\
                    Fix: Ensure page/per_page are valid positive integers\n\n\
                 BEST PRACTICES:\n\
                 - Filter by state to find blocking reviews\n\
                 - Check commit_id to ensure reviews are current\n\
                 - Use submitted_at to track review timeline\n\
                 - Count APPROVED states for merge eligibility\n\
                 - Identify CHANGES_REQUESTED for blocking issues\n\
                 - Handle DISMISSED reviews appropriately\n\
                 - Ignore COMMENTED-only reviews for approval count\n\
                 - Combine with branch protection rules for automation\n\
                 - Cache reviews and update incrementally\n\
                 - Use pagination for PRs with many reviews",
            ),
        },    ]
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]  // No customization arguments for this tool
    }
}

