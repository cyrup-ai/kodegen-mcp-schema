//! Prompt messages for github_get_pr tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GithubGetPrPromptArgs;

/// Prompt provider for github_get_pr tool
///
/// This is the ONLY way to provide prompts for github_get_pr - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GithubGetPrPrompts;

impl PromptProvider for GithubGetPrPrompts {
    type PromptArgs = GithubGetPrPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("status") => prompt_status(),
            Some("changes") => prompt_changes(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, status, changes, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GITHUB_GET_PR
// ============================================================================

/// Basic PR retrieval
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I get pull request details using the github_get_pr tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_pr tool retrieves detailed information about a specific pull request. Here's how to use it:\n\n\
                 BASIC PR RETRIEVAL:\n\
                 github_get_pr({\n\
                   \"owner\": \"facebook\",\n\
                   \"repo\": \"react\",\n\
                   \"pull_number\": 28000\n\
                 })\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (user or organization)\n\
                 - repo (required): Repository name\n\
                 - pull_number (required): Pull request number (NOT the commit SHA)\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"number\": 28000,\n\
                   \"title\": \"Add new feature\",\n\
                   \"state\": \"open\",\n\
                   \"body\": \"## Summary\\nThis PR adds...\",\n\
                   \"html_url\": \"https://github.com/facebook/react/pull/28000\",\n\
                   \"user\": {\n\
                     \"login\": \"contributor\",\n\
                     \"avatar_url\": \"https://...\"\n\
                   },\n\
                   \"head\": {\n\
                     \"ref\": \"feature-branch\",\n\
                     \"sha\": \"abc123...\",\n\
                     \"repo\": {...}\n\
                   },\n\
                   \"base\": {\n\
                     \"ref\": \"main\",\n\
                     \"sha\": \"def456...\",\n\
                     \"repo\": {...}\n\
                   },\n\
                   \"created_at\": \"2024-01-15T10:30:00Z\",\n\
                   \"updated_at\": \"2024-01-20T14:20:00Z\",\n\
                   \"merged_at\": null,\n\
                   \"closed_at\": null\n\
                 }\n\n\
                 KEY FIELDS EXPLAINED:\n\
                 - number: PR identifier (use this to reference the PR)\n\
                 - title: PR title/summary\n\
                 - state: \"open\", \"closed\", or \"merged\"\n\
                 - body: PR description (markdown formatted)\n\
                 - html_url: Direct link to view PR on GitHub\n\
                 - user: PR author information\n\
                 - head: Source branch (the branch being merged FROM)\n\
                 - base: Target branch (the branch being merged INTO)\n\
                 - created_at: When PR was opened\n\
                 - updated_at: Last modification time\n\
                 - merged_at: When PR was merged (null if not merged)\n\
                 - closed_at: When PR was closed (null if still open)\n\n\
                 COMMON USE CASES:\n\
                 1. Get PR details to understand proposed changes:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 123})\n\n\
                 2. Check if PR is still open:\n\
                    // Check state field in response\n\
                    state === \"open\" means PR is active\n\n\
                 3. Get branch names for local checkout:\n\
                    // Use head.ref for source branch name\n\
                    // Use base.ref for target branch name\n\n\
                 4. Find PR author:\n\
                    // Check user.login field\n\n\
                 UNDERSTANDING STATE:\n\
                 - \"open\": PR is active and awaiting review/merge\n\
                 - \"closed\": PR was closed without merging\n\
                 - \"merged\": PR was successfully merged (merged_at will have timestamp)\n\n\
                 UNDERSTANDING BRANCHES:\n\
                 - head.ref: Source branch (where changes come from)\n\
                 - base.ref: Target branch (where changes go to)\n\
                 - Example: PR from \"feature/dark-mode\" → \"main\"\n\
                   head.ref = \"feature/dark-mode\"\n\
                   base.ref = \"main\"\n\n\
                 FINDING PR NUMBER:\n\
                 - From GitHub URL: https://github.com/owner/repo/pull/NUMBER\n\
                 - Use github_list_pull_requests to find PRs\n\
                 - Use github_search_issues with \"type:pr\" to search PRs\n\n\
                 ERROR HANDLING:\n\
                 - 404: PR doesn't exist or repo not found\n\
                 - 403: No access to private repo\n\
                 - Invalid pull_number: Must be positive integer\n\n\
                 RELATED TOOLS:\n\
                 - github_list_pull_requests: List all PRs in a repo\n\
                 - github_get_pull_request_files: Get files changed in PR\n\
                 - github_get_pull_request_reviews: Get PR reviews\n\
                 - github_get_pull_request_status: Get CI/CD status checks",
            ),
        },
    ]
}

