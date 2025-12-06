//! Prompt messages for github_list_branches tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ListBranchesPromptArgs;

/// Prompt provider for list_branches tool
///
/// This is the ONLY way to provide prompts for list_branches - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ListBranchesPrompts;

impl PromptProvider for ListBranchesPrompts {
    type PromptArgs = ListBranchesPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("pagination") => prompt_pagination(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, pagination)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic branch listing examples
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list branches in a GitHub repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_list_branches tool retrieves all branches in a GitHub repository without requiring a local clone.\n\n\
                 BASIC BRANCH LISTING:\n\
                 1. List all branches in a repository:\n\
                    github_list_branches({\"owner\": \"tokio-rs\", \"repo\": \"tokio\"})\n\n\
                 2. List branches with default pagination:\n\
                    github_list_branches({\"owner\": \"rust-lang\", \"repo\": \"rust\"})\n\n\
                 3. List first 50 branches:\n\
                    github_list_branches({\"owner\": \"actix\", \"repo\": \"actix-web\", \"per_page\": 50})\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"branches\": [\n\
                     {\n\
                       \"name\": \"main\",\n\
                       \"commit\": {\"sha\": \"abc1234567890abcdef1234567890abcdef12345\"},\n\
                       \"protected\": true\n\
                     },\n\
                     {\n\
                       \"name\": \"develop\",\n\
                       \"commit\": {\"sha\": \"def5678901234def5678901234def56789012345\"},\n\
                       \"protected\": false\n\
                     },\n\
                     {\n\
                       \"name\": \"feature/new-api\",\n\
                       \"commit\": {\"sha\": \"ghi9012345678ghi9012345678ghi90123456789\"},\n\
                       \"protected\": false\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 BRANCH INFORMATION:\n\
                 - name: Branch name (string)\n\
                 - commit.sha: Latest commit SHA on this branch\n\
                 - protected: Whether branch has protection rules enabled\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - per_page (optional): Number of results per page (default: 30, max: 100)\n\
                 - page (optional): Page number for pagination (default: 1)\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable:\n\
                 - Public repos: Any valid token or none\n\
                 - Private repos: Token with 'repo' scope\n\n\
                 COMMON USE CASES:\n\
                 1. Explore repository structure:\n\
                    github_list_branches({\"owner\": \"facebook\", \"repo\": \"react\"})\n\
                    Review all branches to understand development workflow\n\n\
                 2. Find main branch:\n\
                    github_list_branches({\"owner\": \"microsoft\", \"repo\": \"vscode\"})\n\
                    Look for 'main', 'master', or 'develop' in results\n\n\
                 3. Check branch exists before operations:\n\
                    github_list_branches({\"owner\": \"vercel\", \"repo\": \"next.js\"})\n\
                    Verify branch name exists before creating PR or merging\n\n\
                 4. Discover feature branches:\n\
                    github_list_branches({\"owner\": \"vuejs\", \"repo\": \"vue\"})\n\
                    Filter results for feature/* or bugfix/* patterns\n\n\
                 RESPONSE HANDLING:\n\
                 - Empty array: Repository has no branches (rare, newly created)\n\
                 - Check 'protected' field to identify critical branches\n\
                 - Use commit.sha to check if branch is up-to-date\n\
                 - Branch names may contain slashes (feature/name, fix/bug)\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found:\n\
                    Repository doesn't exist or not accessible\n\
                    Verify owner and repo names are correct\n\n\
                 2. 403 Forbidden:\n\
                    Token lacks permissions for private repository\n\
                    Ensure GITHUB_TOKEN has 'repo' scope\n\n\
                 3. Rate limit exceeded:\n\
                    Too many API requests\n\
                    Wait for rate limit reset or use authenticated token\n\n\
                 BEST PRACTICES:\n\
                 - List branches before creating new ones to avoid name conflicts\n\
                 - Check protected status before attempting destructive operations\n\
                 - Use pagination for repositories with many branches\n\
                 - Cache results when listing branches multiple times\n\
                 - Combine with github_get_commit for detailed branch analysis",
            ),
        },
    ]
}

