//! Prompt messages for github_code_scanning_alerts tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::CodeScanningAlertsPromptArgs;

/// Prompt provider for code_scanning_alerts tool
///
/// This is the ONLY way to provide prompts for code_scanning_alerts - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct CodeScanningAlertsPrompts;

impl PromptProvider for CodeScanningAlertsPrompts {
    type PromptArgs = CodeScanningAlertsPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            _ => prompt_filtering(),
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
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE CODE SCANNING ALERTS
// ============================================================================

/// Basic code scanning alerts listing and details
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list and view GitHub code scanning alerts?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_code_scanning_alerts tool retrieves security alerts from GitHub Advanced Security (CodeQL, third-party scanners). Here's how to list and view alerts:\n\n\
                 LISTING CODE SCANNING ALERTS:\n\
                 1. Get all alerts for a repository:\n\
                    github_code_scanning_alerts({\n\
                        \"owner\": \"microsoft\",\n\
                        \"repo\": \"vscode\"\n\
                    })\n\n\
                 2. Get specific alert by number:\n\
                    github_code_scanning_alerts({\n\
                        \"owner\": \"microsoft\",\n\
                        \"repo\": \"vscode\",\n\
                        \"alert_number\": 42\n\
                    })\n\n\
                 3. Get alerts for specific branch:\n\
                    github_code_scanning_alerts({\n\
                        \"owner\": \"facebook\",\n\
                        \"repo\": \"react\",\n\
                        \"ref\": \"main\"\n\
                    })\n\n\
                 4. Paginate through results:\n\
                    github_code_scanning_alerts({\n\
                        \"owner\": \"rust-lang\",\n\
                        \"repo\": \"rust\",\n\
                        \"per_page\": 50,\n\
                        \"page\": 1\n\
                    })\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner (username or organization)\n\
                 - repo: Repository name\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - alert_number: Get specific alert details\n\
                 - ref: Git reference (branch, tag, commit SHA)\n\
                 - page: Page number (default: 1)\n\
                 - per_page: Results per page (max: 100, default: 30)\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"microsoft\",\n\
                   \"repo\": \"vscode\",\n\
                   \"total_count\": 15,\n\
                   \"alerts\": [\n\
                     {\n\
                       \"number\": 1,\n\
                       \"state\": \"open\",\n\
                       \"rule\": {\n\
                         \"id\": \"js/sql-injection\",\n\
                         \"severity\": \"high\",\n\
                         \"description\": \"SQL Injection vulnerability\",\n\
                         \"name\": \"SQL injection\"\n\
                       },\n\
                       \"tool\": {\n\
                         \"name\": \"CodeQL\",\n\
                         \"version\": \"2.15.0\"\n\
                       },\n\
                       \"most_recent_instance\": {\n\
                         \"location\": {\n\
                           \"path\": \"src/db/query.js\",\n\
                           \"start_line\": 42,\n\
                           \"end_line\": 42\n\
                         },\n\
                         \"message\": {\n\
                           \"text\": \"Unsafe SQL query construction\"\n\
                         }\n\
                       },\n\
                       \"created_at\": \"2024-01-15T10:30:00Z\",\n\
                       \"updated_at\": \"2024-01-15T10:30:00Z\",\n\
                       \"html_url\": \"https://github.com/microsoft/vscode/security/code-scanning/1\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 WHAT'S REPORTED:\n\
                 - Alert number: Unique identifier for the alert\n\
                 - State: open, fixed, dismissed, closed\n\
                 - Rule information: Vulnerability type, severity, description\n\
                 - Tool information: Analysis tool name and version\n\
                 - Location: File path and line numbers where issue found\n\
                 - Message: Detailed explanation of the security issue\n\
                 - Timestamps: When alert was created and last updated\n\
                 - URL: Direct link to view alert on GitHub\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with:\n\
                 - security_events scope (required for code scanning)\n\
                 - repo scope (for private repositories)\n\
                 Repository must have GitHub Advanced Security enabled\n\n\
                 COMMON USE CASES:\n\
                 1. Security dashboard - List all alerts to see security posture\n\
                 2. Alert investigation - Get specific alert details for triage\n\
                 3. Branch analysis - Check security status of specific branch",
            ),
        },
    ]
}

/// Filtering alerts by severity, state, and other criteria
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I filter code scanning alerts by severity, state, or other criteria?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use filter parameters to narrow down code scanning alerts by severity, state, and tool.\n\n\
                 FILTERING BY SEVERITY:\n\
                 1. Critical issues only:\n\
                    github_code_scanning_alerts({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"myapp\",\n\
                        \"severity\": \"critical\"\n\
                    })\n\n\
                 2. High severity issues:\n\
                    github_code_scanning_alerts({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"myapp\",\n\
                        \"severity\": \"high\"\n\
                    })\n\n\
                 3. Combine severity with state:\n\
                    github_code_scanning_alerts({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"myapp\",\n\
                        \"severity\": \"critical\",\n\
                        \"state\": \"open\"\n\
                    })\n\n\
                 SEVERITY LEVELS:\n\
                 - critical: Immediate action required\n\
                 - high: Address promptly\n\
                 - medium: Should fix\n\
                 - low: Consider fixing\n\n\
                 FILTERING BY STATE:\n\
                 1. Open alerts:\n\
                    github_code_scanning_alerts({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"myapp\",\n\
                        \"state\": \"open\"\n\
                    })\n\n\
                 2. Fixed alerts:\n\
                    github_code_scanning_alerts({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"myapp\",\n\
                        \"state\": \"fixed\"\n\
                    })\n\n\
                 STATE MEANINGS:\n\
                 - open: Active alert requiring attention\n\
                 - fixed: Code changed to resolve vulnerability\n\
                 - dismissed: Marked as false positive\n\
                 - closed: Either fixed or dismissed\n\n\
                 FILTERING BY TOOL:\n\
                 1. CodeQL alerts:\n\
                    github_code_scanning_alerts({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"myapp\",\n\
                        \"tool_name\": \"CodeQL\"\n\
                    })\n\n\
                 2. Third-party scanner:\n\
                    github_code_scanning_alerts({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"myapp\",\n\
                        \"tool_name\": \"Snyk\"\n\
                    })",
            ),
        },
    ]
}