/// PR status information
fn prompt_status() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I check the merge and review status of a pull request?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_pr response includes comprehensive status information about merge readiness and review state.\n\n\
                 STATUS FIELDS IN RESPONSE:\n\
                 {\n\
                   \"mergeable\": true,\n\
                   \"mergeable_state\": \"clean\",\n\
                   \"merged\": false,\n\
                   \"merged_by\": null,\n\
                   \"merge_commit_sha\": null,\n\
                   \"draft\": false,\n\
                   \"locked\": false,\n\
                   \"review_comments\": 5,\n\
                   \"comments\": 12,\n\
                   \"commits\": 3,\n\
                   \"additions\": 150,\n\
                   \"deletions\": 20,\n\
                   \"changed_files\": 8\n\
                 }\n\n\
                 MERGEABLE FIELD:\n\
                 - true: PR can be merged (no conflicts)\n\
                 - false: PR has conflicts that must be resolved\n\
                 - null: GitHub is still calculating merge status (check again shortly)\n\n\
                 MERGEABLE_STATE VALUES:\n\
                 - \"clean\": Ready to merge! All checks passed, no conflicts\n\
                 - \"unstable\": Mergeable but CI checks are failing\n\
                 - \"blocked\": Cannot merge - blocked by required reviews or checks\n\
                 - \"behind\": Branch is behind base, needs update\n\
                 - \"dirty\": Has merge conflicts, must resolve\n\
                 - \"unknown\": Status still being calculated\n\
                 - \"draft\": PR is in draft state\n\n\
                 MERGE STATUS:\n\
                 - merged (boolean): true if PR was merged\n\
                 - merged_at (timestamp): When PR was merged\n\
                 - merged_by (object): User who merged the PR\n\
                 - merge_commit_sha (string): SHA of merge commit\n\n\
                 DRAFT STATUS:\n\
                 - draft: true means PR is marked as work-in-progress\n\
                 - Draft PRs cannot be merged until marked ready\n\
                 - Draft PRs may not trigger full CI/CD pipelines\n\n\
                 REVIEW STATUS:\n\
                 - review_comments: Number of line-specific review comments\n\
                 - comments: Total number of issue comments on PR\n\
                 - To see actual reviews, use github_get_pull_request_reviews\n\n\
                 CHANGE METRICS:\n\
                 - commits: Number of commits in PR\n\
                 - additions: Total lines added\n\
                 - deletions: Total lines deleted\n\
                 - changed_files: Number of files modified\n\n\
                 CHECKING MERGE READINESS:\n\
                 1. Get PR status:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 123})\n\n\
                 2. Check critical fields:\n\
                    if (mergeable === true && mergeable_state === \"clean\") {\n\
                      // Ready to merge!\n\
                    }\n\n\
                 3. Handle different states:\n\
                    - \"clean\": Safe to merge\n\
                    - \"unstable\": Check CI logs, may still merge if allowed\n\
                    - \"blocked\": Need approvals or passing checks\n\
                    - \"dirty\": Resolve conflicts first\n\
                    - \"behind\": Update branch with base branch\n\n\
                 WORKFLOW: CHECK BEFORE MERGE:\n\
                 1. Get PR details:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 456})\n\n\
                 2. Verify merge readiness:\n\
                    - mergeable: true\n\
                    - mergeable_state: \"clean\" or \"unstable\" (if CI failures allowed)\n\
                    - draft: false\n\
                    - merged: false\n\n\
                 3. Check reviews if required:\n\
                    github_get_pull_request_reviews({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 456})\n\
                    // Ensure sufficient approvals\n\n\
                 4. Check CI status:\n\
                    github_get_pull_request_status({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 456})\n\
                    // Verify required checks passed\n\n\
                 5. Merge if ready:\n\
                    github_merge_pull_request({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 456})\n\n\
                 UNDERSTANDING CONFLICTS:\n\
                 If mergeable === false:\n\
                 1. Get PR details to confirm conflict\n\
                 2. Check mergeable_state === \"dirty\"\n\
                 3. Options:\n\
                    a) Update branch from base: git merge main\n\
                    b) Rebase branch: git rebase main\n\
                    c) Resolve conflicts manually\n\
                 4. Push updated branch\n\
                 5. Check again with github_get_pr\n\n\
                 NULL MERGEABLE STATUS:\n\
                 If mergeable === null:\n\
                 - GitHub is still calculating\n\
                 - This happens for newly opened/updated PRs\n\
                 - Wait 10-30 seconds and call github_get_pr again\n\
                 - Eventually will become true or false\n\n\
                 LOCKED PRs:\n\
                 - locked: true means PR conversation is locked\n\
                 - Only collaborators can comment on locked PRs\n\
                 - Can still be merged if otherwise ready\n\n\
                 BEST PRACTICES:\n\
                 1. Always check mergeable before attempting merge\n\
                 2. If mergeable_state is not \"clean\", investigate further\n\
                 3. For \"blocked\" state, check required status checks\n\
                 4. For \"dirty\" state, resolve conflicts\n\
                 5. For \"behind\" state, update branch from base\n\
                 6. Handle null mergeable by retrying after delay",
            ),
        },
    ]
}

