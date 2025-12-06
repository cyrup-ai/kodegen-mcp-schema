//! Prompt messages for github_list_repos tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GithubListReposPromptArgs;

/// Prompt provider for list_repos tool
///
/// This is the ONLY way to provide prompts for list_repos - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GithubListReposPrompts;

impl PromptProvider for GithubListReposPrompts {
    type PromptArgs = GithubListReposPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("user") => prompt_user(),
            Some("organization") => prompt_organization(),
            Some("sorting") => prompt_sorting(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (user, organization, sorting, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST REPOSITORIES
// ============================================================================

/// User repositories listing
fn prompt_user() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list user repositories with github_list_repos?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_list_repos tool lists repositories for a specific user or the authenticated user. It supports filtering and pagination.\n\n\
                 USER REPOSITORIES:\n\n\
                 1. List authenticated user's repos:\n\
                    github_list_repos({})\n\
                    // Lists all repos for the authenticated user (requires GITHUB_TOKEN)\n\n\
                 2. List another user's public repos:\n\
                    github_list_repos({\"username\": \"octocat\"})\n\
                    // Lists public repositories for the specified user\n\n\
                 3. List with type filter:\n\
                    github_list_repos({\"type\": \"owner\"})\n\
                    // Only repos owned by authenticated user (not forks, not member repos)\n\n\
                 4. List private repos:\n\
                    github_list_repos({\"type\": \"private\"})\n\
                    // Only private repositories (requires authentication)\n\n\
                 TYPE OPTIONS:\n\
                 - \"all\": All repositories (default)\n\
                 - \"owner\": Repositories owned by user\n\
                 - \"public\": Public repositories only\n\
                 - \"private\": Private repositories only (authenticated user)\n\
                 - \"member\": Repositories where user is a member\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"repos\": [\n\
                     {\n\
                       \"name\": \"project-name\",\n\
                       \"full_name\": \"username/project-name\",\n\
                       \"private\": false,\n\
                       \"owner\": {\n\
                         \"login\": \"username\",\n\
                         \"type\": \"User\"\n\
                       },\n\
                       \"html_url\": \"https://github.com/username/project-name\",\n\
                       \"description\": \"Project description\",\n\
                       \"fork\": false,\n\
                       \"created_at\": \"2024-01-15T10:30:00Z\",\n\
                       \"updated_at\": \"2024-11-20T14:45:00Z\",\n\
                       \"pushed_at\": \"2024-11-20T14:45:00Z\",\n\
                       \"size\": 1024,\n\
                       \"stargazers_count\": 150,\n\
                       \"watchers_count\": 150,\n\
                       \"language\": \"Rust\",\n\
                       \"forks_count\": 25,\n\
                       \"archived\": false,\n\
                       \"disabled\": false,\n\
                       \"open_issues_count\": 5,\n\
                       \"license\": {\n\
                         \"key\": \"mit\",\n\
                         \"name\": \"MIT License\"\n\
                       },\n\
                       \"topics\": [\"rust\", \"cli\", \"tool\"],\n\
                       \"visibility\": \"public\",\n\
                       \"default_branch\": \"main\"\n\
                     }\n\
                   ],\n\
                   \"total_count\": 42\n\
                 }\n\n\
                 PAGINATION:\n\
                 1. First page (default 30 results):\n\
                    github_list_repos({\"username\": \"octocat\"})\n\n\
                 2. Custom page size:\n\
                    github_list_repos({\"username\": \"octocat\", \"per_page\": 100})\n\
                    // Maximum: 100 repos per page\n\n\
                 3. Next page:\n\
                    github_list_repos({\"username\": \"octocat\", \"page\": 2, \"per_page\": 100})\n\n\
                 AUTHENTICATION:\n\
                 - No token: Only public repos visible\n\
                 - With token: Can see private repos and member repos\n\
                 - Set GITHUB_TOKEN environment variable\n\n\
                 COMMON USER SCENARIOS:\n\
                 1. Audit your repos:\n\
                    github_list_repos({\"type\": \"owner\"})\n\
                    // See all repos you own\n\n\
                 2. Check someone's public work:\n\
                    github_list_repos({\"username\": \"torvalds\"})\n\
                    // Browse another user's repos\n\n\
                 3. List your private repos:\n\
                    github_list_repos({\"type\": \"private\"})\n\
                    // Private repos only\n\n\
                 4. List repos where you're a collaborator:\n\
                    github_list_repos({\"type\": \"member\"})\n\
                    // Repos where you have access but don't own\n\n\
                 BEST PRACTICES:\n\
                 - Use type filter to narrow results\n\
                 - Start with per_page: 100 for efficiency\n\
                 - Check total_count to know if pagination needed\n\
                 - Combine with sorting for better discovery\n\
                 - Use username parameter for public exploration",
            ),
        },
    ]
}

/// Organization repositories listing
fn prompt_organization() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list organization repositories?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "List organization repositories using the org parameter. Organizations have different type filters than users.\n\n\
                 ORGANIZATION REPOSITORIES:\n\n\
                 1. List all org repos:\n\
                    github_list_repos({\"org\": \"microsoft\"})\n\
                    // Lists public repositories for the organization\n\n\
                 2. List by type:\n\
                    github_list_repos({\"org\": \"google\", \"type\": \"public\"})\n\
                    // Filter by repository visibility\n\n\
                 3. List private org repos:\n\
                    github_list_repos({\"org\": \"my-company\", \"type\": \"private\"})\n\
                    // Requires org member authentication\n\n\
                 4. List forks only:\n\
                    github_list_repos({\"org\": \"kubernetes\", \"type\": \"forks\"})\n\
                    // Only forked repositories\n\n\
                 5. List source repos (non-forks):\n\
                    github_list_repos({\"org\": \"rust-lang\", \"type\": \"sources\"})\n\
                    // Only original repositories\n\n\
                 ORG TYPE OPTIONS:\n\
                 - \"all\": All accessible repositories (default)\n\
                 - \"public\": Public repositories only\n\
                 - \"private\": Private repositories (requires org membership)\n\
                 - \"forks\": Forked repositories only\n\
                 - \"sources\": Source/original repositories (not forks)\n\
                 - \"member\": Repositories accessible to authenticated user\n\n\
                 DIFFERENCE: USER vs ORG TYPES:\n\
                 User types: all, owner, public, private, member\n\
                 Org types: all, public, private, forks, sources, member\n\n\
                 Key difference:\n\
                 - Users have \"owner\" (repos they own)\n\
                 - Orgs have \"forks\" and \"sources\" (fork status)\n\n\
                 AUTHENTICATION & PERMISSIONS:\n\
                 - Public orgs: No token needed for public repos\n\
                 - Private orgs: Requires GITHUB_TOKEN with org access\n\
                 - Member repos: Token must belong to org member\n\
                 - Private repos: Token needs appropriate scope\n\n\
                 RESPONSE STRUCTURE:\n\
                 Same as user repos, but owner.type will be \"Organization\":\n\
                 {\n\
                   \"repos\": [\n\
                     {\n\
                       \"name\": \"repo-name\",\n\
                       \"full_name\": \"org-name/repo-name\",\n\
                       \"owner\": {\n\
                         \"login\": \"org-name\",\n\
                         \"type\": \"Organization\"\n\
                       },\n\
                       \"organization\": {\n\
                         \"login\": \"org-name\",\n\
                         \"id\": 12345\n\
                       },\n\
                       \"private\": false,\n\
                       \"fork\": false\n\
                       // ... other fields\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 COMMON ORG SCENARIOS:\n\
                 1. Audit org repositories:\n\
                    github_list_repos({\"org\": \"my-company\", \"type\": \"all\"})\n\
                    // See all accessible repos in organization\n\n\
                 2. Find org's original work:\n\
                    github_list_repos({\"org\": \"facebook\", \"type\": \"sources\"})\n\
                    // Exclude forks, see original projects\n\n\
                 3. Review org forks:\n\
                    github_list_repos({\"org\": \"my-company\", \"type\": \"forks\"})\n\
                    // Check which external projects org has forked\n\n\
                 4. List public org repos:\n\
                    github_list_repos({\"org\": \"apache\", \"type\": \"public\"})\n\
                    // Browse public projects\n\n\
                 5. Access private org repos:\n\
                    github_list_repos({\"org\": \"my-company\", \"type\": \"private\"})\n\
                    // Internal/private projects (requires membership)\n\n\
                 PAGINATION WITH ORGS:\n\
                 Large organizations may have hundreds of repos:\n\
                 1. Start with max page size:\n\
                    github_list_repos({\"org\": \"microsoft\", \"per_page\": 100})\n\n\
                 2. Get next pages:\n\
                    github_list_repos({\"org\": \"microsoft\", \"per_page\": 100, \"page\": 2})\n\n\
                 3. Check total_count in response to know total pages\n\n\
                 BEST PRACTICES:\n\
                 - Use \"sources\" to exclude forks for cleaner results\n\
                 - Use \"public\" when you don't need private repos\n\
                 - Combine with sorting for better organization\n\
                 - Request 100 per_page for large orgs\n\
                 - Filter by type to reduce API calls\n\
                 - Check fork field to identify forked repos\n\
                 - Use archived field to exclude old projects",
            ),
        },
    ]
}

