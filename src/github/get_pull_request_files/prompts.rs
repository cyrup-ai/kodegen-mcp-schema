//! Prompt messages for github_get_pull_request_files tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetPullRequestFilesPromptArgs;

/// Prompt provider for get_pull_request_files tool
///
/// This is the ONLY way to provide prompts for get_pull_request_files - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GetPullRequestFilesPrompts;

impl PromptProvider for GetPullRequestFilesPrompts {
    type PromptArgs = GetPullRequestFilesPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("analysis") => prompt_analysis(),
            _ => prompt_analysis(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, analysis)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO WORK WITH PR CHANGES
// ============================================================================

/// Basic PR file change retrieval
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I get the list of files changed in a pull request?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_pull_request_files tool retrieves all files changed in a PR with detailed diff information.\n\n\
                 BASIC USAGE:\n\
                 1. Get all changed files:\n\
                    github_get_pull_request_files({\n\
                        \"owner\": \"rust-lang\",\n\
                        \"repo\": \"rust\",\n\
                        \"pull_number\": 12345\n\
                    })\n\n\
                 2. With pagination for large PRs:\n\
                    github_get_pull_request_files({\n\
                        \"owner\": \"microsoft\",\n\
                        \"repo\": \"vscode\",\n\
                        \"pull_number\": 5678,\n\
                        \"per_page\": 100,\n\
                        \"page\": 1\n\
                    })\n\n\
                 3. Second page of results:\n\
                    github_get_pull_request_files({\n\
                        \"owner\": \"facebook\",\n\
                        \"repo\": \"react\",\n\
                        \"pull_number\": 999,\n\
                        \"per_page\": 50,\n\
                        \"page\": 2\n\
                    })\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner (username or organization)\n\
                 - repo: Repository name\n\
                 - pull_number: Pull request number\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - page: Page number for pagination (default: 1)\n\
                 - per_page: Results per page (default: 30, max: 100)\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"rust-lang\",\n\
                   \"repo\": \"rust\",\n\
                   \"pr_number\": 12345,\n\
                   \"count\": 5,\n\
                   \"files\": [\n\
                     {\n\
                       \"filename\": \"src/main.rs\",\n\
                       \"status\": \"modified\",\n\
                       \"additions\": 50,\n\
                       \"deletions\": 10,\n\
                       \"changes\": 60,\n\
                       \"patch\": \"@@ -10,7 +10,15 @@...\",\n\
                       \"blob_url\": \"https://github.com/rust-lang/rust/blob/abc123/src/main.rs\",\n\
                       \"raw_url\": \"https://github.com/rust-lang/rust/raw/abc123/src/main.rs\"\n\
                     },\n\
                     {\n\
                       \"filename\": \"src/lib.rs\",\n\
                       \"status\": \"added\",\n\
                       \"additions\": 100,\n\
                       \"deletions\": 0,\n\
                       \"changes\": 100,\n\
                       \"patch\": \"@@ -0,0 +1,100 @@...\"\n\
                     },\n\
                     {\n\
                       \"filename\": \"tests/old_test.rs\",\n\
                       \"status\": \"removed\",\n\
                       \"additions\": 0,\n\
                       \"deletions\": 75,\n\
                       \"changes\": 75\n\
                     },\n\
                     {\n\
                       \"filename\": \"src/utils.rs\",\n\
                       \"status\": \"renamed\",\n\
                       \"additions\": 5,\n\
                       \"deletions\": 5,\n\
                       \"changes\": 10,\n\
                       \"previous_filename\": \"src/helpers.rs\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 FILE STATUS VALUES:\n\
                 - added: Newly created file\n\
                 - modified: Existing file with changes\n\
                 - removed: Deleted file\n\
                 - renamed: File moved or renamed (may also have changes)\n\n\
                 UNDERSTANDING CHANGES:\n\
                 - additions: Number of lines added (green in diffs)\n\
                 - deletions: Number of lines removed (red in diffs)\n\
                 - changes: Total lines affected (additions + deletions)\n\
                 - patch: Unified diff format showing exact changes\n\n\
                 PATCH FORMAT:\n\
                 The patch field contains unified diff format:\n\
                 @@ -10,7 +10,15 @@ means:\n\
                 - Old file: starting at line 10, showing 7 lines\n\
                 - New file: starting at line 10, showing 15 lines\n\
                 Lines starting with:\n\
                 - '+' are additions\n\
                 - '-' are deletions\n\
                 - ' ' (space) are unchanged context\n\n\
                 PAGINATION:\n\
                 GitHub limits results to 30 files per page by default.\n\
                 For PRs with many files:\n\
                 1. First request:\n\
                    github_get_pull_request_files({owner, repo, pull_number, per_page: 100})\n\
                 2. If count == 100, fetch next page:\n\
                    github_get_pull_request_files({owner, repo, pull_number, per_page: 100, page: 2})\n\
                 3. Continue until count < 100\n\n\
                 COMMON PATTERNS:\n\
                 1. Quick file list:\n\
                    github_get_pull_request_files({owner, repo, pull_number})\n\
                    // Review filenames and status fields\n\n\
                 2. Count total changes:\n\
                    github_get_pull_request_files({owner, repo, pull_number})\n\
                    // Sum all 'changes' values\n\n\
                 3. Find specific files:\n\
                    github_get_pull_request_files({owner, repo, pull_number, per_page: 100})\n\
                    // Filter files array by filename pattern\n\n\
                 4. Get all files in large PR:\n\
                    page = 1\n\
                    all_files = []\n\
                    loop:\n\
                      result = github_get_pull_request_files({owner, repo, pull_number, per_page: 100, page})\n\
                      all_files.extend(result.files)\n\
                      if result.count < 100: break\n\
                      page += 1\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable.\n\
                 Scopes needed:\n\
                 - public_repo: For public repositories\n\
                 - repo: For private repositories\n\n\
                 RATE LIMITS:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Unauthenticated: 60 requests/hour\n\
                 Check response headers: X-RateLimit-Remaining\n\n\
                 ERROR HANDLING:\n\
                 1. 404 Not Found:\n\
                    - PR doesn't exist\n\
                    - Repository doesn't exist\n\
                    - Check owner/repo/pull_number values\n\n\
                 2. 403 Forbidden:\n\
                    - No access to private repository\n\
                    - GITHUB_TOKEN missing or invalid\n\
                    - Token lacks required scopes\n\n\
                 3. 500 Internal Server Error:\n\
                    - PR has too many files for single request\n\
                    - Use pagination with smaller per_page values\n\n\
                 4. 422 Unprocessable Entity:\n\
                    - Invalid parameter values\n\
                    - Check page/per_page are positive integers\n\n\
                 BEST PRACTICES:\n\
                 - Use per_page: 100 for large PRs to minimize requests\n\
                 - Cache results to avoid repeated API calls\n\
                 - Check status field to handle different change types\n\
                 - Handle renamed files with previous_filename\n\
                 - Parse patch field for detailed line-by-line analysis\n\
                 - Use blob_url to link to specific file versions\n\
                 - Paginate through all results for complete analysis",
            ),
        },
    ]
}

