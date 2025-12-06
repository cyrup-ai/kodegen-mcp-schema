//! Prompt messages for github_get_issue tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetIssuePromptArgs;

/// Prompt provider for get_issue tool
///
/// This is the ONLY way to provide prompts for get_issue - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GetIssuePrompts;

impl PromptProvider for GetIssuePrompts {
    type PromptArgs = GetIssuePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("metadata") => prompt_metadata(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, metadata)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic issue retrieval
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I retrieve basic information about a GitHub issue?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_issue tool retrieves detailed information about a specific GitHub issue. Here's how to use it:\n\n\
                 GETTING ISSUE DETAILS:\n\n\
                 1. Get issue by number:\n\
                    github_get_issue({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"issue_number\": 123\n\
                    })\n\n\
                 2. Get issue from organization:\n\
                    github_get_issue({\n\
                        \"owner\": \"rust-lang\",\n\
                        \"repo\": \"rust\",\n\
                        \"issue_number\": 98765\n\
                    })\n\n\
                 3. Get issue from your own repository:\n\
                    github_get_issue({\n\
                        \"owner\": \"myusername\",\n\
                        \"repo\": \"myproject\",\n\
                        \"issue_number\": 1\n\
                    })\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"number\": 123,\n\
                   \"title\": \"Bug: Login fails on mobile\",\n\
                   \"state\": \"open\",\n\
                   \"body\": \"## Description\\nLogin button doesn't work...\",\n\
                   \"user\": {\"login\": \"reporter\"},\n\
                   \"created_at\": \"2024-01-15T10:00:00Z\",\n\
                   \"updated_at\": \"2024-01-16T15:30:00Z\",\n\
                   \"html_url\": \"https://github.com/user/project/issues/123\"\n\
                 }\n\n\
                 KEY ISSUE FIELDS:\n\
                 - number: Unique issue identifier\n\
                 - title: Short summary of the issue\n\
                 - state: \"open\" or \"closed\"\n\
                 - body: Full description in Markdown format (can be null)\n\
                 - user: Creator information (login, id, avatar_url)\n\
                 - created_at: When issue was created (ISO 8601)\n\
                 - updated_at: Last modification time (ISO 8601)\n\
                 - html_url: Direct link to issue on GitHub\n\n\
                 READING ISSUE CONTENT:\n\
                 1. Check title for quick context:\n\
                    Issue title is a concise summary of the problem\n\n\
                 2. Read body for full details:\n\
                    Body contains description, steps to reproduce, expected behavior\n\
                    Often formatted with Markdown (headers, lists, code blocks)\n\
                    Can be null if no description provided\n\n\
                 3. Check state for status:\n\
                    \"open\" = issue is active and needs attention\n\
                    \"closed\" = issue has been resolved or rejected\n\n\
                 4. Review timestamps:\n\
                    created_at: How old is this issue?\n\
                    updated_at: When was last activity?\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - issue_number (required): Issue number (positive integer)\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable:\n\
                 - Public repositories: Any token with public_repo scope\n\
                 - Private repositories: Token with repo scope\n\
                 - No token: Can only access public repositories (rate limited)\n\n\
                 COMMON USE CASES:\n\
                 1. Check if issue exists:\n\
                    Get issue and verify success field in response\n\n\
                 2. Read bug report:\n\
                    Get issue to understand problem before fixing\n\n\
                 3. Check feature request:\n\
                    Get issue to understand what's being requested\n\n\
                 4. Verify issue state:\n\
                    Get issue to see if it's open or closed\n\n\
                 ERROR HANDLING:\n\
                 - 404: Issue or repository not found (check owner/repo/number)\n\
                 - 403: No access (need authentication or permissions)\n\
                 - 410: Issue was deleted or transferred\n\
                 - 401: Invalid or missing authentication token\n\n\
                 BEST PRACTICES:\n\
                 - Always check the state field first\n\
                 - Parse body as Markdown for proper formatting\n\
                 - Handle null body gracefully (some issues have no description)\n\
                 - Use html_url to provide links back to GitHub\n\
                 - Check user.login to identify who created the issue",
            ),
        },
    ]
}

/// Issue metadata (labels, assignees, milestone)
fn prompt_metadata() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I access issue metadata like labels, assignees, and milestones?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "GitHub issues include rich metadata for categorization and tracking. The github_get_issue tool returns all this information.\n\n\
                 ISSUE METADATA FIELDS:\n\n\
                 RESPONSE includes:\n\
                 {\n\
                   \"labels\": [\n\
                     {\"name\": \"bug\", \"color\": \"d73a4a\", \"description\": \"Something isn't working\"},\n\
                     {\"name\": \"priority-high\", \"color\": \"ff0000\", \"description\": \"High priority\"}\n\
                   ],\n\
                   \"assignees\": [\n\
                     {\"login\": \"developer1\", \"id\": 12345},\n\
                     {\"login\": \"developer2\", \"id\": 67890}\n\
                   ],\n\
                   \"assignee\": {\"login\": \"developer1\", \"id\": 12345},\n\
                   \"milestone\": {\n\
                     \"title\": \"v1.0.0\",\n\
                     \"number\": 5,\n\
                     \"state\": \"open\",\n\
                     \"description\": \"First major release\",\n\
                     \"due_on\": \"2024-12-31T00:00:00Z\"\n\
                   },\n\
                   \"comments\": 3,\n\
                   \"pull_request\": null\n\
                 }\n\n\
                 LABELS:\n\
                 Labels categorize and classify issues:\n\
                 - name: Label text (e.g., \"bug\", \"enhancement\")\n\
                 - color: Hex color code without # (e.g., \"d73a4a\")\n\
                 - description: Optional explanation\n\
                 - Common patterns: bug, enhancement, priority-high, wontfix, duplicate\n\n\
                 ASSIGNEES:\n\
                 People assigned to work on the issue:\n\
                 - assignees: Array of all assigned users\n\
                 - assignee: Primary assignee (first in array, can be null)\n\
                 - Each has: login, id, avatar_url\n\
                 - Empty array = unassigned\n\n\
                 MILESTONE:\n\
                 Groups issues for planned releases:\n\
                 - title: Milestone name (e.g., \"v1.0.0\")\n\
                 - number: Unique identifier\n\
                 - state: \"open\" or \"closed\"\n\
                 - due_on: Target date (ISO 8601, can be null)\n\
                 - Can be null if not assigned to milestone\n\n\
                 COMMENTS COUNT:\n\
                 - comments: Number of comments on the issue\n\
                 - Use to determine if there's discussion\n\
                 - Fetch full comments with github_get_issue_comments if needed\n\n\
                 PULL REQUEST LINK:\n\
                 - pull_request: Object if issue has linked PR, null otherwise\n\
                 - Contains: url, html_url, diff_url, patch_url\n\
                 - Indicates issue is being addressed by a PR\n\n\
                 USING METADATA:\n\
                 Check labels: labels.some(l => l.name === \"bug\")\n\
                 Check assignment: assignees.length > 0\n\
                 Check milestone: if (milestone) { console.log(milestone.title); }\n\
                 Check discussion: if (comments > 5) { /* fetch comments */ }\n\
                 Check PR status: if (pull_request) { /* fix in progress */ }\n\n\
                 BEST PRACTICES:\n\
                 - Check labels array length before accessing\n\
                 - Handle null milestone gracefully\n\
                 - Use assignees array, not singular assignee\n\
                 - Check pull_request field for fix status\n\
                 - Use comments count to decide if fetching full thread",
            ),
        },
    ]
}