/// Pagination handling for large branch lists
fn prompt_pagination() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I handle pagination when listing branches in large repositories?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Large repositories may have hundreds of branches. The github_list_branches tool supports pagination to handle this efficiently.\n\n\
                 PAGINATION BASICS:\n\
                 GitHub API returns branches in pages:\n\
                 - Default: 30 branches per page\n\
                 - Maximum: 100 branches per page\n\
                 - Use 'page' parameter to get additional pages\n\n\
                 BASIC PAGINATION:\n\
                 1. Get first page (default):\n\
                    github_list_branches({\"owner\": \"kubernetes\", \"repo\": \"kubernetes\"})\n\
                    // Returns first 30 branches\n\n\
                 2. Get first page with more results:\n\
                    github_list_branches({\"owner\": \"kubernetes\", \"repo\": \"kubernetes\", \"per_page\": 100})\n\
                    // Returns first 100 branches\n\n\
                 3. Get second page:\n\
                    github_list_branches({\"owner\": \"kubernetes\", \"repo\": \"kubernetes\", \"per_page\": 100, \"page\": 2})\n\
                    // Returns branches 101-200\n\n\
                 4. Get third page:\n\
                    github_list_branches({\"owner\": \"kubernetes\", \"repo\": \"kubernetes\", \"per_page\": 100, \"page\": 3})\n\
                    // Returns branches 201-300\n\n\
                 PAGINATION PARAMETERS:\n\
                 - per_page: Items per page (1-100, default: 30)\n\
                 - page: Page number (1-based, default: 1)\n\n\
                 WHEN TO USE PAGINATION:\n\
                 1. Repository has many branches (50+)\n\
                 2. You need complete branch list\n\
                 3. Searching for specific branch\n\
                 4. Generating comprehensive reports\n\
                 5. Branch cleanup operations\n\n\
                 EFFICIENT PAGINATION WORKFLOW:\n\
                 \n\
                 Step 1: Start with maximum page size\n\
                 github_list_branches({\"owner\": \"org\", \"repo\": \"project\", \"per_page\": 100, \"page\": 1})\n\n\
                 Step 2: Check if more pages exist\n\
                 // If returned 100 results, likely more pages\n\
                 // If returned < 100, this is the last page\n\n\
                 Step 3: Fetch additional pages as needed\n\
                 github_list_branches({\"owner\": \"org\", \"repo\": \"project\", \"per_page\": 100, \"page\": 2})\n\
                 github_list_branches({\"owner\": \"org\", \"repo\": \"project\", \"per_page\": 100, \"page\": 3})\n\n\
                 Step 4: Continue until page returns < 100 results\n\
                 // This indicates last page reached\n\n\
                 DETERMINING IF MORE PAGES EXIST:\n\
                 Method 1: Count returned results\n\
                 - If count < per_page: Last page reached\n\
                 - If count = per_page: More pages may exist\n\n\
                 Method 2: Check response headers (if exposed)\n\
                 - Link header contains pagination info\n\
                 - Shows next, last, first, prev pages\n\n\
                 PAGINATION EXAMPLES:\n\
                 \n\
                 Example 1: Get all branches efficiently\n\
                 // Page 1\n\
                 github_list_branches({\"owner\": \"torvalds\", \"repo\": \"linux\", \"per_page\": 100, \"page\": 1})\n\
                 // Returns 100 branches, continue...\n\
                 \n\
                 // Page 2\n\
                 github_list_branches({\"owner\": \"torvalds\", \"repo\": \"linux\", \"per_page\": 100, \"page\": 2})\n\
                 // Returns 100 branches, continue...\n\
                 \n\
                 // Page 3\n\
                 github_list_branches({\"owner\": \"torvalds\", \"repo\": \"linux\", \"per_page\": 100, \"page\": 3})\n\
                 // Returns 47 branches, STOP (last page)\n\n\
                 Example 2: Search across pages for specific branch\n\
                 // Page 1\n\
                 github_list_branches({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"per_page\": 100, \"page\": 1})\n\
                 // Check if \"feature/async-await\" in results\n\
                 // If not found, continue...\n\
                 \n\
                 // Page 2\n\
                 github_list_branches({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"per_page\": 100, \"page\": 2})\n\
                 // Check again\n\
                 // If found, STOP\n\
                 // If not found and < 100 results, branch doesn't exist\n\n\
                 Example 3: Small repo (no pagination needed)\n\
                 github_list_branches({\"owner\": \"smallteam\", \"repo\": \"tool\"})\n\
                 // Returns 12 branches\n\
                 // Only one page needed, done!\n\n\
                 PAGINATION STRATEGIES:\n\
                 \n\
                 Strategy 1: FETCH ALL (comprehensive)\n\
                 Use when:\n\
                 - Need complete branch inventory\n\
                 - Generating reports\n\
                 - Branch cleanup\n\
                 - Analyzing repository structure\n\
                 \n\
                 Approach:\n\
                 - Start page 1, per_page 100\n\
                 - Fetch all pages sequentially\n\
                 - Combine results\n\
                 - Process complete list\n\n\
                 Strategy 2: SEARCH AND STOP (efficient)\n\
                 Use when:\n\
                 - Looking for specific branch\n\
                 - Checking branch existence\n\
                 - Verifying branch name\n\
                 \n\
                 Approach:\n\
                 - Start page 1, per_page 100\n\
                 - Check if target found\n\
                 - If found, stop\n\
                 - If not found and more pages, continue\n\
                 - If not found and last page, doesn't exist\n\n\
                 Strategy 3: FIRST PAGE ONLY (quick check)\n\
                 Use when:\n\
                 - Quick branch overview\n\
                 - Finding common branches (main, develop)\n\
                 - Checking most recent branches\n\
                 \n\
                 Approach:\n\
                 - Fetch page 1 only\n\
                 - Usually contains main branches\n\
                 - Fast and minimal API calls\n\n\
                 PAGINATION PERFORMANCE:\n\
                 \n\
                 API Call Cost:\n\
                 - Each page = 1 API call\n\
                 - 250 branches = 3 API calls (per_page: 100)\n\
                 - 250 branches = 9 API calls (per_page: 30)\n\
                 \n\
                 Optimization:\n\
                 - Always use per_page: 100 for efficiency\n\
                 - Minimizes total API calls\n\
                 - Faster for large repositories\n\
                 - Reduces rate limit usage\n\n\
                 RATE LIMIT CONSIDERATIONS:\n\
                 \n\
                 Rate Limits:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Unauthenticated: 60 requests/hour\n\
                 \n\
                 Pagination Impact:\n\
                 - 100 pages = 100 API calls\n\
                 - Can quickly consume rate limit\n\
                 - Use per_page: 100 to minimize\n\
                 - Cache results when possible\n\
                 - Implement rate limit handling\n\n\
                 PAGINATION EDGE CASES:\n\
                 \n\
                 1. Empty repository:\n\
                    github_list_branches({\"owner\": \"user\", \"repo\": \"new\"})\n\
                    // Returns empty array\n\
                    // No pagination needed\n\n\
                 2. Exactly 100 branches:\n\
                    github_list_branches({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 100})\n\
                    // Returns 100 branches\n\
                    // Check page 2 to confirm no more\n\
                    github_list_branches({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 100, \"page\": 2})\n\
                    // Returns 0 branches (empty)\n\
                    // Confirms exactly 100 branches\n\n\
                 3. Very large repository (1000+ branches):\n\
                    // Will need 10+ API calls\n\
                    // Consider if full list needed\n\
                    // Maybe filter or search instead\n\n\
                 BEST PRACTICES:\n\
                 \n\
                 1. Always use per_page: 100:\n\
                    - Maximum efficiency\n\
                    - Minimum API calls\n\
                    - Fastest retrieval\n\n\
                 2. Implement early exit:\n\
                    - Stop when target found\n\
                    - Don't fetch unnecessary pages\n\
                    - Save API calls\n\n\
                 3. Detect last page:\n\
                    - Check returned count < per_page\n\
                    - Stop pagination loop\n\
                    - Avoid empty page requests\n\n\
                 4. Cache when appropriate:\n\
                    - Branch lists change slowly\n\
                    - Cache for minutes/hours\n\
                    - Reduce repeated API calls\n\n\
                 5. Handle rate limits:\n\
                    - Check rate limit headers\n\
                    - Implement backoff if needed\n\
                    - Use authenticated requests\n\n\
                 6. Batch processing:\n\
                    - Process each page as received\n\
                    - Don't wait for all pages\n\
                    - Stream results efficiently\n\n\
                 PAGINATION ANTI-PATTERNS:\n\
                 \n\
                 DON'T:\n\
                 - Use per_page < 100 (inefficient)\n\
                 - Fetch all pages when only first needed\n\
                 - Ignore last page detection\n\
                 - Make unnecessary page requests\n\
                 - Forget to handle rate limits\n\n\
                 DO:\n\
                 - Use per_page: 100 always\n\
                 - Stop early when possible\n\
                 - Detect last page properly\n\
                 - Cache results appropriately\n\
                 - Monitor rate limit usage",
            ),
        },
    ]
}