/// PR changes information
fn prompt_changes() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I get information about changes in a pull request?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_pr response includes summary metrics about changes, and you can get detailed file changes with other tools.\n\n\
                 CHANGE METRICS IN RESPONSE:\n\
                 {\n\
                   \"commits\": 3,\n\
                   \"additions\": 150,\n\
                   \"deletions\": 20,\n\
                   \"changed_files\": 8,\n\
                   \"head\": {\n\
                     \"sha\": \"abc123...\"\n\
                   },\n\
                   \"base\": {\n\
                     \"sha\": \"def456...\"\n\
                   }\n\
                 }\n\n\
                 UNDERSTANDING METRICS:\n\
                 - commits: Number of commits in the PR\n\
                   - Each commit represents a set of changes\n\
                   - Use github_list_commits to see individual commits\n\n\
                 - additions: Total lines of code added\n\
                   - Green lines in diff view\n\
                   - Includes new files and added lines in existing files\n\n\
                 - deletions: Total lines of code removed\n\
                   - Red lines in diff view\n\
                   - Includes deleted files and removed lines\n\n\
                 - changed_files: Number of files modified\n\
                   - Includes new, modified, and deleted files\n\
                   - Use github_get_pull_request_files for details\n\n\
                 GETTING DETAILED FILE CHANGES:\n\
                 Use github_get_pull_request_files to see what files changed:\n\
                 github_get_pull_request_files({\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"pull_number\": 123\n\
                 })\n\n\
                 Returns array of changed files:\n\
                 [\n\
                   {\n\
                     \"filename\": \"src/app.js\",\n\
                     \"status\": \"modified\",\n\
                     \"additions\": 50,\n\
                     \"deletions\": 10,\n\
                     \"changes\": 60,\n\
                     \"patch\": \"@@ -1,5 +1,10 @@\\n...\"\n\
                   },\n\
                   {\n\
                     \"filename\": \"src/new-feature.js\",\n\
                     \"status\": \"added\",\n\
                     \"additions\": 100,\n\
                     \"deletions\": 0,\n\
                     \"changes\": 100\n\
                   }\n\
                 ]\n\n\
                 FILE STATUS VALUES:\n\
                 - \"added\": New file created\n\
                 - \"modified\": Existing file changed\n\
                 - \"removed\": File deleted\n\
                 - \"renamed\": File moved/renamed\n\n\
                 REVIEWING PR CHANGES WORKFLOW:\n\
                 1. Get PR summary:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 123})\n\
                    // Check commits, additions, deletions, changed_files\n\n\
                 2. Assess PR size:\n\
                    - Small PR: < 100 additions, < 5 files\n\
                    - Medium PR: 100-500 additions, 5-15 files\n\
                    - Large PR: > 500 additions, > 15 files\n\n\
                 3. Get detailed file list:\n\
                    github_get_pull_request_files({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 123})\n\n\
                 4. Review specific files:\n\
                    github_get_file_contents({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \"src/app.js\",\n\
                      \"ref\": \"feature-branch\"  // Use head.ref from PR\n\
                    })\n\n\
                 ANALYZING CHANGE IMPACT:\n\
                 1. Check number of commits:\n\
                    - Single commit: Atomic change\n\
                    - Multiple commits: Evolution of feature\n\
                    - Many commits (>10): May need squashing\n\n\
                 2. Evaluate code churn:\n\
                    - High additions + low deletions: New feature\n\
                    - High deletions + low additions: Refactoring/cleanup\n\
                    - High both: Major rewrite\n\n\
                 3. Assess scope:\n\
                    - Few files: Focused change\n\
                    - Many files: Wide-reaching impact\n\
                    - Check if tests included (test/ directories)\n\n\
                 GETTING COMMIT DETAILS:\n\
                 Use github_list_commits to see individual commits:\n\
                 github_list_commits({\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"sha\": \"feature-branch\"  // Use head.ref from PR\n\
                 })\n\n\
                 COMPARING BRANCHES:\n\
                 To see what changed between base and head:\n\
                 1. Get PR to find branch names:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 123})\n\
                    // Note: head.ref (source), base.ref (target)\n\n\
                 2. Get files changed:\n\
                    github_get_pull_request_files({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 123})\n\n\
                 3. Review diffs in patch field of each file\n\n\
                 CHECKING FOR SPECIFIC CHANGES:\n\
                 1. Get file list:\n\
                    github_get_pull_request_files({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 123})\n\n\
                 2. Filter by filename or path:\n\
                    files.filter(f => f.filename.includes('test'))\n\
                    files.filter(f => f.status === 'added')\n\n\
                 3. Check specific file types:\n\
                    const jsFiles = files.filter(f => f.filename.endsWith('.js'))\n\
                    const testFiles = files.filter(f => f.filename.includes('test'))\n\n\
                 BEST PRACTICES:\n\
                 1. Always check changed_files count to gauge review effort\n\
                 2. Large PRs (>500 additions) should be reviewed in chunks\n\
                 3. Use additions/deletions ratio to understand change type\n\
                 4. Check if tests were added (look for test files)\n\
                 5. Use github_get_pull_request_files for detailed review\n\
                 6. For large PRs, consider requesting smaller PRs\n\
                 7. Review patch diffs for line-by-line changes",
            ),
        },
    ]
}