/// Analyzing PR changes for complexity and impact
fn prompt_analysis() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I analyze the scope and complexity of changes in a pull request?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use github_get_pull_request_files to perform comprehensive change analysis and impact assessment.\n\n\
                 COMPLEXITY METRICS:\n\
                 1. Count total files changed:\n\
                    result = github_get_pull_request_files({owner, repo, pull_number, per_page: 100})\n\
                    total_files = result.count\n\
                    // If count == 100, paginate to get full count\n\n\
                 2. Calculate lines changed:\n\
                    result = github_get_pull_request_files({owner, repo, pull_number, per_page: 100})\n\
                    total_additions = sum(file.additions for file in result.files)\n\
                    total_deletions = sum(file.deletions for file in result.files)\n\
                    total_changes = sum(file.changes for file in result.files)\n\n\
                 3. Identify change types:\n\
                    added_files = filter(file.status == 'added')\n\
                    modified_files = filter(file.status == 'modified')\n\
                    removed_files = filter(file.status == 'removed')\n\
                    renamed_files = filter(file.status == 'renamed')\n\n\
                 4. Find largest changes:\n\
                    sorted_by_changes = sort(files, key=file.changes, reverse=True)\n\
                    top_10_changed = sorted_by_changes[:10]\n\
                    // Focus review on files with most changes\n\n\
                 IMPACT ANALYSIS:\n\
                 1. Identify affected directories:\n\
                    result = github_get_pull_request_files({owner, repo, pull_number, per_page: 100})\n\
                    directories = set(dirname(file.filename) for file in result.files)\n\
                    // Shows which parts of codebase are touched\n\n\
                 2. Categorize by file type:\n\
                    rust_files = filter(file.filename.endsWith('.rs'))\n\
                    test_files = filter(file.filename.includes('test'))\n\
                    config_files = filter(file.filename.endsWith('.toml', '.json', '.yaml'))\n\
                    doc_files = filter(file.filename.endsWith('.md'))\n\n\
                 3. Detect scope:\n\
                    if all changes in single directory:\n\
                      scope = \"localized\" (low risk)\n\
                    if changes span multiple directories:\n\
                      scope = \"broad\" (requires careful review)\n\
                    if changes touch core modules:\n\
                      scope = \"critical\" (requires thorough testing)\n\n\
                 4. Calculate change ratio:\n\
                    if total_additions > total_deletions * 3:\n\
                      type = \"expansion\" (new features, may need tests)\n\
                    if total_deletions > total_additions * 3:\n\
                      type = \"cleanup\" (removing code, verify no regressions)\n\
                    else:\n\
                      type = \"refactoring\" (check for behavior changes)\n\n\
                 PR SIZE ASSESSMENT:\n\
                 Based on total changes:\n\
                 - 0-50 lines: XS (trivial, quick review)\n\
                 - 51-200 lines: S (small, 15-30 min review)\n\
                 - 201-500 lines: M (medium, 30-60 min review)\n\
                 - 501-1000 lines: L (large, 1-2 hour review)\n\
                 - 1000+ lines: XL (very large, consider splitting)\n\n\
                 RISK ASSESSMENT:\n\
                 High risk indicators:\n\
                 - Changes to core/critical files\n\
                 - Large number of modified files (>20)\n\
                 - High deletion ratio (data loss risk)\n\
                 - No corresponding test changes\n\
                 - Changes span multiple subsystems\n\n\
                 Low risk indicators:\n\
                 - Only test file changes\n\
                 - Documentation only\n\
                 - Configuration updates\n\
                 - Single file modifications\n\
                 - Matched test coverage additions\n\n\
                 CHANGE PATTERNS:\n\
                 1. Feature addition:\n\
                    - New files added (status: 'added')\n\
                    - Test files added in parallel\n\
                    - Documentation updated\n\
                    - Additions > deletions\n\n\
                 2. Bug fix:\n\
                    - Few files modified (1-5)\n\
                    - Small line changes (10-100)\n\
                    - Test cases added\n\
                    - Focused in specific area\n\n\
                 3. Refactoring:\n\
                    - Many files modified\n\
                    - Similar additions/deletions ratio\n\
                    - No new features\n\
                    - May include renames\n\n\
                 4. Breaking change:\n\
                    - API file modifications\n\
                    - Many dependent files changed\n\
                    - Large deletion count\n\
                    - Version number changes\n\n\
                 BEST PRACTICES:\n\
                 - Always analyze change distribution, not just total lines\n\
                 - Look for imbalanced changes (all additions or deletions)\n\
                 - Identify if tests were added/updated\n\
                 - Check for configuration or migration changes\n\
                 - Flag PRs with >500 lines for potential splitting\n\
                 - Verify renamed files didn't also change significantly\n\
                 - Compare changed files against known critical paths\n\
                 - Use metrics to prioritize review focus areas",
            ),
        },
    ]
}
