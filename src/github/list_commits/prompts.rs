//! Prompt messages for github_list_commits tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ListCommitsPromptArgs;

/// Prompt provider for list_commits tool
///
/// This is the ONLY way to provide prompts for list_commits - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ListCommitsPrompts;

impl PromptProvider for ListCommitsPrompts {
    type PromptArgs = ListCommitsPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
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

/// Basic commit listing
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list commits from a GitHub repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_list_commits tool retrieves commit history from any GitHub repository without cloning. It's perfect for analyzing project activity, generating changelogs, and tracking contributions.\n\n\
                 LISTING COMMITS:\n\n\
                 1. Recent commits:\n\
                    github_list_commits({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\"\n\
                    })\n\
                    Returns the most recent commits from the default branch\n\n\
                 2. With pagination:\n\
                    github_list_commits({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"per_page\": 50\n\
                    })\n\
                    Get up to 50 commits per request (max: 100, default: 30)\n\n\
                 3. Specific page:\n\
                    github_list_commits({\n\
                        \"owner\": \"rust-lang\",\n\
                        \"repo\": \"rust\",\n\
                        \"page\": 2,\n\
                        \"per_page\": 100\n\
                    })\n\
                    Navigate through commit history page by page\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"count\": 30,\n\
                   \"commits\": [\n\
                     {\n\
                       \"sha\": \"abc1234567890abcdef1234567890abcdef12345\",\n\
                       \"message\": \"Add new feature\",\n\
                       \"author_name\": \"John Doe\",\n\
                       \"author_email\": \"john@example.com\",\n\
                       \"date\": \"2024-01-15T10:30:00Z\",\n\
                       \"html_url\": \"https://github.com/user/project/commit/abc1234\"\n\
                     },\n\
                     // ... more commits\n\
                   ]\n\
                 }\n\n\
                 BASIC PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - page (optional): Page number for pagination (default: 1)\n\
                 - per_page (optional): Results per page (default: 30, max: 100)\n\n\
                 RESPONSE FIELDS:\n\
                 - sha: Unique commit identifier (40-character hex)\n\
                 - message: Full commit message (first line + body)\n\
                 - author_name: Commit author's name\n\
                 - author_email: Commit author's email\n\
                 - date: Commit timestamp in ISO 8601 format\n\
                 - html_url: Direct link to commit on GitHub\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable:\n\
                 - Public repos: Works with any token\n\
                 - Private repos: Token needs 'repo' scope\n\
                 - Rate limit: 5,000 requests/hour when authenticated\n\n\
                 BEST PRACTICES:\n\
                 - Use per_page: 100 to reduce API calls\n\
                 - Check count field to know how many commits returned\n\
                 - Use sha for unique identification",
            ),
        },
    ]
}

/// Filtering commits
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I filter commits by author, file path, or date range?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_list_commits tool provides powerful filtering capabilities to narrow down commit history by author, path, and date range.\n\n\
                 FILTERING COMMITS:\n\n\
                 1. By author:\n\
                    github_list_commits({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"author\": \"developer\"\n\
                    })\n\
                    Returns commits by specific GitHub username or email\n\
                    Author can be: GitHub username OR email address\n\n\
                 2. By path:\n\
                    github_list_commits({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"src/main.rs\"\n\
                    })\n\
                    Returns only commits that modified this specific file\n\
                    Path can be: file path OR directory path\n\n\
                 3. By directory:\n\
                    github_list_commits({\n\
                        \"owner\": \"rust-lang\",\n\
                        \"repo\": \"rust\",\n\
                        \"path\": \"compiler/rustc_codegen\"\n\
                    })\n\
                    Returns commits affecting any file in this directory\n\n\
                 4. By date range:\n\
                    github_list_commits({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"since\": \"2024-01-01T00:00:00Z\",\n\
                        \"until\": \"2024-01-31T23:59:59Z\"\n\
                    })\n\
                    Returns commits within specific time window\n\n\
                 FILTER OPTIONS:\n\n\
                 AUTHOR FILTER:\n\
                 - author: GitHub username or email address\n\
                 - Case-insensitive matching\n\
                 - Examples: \"octocat\", \"user@example.com\"\n\n\
                 PATH FILTER:\n\
                 - path: Relative path from repository root\n\
                 - Can be file or directory\n\
                 - Use forward slashes (/) as separators\n\
                 - Examples: \"src/main.rs\", \"docs/\", \"package.json\"\n\n\
                 DATE FILTERS:\n\
                 - since: Only commits after this date (inclusive)\n\
                 - until: Only commits before this date (inclusive)\n\
                 - Format: ISO 8601 (YYYY-MM-DDTHH:MM:SSZ)\n\
                 - Timezone: Use Z for UTC or specify offset\n\n\
                 TIPS FOR EFFECTIVE FILTERING:\n\
                 - Use author filter for contributor tracking and attribution\n\
                 - Use path filter for file history and blame analysis\n\
                 - Use date filters for release planning and retrospectives\n\
                 - Combine filters to answer specific questions\n\
                 - Use per_page: 100 when expecting many results",
            ),
        },
    ]
}
