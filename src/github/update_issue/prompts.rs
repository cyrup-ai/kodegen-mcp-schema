//! Prompt messages for github_update_issue tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::UpdateIssuePromptArgs;

/// Prompt provider for update_issue tool
///
/// This is the ONLY way to provide prompts for update_issue - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct UpdateIssuePrompts;

impl PromptProvider for UpdateIssuePrompts {
    type PromptArgs = UpdateIssuePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("metadata") => prompt_metadata(),
            _ => prompt_state(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (state, metadata)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO UPDATE GITHUB ISSUES
// ============================================================================

/// State management - opening and closing issues
fn prompt_state() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I change the state of GitHub issues using github_update_issue?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_update_issue tool can change issue state between open and closed. Here's how to manage issue states effectively:\n\n\
                 CHANGING ISSUE STATE:\n\n\
                 STATE VALUES:\n\
                 - \"open\": Issue is active and needs attention\n\
                 - \"closed\": Issue is resolved, completed, or no longer relevant\n\n\
                 CLOSING AN ISSUE:\n\
                 1. Simple close:\n\
                    github_update_issue({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"issue_number\": 12345,\n\
                      \"state\": \"closed\"\n\
                    })\n\n\
                 2. Close with explanation:\n\
                    // First add a comment explaining why\n\
                    github_add_issue_comment({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"issue_number\": 12345,\n\
                      \"body\": \"Fixed in PR #67890\"\n\
                    })\n\
                    // Then close the issue\n\
                    github_update_issue({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"issue_number\": 12345,\n\
                      \"state\": \"closed\"\n\
                    })\n\n\
                 3. Close and update body:\n\
                    github_update_issue({\n\
                      \"owner\": \"tokio-rs\",\n\
                      \"repo\": \"tokio\",\n\
                      \"issue_number\": 5432,\n\
                      \"state\": \"closed\",\n\
                      \"body\": \"## Resolution\\n\\nFixed in v1.25.0\\n\\nSee PR #6789 for details.\"\n\
                    })\n\n\
                 REOPENING AN ISSUE:\n\
                 1. Reopen with reason:\n\
                    github_update_issue({\n\
                      \"owner\": \"actix\",\n\
                      \"repo\": \"actix-web\",\n\
                      \"issue_number\": 2345,\n\
                      \"state\": \"open\"\n\
                    })\n\
                    github_add_issue_comment({\n\
                      \"owner\": \"actix\",\n\
                      \"repo\": \"actix-web\",\n\
                      \"issue_number\": 2345,\n\
                      \"body\": \"Reopening: Issue still occurs in v4.0.1\"\n\
                    })\n\n\
                 2. Reopen with updated info:\n\
                    github_update_issue({\n\
                      \"owner\": \"serde-rs\",\n\
                      \"repo\": \"serde\",\n\
                      \"issue_number\": 1234,\n\
                      \"state\": \"open\",\n\
                      \"title\": \"[REOPENED] Deserialization fails with nested enums\",\n\
                      \"labels\": [\"bug\", \"regression\", \"needs-investigation\"]\n\
                    })\n\n\
                 BEST PRACTICES:\n\
                 - ALWAYS add a comment explaining why you're closing or reopening\n\
                 - Update labels when changing state (e.g., add 'fixed', 'wontfix', 'duplicate')\n\
                 - Link to related PRs, commits, or issues in close comments\n\
                 - When reopening, explain what changed or what new information emerged\n\
                 - Use descriptive labels to categorize closures (fixed/wontfix/duplicate/stale)\n\
                 - Consider updating the issue body with resolution details\n\
                 - For duplicate issues, link to the canonical issue\n\
                 - Document version numbers when closing bug reports\n\n\
                 PERMISSIONS REQUIRED:\n\
                 - Write access to the repository\n\
                 - GITHUB_TOKEN with 'repo' scope (private repos) or 'public_repo' (public repos)\n\n\
                 STATE CHANGE WORKFLOW:\n\
                 1. Verify issue number and repository are correct\n\
                 2. Add explanatory comment first (github_add_issue_comment)\n\
                 3. Update state with github_update_issue\n\
                 4. Update labels to reflect new state if needed\n\
                 5. Verify success by checking the response\n\n\
                 ERROR HANDLING:\n\
                 - 404: Issue or repository doesn't exist\n\
                 - 403: Insufficient permissions to update issue\n\
                 - 422: Invalid state value (must be 'open' or 'closed')\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"rust-lang\",\n\
                   \"repo\": \"rust\",\n\
                   \"issue_number\": 12345,\n\
                   \"message\": \"Issue updated successfully\"\n\
                 }",
            ),
        },
    ]
}