/// PR review workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows for reviewing and working with pull requests?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Here are comprehensive workflows for common PR operations using github_get_pr and related tools.\n\n\
                 =============================================================================\n\
                 WORKFLOW 1: REVIEW A PULL REQUEST\n\
                 =============================================================================\n\n\
                 1. Get PR details:\n\
                    github_get_pr({\n\
                      \"owner\": \"facebook\",\n\
                      \"repo\": \"react\",\n\
                      \"pull_number\": 28000\n\
                    })\n\
                    // Review: title, body, user, state, branches\n\n\
                 2. Check change metrics:\n\
                    // From response: commits, additions, deletions, changed_files\n\
                    // Assess PR size and complexity\n\n\
                 3. Get list of changed files:\n\
                    github_get_pull_request_files({\n\
                      \"owner\": \"facebook\",\n\
                      \"repo\": \"react\",\n\
                      \"pull_number\": 28000\n\
                    })\n\
                    // Review each file's changes\n\n\
                 4. Check existing reviews:\n\
                    github_get_pull_request_reviews({\n\
                      \"owner\": \"facebook\",\n\
                      \"repo\": \"react\",\n\
                      \"pull_number\": 28000\n\
                    })\n\
                    // See what others have said\n\n\
                 5. Add review comments:\n\
                    github_create_pull_request_review({\n\
                      \"owner\": \"facebook\",\n\
                      \"repo\": \"react\",\n\
                      \"pull_number\": 28000,\n\
                      \"body\": \"Great work! LGTM\",\n\
                      \"event\": \"APPROVE\"\n\
                    })\n\n\
                 =============================================================================\n\
                 WORKFLOW 2: CHECK BEFORE MERGING\n\
                 =============================================================================\n\n\
                 1. Get current PR status:\n\
                    github_get_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456\n\
                    })\n\n\
                 2. Verify merge readiness:\n\
                    // Check response fields:\n\
                    if (mergeable === true && \n\
                        mergeable_state === \"clean\" && \n\
                        draft === false && \n\
                        merged === false) {\n\
                      // Ready to proceed\n\
                    }\n\n\
                 3. Check required reviews:\n\
                    github_get_pull_request_reviews({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456\n\
                    })\n\
                    // Count approvals, ensure no request_changes\n\n\
                 4. Check CI status:\n\
                    github_get_pull_request_status({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456\n\
                    })\n\
                    // Verify all required checks passed\n\n\
                 5. Merge PR:\n\
                    github_merge_pull_request({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 456,\n\
                      \"merge_method\": \"squash\"  // or \"merge\", \"rebase\"\n\
                    })\n\n\
                 =============================================================================\n\
                 WORKFLOW 3: CHECKOUT PR LOCALLY\n\
                 =============================================================================\n\n\
                 1. Get PR branch information:\n\
                    github_get_pr({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 789\n\
                    })\n\
                    // Note head.ref (branch name) and head.sha (commit)\n\n\
                 2. Fetch repository updates:\n\
                    git_fetch({\n\
                      \"path\": \"/local/project\"\n\
                    })\n\n\
                 3. Checkout PR branch:\n\
                    git_checkout({\n\
                      \"path\": \"/local/project\",\n\
                      \"branch\": \"feature-branch\"  // Use head.ref from step 1\n\
                    })\n\n\
                 4. Alternative - Checkout by PR number:\n\
                    // Fetch PR as local branch\n\
                    terminal({\n\
                      \"command\": \"cd /local/project && git fetch origin pull/789/head:pr-789 && git checkout pr-789\"\n\
                    })\n\n\
                 5. Test changes locally:\n\
                    terminal({\n\
                      \"command\": \"npm test\"  // or cargo test, etc.\n\
                    })\n\n\
                 =============================================================================\n\
                 WORKFLOW 4: MONITOR PR UPDATES\n\
                 =============================================================================\n\n\
                 1. Get initial PR state:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 123})\n\
                    // Note updated_at timestamp\n\n\
                 2. Check for updates later:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 123})\n\
                    // Compare updated_at to previous value\n\n\
                 3. If updated, check what changed:\n\
                    // Compare commits count\n\
                    // Check new review_comments count\n\
                    // Check mergeable_state changes\n\n\
                 4. Get new comments:\n\
                    github_get_issue_comments({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"issue_number\": 123  // PRs use issue number\n\
                    })\n\n\
                 5. Get new reviews:\n\
                    github_get_pull_request_reviews({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 123\n\
                    })\n\n\
                 =============================================================================\n\
                 WORKFLOW 5: HANDLE MERGE CONFLICTS\n\
                 =============================================================================\n\n\
                 1. Detect conflicts:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 555})\n\
                    // Check: mergeable === false && mergeable_state === \"dirty\"\n\n\
                 2. Get conflicting files:\n\
                    github_get_pull_request_files({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 555\n\
                    })\n\
                    // Files with conflicts will be marked\n\n\
                 3. Checkout PR locally (see Workflow 3)\n\n\
                 4. Update from base branch:\n\
                    terminal({\n\
                      \"command\": \"cd /local/project && git fetch origin && git merge origin/main\"\n\
                    })\n\
                    // Conflicts will appear\n\n\
                 5. Resolve conflicts:\n\
                    // Read conflicting files\n\
                    fs_read_file({\"path\": \"/local/project/src/app.js\"})\n\
                    // Edit to resolve conflicts\n\
                    fs_edit_block({...})\n\n\
                 6. Commit resolution:\n\
                    terminal({\n\
                      \"command\": \"cd /local/project && git add . && git commit -m 'Resolve merge conflicts'\"\n\
                    })\n\n\
                 7. Push update:\n\
                    terminal({\n\
                      \"command\": \"cd /local/project && git push\"\n\
                    })\n\n\
                 8. Verify resolution:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 555})\n\
                    // Wait ~30s, then check: mergeable === true\n\n\
                 =============================================================================\n\
                 WORKFLOW 6: FIND AND REVIEW RELATED PRS\n\
                 =============================================================================\n\n\
                 1. Get current PR:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 100})\n\
                    // Note: user.login, head.ref, base.ref\n\n\
                 2. Find related PRs by same author:\n\
                    github_list_pull_requests({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"state\": \"open\"\n\
                    })\n\
                    // Filter by user.login from step 1\n\n\
                 3. Find PRs to same target branch:\n\
                    github_list_pull_requests({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"base\": \"main\",\n\
                      \"state\": \"open\"\n\
                    })\n\n\
                 4. Search for PRs with keywords:\n\
                    github_search_issues({\n\
                      \"query\": \"repo:user/project type:pr is:open feature\"\n\
                    })\n\n\
                 =============================================================================\n\
                 WORKFLOW 7: UPDATE PR DETAILS\n\
                 =============================================================================\n\n\
                 1. Get current PR:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 200})\n\
                    // Review current title, body, state\n\n\
                 2. Update PR:\n\
                    github_update_pull_request({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"pull_number\": 200,\n\
                      \"title\": \"New title\",\n\
                      \"body\": \"Updated description\"\n\
                    })\n\n\
                 3. Verify update:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 200})\n\
                    // Check updated_at timestamp changed\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. ALWAYS check mergeable before attempting merge\n\
                 2. Review mergeable_state to understand why PR can't merge\n\
                 3. Check draft status - draft PRs cannot be merged\n\
                 4. Use head.ref and base.ref for branch operations\n\
                 5. If mergeable is null, wait 10-30s and check again\n\
                 6. Check review_comments and get full reviews before merging\n\
                 7. Use github_get_pull_request_status to verify CI checks\n\
                 8. For large PRs, review github_get_pull_request_files first\n\
                 9. Always verify merged field before attempting merge\n\
                 10. Use updated_at to detect if PR changed since last check\n\n\
                 =============================================================================\n\
                 COMMON FIELD USAGE\n\
                 =============================================================================\n\n\
                 number: Reference PR in other API calls\n\
                 html_url: Share link with users\n\
                 user.login: Find PR author\n\
                 head.ref: Checkout source branch locally\n\
                 head.sha: Reference specific commit\n\
                 base.ref: Know target branch\n\
                 mergeable: Check if PR can be merged\n\
                 mergeable_state: Understand merge blockers\n\
                 state: Check if open/closed/merged\n\
                 draft: Know if PR is ready\n\
                 merged: Check if already merged\n\
                 commits: Assess PR complexity\n\
                 changed_files: Gauge review effort\n\
                 created_at: Know PR age\n\
                 updated_at: Detect changes",
            ),
        },
    ]
}

