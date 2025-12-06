//! Prompt messages for github_secret_scanning_alerts tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::SecretScanningAlertsPromptArgs;

/// Prompt provider for secret_scanning_alerts tool
///
/// This is the ONLY way to provide prompts for secret_scanning_alerts - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct SecretScanningAlertsPrompts;

impl PromptProvider for SecretScanningAlertsPrompts {
    type PromptArgs = SecretScanningAlertsPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario type (basic only - shows alert retrieval and filtering)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic alert listing and understanding alert data
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_secret_scanning_alerts to find exposed secrets in my repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_secret_scanning_alerts tool retrieves secret scanning alerts from GitHub Advanced Security, helping you find exposed credentials, API keys, and tokens.\n\n\
                 BASIC ALERT RETRIEVAL:\n\
                 1. Get all alerts for a repository:\n\
                    github_secret_scanning_alerts({\"owner\": \"myorg\", \"repo\": \"myapp\"})\n\n\
                 2. Get only open (unresolved) alerts:\n\
                    github_secret_scanning_alerts({\"owner\": \"company\", \"repo\": \"backend\", \"state\": \"open\"})\n\n\
                 3. Get resolved alerts:\n\
                    github_secret_scanning_alerts({\"owner\": \"team\", \"repo\": \"api\", \"state\": \"resolved\"})\n\n\
                 4. Filter by specific secret type:\n\
                    github_secret_scanning_alerts({\"owner\": \"org\", \"repo\": \"project\", \"secret_type\": \"aws_access_key_id\"})\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner (username or organization)\n\
                 - repo: Repository name\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - state: Filter by state (\"open\" or \"resolved\")\n\
                 - secret_type: Filter by secret type (e.g., \"github_token\", \"aws_access_key_id\", \"azure_devops_personal_access_token\")\n\
                 - resolution: Filter by resolution type (\"false_positive\", \"wont_fix\", \"revoked\", \"used_in_tests\")\n\
                 - page: Page number for pagination (default: 1)\n\
                 - per_page: Results per page (max 100, default: 30)\n\n\
                 UNDERSTANDING THE RESPONSE:\n\
                 The tool returns a JSON object with:\n\
                 - success: Boolean indicating if request succeeded\n\
                 - owner: Repository owner\n\
                 - repo: Repository name\n\
                 - total_count: Number of alerts returned\n\
                 - alerts: Array of alert objects\n\n\
                 ALERT OBJECT STRUCTURE:\n\
                 Each alert contains:\n\
                 - number: Unique alert identifier (use for updates)\n\
                 - state: \"open\" or \"resolved\"\n\
                 - secret_type: Type of secret detected (e.g., \"aws_access_key_id\", \"github_token\")\n\
                 - secret_type_display_name: Human-readable secret type name\n\
                 - secret: Partially redacted secret value (for verification)\n\
                 - resolution: Why alert was resolved (\"false_positive\", \"wont_fix\", \"revoked\", \"used_in_tests\")\n\
                 - created_at: When secret was first detected (ISO 8601 timestamp)\n\
                 - resolved_at: When alert was resolved (null if open)\n\
                 - resolved_by: User who resolved the alert (null if open)\n\
                 - html_url: GitHub web UI link to alert details\n\
                 - locations_url: API endpoint for secret locations\n\
                 - locations: Array of file locations where secret appears:\n\
                   - type: Location type (usually \"commit\")\n\
                   - details:\n\
                     - path: File path in repository\n\
                     - start_line: Line number where secret starts\n\
                     - end_line: Line number where secret ends\n\
                     - start_column: Column where secret starts\n\
                     - end_column: Column where secret ends\n\
                     - blob_sha: Git blob SHA for the file\n\
                     - blob_url: API URL for blob contents\n\
                     - commit_sha: Commit where secret was introduced\n\
                     - commit_url: API URL for commit details\n\n\
                 COMMON SECRET TYPES:\n\
                 - github_token: GitHub personal access tokens, OAuth tokens\n\
                 - aws_access_key_id: AWS access keys\n\
                 - aws_secret_access_key: AWS secret keys\n\
                 - azure_devops_personal_access_token: Azure DevOps PATs\n\
                 - google_api_key: Google API keys\n\
                 - slack_api_token: Slack API tokens\n\
                 - stripe_api_key: Stripe API keys\n\
                 - npm_access_token: NPM authentication tokens\n\
                 - private_key_pem: Private keys in PEM format\n\
                 - ssh_private_key: SSH private keys\n\n\
                 PAGINATION:\n\
                 For repositories with many alerts:\n\
                 1. First page (default):\n\
                    github_secret_scanning_alerts({\"owner\": \"org\", \"repo\": \"project\", \"per_page\": 100})\n\
                 2. Subsequent pages:\n\
                    github_secret_scanning_alerts({\"owner\": \"org\", \"repo\": \"project\", \"per_page\": 100, \"page\": 2})\n\
                 3. Continue until total_count < per_page or alerts array is empty\n\n\
                 AUTHENTICATION REQUIREMENTS:\n\
                 - Requires GITHUB_TOKEN environment variable\n\
                 - Token must have these scopes:\n\
                   - security_events: Required for secret scanning API access\n\
                   - repo: Required for private repository access\n\
                 - Repository must have GitHub Advanced Security enabled\n\
                 - You must have admin or security manager role in the organization\n\n\
                 EXAMPLE WORKFLOW:\n\
                 1. Check for open alerts:\n\
                    github_secret_scanning_alerts({\"owner\": \"mycompany\", \"repo\": \"backend-api\", \"state\": \"open\"})\n\
                 2. Review alert details:\n\
                    - Check secret_type to understand what was exposed\n\
                    - Review locations to see where secret appears\n\
                    - Check created_at to assess exposure duration\n\
                 3. Take action based on findings (see remediation scenario)\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found:\n\
                    - Repository doesn't exist or you don't have access\n\
                    - Verify owner/repo names are correct\n\
                    - Check GITHUB_TOKEN has repo scope\n\
                 2. 403 Forbidden:\n\
                    - Missing security_events scope on token\n\
                    - Repository doesn't have Advanced Security enabled\n\
                    - Insufficient permissions (need admin or security role)\n\
                 3. 503 Service Unavailable:\n\
                    - Secret scanning service temporarily unavailable\n\
                    - Retry with exponential backoff\n\n\
                 RATE LIMITING:\n\
                 - Authenticated requests: 5,000 per hour\n\
                 - Check response headers:\n\
                   - X-RateLimit-Limit: Total requests allowed\n\
                   - X-RateLimit-Remaining: Requests remaining\n\
                   - X-RateLimit-Reset: When limit resets (Unix timestamp)\n\n\
                 BEST PRACTICES FOR LISTING:\n\
                 - Always filter by state: \"open\" for active incidents\n\
                 - Use per_page: 100 for efficient pagination\n\
                 - Filter by secret_type for targeted investigations\n\
                 - Check locations array to understand exposure scope\n\
                 - Review created_at to prioritize recent exposures\n\
                 - Use html_url for detailed investigation in GitHub UI",
            ),
        },
    ]
}