/// Metadata updates - labels, assignees, milestones
fn prompt_metadata() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I update issue metadata like labels, assignees, and milestones?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_update_issue tool lets you update labels, assignees, and milestones. These metadata updates help organize and track issues.\n\n\
                 UPDATING LABELS:\n\n\
                 IMPORTANT: Labels array REPLACES all existing labels (not additive)\n\n\
                 1. Add single label:\n\
                    github_update_issue({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"issue_number\": 12345,\n\
                      \"labels\": [\"bug\"]\n\
                    })\n\n\
                 2. Add multiple labels:\n\
                    github_update_issue({\n\
                      \"owner\": \"tokio-rs\",\n\
                      \"repo\": \"tokio\",\n\
                      \"issue_number\": 5432,\n\
                      \"labels\": [\"bug\", \"priority-high\", \"needs-review\", \"performance\"]\n\
                    })\n\n\
                 3. Clear all labels:\n\
                    github_update_issue({\n\
                      \"owner\": \"actix\",\n\
                      \"repo\": \"actix-web\",\n\
                      \"issue_number\": 2345,\n\
                      \"labels\": []\n\
                    })\n\n\
                 4. Replace labels completely:\n\
                    // Old labels: [\"bug\", \"investigating\"]\n\
                    // New labels: [\"bug\", \"confirmed\", \"ready-for-pr\"]\n\
                    github_update_issue({\n\
                      \"owner\": \"serde-rs\",\n\
                      \"repo\": \"serde\",\n\
                      \"issue_number\": 1234,\n\
                      \"labels\": [\"bug\", \"confirmed\", \"ready-for-pr\"]\n\
                    })\n\n\
                 COMMON LABEL PATTERNS:\n\
                 - Type: bug, enhancement, feature, documentation\n\
                 - Priority: priority-critical, priority-high, priority-low\n\
                 - Status: needs-review, in-progress, blocked, waiting-on-author\n\
                 - Area: area-async, area-docs, area-performance\n\
                 - Meta: good-first-issue, help-wanted, wontfix, duplicate\n\n\
                 UPDATING ASSIGNEES:\n\n\
                 IMPORTANT: Assignees array REPLACES all existing assignees (not additive)\n\n\
                 1. Assign single user:\n\
                    github_update_issue({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"issue_number\": 12345,\n\
                      \"assignees\": [\"alice\"]\n\
                    })\n\n\
                 2. Assign multiple users:\n\
                    github_update_issue({\n\
                      \"owner\": \"tokio-rs\",\n\
                      \"repo\": \"tokio\",\n\
                      \"issue_number\": 5432,\n\
                      \"assignees\": [\"alice\", \"bob\", \"charlie\"]\n\
                    })\n\n\
                 3. Unassign all:\n\
                    github_update_issue({\n\
                      \"owner\": \"actix\",\n\
                      \"repo\": \"actix-web\",\n\
                      \"issue_number\": 2345,\n\
                      \"assignees\": []\n\
                    })\n\n\
                 4. Reassign to different user:\n\
                    // Old assignee: [\"alice\"]\n\
                    // New assignee: [\"bob\"]\n\
                    github_update_issue({\n\
                      \"owner\": \"serde-rs\",\n\
                      \"repo\": \"serde\",\n\
                      \"issue_number\": 1234,\n\
                      \"assignees\": [\"bob\"]\n\
                    })\n\n\
                 UPDATING MILESTONES:\n\n\
                 1. Set milestone:\n\
                    github_update_issue({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"issue_number\": 12345,\n\
                      \"milestone\": 10\n\
                    })\n\n\
                 2. Change milestone:\n\
                    github_update_issue({\n\
                      \"owner\": \"tokio-rs\",\n\
                      \"repo\": \"tokio\",\n\
                      \"issue_number\": 5432,\n\
                      \"milestone\": 15\n\
                    })\n\n\
                 3. Remove milestone:\n\
                    github_update_issue({\n\
                      \"owner\": \"actix\",\n\
                      \"repo\": \"actix-web\",\n\
                      \"issue_number\": 2345,\n\
                      \"milestone\": null\n\
                    })\n\n\
                 COMBINED METADATA UPDATES:\n\n\
                 1. Triage new issue:\n\
                    github_update_issue({\n\
                      \"owner\": \"org\",\n\
                      \"repo\": \"project\",\n\
                      \"issue_number\": 100,\n\
                      \"labels\": [\"bug\", \"triaged\", \"priority-high\"],\n\
                      \"assignees\": [\"lead-developer\"],\n\
                      \"milestone\": 5\n\
                    })\n\n\
                 2. Escalate priority:\n\
                    github_update_issue({\n\
                      \"owner\": \"org\",\n\
                      \"repo\": \"project\",\n\
                      \"issue_number\": 300,\n\
                      \"labels\": [\"bug\", \"priority-critical\", \"regression\"],\n\
                      \"assignees\": [\"senior-dev-1\", \"senior-dev-2\"],\n\
                      \"milestone\": 4\n\
                    })\n\n\
                 BEST PRACTICES:\n\n\
                 LABELS:\n\
                 - Use consistent label naming (lowercase, hyphens)\n\
                 - Verify labels exist in repository before applying\n\
                 - Combine type + priority + status labels for clarity\n\
                 - Remove outdated labels when updating\n\
                 - Document label meanings in repository documentation\n\n\
                 ASSIGNEES:\n\
                 - Verify users have repository access before assigning\n\
                 - Limit assignees to 2-3 for accountability\n\
                 - Unassign when work is blocked or reassigned\n\
                 - Consider team workload when assigning\n\
                 - Use assignees for who's actively working, not just interested\n\n\
                 MILESTONES:\n\
                 - Use milestone numbers, not names\n\
                 - Get milestone number from github_list_milestones or repository settings\n\
                 - Align milestones with release planning\n\
                 - Move issues to next milestone if they slip\n\
                 - Clear milestone if no longer relevant to any release\n\n\
                 WORKFLOW TIPS:\n\
                 1. Fetch current issue state first (github_get_issue)\n\
                 2. Determine what metadata needs updating\n\
                 3. If adding to existing labels/assignees, include current ones in array\n\
                 4. Make atomic updates when possible\n\
                 5. Add comment explaining significant metadata changes\n\n\
                 ERROR SCENARIOS:\n\
                 - 422 Unprocessable: Invalid label (doesn't exist in repo)\n\
                 - 422 Unprocessable: Invalid assignee (user doesn't have access)\n\
                 - 422 Unprocessable: Invalid milestone (doesn't exist)\n\
                 - 403 Forbidden: Insufficient permissions\n\n\
                 PRESERVING EXISTING METADATA:\n\
                 To ADD a label without removing existing ones:\n\
                 1. Get current issue: github_get_issue\n\
                 2. Extract current labels from response\n\
                 3. Add new label to array: existing_labels + [\"new-label\"]\n\
                 4. Update with combined array\n\
                 Same pattern applies to assignees!",
            ),
        },
    ]
}