/// Sorting options and usage
fn prompt_sorting() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I sort repository listings?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Sort repositories using the sort and direction parameters. Different sort options help discover repos by activity, popularity, or name.\n\n\
                 SORTING REPOSITORIES:\n\n\
                 1. By stars (most popular first):\n\
                    github_list_repos({\"sort\": \"stars\", \"direction\": \"desc\"})\n\
                    // Most starred repos first\n\n\
                 2. By recently updated:\n\
                    github_list_repos({\"sort\": \"updated\", \"direction\": \"desc\"})\n\
                    // Most recently updated repos first\n\n\
                 3. By creation date:\n\
                    github_list_repos({\"sort\": \"created\", \"direction\": \"desc\"})\n\
                    // Newest repos first\n\n\
                 4. By last push:\n\
                    github_list_repos({\"sort\": \"pushed\", \"direction\": \"desc\"})\n\
                    // Most recently pushed (active) repos first\n\n\
                 5. By name (alphabetical):\n\
                    github_list_repos({\"sort\": \"full_name\", \"direction\": \"asc\"})\n\
                    // Alphabetically sorted: A-Z\n\n\
                 SORT OPTIONS:\n\
                 - \"created\": Sort by creation date\n\
                 - \"updated\": Sort by last update time\n\
                 - \"pushed\": Sort by last push/commit time\n\
                 - \"full_name\": Sort alphabetically by full name (owner/repo)\n\
                 - \"stars\": Sort by star count (popularity)\n\n\
                 DIRECTION OPTIONS:\n\
                 - \"asc\": Ascending (oldest first, A-Z, least stars)\n\
                 - \"desc\": Descending (newest first, Z-A, most stars)\n\
                 - Default: \"desc\" for most sort options\n\n\
                 UNDERSTANDING DATES:\n\
                 - created: When repo was initially created\n\
                 - updated: Last metadata update (description, settings, etc.)\n\
                 - pushed: Last commit pushed (best indicator of activity)\n\n\
                 For active repos, use \"pushed\" not \"updated\"\n\n\
                 SORT BY USE CASE:\n\n\
                 Find most popular repos:\n\
                 github_list_repos({\"username\": \"octocat\", \"sort\": \"stars\", \"direction\": \"desc\"})\n\
                 // See what repos got most attention\n\n\
                 Find active repos:\n\
                 github_list_repos({\"org\": \"rust-lang\", \"sort\": \"pushed\", \"direction\": \"desc\"})\n\
                 // See what's being actively developed\n\n\
                 Find newest repos:\n\
                 github_list_repos({\"sort\": \"created\", \"direction\": \"desc\"})\n\
                 // See latest projects\n\n\
                 Find oldest repos:\n\
                 github_list_repos({\"username\": \"torvalds\", \"sort\": \"created\", \"direction\": \"asc\"})\n\
                 // Historical projects\n\n\
                 Browse alphabetically:\n\
                 github_list_repos({\"org\": \"apache\", \"sort\": \"full_name\", \"direction\": \"asc\"})\n\
                 // Organized A-Z listing\n\n\
                 COMBINING SORT WITH FILTERS:\n\
                 1. Most popular original repos:\n\
                    github_list_repos({\n\
                      \"org\": \"facebook\",\n\
                      \"type\": \"sources\",\n\
                      \"sort\": \"stars\",\n\
                      \"direction\": \"desc\"\n\
                    })\n\n\
                 2. Recently active public repos:\n\
                    github_list_repos({\n\
                      \"username\": \"octocat\",\n\
                      \"type\": \"public\",\n\
                      \"sort\": \"pushed\",\n\
                      \"direction\": \"desc\"\n\
                    })\n\n\
                 3. Alphabetical private repos:\n\
                    github_list_repos({\n\
                      \"type\": \"private\",\n\
                      \"sort\": \"full_name\",\n\
                      \"direction\": \"asc\"\n\
                    })\n\n\
                 DEFAULT SORTING:\n\
                 If sort is not specified:\n\
                 - User repos: sorted by pushed (most recent activity)\n\
                 - Org repos: sorted by created (newest first)\n\
                 - Direction: desc (descending)\n\n\
                 PERFORMANCE NOTES:\n\
                 - Sorting is done server-side by GitHub\n\
                 - No additional API calls needed\n\
                 - All sort options are equally fast\n\
                 - Combine with pagination for large result sets\n\n\
                 BEST PRACTICES:\n\
                 - Use \"pushed\" to find active repos\n\
                 - Use \"stars\" to find popular repos\n\
                 - Use \"full_name\" for organized browsing\n\
                 - Use \"created\" to see project timeline\n\
                 - Combine sorting with type filters\n\
                 - Consider desc direction for most cases\n\
                 - Use asc for alphabetical or historical views",
            ),
        },
    ]
}