/// Comprehensive guide to github_get_pr
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using the github_get_pr tool effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_pr tool retrieves comprehensive information about a pull request, including status, changes, and metadata.\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 github_get_pr({\n\
                   \"owner\": \"facebook\",\n\
                   \"repo\": \"react\",\n\
                   \"pull_number\": 28000\n\
                 })\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner (user or organization)\n\
                 - repo: Repository name\n\
                 - pull_number: PR number (from URL or list)\n\n\
                 =============================================================================\n\
                 COMPLETE RESPONSE STRUCTURE\n\
                 =============================================================================\n\n\
                 {\n\
                   // BASIC INFO\n\
                   \"number\": 28000,\n\
                   \"title\": \"Add suspense support\",\n\
                   \"state\": \"open\",\n\
                   \"body\": \"## Summary\\nThis PR adds...\",\n\
                   \"html_url\": \"https://github.com/facebook/react/pull/28000\",\n\
                   \"draft\": false,\n\
                   \"locked\": false,\n\n\
                   // AUTHOR\n\
                   \"user\": {\n\
                     \"login\": \"contributor\",\n\
                     \"id\": 12345,\n\
                     \"avatar_url\": \"https://...\",\n\
                     \"html_url\": \"https://github.com/contributor\"\n\
                   },\n\n\
                   // BRANCHES\n\
                   \"head\": {\n\
                     \"label\": \"contributor:feature-branch\",\n\
                     \"ref\": \"feature-branch\",\n\
                     \"sha\": \"abc123def456...\",\n\
                     \"user\": {...},\n\
                     \"repo\": {...}\n\
                   },\n\
                   \"base\": {\n\
                     \"label\": \"facebook:main\",\n\
                     \"ref\": \"main\",\n\
                     \"sha\": \"def456ghi789...\",\n\
                     \"user\": {...},\n\
                     \"repo\": {...}\n\
                   },\n\n\
                   // MERGE STATUS\n\
                   \"mergeable\": true,\n\
                   \"mergeable_state\": \"clean\",\n\
                   \"merged\": false,\n\
                   \"merged_at\": null,\n\
                   \"merged_by\": null,\n\
                   \"merge_commit_sha\": null,\n\n\
                   // REVIEW INFO\n\
                   \"review_comments\": 5,\n\
                   \"comments\": 12,\n\n\
                   // CHANGE METRICS\n\
                   \"commits\": 3,\n\
                   \"additions\": 150,\n\
                   \"deletions\": 20,\n\
                   \"changed_files\": 8,\n\n\
                   // TIMESTAMPS\n\
                   \"created_at\": \"2024-01-15T10:30:00Z\",\n\
                   \"updated_at\": \"2024-01-20T14:20:00Z\",\n\
                   \"closed_at\": null\n\
                 }\n\n\
                 =============================================================================\n\
                 KEY FIELDS REFERENCE\n\
                 =============================================================================\n\n\
                 IDENTIFICATION:\n\
                 number          PR number for API calls\n\
                 html_url        Direct GitHub link\n\
                 title           PR summary\n\
                 body            Full description (markdown)\n\n\
                 STATE FIELDS:\n\
                 state           \"open\", \"closed\", or \"merged\"\n\
                 draft           Is PR marked as work-in-progress\n\
                 locked          Is PR conversation locked\n\
                 merged          Was PR successfully merged\n\
                 merged_at       Merge timestamp (null if not merged)\n\
                 closed_at       Close timestamp (null if open)\n\n\
                 MERGE READINESS:\n\
                 mergeable           Can merge? (true/false/null)\n\
                 mergeable_state     Why/why not (clean/dirty/blocked/etc)\n\
                 merged              Already merged?\n\
                 merge_commit_sha    Merge commit (if merged)\n\
                 merged_by           User who merged (if merged)\n\n\
                 AUTHOR:\n\
                 user.login      GitHub username\n\
                 user.id         User ID\n\
                 user.html_url   Profile link\n\n\
                 BRANCHES:\n\
                 head.ref        Source branch name\n\
                 head.sha        Latest commit SHA\n\
                 base.ref        Target branch name\n\
                 base.sha        Base commit SHA\n\n\
                 CHANGES:\n\
                 commits         Number of commits\n\
                 additions       Lines added (+)\n\
                 deletions       Lines removed (-)\n\
                 changed_files   Files modified\n\n\
                 REVIEWS:\n\
                 review_comments     Line-specific review comments\n\
                 comments            General PR comments\n\n\
                 TIMESTAMPS:\n\
                 created_at      When PR opened\n\
                 updated_at      Last modification\n\
                 merged_at       When merged (null if not)\n\
                 closed_at       When closed (null if open)\n\n\
                 =============================================================================\n\
                 STATE VALUES EXPLAINED\n\
                 =============================================================================\n\n\
                 PR STATE (state field):\n\
                 - \"open\": PR is active, awaiting review/merge\n\
                 - \"closed\": PR was closed without merging\n\
                 - \"merged\": PR was successfully merged (check merged_at)\n\n\
                 MERGEABLE VALUES:\n\
                 - true: No conflicts, can merge\n\
                 - false: Has conflicts, must resolve\n\
                 - null: GitHub still calculating (check again in 10-30s)\n\n\
                 MERGEABLE_STATE VALUES:\n\
                 - \"clean\": Ready to merge! All clear\n\
                 - \"unstable\": Mergeable but CI failing\n\
                 - \"blocked\": Blocked by required reviews/checks\n\
                 - \"behind\": Branch behind base, needs update\n\
                 - \"dirty\": Merge conflicts exist\n\
                 - \"unknown\": Status being calculated\n\
                 - \"draft\": PR is in draft mode\n\n\
                 =============================================================================\n\
                 COMMON USE CASES\n\
                 =============================================================================\n\n\
                 1. CHECK PR STATUS:\n\
                 github_get_pr({\"owner\": \"user\", \"repo\": \"project\", \"pull_number\": 123})\n\
                 // Look at: state, mergeable, mergeable_state, draft\n\n\
                 2. GET BRANCH NAMES:\n\
                 // Use head.ref (source) and base.ref (target) from response\n\
                 // Example: Merging \"feature-branch\" into \"main\"\n\n\
                 3. CHECK MERGE READINESS:\n\
                 // Verify: mergeable === true && mergeable_state === \"clean\"\n\n\
                 4. ASSESS PR SIZE:\n\
                 // Check: commits, additions, deletions, changed_files\n\
                 // Small: <100 additions, <5 files\n\
                 // Large: >500 additions, >15 files\n\n\
                 5. FIND PR AUTHOR:\n\
                 // Use: user.login\n\n\
                 6. GET REVIEW COUNT:\n\
                 // Check: review_comments + comments\n\n\
                 7. DETECT UPDATES:\n\
                 // Compare updated_at timestamp\n\n\
                 8. CHECK IF MERGED:\n\
                 // Look at: merged === true && merged_at !== null\n\n\
                 =============================================================================\n\
                 RELATED TOOLS\n\
                 =============================================================================\n\n\
                 GET MORE DETAILS:\n\
                 github_get_pull_request_files       List changed files with diffs\n\
                 github_get_pull_request_reviews     Get all reviews\n\
                 github_get_pull_request_status      Get CI/CD check status\n\
                 github_get_issue_comments           Get PR comments\n\
                 github_list_commits                 Get commit details\n\n\
                 MODIFY PR:\n\
                 github_update_pull_request          Change title/body/state\n\
                 github_merge_pull_request           Merge the PR\n\
                 github_create_pull_request_review   Add review\n\
                 github_add_pull_request_review_comment  Add line comment\n\n\
                 FIND PRS:\n\
                 github_list_pull_requests           List repo PRs\n\
                 github_search_issues                Search PRs (type:pr)\n\n\
                 =============================================================================\n\
                 DECISION TREES\n\
                 =============================================================================\n\n\
                 CAN I MERGE THIS PR?\n\
                 1. Check mergeable field:\n\
                    - null: Wait 30s, check again\n\
                    - false: Has conflicts, resolve first\n\
                    - true: Continue to step 2\n\n\
                 2. Check mergeable_state:\n\
                    - \"clean\": Ready to merge!\n\
                    - \"unstable\": Check if CI failures allowed\n\
                    - \"blocked\": Need approvals/passing checks\n\
                    - \"dirty\": Resolve conflicts\n\
                    - \"behind\": Update branch\n\
                    - \"draft\": Mark PR as ready\n\n\
                 3. Check merged:\n\
                    - true: Already merged, don't merge again\n\
                    - false: Safe to merge if above checks pass\n\n\
                 WHAT SIZE IS THIS PR?\n\
                 Check additions + changed_files:\n\
                 - <100 additions, <5 files: Small (quick review)\n\
                 - 100-500 additions, 5-15 files: Medium (thorough review)\n\
                 - >500 additions, >15 files: Large (consider splitting)\n\n\
                 IS PR READY FOR REVIEW?\n\
                 Check these fields:\n\
                 - state === \"open\": PR is active\n\
                 - draft === false: Not work-in-progress\n\
                 - commits > 0: Has changes\n\
                 If all true: Ready for review\n\n\
                 WAS PR MERGED OR CLOSED?\n\
                 - merged === true: PR was merged\n\
                 - state === \"closed\" && merged === false: Closed without merging\n\
                 - state === \"open\": Still active\n\n\
                 =============================================================================\n\
                 WORKFLOWS\n\
                 =============================================================================\n\n\
                 REVIEW WORKFLOW:\n\
                 1. github_get_pr(...) - Get overview\n\
                 2. github_get_pull_request_files(...) - See changed files\n\
                 3. github_get_pull_request_reviews(...) - Check existing reviews\n\
                 4. github_create_pull_request_review(...) - Add your review\n\n\
                 MERGE WORKFLOW:\n\
                 1. github_get_pr(...) - Check status\n\
                 2. Verify mergeable === true && mergeable_state === \"clean\"\n\
                 3. github_get_pull_request_reviews(...) - Check approvals\n\
                 4. github_merge_pull_request(...) - Merge if ready\n\n\
                 LOCAL CHECKOUT WORKFLOW:\n\
                 1. github_get_pr(...) - Get head.ref and head.sha\n\
                 2. git_fetch(...) - Update local repo\n\
                 3. git_checkout(...) - Checkout branch using head.ref\n\
                 4. Test locally\n\n\
                 CONFLICT RESOLUTION WORKFLOW:\n\
                 1. github_get_pr(...) - Detect conflicts (mergeable === false)\n\
                 2. Checkout PR locally (see above)\n\
                 3. Merge/rebase with base branch\n\
                 4. Resolve conflicts\n\
                 5. Push changes\n\
                 6. github_get_pr(...) - Verify mergeable === true\n\n\
                 =============================================================================\n\
                 ERROR HANDLING\n\
                 =============================================================================\n\n\
                 404 ERROR:\n\
                 - PR doesn't exist\n\
                 - Wrong pull_number\n\
                 - Repository doesn't exist\n\
                 - Private repo without access\n\n\
                 403 ERROR:\n\
                 - No permission to access repo\n\
                 - Rate limit exceeded\n\
                 - Token doesn't have required scope\n\n\
                 422 ERROR:\n\
                 - Invalid parameters\n\
                 - pull_number must be positive integer\n\n\
                 NULL MERGEABLE:\n\
                 - Not an error - GitHub still calculating\n\
                 - Wait 10-30 seconds\n\
                 - Call github_get_pr again\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. Always check state before attempting operations\n\
                 2. Handle null mergeable by retrying after delay\n\
                 3. Use head.ref and base.ref for branch operations, not labels\n\
                 4. Check draft status before requesting reviews\n\
                 5. Use updated_at to detect if PR changed since last check\n\
                 6. For large PRs (>500 additions), get file list first\n\
                 7. Check mergeable_state to understand merge blockers\n\
                 8. Verify merged === false before attempting merge\n\
                 9. Use review_comments count to gauge discussion activity\n\
                 10. Store pull_number, not SHA, for PR references\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Get PR: github_get_pr({owner, repo, pull_number})\n\
                 Check mergeable: response.mergeable && response.mergeable_state\n\
                 Get branches: response.head.ref (source), response.base.ref (target)\n\
                 Check merged: response.merged === true\n\
                 Get size: response.additions, response.deletions, response.changed_files\n\
                 Get author: response.user.login\n\
                 Check draft: response.draft === true\n\
                 Get updates: response.updated_at timestamp\n\
                 Direct link: response.html_url\n\n\
                 Remember: github_get_pr is your starting point for all PR operations. Use it to check status, get metadata, and coordinate with other GitHub tools!",
            ),
        },
    ]
}
