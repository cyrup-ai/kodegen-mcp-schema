//! Prompt messages for github_add_pull_request_review_comment tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::AddPullRequestReviewCommentPromptArgs;

/// Prompt provider for add_pull_request_review_comment tool
///
/// This is the ONLY way to provide prompts for add_pull_request_review_comment - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct AddPullRequestReviewCommentPrompts;

impl PromptProvider for AddPullRequestReviewCommentPrompts {
    type PromptArgs = AddPullRequestReviewCommentPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_add_pull_request_review_comment to add inline code review comments?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_add_pull_request_review_comment tool adds inline comments to specific lines in a pull request's diff for detailed code review.\n\n\
                 BASIC USAGE:\n\
                 1. Comment on single line:\n\
                    github_add_pull_request_review_comment({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"pull_number\": 5678, \"body\": \"Consider using `match` here\", \"commit_id\": \"abc123\", \"path\": \"src/lib.rs\", \"line\": 42})\n\n\
                 2. Multi-line comment:\n\
                    github_add_pull_request_review_comment({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"pull_number\": 123, \"body\": \"This entire block could be simplified\", \"commit_id\": \"def456\", \"path\": \"compiler/rustc/src/main.rs\", \"start_line\": 100, \"line\": 105})\n\n\
                 3. Comment on deletion (LEFT side):\n\
                    github_add_pull_request_review_comment({\"owner\": \"actix\", \"repo\": \"actix-web\", \"pull_number\": 999, \"body\": \"Why remove this?\", \"commit_id\": \"ghi789\", \"path\": \"src/handler.rs\", \"line\": 20, \"side\": \"LEFT\"})\n\n\
                 4. Reply to existing comment:\n\
                    github_add_pull_request_review_comment({\"owner\": \"serde-rs\", \"repo\": \"serde\", \"pull_number\": 456, \"body\": \"Good point! Fixed in latest commit\", \"in_reply_to\": 987654321})\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - pull_number (required): Pull request number\n\
                 - body (required): Comment text (Markdown supported)\n\
                 - commit_id (required for new comments): Commit SHA\n\
                 - path (required for new comments): File path in repository\n\
                 - line (required for new comments): Line number in diff\n\
                 - side (optional): \"LEFT\" (deletion) or \"RIGHT\" (addition, default)\n\
                 - start_line (optional): Start line for multi-line comments\n\
                 - start_side (optional): Side of start_line (\"LEFT\" or \"RIGHT\")\n\
                 - in_reply_to (optional): Comment ID to reply to for threaded discussions\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo (for private repositories)\n\
                 - public_repo (for public repositories only)\n\n\
                 RESPONSE:\n\
                 Returns JSON with:\n\
                 - success: true/false\n\
                 - owner, repo: Repository identifiers\n\
                 - pr_number: Pull request number\n\
                 - comment_id: Created comment ID\n\
                 - message: Status message\n\n\
                 COMMON WORKFLOWS:\n\
                 1. Automated code review:\n\
                    - Analyze PR diff for patterns\n\
                    - Add inline suggestions on specific lines\n\
                    - Focus on security, performance, style issues\n\
                 2. Documentation review:\n\
                    - Check doc comments for completeness\n\
                    - Suggest improvements on specific functions\n\
                    - Link to style guide examples\n\
                 3. Test coverage feedback:\n\
                    - Identify untested code paths\n\
                    - Comment on functions needing tests\n\
                    - Suggest test scenarios\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Unauthenticated: 60 requests/hour\n\
                 - Check X-RateLimit-Remaining header\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found: PR, commit, or file path doesn't exist\n\
                    Fix: Verify pull_number and commit_id are valid\n\
                 2. 422 Unprocessable: Invalid line number or path\n\
                    Fix: Line must exist in commit's diff, not entire file\n\
                 3. 403 Forbidden: Token lacks required scopes\n\
                    Fix: Generate new token with 'repo' scope\n\n\
                 BEST PRACTICES:\n\
                 - Use commit_id from latest PR commit for accuracy\n\
                 - Line numbers are relative to diff, not file line numbers\n\
                 - Multi-line comments must have start_line < line\n\
                 - Use RIGHT side for new/modified code (default)\n\
                 - Use LEFT side for deleted code or original version\n\
                 - Reply with in_reply_to for threaded conversations\n\
                 - Keep comments constructive and specific\n\
                 - Use code suggestions (```suggestion) when possible",
            ),
        },
    ]
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}
