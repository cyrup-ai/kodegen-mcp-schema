//! Prompt messages for github_create_pull_request_review tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::CreatePullRequestReviewPromptArgs;

/// Prompt provider for create_pull_request_review tool
///
/// This is the ONLY way to provide prompts for create_pull_request_review - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct CreatePullRequestReviewPrompts;

impl PromptProvider for CreatePullRequestReviewPrompts {
    type PromptArgs = CreatePullRequestReviewPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("inline_comments") => prompt_inline_comments(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, inline_comments, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO REVIEW PULL REQUESTS
// ============================================================================

/// Basic PR review with event types (APPROVE, REQUEST_CHANGES, COMMENT)
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I review pull requests with different approval statuses?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_create_pull_request_review tool submits reviews with three event types.\n\n\
                 EVENT TYPES:\n\
                 - APPROVE: Approves changes, unblocks merge\n\
                 - REQUEST_CHANGES: Requests changes, blocks merge\n\
                 - COMMENT: Adds feedback without blocking\n\n\
                 APPROVE EXAMPLE:\n\
                 Use when code is production-ready:\n\
                 github_create_pull_request_review({\n\
                     \"owner\": \"tokio-rs\",\n\
                     \"repo\": \"tokio\",\n\
                     \"pull_number\": 5678,\n\
                     \"event\": \"APPROVE\",\n\
                     \"body\": \"LGTM! Excellent async stream implementation.\"\n\
                 })\n\n\
                 REQUEST_CHANGES EXAMPLE:\n\
                 Use for blocking issues:\n\
                 github_create_pull_request_review({\n\
                     \"owner\": \"rust-lang\",\n\
                     \"repo\": \"rust\",\n\
                     \"pull_number\": 123,\n\
                     \"event\": \"REQUEST_CHANGES\",\n\
                     \"body\": \"Please address:\\n1. Unsafe block needs safety docs\\n2. Add error handling\\n3. Include tests\"\n\
                 })\n\n\
                 COMMENT EXAMPLE:\n\
                 Use for non-blocking feedback:\n\
                 github_create_pull_request_review({\n\
                     \"owner\": \"actix\",\n\
                     \"repo\": \"actix-web\",\n\
                     \"pull_number\": 999,\n\
                     \"event\": \"COMMENT\",\n\
                     \"body\": \"Optional: consider extracting validation logic.\"\n\
                 })\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner\n\
                 - repo: Repository name\n\
                 - pull_number: PR number\n\
                 - event: \"APPROVE\", \"REQUEST_CHANGES\", or \"COMMENT\"\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - body: Review summary (recommended)\n\
                 - commit_id: Specific commit SHA\n\
                 - comments: Inline comments (see inline_comments scenario)\n\n\
                 RESPONSE:\n\
                 {\"success\": true, \"review_id\": 123456, \"state\": \"APPROVED\"}\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN with repo or public_repo scope.\n\n\
                 COMMON ERRORS:\n\
                 1. 403: Cannot approve own PR\n\
                 2. 422: Already reviewed (dismiss previous first)\n\
                 3. 404: PR doesn't exist\n\n\
                 BEST PRACTICES:\n\
                 - Use APPROVE only when thoroughly reviewed\n\
                 - REQUEST_CHANGES requires clear, actionable feedback\n\
                 - COMMENT for suggestions and draft PRs\n\
                 - Always provide body explaining your decision\n\
                 - Be specific and constructive\n\n\
                 DECISION TREE:\n\
                 - Blocking issues? → REQUEST_CHANGES\n\
                 - Production-ready? → APPROVE\n\
                 - Just feedback? → COMMENT",
            ),
        },
    ]
}

/// Inline comments on specific lines in PR diffs
fn prompt_inline_comments() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I add inline comments to specific lines in a pull request review?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Inline comments provide line-specific feedback in PR diffs.\n\n\
                 SINGLE COMMENT EXAMPLE:\n\
                 github_create_pull_request_review({\n\
                     \"owner\": \"serde-rs\",\n\
                     \"repo\": \"serde\",\n\
                     \"pull_number\": 456,\n\
                     \"event\": \"COMMENT\",\n\
                     \"body\": \"One suggestion on error handling.\",\n\
                     \"comments\": [{\n\
                         \"path\": \"src/de/mod.rs\",\n\
                         \"line\": 15,\n\
                         \"body\": \"Use map_err to provide context about which field failed.\"\n\
                     }]\n\
                 })\n\n\
                 MULTIPLE COMMENTS EXAMPLE:\n\
                 github_create_pull_request_review({\n\
                     \"owner\": \"rust-lang\",\n\
                     \"repo\": \"rust\",\n\
                     \"pull_number\": 789,\n\
                     \"event\": \"REQUEST_CHANGES\",\n\
                     \"body\": \"Found several issues.\",\n\
                     \"comments\": [\n\
                         {\n\
                             \"path\": \"src/lib.rs\",\n\
                             \"line\": 42,\n\
                             \"body\": \"Unsafe block needs safety documentation.\"\n\
                         },\n\
                         {\n\
                             \"path\": \"tests/integration.rs\",\n\
                             \"line\": 23,\n\
                             \"body\": \"Add test for empty input.\"\n\
                         }\n\
                     ]\n\
                 })\n\n\
                 COMMENT STRUCTURE:\n\
                 - path (required): File path relative to repo root\n\
                 - line (required): Line number in diff\n\
                 - body (required): Comment text\n\
                 - side (optional): \"LEFT\" or \"RIGHT\" (default: \"RIGHT\")\n\n\
                 LEFT VS RIGHT:\n\
                 - RIGHT: New code (after changes) - default\n\
                 - LEFT: Old code (before changes)\n\n\
                 Example on deleted code:\n\
                 {\"path\": \"src/old.rs\", \"line\": 10, \"body\": \"Why was this removed?\", \"side\": \"LEFT\"}\n\n\
                 LINE NUMBERS:\n\
                 CRITICAL: Line numbers are diff positions, not file line numbers.\n\
                 - Count lines within each diff hunk\n\
                 - Each hunk starts at line 1\n\
                 - Include context lines\n\n\
                 Use github_get_pull_request_files to see diff structure.\n\n\
                 TARGETING COMMITS:\n\
                 Review specific commit with commit_id:\n\
                 github_create_pull_request_review({\n\
                     \"owner\": \"tokio-rs\",\n\
                     \"repo\": \"tokio\",\n\
                     \"pull_number\": 100,\n\
                     \"commit_id\": \"abc123\",\n\
                     \"event\": \"COMMENT\",\n\
                     \"comments\": [{\"path\": \"src/runtime.rs\", \"line\": 5, \"body\": \"Clever!\"}]\n\
                 })\n\n\
                 BEST PRACTICES:\n\
                 1. Be specific: \"Add bounds check\" not \"This is wrong\"\n\
                 2. Explain why: \"Creates memory leak because buffer never freed\"\n\
                 3. Suggest solutions: \"Use Result::map_err to preserve context\"\n\
                 4. Include code examples when helpful\n\
                 5. Keep tone constructive\n\
                 6. Acknowledge good code too\n\
                 7. One issue per comment\n\n\
                 COMMON ERRORS:\n\
                 1. Invalid line (422): Line doesn't exist in diff\n\
                 2. File not found (422): Path doesn't match changed files\n\
                 3. Unchanged line: Can only comment on changed lines",
            ),
        },
    ]
}

