//! Prompt messages for github_update_pull_request tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::UpdatePullRequestPromptArgs;

/// Prompt provider for github_update_pull_request tool
///
/// This is the ONLY way to provide prompts for github_update_pull_request - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct UpdatePullRequestPrompts;

impl PromptProvider for UpdatePullRequestPrompts {
    type PromptArgs = UpdatePullRequestPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("content") => prompt_content(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_content(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (content, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// Helper functions for PR update scenarios

/// Updating PR content (title, body, base)
fn prompt_content() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I update pull request content like title, description, or base branch?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use github_update_pr to modify PR title, description, or target branch. Update only the fields you want to change.\n\n\
                 UPDATING CONTENT:\n\n\
                 1. Update title:\n\
                    github_update_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"title\": \"feat: Updated feature title\"\n\
                    })\n\
                    - Follow repository's title conventions\n\
                    - Common prefixes: feat:, fix:, docs:, refactor:, test:, chore:\n\
                    - Keep under 72 characters for readability\n\
                    - Be descriptive and specific\n\n\
                 2. Update description (body):\n\
                    github_update_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"body\": \"## Summary\\nImplemented new authentication system\\n\\n## Changes\\n- Added JWT support\\n- Updated user model\\n- Added tests\\n\\n## Testing\\n- Unit tests pass\\n- Integration tests added\\n\\nFixes #123\"\n\
                    })\n\
                    - Use Markdown formatting\n\
                    - Structure with clear sections\n\
                    - Link related issues: Fixes #123, Closes #456, Resolves #789\n\
                    - Describe what changed and why\n\
                    - Include testing notes\n\n\
                 3. Change base branch:\n\
                    github_update_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"base\": \"develop\"\n\
                    })\n\
                    - Target branch must exist\n\
                    - May introduce merge conflicts\n\
                    - GitHub will re-run CI checks\n\
                    - Use when PR was opened against wrong branch\n\n\
                 4. Update multiple fields:\n\
                    github_update_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"title\": \"feat: Add authentication system\",\n\
                      \"body\": \"## Summary\\nComplete auth implementation\\n\\nFixes #123\",\n\
                      \"base\": \"main\"\n\
                    })\n\
                    - Can update multiple fields in single call\n\
                    - Only specify fields you want to change\n\
                    - Omitted fields remain unchanged\n\n\
                 CONTENT FIELD DETAILS:\n\n\
                 title (string):\n\
                 - Brief summary of PR changes\n\
                 - Recommended: 50-72 characters\n\
                 - Use conventional commit format\n\
                 - Example: \"fix: Resolve memory leak in connection pool\"\n\n\
                 body (string):\n\
                 - Full description with Markdown\n\
                 - Common sections: Summary, Changes, Testing, Notes\n\
                 - Link issues: \"Fixes #123\", \"Closes #456\", \"Related to #789\"\n\
                 - Describe motivation and implementation approach\n\
                 - Include breaking changes warnings if applicable\n\
                 - Empty string clears the body\n\n\
                 base (string):\n\
                 - Target branch name (without refs/heads/)\n\
                 - Branch must exist in repository\n\
                 - Changing base recalculates diff and conflicts\n\
                 - Common values: \"main\", \"develop\", \"staging\", \"v2.0\"\n\
                 - Cannot change base of merged PR\n\n\
                 MARKDOWN FORMATTING:\n\
                 Use standard GitHub Markdown:\n\
                 - Headers: # H1, ## H2, ### H3\n\
                 - Lists: - item or 1. item\n\
                 - Code blocks: ```language\\ncode\\n```\n\
                 - Inline code: `code`\n\
                 - Links: [text](url)\n\
                 - Images: ![alt](url)\n\
                 - Tables: | Col1 | Col2 |\\n|------|------|\n\
                 - Task lists: - [ ] task or - [x] completed\n\
                 - Mentions: @username\n\
                 - Issue refs: #123\n\n\
                 TITLE CONVENTIONS:\n\
                 Conventional Commits format:\n\
                 - feat: New feature\n\
                 - fix: Bug fix\n\
                 - docs: Documentation changes\n\
                 - style: Formatting, missing semicolons, etc.\n\
                 - refactor: Code restructuring\n\
                 - test: Adding or fixing tests\n\
                 - chore: Maintenance tasks\n\
                 - perf: Performance improvements\n\
                 - ci: CI/CD changes\n\
                 - build: Build system changes\n\
                 - revert: Revert previous commit\n\n\
                 BEST PRACTICES:\n\
                 1. Title:\n\
                    - Start with type prefix (feat:, fix:, etc.)\n\
                    - Use imperative mood (\"Add feature\" not \"Added feature\")\n\
                    - Keep concise but descriptive\n\
                    - Avoid generic titles like \"Update code\" or \"Fix bug\"\n\n\
                 2. Body:\n\
                    - Use structured sections with Markdown headers\n\
                    - Link all related issues\n\
                    - Explain \"why\" not just \"what\"\n\
                    - Include testing instructions\n\
                    - Note breaking changes prominently\n\
                    - Add screenshots for UI changes\n\n\
                 3. Base branch:\n\
                    - Verify target branch before changing\n\
                    - Expect CI to re-run after change\n\
                    - Check for new conflicts\n\
                    - Consider if reviewers need to re-review\n\n\
                 COMMON WORKFLOWS:\n\
                 1. Improve PR for review:\n\
                    github_update_pr({\n\
                      \"owner\": \"org\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 123,\n\
                      \"title\": \"feat: Add user authentication\",\n\
                      \"body\": \"## Summary\\nImplemented JWT-based authentication\\n\\n## Testing\\n- All tests pass\\n- Manual testing completed\\n\\nFixes #456\"\n\
                    })\n\n\
                 2. Fix wrong target branch:\n\
                    github_update_pr({\n\
                      \"owner\": \"org\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 789,\n\
                      \"base\": \"main\"\n\
                    })\n\
                    // Check for conflicts after\n\n\
                 3. Update after feedback:\n\
                    github_update_pr({\n\
                      \"owner\": \"org\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"body\": \"## Summary\\n...\\n\\n## Updates\\n- Addressed review comments\\n- Added requested tests\\n- Fixed edge cases\"\n\
                    })\n\n\
                 ERROR HANDLING:\n\
                 - 404: PR, repository, or base branch not found\n\
                 - 403: No write access to repository\n\
                 - 422: Invalid base branch name\n\
                 - 422: Base branch doesn't exist\n\
                 - 422: Cannot modify merged PR\n\n\
                 PERMISSIONS:\n\
                 - PR author can update their PR\n\
                 - Repository maintainers can update any PR\n\
                 - Changing base requires write access",
            ),
        },
    ]
}

