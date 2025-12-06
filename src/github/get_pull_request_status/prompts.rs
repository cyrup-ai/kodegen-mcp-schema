//! Prompt messages for github_get_pull_request_status tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetPullRequestStatusPromptArgs;

/// Prompt provider for get_pull_request_status tool
///
/// This is the ONLY way to provide prompts for get_pull_request_status - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GetPullRequestStatusPrompts;

impl PromptProvider for GetPullRequestStatusPrompts {
    type PromptArgs = GetPullRequestStatusPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("merge_decision") => Self::prompt_merge_decision(),
            _ => Self::prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, merge_decision)".to_string()),
                required: Some(false),
            }
        ]
    }
}

impl GetPullRequestStatusPrompts {
    fn prompt_basic() -> Vec<PromptMessage> {
        vec![
            PromptMessage {
                role: PromptMessageRole::User,
                content: PromptMessageContent::text(
                    "How do I use github_get_pull_request_status to get PR information?",
                ),
            },
            PromptMessage {
                role: PromptMessageRole::Assistant,
                content: PromptMessageContent::text(
                    "The github_get_pull_request_status tool retrieves pull request status including state, checks, \
                     and mergeability information.\n\n\
                     BASIC USAGE:\n\
                     1. Check PR status:\n\
                        github_get_pull_request_status({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"pull_number\": 5678})\n\n\
                     2. Verify PR is open and check CI:\n\
                        github_get_pull_request_status({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"pull_number\": 123})\n\n\
                     3. Monitor PR state changes:\n\
                        github_get_pull_request_status({\"owner\": \"actix\", \"repo\": \"actix-web\", \"pull_number\": 999})\n\n\
                     PARAMETERS:\n\
                     - owner (required): Repository owner (username or organization)\n\
                     - repo (required): Repository name\n\
                     - pull_number (required): Pull request number\n\n\
                     REQUIRED: GITHUB_TOKEN environment variable with scopes:\n\
                     - repo (for private repositories)\n\
                     - public_repo (for public repositories only)\n\n\
                     RESPONSE FIELDS:\n\
                     - success: true/false - Operation succeeded\n\
                     - state: \"open\", \"closed\", or \"merged\" - PR state\n\
                     - merged: true/false - If PR was merged\n\
                     - mergeable: true/false/null - Can merge (null = calculating)\n\
                     - mergeable_state: \"clean\", \"dirty\", \"blocked\", \"unstable\", \"behind\" - Merge status\n\
                     - draft: true/false - Is draft PR\n\
                     - checks_status: \"success\", \"pending\", \"failure\" - Combined CI status\n\
                     - review_decision: \"APPROVED\", \"CHANGES_REQUESTED\", \"REVIEW_REQUIRED\"\n\
                     - title: Pull request title\n\
                     - head: Source branch SHA and ref\n\
                     - base: Target branch SHA and ref\n\
                     - created_at, updated_at: ISO timestamps\n\
                     - html_url: Link to PR on GitHub\n\n\
                     WHEN TO USE:\n\
                     - Always read PR status BEFORE attempting any merge operations\n\
                     - Check mergeable_state to diagnose why PR can't merge\n\
                     - Monitor checks_status while waiting for CI to complete\n\
                     - Verify review_decision to ensure approval requirements met\n\
                     - Compare head/base SHAs to confirm PR not stale\n\n\
                     COMMON PATTERNS:\n\
                     - Get fresh PR state: Call with owner, repo, pull_number\n\
                     - Check if ready to merge: mergeable=true AND checks_status=\"success\" AND review_decision=\"APPROVED\"\n\
                     - Handle mergeable=null: GitHub calculating; retry after 1-2 seconds\n\
                     - Interpret mergeable_state: \"behind\"=update branch, \"dirty\"=resolve conflicts, \"blocked\"=check requirements\n\n\
                     RATE LIMITS:\n\
                     - Authenticated: 5,000 requests/hour\n\
                     - Unauthenticated: 60 requests/hour\n\
                     - Monitor X-RateLimit-Remaining response header\n\n\
                     COMMON ERRORS:\n\
                     - 404 Not Found: PR or repository doesn't exist - verify owner/repo/pull_number\n\
                     - 403 Forbidden: No access to private repository - verify GITHUB_TOKEN has repo scope\n\
                     - mergeable=null: GitHub still calculating - normal for new PRs, wait and retry",
                ),
            },
        ]
    }

    fn prompt_merge_decision() -> Vec<PromptMessage> {
        vec![
            PromptMessage {
                role: PromptMessageRole::User,
                content: PromptMessageContent::text(
                    "How do I use github_get_pull_request_status to determine if a PR is ready to merge?",
                ),
            },
            PromptMessage {
                role: PromptMessageRole::Assistant,
                content: PromptMessageContent::text(
                    "Use github_get_pull_request_status to evaluate merge readiness through workflows and \
                     decision trees.\n\n\
                     AUTO-MERGE ELIGIBILITY CHECK:\n\
                     1. Get PR status: Call github_get_pull_request_status({\"owner\": \"...\", \"repo\": \"...\", \"pull_number\": ...})\n\
                     2. Verify state open: Check response.state == \"open\" (not \"closed\" or \"merged\")\n\
                     3. Verify mergeable: Check response.mergeable == true (false = conflicts, null = wait)\n\
                     4. Verify CI passed: Check response.checks_status == \"success\"\n\
                     5. Verify approved: Check response.review_decision == \"APPROVED\"\n\
                     6. Skip drafts: Check response.draft == false (don't merge WIPs)\n\
                     7. Proceed: If all pass, PR is safe to merge\n\n\
                     CI/CD MONITORING PATTERN:\n\
                     - Poll PR status periodically during CI runs\n\
                     - Wait for checks_status: \"pending\" → \"success\" or \"failure\"\n\
                     - Alert team on \"failure\" - CI broken\n\
                     - Track time from creation to green status\n\
                     - Cancel polling when state != \"open\"\n\n\
                     BRANCH CONFLICT RESOLUTION:\n\
                     - If mergeable_state == \"behind\": Update base branch from remote\n\
                     - If mergeable_state == \"dirty\": Resolve merge conflicts in source branch\n\
                     - If mergeable_state == \"blocked\": Check branch protection rules required\n\
                     - Retry after fixes - mergeable_state recalculates\n\n\
                     BEST PRACTICES:\n\
                     - Always check mergeable BEFORE merge operations\n\
                     - Handle mergeable=null by retrying after 1-2 second delay\n\
                     - Use mergeable_state for detailed diagnosis (not just boolean)\n\
                     - Verify checks_status before auto-merge\n\
                     - Prevent merging draft PRs (check draft field)\n\
                     - Require approval (review_decision == \"APPROVED\")\n\
                     - Compare head/base SHAs - if different, PR has new commits\n\
                     - Use state field to ignore closed/merged PRs\n\
                     - Implement exponential backoff (1s, 2s, 4s) when polling\n\
                     - Set timeout: If mergeable null after 30s, fail safe\n\n\
                     DECISION TREE:\n\
                     Is state \"open\"? → NO: Already merged or closed, skip merge attempt\n\
                     Is draft true? → YES: WIP PR, stop, notify author\n\
                     Is mergeable null? → YES: Still calculating, wait 2 seconds and retry\n\
                     Is mergeable true? → NO: Has conflicts, notify to fix\n\
                     Is checks_status \"success\"? → NO: CI failed, alert team\n\
                     Is review_decision \"APPROVED\"? → NO: Missing approval, add to queue\n\
                     All passed? → YES: Safe to merge",
                ),
            },
        ]
    }
}