/// Common repository discovery workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows for discovering repositories?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Here are practical workflows for discovering and analyzing repositories using github_list_repos.\n\n\
                 WORKFLOW 1: FIND ACTIVE REPOSITORIES\n\
                 Goal: Identify repos with recent development activity\n\n\
                 1. List by recent activity:\n\
                    github_list_repos({\n\
                      \"username\": \"octocat\",\n\
                      \"sort\": \"pushed\",\n\
                      \"direction\": \"desc\",\n\
                      \"per_page\": 100\n\
                    })\n\
                    // Most recently pushed repos appear first\n\n\
                 2. Check response:\n\
                    - Look at pushed_at dates\n\
                    - Check open_issues_count for activity\n\
                    - Verify archived: false\n\n\
                 3. Filter results:\n\
                    - Skip archived repos\n\
                    - Focus on repos pushed in last 30 days\n\
                    - Check language matches your interest\n\n\
                 WORKFLOW 2: DISCOVER POPULAR REPOSITORIES\n\
                 Goal: Find most starred/successful projects\n\n\
                 1. Sort by stars:\n\
                    github_list_repos({\n\
                      \"username\": \"torvalds\",\n\
                      \"sort\": \"stars\",\n\
                      \"direction\": \"desc\"\n\
                    })\n\n\
                 2. For organizations:\n\
                    github_list_repos({\n\
                      \"org\": \"microsoft\",\n\
                      \"type\": \"sources\",\n\
                      \"sort\": \"stars\",\n\
                      \"direction\": \"desc\",\n\
                      \"per_page\": 50\n\
                    })\n\
                    // Exclude forks, see original popular projects\n\n\
                 3. Analyze results:\n\
                    - Compare stargazers_count\n\
                    - Check forks_count for engagement\n\
                    - Review topics for categorization\n\
                    - Note language distribution\n\n\
                 WORKFLOW 3: AUDIT ORGANIZATION REPOS\n\
                 Goal: Review all repos in an organization\n\n\
                 1. List all repos:\n\
                    github_list_repos({\n\
                      \"org\": \"my-company\",\n\
                      \"type\": \"all\",\n\
                      \"per_page\": 100\n\
                    })\n\n\
                 2. Identify issues:\n\
                    - Check for archived repos\n\
                    - Find repos without descriptions\n\
                    - Identify repos without licenses\n\
                    - Find stale repos (old pushed_at)\n\n\
                 3. Generate report:\n\
                    - Count public vs private\n\
                    - List repos by language\n\
                    - Find repos with many open issues\n\
                    - Identify unmaintained repos\n\n\
                 WORKFLOW 4: ANALYZE USER'S TECHNOLOGY STACK\n\
                 Goal: Understand what technologies a user works with\n\n\
                 1. Get all public repos:\n\
                    github_list_repos({\n\
                      \"username\": \"developer\",\n\
                      \"type\": \"owner\",\n\
                      \"per_page\": 100\n\
                    })\n\n\
                 2. Analyze response:\n\
                    - Group by language field\n\
                    - Check topics for frameworks\n\
                    - Note repo creation timeline\n\
                    - Identify technology trends\n\n\
                 3. Insights:\n\
                    - Primary languages used\n\
                    - Framework preferences\n\
                    - Technology evolution over time\n\
                    - Areas of expertise\n\n\
                 WORKFLOW 5: FIND COLLABORATION OPPORTUNITIES\n\
                 Goal: Discover repos where you can contribute\n\n\
                 1. Find active repos:\n\
                    github_list_repos({\n\
                      \"org\": \"rust-lang\",\n\
                      \"sort\": \"updated\",\n\
                      \"direction\": \"desc\"\n\
                    })\n\n\
                 2. Filter criteria:\n\
                    - open_issues_count > 0 (has issues to solve)\n\
                    - Not archived\n\
                    - Recent pushed_at (active maintenance)\n\
                    - Language you know\n\n\
                 3. Next steps:\n\
                    - Use github_list_issues to see specific issues\n\
                    - Check contribution guidelines\n\
                    - Review issue labels for \"good first issue\"\n\n\
                 WORKFLOW 6: MONITOR YOUR REPOSITORIES\n\
                 Goal: Keep track of your own repos\n\n\
                 1. List your repos:\n\
                    github_list_repos({\n\
                      \"type\": \"owner\",\n\
                      \"sort\": \"pushed\",\n\
                      \"direction\": \"desc\"\n\
                    })\n\n\
                 2. Check for:\n\
                    - Repos not pushed recently (may need attention)\n\
                    - Growing star counts (gaining traction)\n\
                    - Open issues accumulating\n\
                    - Fork activity\n\n\
                 3. Maintenance actions:\n\
                    - Archive inactive repos\n\
                    - Update descriptions\n\
                    - Address open issues\n\
                    - Update documentation\n\n\
                 WORKFLOW 7: RESEARCH COMPETITORS\n\
                 Goal: Analyze similar projects\n\n\
                 1. List competitor's repos:\n\
                    github_list_repos({\n\
                      \"username\": \"competitor\",\n\
                      \"type\": \"owner\",\n\
                      \"sort\": \"stars\",\n\
                      \"direction\": \"desc\"\n\
                    })\n\n\
                 2. Compare:\n\
                    - Star counts vs your repos\n\
                    - Languages and technologies used\n\
                    - Update frequency\n\
                    - Project descriptions and topics\n\n\
                 3. Learn:\n\
                    - Popular features (from descriptions)\n\
                    - Technology choices\n\
                    - Project organization\n\
                    - Marketing approach (topics, descriptions)\n\n\
                 COMBINING WITH OTHER TOOLS:\n\
                 After listing repos, use these tools for deeper analysis:\n\
                 - github_get_file_contents: Read specific files\n\
                 - github_list_issues: Check issue activity\n\
                 - github_list_pull_requests: Review PR activity\n\
                 - github_list_commits: Analyze commit history\n\
                 - github_list_branches: Check branch structure\n\n\
                 PAGINATION STRATEGY:\n\
                 For large result sets:\n\
                 1. First request: per_page: 100, page: 1\n\
                 2. Check total_count in response\n\
                 3. Calculate total pages: ceil(total_count / 100)\n\
                 4. Fetch remaining pages if needed\n\
                 5. Process results incrementally\n\n\
                 BEST PRACTICES:\n\
                 - Start with appropriate filters (type, org/username)\n\
                 - Use sorting to prioritize results\n\
                 - Request 100 per_page for efficiency\n\
                 - Check total_count before pagination\n\
                 - Look for patterns in topics and languages\n\
                 - Filter archived repos in analysis\n\
                 - Use pushed_at for activity assessment\n\
                 - Combine multiple API calls for complete picture",
            ),
        },
    ]
}