/// PR management workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows for managing pull requests?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Here are common workflows for managing pull requests using github_update_pr and related tools.\n\n\
                 PR MANAGEMENT:\n\n\
                 1. Prepare for review:\n\
                    github_update_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"title\": \"feat: Final feature title\",\n\
                      \"body\": \"## Summary\\nComplete implementation of feature\\n\\n## Changes\\n- Core functionality\\n- Tests added\\n- Docs updated\\n\\n## Test Plan\\n- Unit tests pass\\n- Integration tests pass\\n- Manual testing completed\\n\\nFixes #123\",\n\
                      \"draft\": false\n\
                    })\n\
                    - Update title to follow conventions\n\
                    - Complete description with all details\n\
                    - Mark as ready for review\n\
                    - Then request reviews from team\n\n\
                 2. Close abandoned PR:\n\
                    // First add explanatory comment\n\
                    github_add_comment({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"issue_number\": 456,\n\
                      \"body\": \"Closing - superseded by #789\"\n\
                    })\n\
                    // Then close the PR\n\
                    github_update_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"state\": \"closed\"\n\
                    })\n\
                    - Always explain why closing\n\
                    - Reference replacement PR if applicable\n\
                    - Thank contributors for their work\n\n\
                 3. Fix wrong target branch:\n\
                    // Check current PR details\n\
                    github_get_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456\n\
                    })\n\
                    // Update base branch\n\
                    github_update_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"base\": \"develop\"\n\
                    })\n\
                    // Check for new conflicts\n\
                    github_get_pull_request_status({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456\n\
                    })\n\n\
                 4. Respond to review feedback:\n\
                    // Convert to draft while addressing feedback\n\
                    github_update_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"draft\": true\n\
                    })\n\
                    // Make changes, push commits\n\
                    // Update PR description\n\
                    github_update_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"body\": \"## Summary\\n...\\n\\n## Updates\\n- Addressed @reviewer comments\\n- Added requested tests\\n- Fixed edge cases\"\n\
                    })\n\
                    // Mark ready again\n\
                    github_update_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"draft\": false\n\
                    })\n\n\
                 5. Reopen closed PR:\n\
                    // Add comment explaining reopening\n\
                    github_add_comment({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"issue_number\": 456,\n\
                      \"body\": \"Reopening - issues have been resolved\"\n\
                    })\n\
                    // Reopen PR\n\
                    github_update_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"state\": \"open\"\n\
                    })\n\
                    // Re-request reviews if needed\n\n\
                 COMPLETE WORKFLOWS:\n\n\
                 WORKFLOW A: Draft to Ready:\n\
                 Step 1: Create draft PR\n\
                 github_create_pull_request({\n\
                   \"owner\": \"org\",\n\
                   \"repo\": \"project\",\n\
                   \"title\": \"feat: Work in progress\",\n\
                   \"body\": \"Early implementation\",\n\
                   \"head\": \"feature\",\n\
                   \"base\": \"main\",\n\
                   \"draft\": true\n\
                 })\n\n\
                 Step 2: Iterate on code\n\
                 // Push commits, check CI, get feedback\n\n\
                 Step 3: Finalize PR\n\
                 github_update_pr({\n\
                   \"owner\": \"org\",\n\
                   \"repo\": \"project\",\n\
                   \"pull_number\": 456,\n\
                   \"title\": \"feat: Add user authentication\",\n\
                   \"body\": \"## Summary\\nComplete implementation\\n\\nFixes #123\",\n\
                   \"draft\": false\n\
                 })\n\n\
                 Step 4: Request reviews\n\
                 // Use github_request_reviews or UI\n\n\
                 WORKFLOW B: Fix Wrong Branch:\n\
                 Step 1: Realize mistake\n\
                 github_get_pr({\"owner\": \"org\", \"repo\": \"project\", \"pull_number\": 456})\n\
                 // See base: \"main\" but should be \"develop\"\n\n\
                 Step 2: Add explanatory comment\n\
                 github_add_comment({\n\
                   \"owner\": \"org\",\n\
                   \"repo\": \"project\",\n\
                   \"issue_number\": 456,\n\
                   \"body\": \"Updating base branch to develop\"\n\
                 })\n\n\
                 Step 3: Update base\n\
                 github_update_pr({\n\
                   \"owner\": \"org\",\n\
                   \"repo\": \"project\",\n\
                   \"pull_number\": 456,\n\
                   \"base\": \"develop\"\n\
                 })\n\n\
                 Step 4: Check for conflicts\n\
                 github_get_pull_request_status({\n\
                   \"owner\": \"org\",\n\
                   \"repo\": \"project\",\n\
                   \"pull_number\": 456\n\
                 })\n\n\
                 WORKFLOW C: Close Superseded PR:\n\
                 Step 1: Create better replacement PR\n\
                 // New PR created as #789\n\n\
                 Step 2: Comment on old PR\n\
                 github_add_comment({\n\
                   \"owner\": \"org\",\n\
                   \"repo\": \"project\",\n\
                   \"issue_number\": 456,\n\
                   \"body\": \"Thank you for this contribution! I've created #789 which takes a different approach. Closing this one.\"\n\
                 })\n\n\
                 Step 3: Close old PR\n\
                 github_update_pr({\n\
                   \"owner\": \"org\",\n\
                   \"repo\": \"project\",\n\
                   \"pull_number\": 456,\n\
                   \"state\": \"closed\"\n\
                 })\n\n\
                 WORKFLOW D: Major Revision:\n\
                 Step 1: Get review feedback\n\
                 github_get_pull_request_reviews({\n\
                   \"owner\": \"org\",\n\
                   \"repo\": \"project\",\n\
                   \"pull_number\": 456\n\
                 })\n\n\
                 Step 2: Convert to draft\n\
                 github_update_pr({\n\
                   \"owner\": \"org\",\n\
                   \"repo\": \"project\",\n\
                   \"pull_number\": 456,\n\
                   \"draft\": true\n\
                 })\n\n\
                 Step 3: Add comment\n\
                 github_add_comment({\n\
                   \"owner\": \"org\",\n\
                   \"repo\": \"project\",\n\
                   \"issue_number\": 456,\n\
                   \"body\": \"Converting to draft while I address the feedback\"\n\
                 })\n\n\
                 Step 4: Make changes\n\
                 // Push new commits\n\n\
                 Step 5: Update description\n\
                 github_update_pr({\n\
                   \"owner\": \"org\",\n\
                   \"repo\": \"project\",\n\
                   \"pull_number\": 456,\n\
                   \"body\": \"## Summary\\n...\\n\\n## Changes Addressing Feedback\\n- Refactored per @reviewer\\n- Added tests\\n- Updated docs\"\n\
                 })\n\n\
                 Step 6: Mark ready\n\
                 github_update_pr({\n\
                   \"owner\": \"org\",\n\
                   \"repo\": \"project\",\n\
                   \"pull_number\": 456,\n\
                   \"draft\": false\n\
                 })\n\n\
                 Step 7: Re-request review\n\
                 // Notify reviewers\n\n\
                 BEST PRACTICES:\n\n\
                 1. Always communicate:\n\
                    - Add comments before major changes\n\
                    - Explain why closing/reopening\n\
                    - Tag relevant people\n\
                    - Be polite and professional\n\n\
                 2. Update description regularly:\n\
                    - Keep summary current\n\
                    - Document significant changes\n\
                    - Update issue links\n\
                    - Add testing notes\n\n\
                 3. Use draft status effectively:\n\
                    - Start draft if uncertain\n\
                    - Convert to draft for major revisions\n\
                    - Only mark ready when truly complete\n\n\
                 4. Check CI after changes:\n\
                    - Base branch changes trigger CI\n\
                    - May introduce new conflicts\n\
                    - Verify all checks pass\n\n\
                 5. Combine operations logically:\n\
                    - Update multiple fields together\n\
                    - Add comment before closing\n\
                    - Check status after base change\n\n\
                 COMMON COMBINATIONS:\n\
                 1. Finalize PR:\n\
                    github_update_pr({..., \"title\": \"...\", \"body\": \"...\", \"draft\": false})\n\n\
                 2. Close with comment:\n\
                    github_add_comment({..., \"body\": \"Reason\"}) then\n\
                    github_update_pr({..., \"state\": \"closed\"})\n\n\
                 3. Fix and reopen:\n\
                    // Push fixes then\n\
                    github_update_pr({..., \"state\": \"open\", \"body\": \"Updated\"})\n\n\
                 4. Change branch and update:\n\
                    github_update_pr({..., \"base\": \"develop\", \"body\": \"Updated base\"}) then\n\
                    github_get_pull_request_status({...})\n\n\
                 TOOLS TO USE TOGETHER:\n\
                 - github_update_pr: Modify PR fields\n\
                 - github_add_comment: Add explanatory comments\n\
                 - github_get_pr: Check current state\n\
                 - github_get_pull_request_status: Check CI/conflicts\n\
                 - github_get_pull_request_reviews: See feedback\n\
                 - github_merge_pull_request: Merge when ready\n\
                 - github_get_pull_request_files: Review changed files\n\n\
                 ERROR PREVENTION:\n\
                 - Check PR exists before updating\n\
                 - Verify base branch exists before changing\n\
                 - Cannot reopen merged PRs\n\
                 - Need write access to update\n\
                 - Draft conversion dismisses reviews",
            ),
        },
    ]
}
