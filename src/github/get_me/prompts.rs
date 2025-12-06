//! Prompt messages for github_get_me tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetMePromptArgs;

/// Prompt provider for github_get_me tool
///
/// This is the ONLY way to provide prompts for github_get_me - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GetMePrompts;

impl PromptProvider for GetMePrompts {
    type PromptArgs = GetMePromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

/// Basic user info retrieval
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I get information about the authenticated GitHub user?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_me tool retrieves profile information for the authenticated GitHub user (the owner of the GITHUB_TOKEN).\n\n\
                 BASIC USAGE:\n\
                 Get authenticated user info:\n\
                 github_get_me({})\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"login\": \"octocat\",\n\
                   \"id\": 12345,\n\
                   \"name\": \"The Octocat\",\n\
                   \"email\": \"octocat@github.com\",\n\
                   \"avatar_url\": \"https://avatars.githubusercontent.com/u/12345\",\n\
                   \"html_url\": \"https://github.com/octocat\",\n\
                   \"bio\": \"Developer and cat enthusiast\",\n\
                   \"location\": \"San Francisco, CA\",\n\
                   \"company\": \"@github\",\n\
                   \"followers\": 1000,\n\
                   \"following\": 50,\n\
                   \"public_repos\": 75,\n\
                   \"created_at\": \"2008-01-14T04:33:35Z\"\n\
                 }\n\n\
                 KEY FIELDS:\n\
                 - login: GitHub username (unique identifier)\n\
                 - id: Numeric user ID (permanent)\n\
                 - name: Display name (can be changed)\n\
                 - email: Primary email address (may be null if private)\n\
                 - avatar_url: Profile picture URL\n\
                 - html_url: Profile page URL\n\
                 - bio: User biography\n\
                 - location: Geographic location\n\
                 - company: Company or organization\n\
                 - followers: Number of followers\n\
                 - following: Number of users followed\n\
                 - public_repos: Count of public repositories\n\
                 - created_at: Account creation timestamp\n\n\
                 NO PARAMETERS REQUIRED:\n\
                 The tool automatically uses the GITHUB_TOKEN environment variable to identify the user.\n\
                 You don't need to pass any username or credentials.\n\n\
                 AUTHENTICATION:\n\
                 - Requires GITHUB_TOKEN environment variable\n\
                 - Works with any valid token scope\n\
                 - Token must not be expired or revoked\n\
                 - Classic tokens and fine-grained tokens both supported\n\n\
                 COMMON USE CASES:\n\
                 1. Get current username:\n\
                    github_get_me({})\n\
                    // Use response.login for username\n\n\
                 2. Display user info:\n\
                    github_get_me({})\n\
                    // Show name, email, profile URL\n\n\
                 3. Get user ID:\n\
                    github_get_me({})\n\
                    // Use response.id as unique identifier\n\n\
                 4. Check account details:\n\
                    github_get_me({})\n\
                    // Review repos, followers, creation date\n\n\
                 RATE LIMITING:\n\
                 - Authenticated requests: 5,000 per hour\n\
                 - This endpoint uses minimal quota\n\
                 - Check X-RateLimit-Remaining header\n\
                 - Safe to call multiple times\n\n\
                 WHEN TO USE:\n\
                 - Need to know who is authenticated\n\
                 - Getting username for git operations\n\
                 - Displaying current user in UI\n\
                 - Verifying token works\n\
                 - Getting user email for commits\n\
                 - Checking account status",
            ),
        },
    ]
}


