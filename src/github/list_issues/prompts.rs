//! Prompt messages for github_list_issues tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GithubListIssuesPromptArgs;

/// Prompt provider for github_list_issues tool
///
/// This is the ONLY way to provide prompts for github_list_issues - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ListIssuesPrompts;

impl PromptProvider for ListIssuesPrompts {
    type PromptArgs = GithubListIssuesPromptArgs;

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

/// Basic issue listing
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list issues from a GitHub repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_list_issues tool retrieves issues from a repository with powerful filtering, sorting, and pagination options.\n\n\
                 BASIC LISTING:\n\
                 1. All open issues:\n\
                    github_list_issues({\n\
                      \"owner\": \"tokio-rs\",\n\
                      \"repo\": \"tokio\"\n\
                    })\n\n\
                 2. List with pagination:\n\
                    github_list_issues({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"per_page\": 50,\n\
                      \"page\": 1\n\
                    })\n\n\
                 3. All issues (open and closed):\n\
                    github_list_issues({\n\
                      \"owner\": \"actix\",\n\
                      \"repo\": \"actix-web\",\n\
                      \"state\": \"all\"\n\
                    })\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner (username or organization)\n\
                 - repo: Repository name\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - state: \"open\" (default), \"closed\", or \"all\"\n\
                 - page: Page number for pagination (default: 1)\n\
                 - per_page: Results per page, max 100 (default: 30)\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"tokio-rs\",\n\
                   \"repo\": \"tokio\",\n\
                   \"count\": 42,\n\
                   \"issues\": [\n\
                     {\n\
                       \"number\": 5432,\n\
                       \"title\": \"Add support for Unix domain sockets\",\n\
                       \"state\": \"open\",\n\
                       \"author\": \"alice\",\n\
                       \"labels\": [\"enhancement\", \"tokio\"],\n\
                       \"assignees\": [\"bob\"],\n\
                       \"created_at\": \"2024-01-15T10:30:00Z\",\n\
                       \"updated_at\": \"2024-01-20T14:22:00Z\",\n\
                       \"comments_count\": 8,\n\
                       \"html_url\": \"https://github.com/tokio-rs/tokio/issues/5432\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 DEFAULT BEHAVIOR:\n\
                 - Returns open issues only\n\
                 - Sorted by creation date (most recent first)\n\
                 - 30 issues per page\n\
                 - Page 1 by default\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable.\n\
                 - Public repos: Works with any token\n\
                 - Private repos: Token must have repo access\n\n\
                 COMMON PATTERNS:\n\
                 1. Quick overview:\n\
                    github_list_issues({\"owner\": \"vercel\", \"repo\": \"next.js\"})\n\
                 2. Get more results:\n\
                    github_list_issues({\"owner\": \"facebook\", \"repo\": \"react\", \"per_page\": 100})\n\
                 3. Check closed issues:\n\
                    github_list_issues({\"owner\": \"vuejs\", \"repo\": \"vue\", \"state\": \"closed\"})\n\
                 4. Browse pages:\n\
                    github_list_issues({\"owner\": \"microsoft\", \"repo\": \"vscode\", \"page\": 2})",
            ),
        },
    ]
}

/// Filtering issues by state, labels, assignee, milestone
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How can I filter issues by labels, assignees, and other criteria?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "GitHub issue filtering provides powerful options to narrow down results by state, labels, assignees, creators, milestones, and more.\n\n\
                 FILTER BY STATE:\n\
                 1. Open issues only (default):\n\
                    github_list_issues({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"state\": \"open\"\n\
                    })\n\n\
                 2. Closed issues only:\n\
                    github_list_issues({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"state\": \"closed\"\n\
                    })\n\n\
                 3. All issues (open + closed):\n\
                    github_list_issues({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"state\": \"all\"\n\
                    })\n\n\
                 FILTER BY LABELS:\n\
                 1. Single label:\n\
                    github_list_issues({\n\
                      \"owner\": \"tokio-rs\",\n\
                      \"repo\": \"tokio\",\n\
                      \"labels\": [\"bug\"]\n\
                    })\n\n\
                 2. Multiple labels (AND logic - must have all):\n\
                    github_list_issues({\n\
                      \"owner\": \"actix\",\n\
                      \"repo\": \"actix-web\",\n\
                      \"labels\": [\"bug\", \"priority-high\"]\n\
                    })\n\n\
                 3. Combined with state:\n\
                    github_list_issues({\n\
                      \"owner\": \"serde-rs\",\n\
                      \"repo\": \"serde\",\n\
                      \"state\": \"open\",\n\
                      \"labels\": [\"enhancement\", \"help-wanted\"]\n\
                    })\n\n\
                 COMBINING FILTERS:\n\
                 Bug triage example:\n\
                 github_list_issues({\n\
                   \"owner\": \"apache\",\n\
                   \"repo\": \"kafka\",\n\
                   \"state\": \"open\",\n\
                   \"labels\": [\"bug\"]\n\
                 })\n\n\
                 LABEL SYNTAX:\n\
                 - Use exact label names as they appear in GitHub\n\
                 - Labels are case-sensitive\n\
                 - Multiple labels = AND logic (must have all)\n\
                 - Use array format: [\"label1\", \"label2\"]\n\n\
                 BEST PRACTICES:\n\
                 - Start with state filter to reduce result set\n\
                 - Use labels for categorization\n\
                 - Combine state and labels for targeted queries",
            ),
        },
    ]
}
