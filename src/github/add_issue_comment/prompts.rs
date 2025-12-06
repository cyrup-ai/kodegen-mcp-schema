//! Prompt messages for github_add_comment tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::AddIssueCommentPromptArgs;

/// Prompt provider for add_issue_comment tool
///
/// This is the ONLY way to provide prompts for add_issue_comment - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct AddIssueCommentPrompts;

impl PromptProvider for AddIssueCommentPrompts {
    type PromptArgs = AddIssueCommentPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("issues") => prompt_issues(),
            Some("prs") => prompt_prs(),
            _ => prompt_issues(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show examples for (issues, prs)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Commenting on issues
fn prompt_issues() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I add comments to GitHub issues?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use github_add_comment to add comments to issues with full Markdown support.\n\n\
                 COMMENTING ON ISSUES:\n\n\
                 1. Simple comment:\n\
                    github_add_comment({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"issue_number\": 123,\n\
                        \"body\": \"Thanks for reporting this!\"\n\
                    })\n\n\
                 2. Detailed response:\n\
                    github_add_comment({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"issue_number\": 123,\n\
                        \"body\": \"I've investigated this issue.\\n\\n## Findings\\n\\nThe problem occurs because...\\n\\n## Proposed Fix\\n\\nWe should update the handler to...\"\n\
                    })\n\n\
                 3. With code example:\n\
                    github_add_comment({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"issue_number\": 123,\n\
                        \"body\": \"Here's a workaround:\\n\\n```rust\\nfn handle_error(e: Error) {\\n    log::error!(\\\"Error: {}\\\", e);\\n    None\\n}\\n```\"\n\
                    })\n\n\
                 4. Status update:\n\
                    github_add_comment({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"issue_number\": 123,\n\
                        \"body\": \"## Update\\n\\nThis has been fixed in version 2.0.\\n\\n- [x] Tests added\\n- [x] Documentation updated\\n- [x] Changelog entry\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"id\": 987654,\n\
                   \"url\": \"https://github.com/user/project/issues/123#issuecomment-987654\",\n\
                   \"success\": true\n\
                 }\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or org)\n\
                 - repo (required): Repository name\n\
                 - issue_number (required): Issue number to comment on\n\
                 - body (required): Comment text in Markdown format\n\n\
                 ISSUE COMMENT USE CASES:\n\
                 - Responding to bug reports\n\
                 - Providing status updates\n\
                 - Asking for more information\n\
                 - Linking to related issues or documentation\n\
                 - Sharing code examples or workarounds\n\
                 - Acknowledging feature requests\n\n\
                 BEST PRACTICES:\n\
                 - Be clear and concise\n\
                 - Use Markdown formatting for readability\n\
                 - Include code blocks for examples\n\
                 - Link to relevant issues with #number\n\
                 - Tag users with @username when appropriate\n\
                 - Break long responses into sections with headers",
            ),
        },
    ]
}

/// Commenting on pull requests
fn prompt_prs() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I add comments to pull requests?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use github_add_comment to add general comments to PRs. For line-specific review comments, use the PR review API instead.\n\n\
                 COMMENTING ON PULL REQUESTS:\n\n\
                 1. Review comment:\n\
                    github_add_comment({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"issue_number\": 456,  // PR number\n\
                        \"body\": \"Looks good! Just a few minor suggestions.\"\n\
                    })\n\n\
                 2. Request changes:\n\
                    github_add_comment({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"issue_number\": 456,\n\
                        \"body\": \"Please address the following:\\n\\n- [ ] Add tests\\n- [ ] Update documentation\\n- [ ] Fix linting errors\"\n\
                    })\n\n\
                 3. Approval with notes:\n\
                    github_add_comment({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"issue_number\": 456,\n\
                        \"body\": \"LGTM! :white_check_mark:\\n\\nNice work on optimizing the performance. The benchmark results look great.\"\n\
                    })\n\n\
                 4. Question about implementation:\n\
                    github_add_comment({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"issue_number\": 456,\n\
                        \"body\": \"Question: Have you considered using a HashMap instead of a Vec for better lookup performance?\"\n\
                    })\n\n\
                 NOTE:\n\
                 - PRs use same issue_number parameter\n\
                 - This adds general PR comment (not tied to specific code lines)\n\
                 - For line-specific comments, use github_add_pull_request_review_comment\n\
                 - For formal review (approve/request changes), use github_create_pull_request_review\n\n\
                 PR COMMENT USE CASES:\n\
                 - General feedback on the PR\n\
                 - High-level architectural comments\n\
                 - Questions about approach\n\
                 - Checklist for required changes\n\
                 - Acknowledgment and approval\n\
                 - Discussion about implementation strategy\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"id\": 987654,\n\
                   \"url\": \"https://github.com/user/project/pull/456#issuecomment-987654\",\n\
                   \"success\": true\n\
                 }\n\n\
                 PR VS ISSUE COMMENTS:\n\
                 - Both use issue_number parameter (PRs are special issues)\n\
                 - General comments use github_add_comment\n\
                 - Line-specific reviews use separate review comment tools\n\
                 - Formal approvals/rejections use review API",
            ),
        },
    ]
}


