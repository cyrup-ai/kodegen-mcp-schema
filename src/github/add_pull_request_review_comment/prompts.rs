//! Prompt messages for github_add_pull_request_review_comment tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::AddPullRequestReviewCommentPromptArgs;

/// Prompt provider for add_pull_request_review_comment tool
///
/// This is the ONLY way to provide prompts for add_pull_request_review_comment - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct AddPullRequestReviewCommentPrompts;

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_add_pull_request_review_comment to add inline code review comments?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_add_pull_request_review_comment tool adds inline comments to specific lines in a pull request's diff for detailed code review.\n\n\
                 BASIC USAGE:\n\
                 1. Single line comment:\n\
                    github_add_pull_request_review_comment({\n\
                      \"owner\": \"tokio-rs\",\n\
                      \"repo\": \"tokio\",\n\
                      \"pull_number\": 5678,\n\
                      \"body\": \"Consider using `match` here instead of `if let` for better readability\",\n\
                      \"commit_id\": \"abc123def456\",\n\
                      \"path\": \"src/lib.rs\",\n\
                      \"line\": 42\n\
                    })\n\n\
                 2. Comment on deletion (LEFT side):\n\
                    github_add_pull_request_review_comment({\n\
                      \"owner\": \"actix\",\n\
                      \"repo\": \"actix-web\",\n\
                      \"pull_number\": 999,\n\
                      \"body\": \"Why remove this error handling? This was protecting against null pointer access.\",\n\
                      \"commit_id\": \"ghi789xyz012\",\n\
                      \"path\": \"src/handler.rs\",\n\
                      \"line\": 20,\n\
                      \"side\": \"LEFT\"\n\
                    })\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - pull_number (required): Pull request number\n\
                 - body (required): Comment text (Markdown supported)\n\
                 - commit_id (required for new comments): Commit SHA from the PR\n\
                 - path (required for new comments): File path in repository\n\
                 - line (required for new comments): Line number in diff\n\
                 - side (optional): \"LEFT\" (deletion) or \"RIGHT\" (addition, default)\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo (for private repositories)\n\
                 - public_repo (for public repositories only)\n\n\
                 Set in your environment:\n\
                 export GITHUB_TOKEN=ghp_your_token_here\n\n\
                 RESPONSE:\n\
                 Returns JSON with:\n\
                 - success: true/false\n\
                 - owner, repo: Repository identifiers\n\
                 - pr_number: Pull request number\n\
                 - comment_id: Created comment ID (use for replies)\n\
                 - message: Status message\n\n\
                 Example response:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"tokio-rs\",\n\
                   \"repo\": \"tokio\",\n\
                   \"pr_number\": 5678,\n\
                   \"comment_id\": 123456789,\n\
                   \"message\": \"Comment added successfully\"\n\
                 }\n\n\
                 COMMON WORKFLOW:\n\
                 1. Automated code review:\n\
                    - Analyze PR diff for patterns (regex, AST analysis)\n\
                    - Add inline suggestions on specific lines\n\
                    - Focus on security issues (SQL injection, XSS)\n\
                    - Flag performance problems (N+1 queries, unnecessary allocations)\n\
                    - Enforce style guidelines (naming conventions, formatting)\n\
                    - Suggest code improvements (DRY violations, complexity)\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Unauthenticated: 60 requests/hour\n\
                 - Check headers: X-RateLimit-Remaining, X-RateLimit-Reset\n\
                 - Plan bulk operations to stay within limits\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found: PR, commit, or file path doesn't exist\n\
                    Fix: Verify pull_number and commit_id are valid\n\
                    - Check PR is open and not deleted\n\
                    - Ensure commit_id is from this PR's branch\n\
                    - Confirm file path exists in the commit\n\n\
                 2. 422 Unprocessable: Invalid line number or path\n\
                    Fix: Line must exist in commit's diff, not entire file\n\
                    - Line numbers are diff-relative, not file-relative\n\
                    - Only lines shown in the PR diff are valid\n\
                    - Check the diff view on GitHub to find valid lines\n\n\
                 3. 403 Forbidden: Token lacks required scopes\n\
                    Fix: Generate new token with 'repo' scope\n\
                    - Go to GitHub Settings > Developer settings > Tokens\n\
                    - Create token with 'repo' scope for private repos\n\
                    - Or use 'public_repo' scope for public repos only\n\n\
                 BEST PRACTICES:\n\
                 - Use commit_id from latest PR commit for accuracy\n\
                 - Get latest commit with: gh pr view <number> --json commits\n\
                 - Line numbers are relative to diff, not file line numbers\n\
                 - Use RIGHT side for new/modified code (default)\n\
                 - Use LEFT side for deleted code or original version\n\
                 - Keep comments constructive and specific\n\
                 - Include actionable suggestions, not just criticism\n\
                 - Use code suggestions (```suggestion) when possible\n\
                 - Reference documentation or best practices with links\n\
                 - Be respectful and professional in tone",
            ),
        },
    ]
}

fn prompt_multiline() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_add_pull_request_review_comment for multi-line comments?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_add_pull_request_review_comment tool supports multi-line comments that span multiple lines in a pull request diff using the start_line parameter.\n\n\
                 MULTI-LINE USAGE:\n\
                 1. Multi-line comment spanning lines:\n\
                    github_add_pull_request_review_comment({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"pull_number\": 123,\n\
                      \"body\": \"This entire error handling block could be simplified using the ? operator pattern\",\n\
                      \"commit_id\": \"def456abc789\",\n\
                      \"path\": \"compiler/rustc/src/main.rs\",\n\
                      \"start_line\": 100,\n\
                      \"line\": 105\n\
                    })\n\n\
                 2. Multi-line deletion (LEFT side):\n\
                    github_add_pull_request_review_comment({\n\
                      \"owner\": \"tokio-rs\",\n\
                      \"repo\": \"tokio\",\n\
                      \"pull_number\": 999,\n\
                      \"body\": \"Good refactoring! This nested callback logic was complex. The new async/await pattern is much clearer.\",\n\
                      \"commit_id\": \"xyz789mno345\",\n\
                      \"path\": \"src/runtime.rs\",\n\
                      \"start_line\": 45,\n\
                      \"line\": 52,\n\
                      \"side\": \"LEFT\"\n\
                    })\n\n\
                 MULTI-LINE PARAMETERS:\n\
                 - start_line (optional): Start line for multi-line comments\n\
                   - Must be less than the 'line' parameter\n\
                   - Defines the beginning of the comment range\n\
                 - line (required): End line number for comment range\n\
                   - The last line the comment applies to\n\
                 - start_side (optional): Side of start_line (\"LEFT\" or \"RIGHT\")\n\
                   - Usually same as 'side' parameter\n\
                   - Defaults to \"RIGHT\" if not specified\n\
                 - side (optional): Side of end line (\"LEFT\" or \"RIGHT\")\n\
                   - \"RIGHT\" for additions/modifications (default)\n\
                   - \"LEFT\" for deletions/original code\n\n\
                 WHEN TO USE EACH SIDE:\n\
                 - RIGHT side (default): For added or modified code in the diff\n\
                   - New feature additions\n\
                   - Bug fixes and improvements\n\
                   - Refactored code\n\
                   - Any green/added lines in the diff\n\
                 - LEFT side: For deleted code or original version before changes\n\
                   - Red/removed lines in the diff\n\
                   - Code that was replaced\n\
                   - Comments about what was removed\n\
                   - Explain why deletion was necessary\n\n\
                 LINE NUMBERING:\n\
                 - Line numbers are relative to the diff, NOT file line numbers\n\
                 - GitHub shows diff line numbers, not absolute file positions\n\
                 - For multi-line comments: start_line must be less than line\n\
                 - Both lines must exist in the commit's diff context\n\
                 - Only lines visible in the PR diff view are valid\n\
                 - Changed lines and surrounding context lines are valid\n\n\
                 COMMON MULTI-LINE WORKFLOW:\n\
                 1. Identify block of code to review (e.g., lines 45-52 in diff)\n\
                 2. Create multi-line comment:\n\
                    - Set start_line to first line of the block (e.g., 45)\n\
                    - Set line to last line of the block (e.g., 52)\n\
                    - Write body explaining the issue with the entire block\n\
                    - Include specific suggestions for refactoring\n\
                 3. Use code fence suggestions when applicable:\n\
                    ```suggestion\n\
                    // Improved version of the code\n\
                    ```\n\n\
                 SIDE PARAMETER EXPLAINED:\n\
                 - RIGHT (addition): Use for new code or modified sections\n\
                   - Default side if not specified\n\
                   - Most common use case\n\
                   - Use for feature additions, bug fixes\n\
                   - Applies to green lines in GitHub diff view\n\
                 - LEFT (deletion): Use for removed code\n\
                   - Comment on what was removed\n\
                   - Explain why removal was good/bad\n\
                   - Or comment on what was removed in context\n\
                   - Applies to red lines in GitHub diff view\n\n\
                 BEST PRACTICES:\n\
                 - Multi-line comments must have start_line < line\n\
                 - Keep multi-line comments focused (max 5-7 lines)\n\
                   - Longer ranges become hard to understand\n\
                   - Split into multiple comments if needed\n\
                 - Use single-line comments for small, specific issues\n\
                 - Use multi-line only when commenting on cohesive blocks\n\
                   - Functions, loops, error handling blocks\n\
                   - Related group of statements\n\
                 - Ensure all lines in range are relevant to the comment\n\
                 - Don't span unrelated code sections\n\
                 - Be specific about which part of the block needs attention\n\n\
                 ERROR SCENARIOS:\n\
                 1. 422 Unprocessable: start_line >= line\n\
                    Fix: Ensure start_line is strictly less than line\n\
                    - Example: start_line: 100, line: 105 ✓\n\
                    - Example: start_line: 105, line: 100 ✗\n\
                    - Example: start_line: 100, line: 100 ✗\n\n\
                 2. 422 Unprocessable: Lines don't exist in diff\n\
                    Fix: Lines must be in the commit's diff context, not just the file\n\
                    - Check PR diff view on GitHub\n\
                    - Only changed lines + context are valid\n\
                    - File line numbers ≠ diff line numbers\n\
                    - Use diff line numbers, not file line numbers",
            ),
        },
    ]
}

fn prompt_reply() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I reply to existing code review comments in github_add_pull_request_review_comment?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_add_pull_request_review_comment tool supports replying to existing comments using the in_reply_to parameter to create threaded discussions.\n\n\
                 REPLY USAGE:\n\
                 1. Reply to existing comment:\n\
                    github_add_pull_request_review_comment({\n\
                      \"owner\": \"serde-rs\",\n\
                      \"repo\": \"serde\",\n\
                      \"pull_number\": 456,\n\
                      \"body\": \"Good point! I've fixed this in commit abc1234. The new implementation uses the suggested pattern.\",\n\
                      \"in_reply_to\": 987654321\n\
                    })\n\n\
                 REPLY PARAMETERS:\n\
                 - in_reply_to (required for replies): Comment ID to reply to\n\
                   - This is the ID of the parent comment\n\
                   - Must be a valid comment ID from the same PR\n\
                   - Do NOT include: commit_id, path, line when using in_reply_to\n\
                   - These are automatically inherited from parent comment\n\
                 - body (required): Reply text (Markdown supported)\n\
                   - Can include code blocks, links, references\n\
                   - Keep professional and constructive\n\
                 - owner, repo, pull_number (required): Repository context\n\
                   - Must match the PR containing the original comment\n\n\
                 WHEN TO USE REPLY MODE:\n\
                 - Author acknowledges feedback: \"Good point! I'll fix this.\"\n\
                 - Explain implementation: \"This was fixed in commit abc1234\"\n\
                 - Ask clarifying questions: \"Did you mean approach X or Y?\"\n\
                 - Continue discussion threads without new diff comments\n\
                 - Provide additional context or reasoning\n\
                 - Disagree respectfully and explain why\n\n\
                 THREADED DISCUSSION WORKFLOW:\n\
                 1. Reviewer adds inline comment on line 50:\n\
                    \"This function has high cyclomatic complexity\"\n\
                 2. Author responds:\n\
                    \"Good catch, but can you clarify which part is most problematic?\"\n\
                 3. Reviewer replies:\n\
                    \"Yes, the nested if statements in lines 52-58. Consider extracting to helper functions.\"\n\
                 4. Author replies:\n\
                    \"Got it, I've refactored this in commit def5678. Each condition is now a separate function.\"\n\n\
                 OBTAINING COMMENT IDS:\n\
                 - From github_search_issues tool searching PR discussions\n\
                 - From previous github_add_pull_request_review_comment calls\n\
                   - The response includes 'comment_id' field\n\
                   - Save this ID for potential replies\n\
                 - From GitHub API or gh CLI:\n\
                   gh api repos/{owner}/{repo}/pulls/{number}/comments\n\
                 - Use grep or jq to parse PR discussion JSON\n\n\
                 REPLY vs NEW COMMENT:\n\
                 - Use in_reply_to for threaded discussions (keeps context)\n\
                   - Maintains conversation flow\n\
                   - Groups related comments together\n\
                   - Easier to follow discussion history\n\
                 - Use commit_id+path+line for new inline comments on diff\n\
                   - Creates new comment thread\n\
                   - Use when addressing different code location\n\
                 - Can't reply to arbitrary comments (must be on same PR)\n\
                   - Cross-PR replies not supported\n\n\
                 BEST PRACTICES:\n\
                 - Keep replies brief and focused\n\
                   - Address the specific point raised\n\
                   - Don't introduce unrelated topics\n\
                 - Reference specific lines if clarifying\n\
                   - \"In line 52, I meant...\"\n\
                   - Include line references for clarity\n\
                 - Use thread conversations to keep PR discussion organized\n\
                   - Don't start new thread when replying\n\
                   - Keeps related comments together\n\
                 - Don't create new comment when reply would suffice\n\
                   - Reply maintains context\n\
                   - New comments clutter the PR\n\
                 - Include implementation details (commit SHA) when applicable\n\
                   - \"Fixed in commit abc1234\"\n\
                   - Links to specific changes\n\
                 - Be respectful in disagreements\n\
                   - Explain your reasoning clearly\n\
                   - Acknowledge valid points\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found: Comment ID doesn't exist\n\
                    Fix: Verify in_reply_to comment ID is valid and on same PR\n\
                    - Check comment wasn't deleted\n\
                    - Ensure comment ID is from this PR, not another\n\
                    - Verify PR number matches the comment's PR\n\n\
                 2. 422 Unprocessable: Trying to reply with commit_id/path/line\n\
                    Fix: When using in_reply_to, do NOT include commit_id, path, or line\n\
                    - These parameters are mutually exclusive\n\
                    - Reply inherits location from parent comment\n\
                    - Remove commit_id, path, and line from request\n\n\
                 3. 403 Forbidden: Token lacks permissions\n\
                    Fix: Use GITHUB_TOKEN with 'repo' scope\n\
                    - Same permissions as creating new comments\n\
                    - Ensure token has write access to repository",
            ),
        },
    ]
}

impl PromptProvider for AddPullRequestReviewCommentPrompts {
    type PromptArgs = AddPullRequestReviewCommentPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("multiline") => prompt_multiline(),
            Some("reply") => prompt_reply(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}
