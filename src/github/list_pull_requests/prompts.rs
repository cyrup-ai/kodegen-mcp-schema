//! Prompt messages for github_list_pull_requests tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ListPullRequestsPromptArgs;

/// Prompt provider for github_list_pull_requests tool
///
/// This is the ONLY way to provide prompts for github_list_pull_requests - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ListPullRequestsPrompts;

impl PromptProvider for ListPullRequestsPrompts {
    type PromptArgs = ListPullRequestsPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("filtering") => prompt_filtering(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, filtering)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST PULL REQUESTS
// ============================================================================

/// Basic PR listing operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list pull requests in a GitHub repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_list_pull_requests tool retrieves pull requests from a repository.\n\n\
                 BASIC LISTING:\n\
                 All open PRs (default):\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\"})\n\n\
                 Specific repository:\n\
                 github_list_pull_requests({\"owner\": \"tokio-rs\", \"repo\": \"tokio\"})\n\n\
                 Organization repository:\n\
                 github_list_pull_requests({\"owner\": \"rust-lang\", \"repo\": \"rust\"})\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"count\": 2,\n\
                   \"pull_requests\": [\n\
                     {\n\
                       \"number\": 456,\n\
                       \"title\": \"Add dark mode support\",\n\
                       \"state\": \"open\",\n\
                       \"author\": \"contributor\",\n\
                       \"head_ref\": \"feature/dark-mode\",\n\
                       \"base_ref\": \"main\",\n\
                       \"created_at\": \"2025-12-01T10:30:00Z\",\n\
                       \"draft\": false\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 KEY FIELDS:\n\
                 - number: PR number (use with other PR tools)\n\
                 - title: PR title/description\n\
                 - state: \"open\" or \"closed\"\n\
                 - author: GitHub username of PR creator\n\
                 - head_ref: Source branch (where changes are)\n\
                 - base_ref: Target branch (where merging to)\n\
                 - created_at: ISO 8601 timestamp\n\
                 - draft: true if PR is in draft state\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner (username or organization)\n\
                 - repo: Repository name\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable for private repos and higher rate limits.\n\
                 - Public repos: 60 requests/hour without token\n\
                 - With token: 5,000 requests/hour\n\
                 - Private repos: Token with 'repo' scope required\n\n\
                 NEXT STEPS:\n\
                 After listing PRs:\n\
                 - Use github_get_pull_request_status for detailed status\n\
                 - Use github_get_pull_request_files to see changed files\n\
                 - Use github_get_pull_request_reviews to check reviews\n\
                 - Use github_merge_pull_request to merge approved PRs",
            ),
        },
    ]
}

/// Filtering PRs by state, branch, and other criteria
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I filter pull requests by state, branch, or other criteria?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_list_pull_requests tool provides filtering options to narrow down PR results.\n\n\
                 FILTERING BY STATE:\n\
                 Open PRs (default):\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"state\": \"open\"})\n\n\
                 Closed PRs:\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"state\": \"closed\"})\n\n\
                 All PRs:\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"state\": \"all\"})\n\n\
                 FILTERING BY BRANCH:\n\
                 Base branch (merge target):\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"base\": \"main\"})\n\n\
                 Head branch (source):\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"head\": \"username:feature-branch\"})\n\
                 Format head as \"username:branch-name\"\n\n\
                 SORTING OPTIONS:\n\
                 By creation date (default):\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"sort\": \"created\", \"direction\": \"desc\"})\n\n\
                 By last update:\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"sort\": \"updated\", \"direction\": \"desc\"})\n\n\
                 By popularity (most commented):\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"sort\": \"popularity\"})\n\n\
                 Long-running (oldest first):\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"sort\": \"long-running\"})\n\n\
                 SORT DIRECTIONS:\n\
                 - \"desc\": Descending (newest/highest first) - DEFAULT\n\
                 - \"asc\": Ascending (oldest/lowest first)\n\n\
                 PAGINATION:\n\
                 Per page (default 30, max 100):\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 100})\n\n\
                 Specific page:\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"page\": 2, \"per_page\": 100})\n\n\
                 COMBINING FILTERS:\n\
                 Open PRs to main, recently updated:\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"state\": \"open\", \"base\": \"main\", \"sort\": \"updated\", \"direction\": \"desc\"})\n\n\
                 Closed PRs to develop:\n\
                 github_list_pull_requests({\"owner\": \"user\", \"repo\": \"project\", \"state\": \"closed\", \"base\": \"develop\", \"sort\": \"updated\", \"direction\": \"asc\"})\n\n\
                 FILTER PARAMETERS SUMMARY:\n\
                 - state: \"open\" (default), \"closed\", \"all\"\n\
                 - base: Target branch name\n\
                 - head: Source branch with format \"user:branch\"\n\
                 - sort: \"created\" (default), \"updated\", \"popularity\", \"long-running\"\n\
                 - direction: \"desc\" (default), \"asc\"\n\
                 - page: Page number (default: 1)\n\
                 - per_page: Results per page (default: 30, max: 100)\n\n\
                 BEST PRACTICES:\n\
                 - Use state=\"open\" to focus on active PRs\n\
                 - Filter by base branch for specific merge targets\n\
                 - Sort by \"updated\" to see recent activity\n\
                 - Sort by \"long-running\" to find stale PRs\n\
                 - Use per_page=100 for maximum efficiency",
            ),
        },
    ]
}