/// Automated workflows and integration patterns
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows for automating pull request reviews?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Automated reviews provide consistent code quality checks.\n\n\
                 AUTOMATED CODE REVIEW:\n\
                 1. Get files: github_get_pull_request_files\n\
                 2. Analyze for issues\n\
                 3. Collect inline comments\n\
                 4. Submit review:\n\n\
                 github_create_pull_request_review({\n\
                     \"owner\": \"org\",\n\
                     \"repo\": \"project\",\n\
                     \"pull_number\": 123,\n\
                     \"event\": has_critical_issues ? \"REQUEST_CHANGES\" : \"APPROVE\",\n\
                     \"body\": \"Automated review: 15 files analyzed, 2 issues found\",\n\
                     \"comments\": [\n\
                         {\"path\": \"src/auth.rs\", \"line\": 42, \"body\": \"SQL injection risk: use parameterized queries\"},\n\
                         {\"path\": \"src/api.rs\", \"line\": 15, \"body\": \"Add error handling\"}\n\
                     ]\n\
                 })\n\n\
                 SECURITY REVIEW PATTERN:\n\
                 Scan for vulnerabilities and block on critical issues:\n\
                 github_create_pull_request_review({\n\
                     \"owner\": \"company\",\n\
                     \"repo\": \"webapp\",\n\
                     \"pull_number\": 456,\n\
                     \"event\": \"REQUEST_CHANGES\",\n\
                     \"body\": \"🔒 Security issues must be fixed:\\n- 1 SQL injection\\n- 2 hardcoded credentials\",\n\
                     \"comments\": [\n\
                         {\n\
                             \"path\": \"src/db.rs\",\n\
                             \"line\": 28,\n\
                             \"body\": \"SQL injection: use sqlx::query($1).bind(user_id)\"\n\
                         },\n\
                         {\n\
                             \"path\": \"src/config.rs\",\n\
                             \"line\": 10,\n\
                             \"body\": \"Hardcoded password: use env::var(DB_PASSWORD)\"\n\
                         }\n\
                     ]\n\
                 })\n\n\
                 DOCUMENTATION REVIEW:\n\
                 Check for missing docs:\n\
                 github_create_pull_request_review({\n\
                     \"owner\": \"org\",\n\
                     \"repo\": \"lib\",\n\
                     \"pull_number\": 789,\n\
                     \"event\": missing_docs ? \"REQUEST_CHANGES\" : \"APPROVE\",\n\
                     \"body\": \"📚 Documentation \" + (missing_docs ? \"incomplete\" : \"complete\"),\n\
                     \"comments\": [{\n\
                         \"path\": \"src/lib.rs\",\n\
                         \"line\": 45,\n\
                         \"body\": \"Add doc comment with params and return value\"\n\
                     }]\n\
                 })\n\n\
                 INTEGRATION PATTERN:\n\
                 // Get PR and files\n\
                 let files = github_get_pull_request_files({owner, repo, pull_number});\n\
                 // Analyze and review\n\
                 let review = github_create_pull_request_review({...});\n\
                 // Request human review if complex\n\
                 if complex { github_request_reviews({reviewers: [\"senior-dev\"]}); }\n\n\
                 ERROR HANDLING:\n\
                 1. Already reviewed (422): Dismiss previous or update\n\
                 2. Invalid line (422): Verify against diff\n\
                 3. Rate limit (429): Check X-RateLimit-Remaining, backoff\n\n\
                 BEST PRACTICES:\n\
                 - Clear bot identification: \"🤖 Automated Review\"\n\
                 - Actionable feedback with code examples\n\
                 - Use severity indicators: 🔴 Critical, ⚠️ Warning, 💡 Suggestion\n\
                 - Avoid false positives\n\
                 - Don't override human reviews\n\
                 - Handle drafts differently (use COMMENT)\n\
                 - Provide metrics: files analyzed, issues found\n\
                 - Keep concise, prioritize critical issues",
            ),
        },
    ]
}