/// Comprehensive guide covering all features
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using github_list_repos effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_list_repos tool provides comprehensive repository listing for users and organizations with filtering, sorting, and pagination.\n\n\
                 =============================================================================\n\
                 OVERVIEW\n\
                 =============================================================================\n\n\
                 PURPOSE:\n\
                 List repositories for a specific user, organization, or authenticated user.\n\
                 Supports filtering by visibility, type, and sorting by various criteria.\n\n\
                 AUTHENTICATION:\n\
                 - Optional: Works without token for public repos\n\
                 - Required: GITHUB_TOKEN for private repos and higher rate limits\n\
                 - Set via environment variable: GITHUB_TOKEN=ghp_xxxx\n\n\
                 TWO MODES:\n\
                 1. User repos: Use username parameter or omit for authenticated user\n\
                 2. Organization repos: Use org parameter\n\n\
                 =============================================================================\n\
                 PARAMETERS\n\
                 =============================================================================\n\n\
                 TARGET PARAMETERS (choose one):\n\
                 - username (optional): GitHub username to list repos for\n\
                 - org (optional): Organization name to list repos for\n\
                 - Omit both: Lists authenticated user's repos\n\n\
                 FILTER PARAMETERS:\n\
                 - type (optional): Repository type filter\n\
                   User types: \"all\", \"owner\", \"public\", \"private\", \"member\"\n\
                   Org types: \"all\", \"public\", \"private\", \"forks\", \"sources\", \"member\"\n\
                   Default: \"all\"\n\n\
                 SORT PARAMETERS:\n\
                 - sort (optional): Sort field\n\
                   Options: \"created\", \"updated\", \"pushed\", \"full_name\", \"stars\"\n\
                   Default: \"pushed\" for users, \"created\" for orgs\n\
                 - direction (optional): Sort direction\n\
                   Options: \"asc\", \"desc\"\n\
                   Default: \"desc\"\n\n\
                 PAGINATION PARAMETERS:\n\
                 - page (optional): Page number (1-based)\n\
                   Default: 1\n\
                 - per_page (optional): Results per page\n\
                   Range: 1-100\n\
                   Default: 30\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 1. List your repos:\n\
                    github_list_repos({})\n\
                    // Authenticated user's repositories\n\n\
                 2. List another user's repos:\n\
                    github_list_repos({\"username\": \"octocat\"})\n\
                    // Public repos for specified user\n\n\
                 3. List org repos:\n\
                    github_list_repos({\"org\": \"microsoft\"})\n\
                    // Organization's public repositories\n\n\
                 4. List with pagination:\n\
                    github_list_repos({\"username\": \"torvalds\", \"per_page\": 100})\n\
                    // Get 100 repos per page\n\n\
                 =============================================================================\n\
                 TYPE FILTERING\n\
                 =============================================================================\n\n\
                 USER REPOSITORY TYPES:\n\
                 - \"all\": All accessible repositories (default)\n\
                 - \"owner\": Repositories owned by user (not member repos)\n\
                 - \"public\": Public repositories only\n\
                 - \"private\": Private repositories (requires authentication)\n\
                 - \"member\": Repositories where user is a member but not owner\n\n\
                 Examples:\n\
                 github_list_repos({\"type\": \"owner\"})\n\
                 github_list_repos({\"username\": \"octocat\", \"type\": \"public\"})\n\
                 github_list_repos({\"type\": \"private\"})\n\n\
                 ORGANIZATION REPOSITORY TYPES:\n\
                 - \"all\": All accessible repositories (default)\n\
                 - \"public\": Public repositories only\n\
                 - \"private\": Private repositories (requires org membership)\n\
                 - \"forks\": Forked repositories only\n\
                 - \"sources\": Original repositories (not forks)\n\
                 - \"member\": Repositories accessible to authenticated member\n\n\
                 Examples:\n\
                 github_list_repos({\"org\": \"apache\", \"type\": \"public\"})\n\
                 github_list_repos({\"org\": \"my-company\", \"type\": \"sources\"})\n\
                 github_list_repos({\"org\": \"rust-lang\", \"type\": \"forks\"})\n\n\
                 =============================================================================\n\
                 SORTING\n\
                 =============================================================================\n\n\
                 SORT OPTIONS:\n\
                 - \"created\": Sort by creation date\n\
                 - \"updated\": Sort by last update time\n\
                 - \"pushed\": Sort by last push/commit time (activity indicator)\n\
                 - \"full_name\": Sort alphabetically by owner/name\n\
                 - \"stars\": Sort by star count (popularity)\n\n\
                 DIRECTION:\n\
                 - \"asc\": Ascending (oldest, A-Z, least stars)\n\
                 - \"desc\": Descending (newest, Z-A, most stars)\n\n\
                 Examples:\n\
                 1. Most popular:\n\
                    github_list_repos({\"username\": \"octocat\", \"sort\": \"stars\", \"direction\": \"desc\"})\n\n\
                 2. Recently active:\n\
                    github_list_repos({\"org\": \"rust-lang\", \"sort\": \"pushed\", \"direction\": \"desc\"})\n\n\
                 3. Alphabetical:\n\
                    github_list_repos({\"org\": \"apache\", \"sort\": \"full_name\", \"direction\": \"asc\"})\n\n\
                 4. Newest first:\n\
                    github_list_repos({\"sort\": \"created\", \"direction\": \"desc\"})\n\n\
                 =============================================================================\n\
                 RESPONSE STRUCTURE\n\
                 =============================================================================\n\n\
                 {\n\
                   \"repos\": [\n\
                     {\n\
                       \"id\": 123456789,\n\
                       \"name\": \"repository-name\",\n\
                       \"full_name\": \"owner/repository-name\",\n\
                       \"owner\": {\n\
                         \"login\": \"owner-name\",\n\
                         \"id\": 987654,\n\
                         \"type\": \"User\" or \"Organization\",\n\
                         \"avatar_url\": \"https://...\"\n\
                       },\n\
                       \"private\": false,\n\
                       \"html_url\": \"https://github.com/owner/repo\",\n\
                       \"description\": \"Repository description\",\n\
                       \"fork\": false,\n\
                       \"created_at\": \"2024-01-15T10:30:00Z\",\n\
                       \"updated_at\": \"2024-11-20T14:45:00Z\",\n\
                       \"pushed_at\": \"2024-11-20T14:45:00Z\",\n\
                       \"size\": 1024,\n\
                       \"stargazers_count\": 150,\n\
                       \"watchers_count\": 150,\n\
                       \"language\": \"Rust\",\n\
                       \"has_issues\": true,\n\
                       \"has_projects\": true,\n\
                       \"has_downloads\": true,\n\
                       \"has_wiki\": true,\n\
                       \"has_pages\": false,\n\
                       \"has_discussions\": true,\n\
                       \"forks_count\": 25,\n\
                       \"archived\": false,\n\
                       \"disabled\": false,\n\
                       \"open_issues_count\": 5,\n\
                       \"license\": {\n\
                         \"key\": \"mit\",\n\
                         \"name\": \"MIT License\",\n\
                         \"spdx_id\": \"MIT\"\n\
                       },\n\
                       \"topics\": [\"rust\", \"cli\", \"tool\"],\n\
                       \"visibility\": \"public\",\n\
                       \"default_branch\": \"main\",\n\
                       \"permissions\": {\n\
                         \"admin\": true,\n\
                         \"push\": true,\n\
                         \"pull\": true\n\
                       }\n\
                     }\n\
                   ],\n\
                   \"total_count\": 42\n\
                 }\n\n\
                 KEY FIELDS:\n\
                 - name: Repository name only\n\
                 - full_name: owner/name format\n\
                 - private: Visibility status\n\
                 - fork: Whether it's a fork\n\
                 - pushed_at: Last commit time (activity)\n\
                 - stargazers_count: Stars (popularity)\n\
                 - language: Primary programming language\n\
                 - open_issues_count: Active issues\n\
                 - archived: Whether repo is archived\n\
                 - topics: Tags/categories\n\
                 - total_count: Total repos matching query\n\n\
                 =============================================================================\n\
                 PAGINATION\n\
                 =============================================================================\n\n\
                 STRATEGY:\n\
                 1. Start with high per_page:\n\
                    github_list_repos({\"org\": \"microsoft\", \"per_page\": 100})\n\n\
                 2. Check total_count in response\n\
                 3. Calculate pages needed: ceil(total_count / per_page)\n\
                 4. Fetch additional pages:\n\
                    github_list_repos({\"org\": \"microsoft\", \"per_page\": 100, \"page\": 2})\n\
                    github_list_repos({\"org\": \"microsoft\", \"per_page\": 100, \"page\": 3})\n\n\
                 LIMITS:\n\
                 - Maximum per_page: 100\n\
                 - GitHub only returns first 1000 results total\n\
                 - If total_count > 1000, use filtering to narrow results\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 WORKFLOW 1: Audit your repositories\n\
                 github_list_repos({\"type\": \"owner\", \"per_page\": 100})\n\
                 Review all repos you own\n\n\
                 WORKFLOW 2: Find active projects\n\
                 github_list_repos({\"org\": \"rust-lang\", \"sort\": \"pushed\", \"direction\": \"desc\"})\n\
                 See recently updated repos\n\n\
                 WORKFLOW 3: Discover popular repos\n\
                 github_list_repos({\"username\": \"octocat\", \"sort\": \"stars\", \"direction\": \"desc\"})\n\
                 Find most starred repositories\n\n\
                 WORKFLOW 4: Review org original work\n\
                 github_list_repos({\"org\": \"facebook\", \"type\": \"sources\", \"sort\": \"stars\", \"direction\": \"desc\"})\n\
                 Exclude forks, see original projects\n\n\
                 WORKFLOW 5: Check private repos\n\
                 github_list_repos({\"type\": \"private\", \"sort\": \"updated\", \"direction\": \"desc\"})\n\
                 Review your private repositories\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 FILTERING:\n\
                 - Use type filter to narrow results\n\
                 - Use \"sources\" for orgs to exclude forks\n\
                 - Use \"owner\" for users to see only their repos\n\
                 - Filter by type before sorting for efficiency\n\n\
                 SORTING:\n\
                 - Use \"pushed\" to find active repos\n\
                 - Use \"stars\" to find popular repos\n\
                 - Use \"full_name\" for organized browsing\n\
                 - Use \"created\" for chronological view\n\n\
                 PAGINATION:\n\
                 - Start with per_page: 100 for large sets\n\
                 - Check total_count to know if pagination needed\n\
                 - Process results incrementally for memory efficiency\n\
                 - Use filtering to stay under 1000 result limit\n\n\
                 ANALYSIS:\n\
                 - Check pushed_at for activity\n\
                 - Check archived field to skip old projects\n\
                 - Use stargazers_count for popularity\n\
                 - Review topics for categorization\n\
                 - Check fork field to identify forked repos\n\
                 - Use language field for technology analysis\n\n\
                 AUTHENTICATION:\n\
                 - Always use token for better rate limits\n\
                 - Required for private repos and org member repos\n\
                 - Token gives access to permissions field\n\
                 - Without token: only public repos visible\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5000 requests/hour\n\
                 - Unauthenticated: 60 requests/hour\n\
                 - Check X-RateLimit-Remaining header\n\
                 - Use pagination wisely to conserve calls\n\n\
                 ERROR HANDLING:\n\
                 - 404: User/org not found\n\
                 - 403: Rate limit exceeded or no access\n\
                 - 401: Invalid or missing authentication\n\n\
                 COMBINING WITH OTHER TOOLS:\n\
                 After listing repos:\n\
                 - Use github_get_file_contents for specific files\n\
                 - Use github_list_issues to check issue activity\n\
                 - Use github_list_commits for commit history\n\
                 - Use github_list_branches for branch analysis\n\
                 - Use github_search_code for content search\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Your repos: github_list_repos({})\n\
                 User repos: github_list_repos({\"username\": \"user\"})\n\
                 Org repos: github_list_repos({\"org\": \"org-name\"})\n\
                 Most popular: github_list_repos({\"sort\": \"stars\", \"direction\": \"desc\"})\n\
                 Most active: github_list_repos({\"sort\": \"pushed\", \"direction\": \"desc\"})\n\
                 Alphabetical: github_list_repos({\"sort\": \"full_name\"})\n\
                 Private only: github_list_repos({\"type\": \"private\"})\n\
                 Org sources: github_list_repos({\"org\": \"name\", \"type\": \"sources\"})\n\
                 Max results: github_list_repos({\"per_page\": 100})\n\
                 Next page: github_list_repos({\"page\": 2, \"per_page\": 100})",
            ),
        },
    ]
}
