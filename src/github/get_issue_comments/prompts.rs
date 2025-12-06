//! Prompt messages for github_get_issue_comments tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetIssueCommentsPromptArgs;

/// Prompt provider for get_issue_comments tool
///
/// This is the ONLY way to provide prompts for get_issue_comments - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GetIssueCommentsPrompts;

impl PromptProvider for GetIssueCommentsPrompts {
    type PromptArgs = GetIssueCommentsPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_get_issue_comments to retrieve issue comments?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_issue_comments tool retrieves all comments on a specific GitHub issue, useful for understanding discussion history.\n\n\
                 BASIC USAGE:\n\
                 1. Get all comments:\n\
                    github_get_issue_comments({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"issue_number\": 1234})\n\n\
                 2. Get comments with pagination:\n\
                    github_get_issue_comments({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"issue_number\": 98765, \"per_page\": 50, \"page\": 1})\n\n\
                 3. Get recent comments:\n\
                    github_get_issue_comments({\"owner\": \"actix\", \"repo\": \"actix-web\", \"issue_number\": 456, \"since\": \"2024-01-01T00:00:00Z\"})\n\n\
                 4. Get sorted comments:\n\
                    github_get_issue_comments({\"owner\": \"serde-rs\", \"repo\": \"serde\", \"issue_number\": 789, \"sort\": \"created\", \"direction\": \"desc\"})\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - issue_number (required): Issue number\n\
                 - since (optional): Only comments updated after this timestamp (ISO 8601)\n\
                 - sort (optional): \"created\" or \"updated\" (default: \"created\")\n\
                 - direction (optional): \"asc\" or \"desc\" (default: \"asc\")\n\
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
                 - issue_number: Issue number\n\
                 - count: Number of comments returned\n\
                 - comments: Array of comment objects:\n\
                   - id: Comment ID\n\
                   - body: Comment text (Markdown)\n\
                   - author: GitHub username\n\
                   - created_at, updated_at: ISO timestamps\n\
                   - html_url: Direct link to comment\n\n\
                 COMMON WORKFLOWS:\n\
                 1. Issue analysis:\n\
                    - Get issue details (github_get_issue)\n\
                    - Retrieve all comments\n\
                    - Analyze discussion for context\n\
                    - Extract key decisions or solutions\n\
                 2. Sentiment analysis:\n\
                    - Fetch all comments\n\
                    - Analyze tone and feedback\n\
                    - Identify concerns or praise\n\
                    - Generate summary report\n\
                 3. Response tracking:\n\
                    - Get recent comments (since parameter)\n\
                    - Check for team responses\n\
                    - Identify unanswered questions\n\
                    - Flag issues needing attention\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Unauthenticated: 60 requests/hour\n\
                 - Check X-RateLimit-Remaining header\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found: Issue or repository doesn't exist\n\
                    Fix: Verify owner/repo/issue_number are correct\n\
                 2. 410 Gone: Issue was deleted or transferred\n\
                    Fix: Issue no longer accessible in this repository\n\
                 3. 403 Forbidden: No access to private repository\n\
                    Fix: Verify GITHUB_TOKEN has repo access\n\n\
                 BEST PRACTICES:\n\
                 - Check issue.comments_count before fetching to avoid empty requests\n\
                 - Use pagination for issues with many comments\n\
                 - Filter by since to get only recent updates\n\
                 - Sort by updated to see recent activity first\n\
                 - Parse comment body for mentions, links, code blocks\n\
                 - Track comment authors for participation metrics\n\
                 - Use timestamps to build timeline of discussion\n\
                 - Combine with github_add_issue_comment for full workflow\n\
                 - Cache comments and fetch incrementally with since\n\
                 - Respect rate limits for bulk comment retrieval",
            ),
        },
    ]
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}
