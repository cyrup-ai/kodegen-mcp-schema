//! Prompt messages for github_user_search tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::SearchUsersPromptArgs;

/// Prompt provider for search_users tool
///
/// This is the ONLY way to provide prompts for search_users - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct SearchUsersPrompts;

impl PromptProvider for SearchUsersPrompts {
    type PromptArgs = SearchUsersPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

/// Basic user search operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I search for GitHub users with github_user_search?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_user_search tool searches for GitHub users and organizations using GitHub's powerful search syntax.\n\n\
                 BASIC SEARCH EXAMPLES:\n\
                 1. Search by username:\n\
                    github_user_search({\"query\": \"rustacean\"})\n\n\
                 2. Search by full name:\n\
                    github_user_search({\"query\": \"Alice Johnson\"})\n\n\
                 3. Search in specific field:\n\
                    github_user_search({\"query\": \"in:login alice\"})\n\
                    github_user_search({\"query\": \"in:name Alice Johnson\"})\n\
                    github_user_search({\"query\": \"in:email alice@example.com\"})\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"query\": \"rustacean\",\n\
                   \"total_count\": 1000,\n\
                   \"items\": [\n\
                     {\n\
                       \"login\": \"rustacean\",\n\
                       \"id\": 12345,\n\
                       \"type\": \"User\",\n\
                       \"name\": \"Rust Developer\",\n\
                       \"location\": \"San Francisco\",\n\
                       \"email\": \"rust@example.com\",\n\
                       \"bio\": \"Passionate Rust developer\",\n\
                       \"company\": \"Tech Corp\",\n\
                       \"blog\": \"https://example.com\",\n\
                       \"followers\": 1500,\n\
                       \"following\": 200,\n\
                       \"public_repos\": 75,\n\
                       \"public_gists\": 10,\n\
                       \"avatar_url\": \"https://avatars.githubusercontent.com/...\",\n\
                       \"html_url\": \"https://github.com/rustacean\",\n\
                       \"created_at\": \"2020-01-15T10:30:00Z\",\n\
                       \"updated_at\": \"2024-12-05T08:45:00Z\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 KEY FIELDS EXPLAINED:\n\
                 - login: GitHub username (unique identifier)\n\
                 - type: \"User\" or \"Organization\"\n\
                 - name: Display name (may be null)\n\
                 - location: Geographic location (if public)\n\
                 - email: Email address (if public)\n\
                 - bio: User biography/description\n\
                 - company: Company affiliation\n\
                 - blog: Website URL\n\
                 - followers: Number of followers\n\
                 - following: Number of users they follow\n\
                 - public_repos: Number of public repositories\n\
                 - public_gists: Number of public gists\n\
                 - avatar_url: Profile picture URL\n\
                 - html_url: Profile page URL\n\
                 - created_at: Account creation date\n\
                 - updated_at: Last profile update\n\n\
                 COMMON PATTERNS:\n\
                 1. Find user by exact username:\n\
                    github_user_search({\"query\": \"octocat\"})\n\n\
                 2. Find users with name containing keyword:\n\
                    github_user_search({\"query\": \"in:name developer\"})\n\n\
                 3. Find users with email domain:\n\
                    github_user_search({\"query\": \"in:email @github.com\"})\n\n\
                 4. Find organizations only:\n\
                    github_user_search({\"query\": \"type:org rust\"})\n\n\
                 AUTHENTICATION:\n\
                 - Requires GITHUB_TOKEN environment variable\n\
                 - Any valid GitHub token scope works\n\
                 - Authenticated requests have higher rate limits\n\n\
                 RATE LIMITS:\n\
                 - 30 requests per minute for search endpoints\n\
                 - Stricter than general API (5000/hour)\n\
                 - Check X-RateLimit-Remaining header\n\
                 - Wait for rate limit reset if exceeded\n\n\
                 ERROR HANDLING:\n\
                 - 422 Unprocessable: Invalid query syntax\n\
                 - 403 Forbidden: Rate limit exceeded\n\
                 - 503 Service Unavailable: Search service down\n\
                 - Always check success field in response\n\n\
                 BEST PRACTICES:\n\
                 - Be specific in queries to reduce result count\n\
                 - Use field qualifiers (in:login, in:name) for precision\n\
                 - Handle rate limits gracefully\n\
                 - Check total_count to understand result scope\n\
                 - Use type filter to distinguish users from organizations",
            ),
        },
    ]
}
