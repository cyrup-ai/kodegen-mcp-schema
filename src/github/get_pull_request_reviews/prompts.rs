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

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("review_analysis") => Self::prompt_review_analysis(),
            _ => Self::prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, review_analysis)".to_string()),
                required: Some(false),
            }
        ]
    }
}

impl GetPullRequestReviewsPrompts {
    /// Scenario 1: Basic review retrieval and usage
    fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_get_pull_request_reviews to check PR reviews?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_pull_request_reviews tool retrieves all reviews submitted on a pull request, including approvals, change requests, and comments.\n\n\
                 BASIC USAGE:\n\
                 1. Get all reviews:\n\
                    github_get_pull_request_reviews({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"pull_number\": 5678})\n\n\
                 2. Paginated reviews (50 per page):\n\
                    github_get_pull_request_reviews({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"pull_number\": 123, \"per_page\": 50, \"page\": 1})\n\n\
                 3. Check if PR is approved:\n\
                    github_get_pull_request_reviews({\"owner\": \"actix\", \"repo\": \"actix-web\", \"pull_number\": 999})\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - pull_number (required): Pull request number\n\
                 - page (optional): Page number for pagination (default: 1)\n\
                 - per_page (optional): Results per page (max 100, default 30)\n\n\
                 RESPONSE:\n\
                 Returns JSON with:\n\
                 - success: true/false\n\
                 - count: Number of reviews returned\n\
                 - reviews: Array of review objects with these fields:\n\
                   - id: Unique review identifier\n\
                   - user: Reviewer's GitHub username\n\
                   - body: Review comment text (may be empty)\n\
                   - state: Review state - one of:\n\
                     * \"APPROVED\" - reviewer approved the changes\n\
                     * \"CHANGES_REQUESTED\" - reviewer requested changes (blocks merge)\n\
                     * \"COMMENTED\" - reviewer commented without explicit approval/rejection\n\
                     * \"DISMISSED\" - review was dismissed by repository admin\n\
                     * \"PENDING\" - review is not yet submitted\n\
                   - commit_id: SHA of the commit that was reviewed\n\
                   - submitted_at: ISO 8601 timestamp of review submission\n\
                   - html_url: Direct link to the review on GitHub\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with appropriate scopes:\n\
                 - repo: Required for private repositories\n\
                 - public_repo: Sufficient for public repositories only\n\n\
                 Without authentication, you can only access public repositories and will have very limited rate limits.\n\n\
                 COMMON PATTERN - Check if PR has required approvals:\n\
                 1. Call github_get_pull_request_reviews with owner/repo/pull_number\n\
                 2. Filter the reviews array for state=\"APPROVED\"\n\
                 3. Count the approved reviews\n\
                 4. Compare count against your team's required approval threshold\n\
                 5. Check that no reviews have state=\"CHANGES_REQUESTED\" (which blocks merging)\n\n\
                 Example: If you need 2 approvals, count APPROVED reviews and ensure count >= 2 and no CHANGES_REQUESTED.\n\n\
                 ERROR HANDLING:\n\
                 1. 404 Not Found: Pull request or repository doesn't exist\n\
                    Fix: Verify owner, repo, and pull_number are correct\n\
                    Common cause: Typo in repository name or wrong pull request number\n\n\
                 2. 403 Forbidden: No access to private repository\n\
                    Fix: Verify GITHUB_TOKEN has 'repo' scope for private repositories\n\
                    Check: Ensure the token belongs to a user with read access to the repository",
            ),
        },
    ]
    }

    /// Scenario 2: Advanced review analysis for automation
    fn prompt_review_analysis() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I analyze PR reviews programmatically for merge automation?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Analyzing pull request reviews enables automated approval workflows, compliance checks, and merge eligibility decisions.\n\n\
                 ANALYZING REVIEW STATES:\n\
                 The 'state' field in each review determines its impact on merge eligibility:\n\n\
                 FILTER BY STATE:\n\
                 1. Find blocking change requests:\n\
                    After calling github_get_pull_request_reviews, filter reviews where state=\"CHANGES_REQUESTED\"\n\
                    Any review with this state blocks merging until addressed\n\
                    Use this to identify what needs fixing before merge\n\n\
                 2. Count approvals:\n\
                    Filter reviews where state=\"APPROVED\"\n\
                    Count these to determine if PR meets team's approval requirements\n\
                    Example: if (approved_count >= 2) { eligible_for_merge = true; }\n\n\
                 3. Identify non-blocking feedback:\n\
                    Filter reviews where state=\"PENDING\" or state=\"COMMENTED\"\n\
                    These provide feedback but don't affect merge eligibility\n\
                    Useful for tracking reviewer engagement without blocking progress\n\n\
                 4. Handle dismissed reviews:\n\
                    Filter reviews where state=\"DISMISSED\"\n\
                    These are reviews that were marked obsolete by a repository admin\n\
                    Should be excluded from approval and blocking calculations\n\
                    Common when requirements change or reviewer is no longer relevant\n\n\
                 CHECK REVIEW FRESHNESS:\n\
                 Reviews can become stale if new commits are pushed after the review.\n\n\
                 To verify review currency:\n\
                 1. Get all reviews with github_get_pull_request_reviews\n\
                 2. For each review, check the commit_id field\n\
                 3. Compare review's commit_id to the PR's latest commit SHA\n\
                 4. If they don't match, the review is outdated\n\
                 5. Outdated reviews should not count toward merge approval\n\n\
                 You can get the PR's latest commit using github_get_pull_request or by checking the PR's head SHA.\n\
                 Only count reviews where commit_id matches the current head for accurate approval status.\n\n\
                 MERGE READINESS AUTOMATION:\n\
                 Complete workflow for automated merge eligibility checking:\n\n\
                 1. Call github_get_pull_request_reviews({owner, repo, pull_number})\n\n\
                 2. For each review in the response:\n\
                    a) Check if commit_id matches PR's latest commit (discard if stale)\n\
                    b) Check state field\n\
                    c) Track submitted_at for review timeline analysis\n\n\
                 3. Calculate merge eligibility:\n\
                    approved_count = reviews where state=\"APPROVED\" and commit_id is current\n\
                    blocking_count = reviews where state=\"CHANGES_REQUESTED\" and commit_id is current\n\n\
                 4. Determine eligibility:\n\
                    IF approved_count >= required_approvals AND blocking_count = 0:\n\
                        RESULT: Ready to merge ✓\n\
                    ELSE IF blocking_count > 0:\n\
                        RESULT: Blocked by change requests ✗\n\
                    ELSE:\n\
                        RESULT: Insufficient approvals ✗\n\n\
                 5. Return decision with details:\n\
                    - Total reviews analyzed\n\
                    - Current approvals count\n\
                    - Blocking reviews count\n\
                    - Stale reviews excluded\n\
                    - Merge recommendation\n\n\
                 RATE LIMITING:\n\
                 GitHub API enforces rate limits on all endpoints:\n\
                 - Authenticated requests: 5,000 requests per hour\n\
                 - Unauthenticated requests: 60 requests per hour\n\n\
                 The response includes rate limit headers:\n\
                 - X-RateLimit-Remaining: Number of requests remaining\n\
                 - X-RateLimit-Reset: Unix timestamp when limit resets\n\n\
                 For automation workflows, batch review requests and cache results to avoid exhausting rate limits.\n\
                 Check X-RateLimit-Remaining regularly and pause if it gets too low.\n\n\
                 ERROR SCENARIOS:\n\
                 1. 403 Forbidden: Authentication token lacks required permissions\n\
                    Fix: Ensure GITHUB_TOKEN has 'repo' scope\n\
                    Check: Verify token is valid and not expired\n\
                    For organizations: Confirm token owner has repository access\n\n\
                 2. 422 Unprocessable Entity: Invalid pagination parameters\n\
                    Fix: Ensure page >= 1 and per_page <= 100\n\
                    Common error: page=0 or per_page=150 (exceeds maximum)\n\
                    Valid range: page starts at 1, per_page max is 100\n\n\
                 3. 429 Rate Limit Exceeded: Too many requests\n\
                    Fix: Wait until the time specified in X-RateLimit-Reset header\n\
                    Implement exponential backoff in automation scripts\n\
                    Consider caching review data to reduce API calls\n\n\
                 BEST PRACTICES FOR AUTOMATION:\n\
                 - Always filter out dismissed reviews (state=\"DISMISSED\") from calculations\n\
                 - Verify commit_id matches latest commit before counting approvals\n\
                 - Use submitted_at to track review timeline and identify stale reviews\n\
                 - Cache review data and only refresh when PR is updated to reduce API calls",
            ),
        },
    ]
    }
}
