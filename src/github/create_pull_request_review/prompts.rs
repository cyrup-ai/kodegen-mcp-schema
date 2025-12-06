//! Prompt messages for github_create_pull_request_review tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::CreatePullRequestReviewPromptArgs;

/// Prompt provider for create_pull_request_review tool
///
/// This is the ONLY way to provide prompts for create_pull_request_review - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct CreatePullRequestReviewPrompts;

impl PromptProvider for CreatePullRequestReviewPrompts {
    type PromptArgs = CreatePullRequestReviewPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_create_pull_request_review to review pull requests?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_create_pull_request_review tool submits a complete review for a pull request with optional inline comments and approval status.\n\n\
                 BASIC USAGE:\n\
                 1. Approve PR:\n\
                    github_create_pull_request_review({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"pull_number\": 5678, \"event\": \"APPROVE\", \"body\": \"LGTM! Great work.\"})\n\n\
                 2. Request changes:\n\
                    github_create_pull_request_review({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"pull_number\": 123, \"event\": \"REQUEST_CHANGES\", \"body\": \"Please address the following issues before merging.\"})\n\n\
                 3. Comment only:\n\
                    github_create_pull_request_review({\"owner\": \"actix\", \"repo\": \"actix-web\", \"pull_number\": 999, \"event\": \"COMMENT\", \"body\": \"Some initial thoughts on the approach.\"})\n\n\
                 4. Review with inline comments:\n\
                    github_create_pull_request_review({\"owner\": \"serde-rs\", \"repo\": \"serde\", \"pull_number\": 456, \"event\": \"APPROVE\", \"body\": \"Looks good overall\", \"comments\": [{\"path\": \"src/lib.rs\", \"line\": 42, \"body\": \"Nice optimization!\"}]})\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - pull_number (required): Pull request number\n\
                 - event (required): \"APPROVE\", \"REQUEST_CHANGES\", or \"COMMENT\"\n\
                 - body (optional): Review summary comment\n\
                 - commit_id (optional): Specific commit SHA to review\n\
                 - comments (optional): Array of inline comment objects:\n\
                   - path: File path in repository\n\
                   - line: Line number in diff\n\
                   - body: Comment text\n\
                   - side: \"LEFT\" or \"RIGHT\" (optional)\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo (for private repositories)\n\
                 - public_repo (for public repositories only)\n\n\
                 RESPONSE:\n\
                 Returns JSON with:\n\
                 - success: true/false\n\
                 - owner, repo: Repository identifiers\n\
                 - pr_number: Pull request number\n\
                 - review_id: Created review ID\n\
                 - state: Review state (APPROVED/CHANGES_REQUESTED/COMMENTED)\n\
                 - message: Status message\n\n\
                 COMMON WORKFLOWS:\n\
                 1. Automated code review:\n\
                    - Analyze PR diff for issues\n\
                    - Add inline comments on specific problems\n\
                    - Request changes if critical issues found\n\
                    - Otherwise approve with minor suggestions\n\
                 2. Security review:\n\
                    - Check for security vulnerabilities\n\
                    - Request changes for any issues\n\
                    - Comment on secure coding practices\n\
                    - Approve only after fixes verified\n\
                 3. Documentation review:\n\
                    - Verify all public APIs documented\n\
                    - Comment on unclear explanations\n\
                    - Request changes if docs incomplete\n\
                    - Approve when documentation meets standards\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Unauthenticated: 60 requests/hour\n\
                 - Check X-RateLimit-Remaining header\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found: PR doesn't exist or not accessible\n\
                    Fix: Verify pull_number and repository access\n\
                 2. 422 Unprocessable: Invalid event or already reviewed\n\
                    Fix: Use valid event type; dismiss previous review first\n\
                 3. 403 Forbidden: Cannot approve own PR or lacking permissions\n\
                    Fix: Authors cannot approve their own PRs in most repos\n\n\
                 BEST PRACTICES:\n\
                 - Use APPROVE only when changes are production-ready\n\
                 - REQUEST_CHANGES for blocking issues\n\
                 - COMMENT for non-blocking suggestions\n\
                 - Provide constructive, specific feedback\n\
                 - Explain reasoning behind requested changes\n\
                 - Use inline comments for line-specific issues\n\
                 - Acknowledge good practices in code\n\
                 - Be respectful and professional in reviews\n\
                 - Follow team's review guidelines",
            ),
        },
    ]
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}
