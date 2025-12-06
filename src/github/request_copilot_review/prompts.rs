//! Prompt messages for github_request_copilot_review tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::RequestCopilotReviewPromptArgs;

/// Prompt provider for request_copilot_review tool
///
/// This is the ONLY way to provide prompts for request_copilot_review - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct RequestCopilotReviewPrompts;

impl PromptProvider for RequestCopilotReviewPrompts {
    type PromptArgs = RequestCopilotReviewPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_request_copilot_review to get AI code reviews?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_request_copilot_review tool requests an AI-powered code review from GitHub Copilot for a pull request.\n\n\
                 BASIC USAGE:\n\
                 1. Request review for open PR:\n\
                    github_request_copilot_review({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"pull_number\": 5678})\n\n\
                 2. Review with custom context:\n\
                    github_request_copilot_review({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"pull_number\": 123, \"context\": \"Focus on performance and security\"})\n\n\
                 3. Review specific aspects:\n\
                    github_request_copilot_review({\"owner\": \"actix\", \"repo\": \"actix-web\", \"pull_number\": 999, \"context\": \"Check for memory safety and concurrency issues\"})\n\n\
                 4. General code review:\n\
                    github_request_copilot_review({\"owner\": \"serde-rs\", \"repo\": \"serde\", \"pull_number\": 456})\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - pull_number (required): Pull request number to review\n\
                 - context (optional): Additional context or focus areas for review\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo (for private repositories)\n\
                 - public_repo (for public repositories only)\n\
                 Note: Repository must have GitHub Copilot access enabled\n\n\
                 RESPONSE:\n\
                 Returns JSON with:\n\
                 - success: true/false\n\
                 - owner, repo: Repository identifiers\n\
                 - pr_number: Pull request number\n\
                 - review_id: Generated review identifier\n\
                 - status: Review request status\n\
                 - message: Status message\n\n\
                 COMMON WORKFLOWS:\n\
                 1. Automated initial review:\n\
                    - New PR created\n\
                    - Request Copilot review automatically\n\
                    - Get quick feedback on common issues\n\
                    - Human reviewers focus on complex logic\n\
                 2. Security-focused review:\n\
                    - Request review with security context\n\
                    - Check for vulnerabilities\n\
                    - Identify unsafe patterns\n\
                    - Supplement manual security audit\n\
                 3. Best practices check:\n\
                    - Request review for style/patterns\n\
                    - Get suggestions for improvements\n\
                    - Identify code smells\n\
                    - Learn from AI recommendations\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Copilot reviews may have additional usage limits\n\
                 - Check X-RateLimit-Remaining header\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found: PR or repository doesn't exist\n\
                    Fix: Verify owner/repo/pull_number are correct\n\
                 2. 403 Forbidden: Copilot not enabled or insufficient permissions\n\
                    Fix: Enable GitHub Copilot for repository; verify token scopes\n\
                 3. 422 Unprocessable: PR already has pending Copilot review\n\
                    Fix: Wait for current review to complete before requesting new one\n\n\
                 BEST PRACTICES:\n\
                 - Use for preliminary automated reviews\n\
                 - Provide specific context for targeted feedback\n\
                 - Don't replace human code reviews entirely\n\
                 - Request early in PR lifecycle\n\
                 - Review Copilot suggestions critically\n\
                 - Use to catch common issues quickly\n\
                 - Combine with automated tests and linters\n\
                 - Specify focus areas in context parameter\n\
                 - Use for educational purposes (learning patterns)\n\
                 - Supplement with human expertise for architecture decisions",
            ),
        },    ]
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]  // No customization arguments for this tool
    }
}

