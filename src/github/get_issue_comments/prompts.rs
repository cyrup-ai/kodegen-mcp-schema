//! Prompt messages for github_get_issue_comments tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetIssueCommentsPromptArgs;

/// Prompt provider for get_issue_comments tool
pub struct GetIssueCommentsPrompts;

impl PromptProvider for GetIssueCommentsPrompts {
    type PromptArgs = GetIssueCommentsPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("pagination") => prompt_pagination(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, pagination)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// SCENARIO PROMPTS - TEACH AI AGENTS HOW TO USE GITHUB_GET_ISSUE_COMMENTS
// ============================================================================

/// Basic issue comment retrieval
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I retrieve comments from a GitHub issue?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_issue_comments tool retrieves all comments on a GitHub issue.\n\n\
                 BASIC USAGE:\n\
                 1. Get all comments:\n\
                    github_get_issue_comments({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"issue_number\": 1234})\n\n\
                 2. Get recent comments:\n\
                    github_get_issue_comments({\"owner\": \"actix\", \"repo\": \"actix-web\", \"issue_number\": 456, \"since\": \"2024-01-01T00:00:00Z\"})\n\n\
                 3. Get sorted comments:\n\
                    github_get_issue_comments({\"owner\": \"serde-rs\", \"repo\": \"serde\", \"issue_number\": 789, \"sort\": \"created\", \"direction\": \"desc\"})\n\n\
                 CORE PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - issue_number (required): Issue number\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - since: Only comments updated after this timestamp (ISO 8601 format)\n\
                 - sort: \"created\" or \"updated\" (default: \"created\")\n\
                 - direction: \"asc\" or \"desc\" (default: \"asc\")\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo (for private repositories)\n\
                 - public_repo (for public repositories)\n\n\
                 RESPONSE STRUCTURE:\n\
                 Returns JSON with:\n\
                 - count: Number of comments returned\n\
                 - comments: Array of comment objects\n\
                   - id: Unique comment ID\n\
                   - body: Comment text (Markdown)\n\
                   - author: GitHub username\n\
                   - created_at: Creation timestamp\n\
                   - updated_at: Last update timestamp\n\
                   - html_url: Direct link to comment\n\n\
                 COMMON USE CASES:\n\
                 1. Issue analysis:\n\
                    - Get issue details first\n\
                    - Retrieve all comments\n\
                    - Analyze discussion to understand context\n\
                    - Extract key decisions or solutions\n\n\
                 2. Sentiment analysis:\n\
                    - Fetch all comments on issue\n\
                    - Analyze tone and feedback patterns\n\
                    - Identify user concerns or suggestions\n\n\
                 3. Response tracking:\n\
                    - Use since parameter for recent comments\n\
                    - Check for team member responses\n\
                    - Identify unanswered questions\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found: Issue or repository doesn't exist\n\
                    Fix: Verify owner/repo/issue_number are correct\n\n\
                 2. 403 Forbidden: No access to private repository\n\
                    Fix: Verify GITHUB_TOKEN has proper repo access\n\n\
                 3. 410 Gone: Issue was deleted or transferred\n\
                    Fix: Issue no longer accessible in this repository\n\n\
                 BEST PRACTICES:\n\
                 - Check issue.comments_count before fetching\n\
                 - Use since parameter to get only recent updates\n\
                 - Parse comment body for mentions, links, code blocks\n\
                 - Use timestamps to build discussion timeline\n\
                 - Combine with github_add_issue_comment for full workflow",
            ),
        },
    ]
}

/// Issue comment pagination and advanced filtering
fn prompt_pagination() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I handle pagination for issues with many comments?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Large GitHub issues can have hundreds of comments. Use pagination to retrieve them efficiently.\n\n\
                 WHY PAGINATION MATTERS:\n\
                 - GitHub returns max 100 results per page\n\
                 - Default is 30 comments per page\n\
                 - Popular issues can have thousands of comments\n\
                 - Use page and per_page parameters to control retrieval\n\n\
                 PAGINATION EXAMPLES:\n\
                 1. Get second page with 50 results:\n\
                    github_get_issue_comments({\n\
                        \"owner\": \"rust-lang\",\n\
                        \"repo\": \"rust\",\n\
                        \"issue_number\": 98765,\n\
                        \"per_page\": 50,\n\
                        \"page\": 2\n\
                    })\n\n\
                 2. Iterate through all pages:\n\
                    for page in 1..max_pages {\n\
                      const comments = github_get_issue_comments({\n\
                        \"owner\": \"rust-lang\",\n\
                        \"repo\": \"rust\",\n\
                        \"issue_number\": 98765,\n\
                        \"per_page\": 100,\n\
                        \"page\": page\n\
                      });\n\
                      if comments.count < 100 break;  // Last page\n\
                    }\n\n\
                 3. Get latest 100 comments (most recent first):\n\
                    github_get_issue_comments({\n\
                        \"owner\": \"tokio-rs\",\n\
                        \"repo\": \"tokio\",\n\
                        \"issue_number\": 1234,\n\
                        \"sort\": \"updated\",\n\
                        \"direction\": \"desc\",\n\
                        \"per_page\": 100\n\
                    })\n\n\
                 ALL PARAMETERS:\n\
                 - owner (required): Repository owner\n\
                 - repo (required): Repository name\n\
                 - issue_number (required): Issue number\n\
                 - since (optional): Only comments after timestamp (ISO 8601)\n\
                 - sort (optional): \"created\" or \"updated\" (default: \"created\")\n\
                 - direction (optional): \"asc\" or \"desc\" (default: \"asc\")\n\
                 - page (optional): Page number for pagination (default: 1)\n\
                 - per_page (optional): Results per page, max 100 (default: 30)\n\n\
                 PAGINATION WORKFLOWS:\n\
                 1. Archive all comments:\n\
                    - Start with page 1, per_page 100\n\
                    - Save all comments locally\n\
                    - Increment page until fewer than 100 results\n\
                    - Combine into single archive\n\n\
                 2. Incremental updates:\n\
                    - Get last stored comment timestamp\n\
                    - Use since parameter to fetch only new comments\n\
                    - Merge with existing local data\n\
                    - Update timestamp for next fetch\n\n\
                 3. Recent activity focus:\n\
                    - Sort by \"updated\" descending\n\
                    - Get first 100 (most recently updated)\n\
                    - Identify active discussion topics\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Each page counts as 1 API request\n\
                 - Issue with 10,000 comments = 100 requests at per_page=100\n\
                 - Check X-RateLimit-Remaining header in response\n\
                 - Plan pagination strategy to stay within limits\n\n\
                 MEMORY EFFICIENCY:\n\
                 - Use per_page=100 to minimize total requests\n\
                 - Don't accumulate all pages in memory for huge threads\n\
                 - Use since parameter to fetch only updates\n\
                 - Process pages incrementally, not all at once\n\
                 - Cache results locally to reduce API calls\n\n\
                 BEST PRACTICES:\n\
                 - Always use per_page=100 for efficient retrieval\n\
                 - Combine since with pagination for incremental fetching\n\
                 - Sort by \"updated\" descending to see recent activity first\n\
                 - Calculate total requests needed before bulk retrieval\n\
                 - Implement backoff if approaching rate limits\n\
                 - Store comments locally and update incrementally\n\
                 - Use caching to avoid re-fetching unchanged data",
            ),
        },
    ]
}
