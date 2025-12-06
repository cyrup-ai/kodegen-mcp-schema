//! Prompt messages for github_search_repositories tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::SearchRepositoriesPromptArgs;

/// Prompt provider for search_repositories tool
///
/// This is the ONLY way to provide prompts for search_repositories - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct SearchRepositoriesPrompts;

impl PromptProvider for SearchRepositoriesPrompts {
    type PromptArgs = SearchRepositoriesPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("syntax") => prompt_syntax(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, syntax, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO SEARCH REPOSITORIES
// ============================================================================

/// Basic repository search
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I search for repositories on GitHub using the github_search_repositories tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_search_repositories tool searches for repositories across GitHub using powerful query syntax. Here's how to use it for basic searches:\n\n\
                 BASIC SEARCH:\n\
                 1. Simple keyword search:\n\
                    github_search_repositories({\"query\": \"rust http server\"})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"total_count\": 500,\n\
                   \"items\": [\n\
                     {\n\
                       \"full_name\": \"actix/actix-web\",\n\
                       \"description\": \"Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust.\",\n\
                       \"stargazers_count\": 15000,\n\
                       \"language\": \"Rust\",\n\
                       \"html_url\": \"https://github.com/actix/actix-web\",\n\
                       \"topics\": [\"web-framework\", \"async\", \"http\"]\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 FILTERING RESULTS:\n\
                 2. Search by language:\n\
                    github_search_repositories({\"query\": \"language:rust http server\"})\n\n\
                 3. Search popular repositories:\n\
                    github_search_repositories({\"query\": \"language:rust stars:>1000\"})\n\n\
                 4. Search recently active projects:\n\
                    github_search_repositories({\"query\": \"language:rust pushed:>2024-01-01\"})\n\n\
                 SORTING AND PAGINATION:\n\
                 5. Sort by stars (most popular first):\n\
                    github_search_repositories({\n\
                      \"query\": \"language:rust web framework\",\n\
                      \"sort\": \"stars\",\n\
                      \"order\": \"desc\"\n\
                    })\n\n\
                 6. Get more results per page:\n\
                    github_search_repositories({\n\
                      \"query\": \"topic:async\",\n\
                      \"per_page\": 50\n\
                    })\n\n\
                 7. Pagination for many results:\n\
                    github_search_repositories({\n\
                      \"query\": \"language:python\",\n\
                      \"page\": 2,\n\
                      \"per_page\": 30\n\
                    })\n\n\
                 PARAMETERS:\n\
                 - query (required): Search query using GitHub syntax\n\
                 - sort (optional): \"stars\", \"forks\", or \"updated\"\n\
                 - order (optional): \"asc\" or \"desc\" (default: \"desc\")\n\
                 - page (optional): Page number (default: 1)\n\
                 - per_page (optional): Results per page (max 100, default 30)\n\n\
                 RESPONSE FIELDS:\n\
                 Each repository in items contains:\n\
                 - full_name: \"owner/repo\"\n\
                 - description: Repository description\n\
                 - language: Primary programming language\n\
                 - stargazers_count: Number of stars\n\
                 - forks_count: Number of forks\n\
                 - topics: Array of topic tags\n\
                 - created_at, updated_at, pushed_at: Timestamps\n\
                 - html_url: GitHub repository URL\n\
                 - license: License information\n\
                 - archived: Whether repository is archived\n\n\
                 COMMON USE CASES:\n\
                 1. Find popular libraries in a language:\n\
                    github_search_repositories({\"query\": \"language:rust stars:>500\", \"sort\": \"stars\"})\n\
                 2. Discover trending projects:\n\
                    github_search_repositories({\"query\": \"created:>2024-01-01 stars:>100\", \"sort\": \"stars\"})\n\
                 3. Find active projects:\n\
                    github_search_repositories({\"query\": \"pushed:>2024-11-01\", \"sort\": \"updated\"})\n\
                 4. Search by topic:\n\
                    github_search_repositories({\"query\": \"topic:machine-learning language:python\"})\n\n\
                 TIPS:\n\
                 - Use specific queries to get relevant results\n\
                 - Combine multiple qualifiers for precision\n\
                 - Sort by stars to find popular projects\n\
                 - Check pushed_at date for active maintenance\n\
                 - Use per_page to get more results at once",
            ),
        },
    ]
}

/// Search syntax reference
fn prompt_syntax() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What search syntax and qualifiers can I use with github_search_repositories?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "GitHub repository search supports powerful qualifiers to filter and refine your search. Here's a comprehensive syntax reference:\n\n\
                 SEARCH QUALIFIERS:\n\n\
                 LANGUAGE FILTERS:\n\
                 - language:rust - Repositories written in Rust\n\
                 - language:python - Repositories written in Python\n\
                 - language:javascript - Repositories written in JavaScript\n\
                 Example:\n\
                 github_search_repositories({\"query\": \"web framework language:rust\"})\n\n\
                 POPULARITY FILTERS:\n\
                 - stars:>1000 - More than 1000 stars\n\
                 - stars:100..500 - Between 100 and 500 stars\n\
                 - stars:<=50 - 50 or fewer stars\n\
                 - forks:>100 - More than 100 forks\n\
                 - forks:10..50 - Between 10 and 50 forks\n\
                 Example:\n\
                 github_search_repositories({\"query\": \"language:rust stars:>1000 forks:>100\"})\n\n\
                 DATE FILTERS:\n\
                 - created:>2024-01-01 - Created after January 1, 2024\n\
                 - created:2023-01-01..2023-12-31 - Created during 2023\n\
                 - pushed:>2024-01-01 - Last commit after January 1, 2024\n\
                 - updated:>2024-01-01 - Updated (any change) after January 1, 2024\n\
                 Example:\n\
                 github_search_repositories({\"query\": \"language:rust pushed:>2024-01-01\"})\n\n\
                 OWNER FILTERS:\n\
                 - user:username - Repositories owned by specific user\n\
                 - org:organization - Repositories owned by specific organization\n\
                 Example:\n\
                 github_search_repositories({\"query\": \"user:torvalds language:c\"})\n\
                 github_search_repositories({\"query\": \"org:tokio-rs\"})\n\n\
                 TOPIC FILTERS:\n\
                 - topic:web - Repositories tagged with \"web\" topic\n\
                 - topic:machine-learning - Repositories tagged with \"machine-learning\"\n\
                 Example:\n\
                 github_search_repositories({\"query\": \"topic:async topic:runtime language:rust\"})\n\n\
                 LICENSE FILTERS:\n\
                 - license:mit - MIT licensed repositories\n\
                 - license:apache-2.0 - Apache 2.0 licensed repositories\n\
                 - license:gpl-3.0 - GPL 3.0 licensed repositories\n\
                 Example:\n\
                 github_search_repositories({\"query\": \"language:rust license:mit stars:>500\"})\n\n\
                 SIZE FILTERS:\n\
                 - size:>1000 - Larger than 1000 KB\n\
                 - size:<100 - Smaller than 100 KB\n\
                 - size:100..500 - Between 100 and 500 KB\n\
                 Example:\n\
                 github_search_repositories({\"query\": \"language:rust size:<500\"})\n\n\
                 STATUS FILTERS:\n\
                 - archived:false - Exclude archived repositories\n\
                 - archived:true - Only archived repositories\n\
                 - is:public - Public repositories only\n\
                 - is:private - Private repositories (requires authentication)\n\
                 - fork:false - Exclude forks\n\
                 - fork:true - Only forks\n\
                 - fork:only - Only show forks\n\
                 Example:\n\
                 github_search_repositories({\"query\": \"language:rust archived:false fork:false\"})\n\n\
                 ACTIVITY FILTERS:\n\
                 - good-first-issues:>3 - Has more than 3 \"good first issue\" labels\n\
                 - help-wanted-issues:>5 - Has more than 5 \"help wanted\" labels\n\
                 Example:\n\
                 github_search_repositories({\"query\": \"language:rust good-first-issues:>3\"})\n\n\
                 COMBINING QUALIFIERS:\n\
                 You can combine multiple qualifiers for precise searches:\n\
                 github_search_repositories({\n\
                   \"query\": \"language:rust stars:>1000 pushed:>2024-01-01 archived:false\"\n\
                 })\n\
                 github_search_repositories({\n\
                   \"query\": \"created:>2024-01-01 topic:async topic:http language:rust\"\n\
                 })\n\n\
                 RANGE SYNTAX:\n\
                 Use .. for ranges:\n\
                 - stars:10..100 - Between 10 and 100 stars\n\
                 - created:2023-01-01..2023-12-31 - Created in 2023\n\
                 - size:100..500 - Size between 100 and 500 KB\n\n\
                 COMPARISON OPERATORS:\n\
                 - > greater than\n\
                 - >= greater than or equal\n\
                 - < less than\n\
                 - <= less than or equal\n\
                 - .. range (inclusive)\n\n\
                 DATE FORMATS:\n\
                 - YYYY-MM-DD: 2024-01-01\n\
                 - Relative dates: >2024-01-01 (after this date)\n\
                 - Ranges: 2023-01-01..2023-12-31\n\n\
                 BOOLEAN LOGIC:\n\
                 - AND: Space between terms (implicit)\n\
                   Example: \"language:rust stars:>1000\" (both conditions)\n\
                 - OR: Use OR between terms\n\
                   Example: \"language:rust OR language:go\"\n\
                 - NOT: Use NOT or - before term\n\
                   Example: \"language:rust NOT archived:true\"\n\n\
                 BEST PRACTICES:\n\
                 - Start with broad queries, then add qualifiers\n\
                 - Use language and topic for relevance\n\
                 - Filter by stars for quality\n\
                 - Check pushed date for active projects\n\
                 - Combine multiple qualifiers for precision",
            ),
        },
    ]
}

/// Discovery workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are effective workflows for discovering and researching repositories on GitHub?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Here are proven workflows for repository discovery and research on GitHub:\n\n\
                 WORKFLOW 1: RESEARCHING LIBRARIES\n\
                 Goal: Find the best library for specific functionality\n\n\
                 Step 1: Broad search with quality filter\n\
                 github_search_repositories({\n\
                   \"query\": \"language:rust topic:async stars:>100\",\n\
                   \"sort\": \"stars\",\n\
                   \"per_page\": 30\n\
                 })\n\
                 Analysis: Get overview of popular async libraries\n\n\
                 Step 2: Refine by recent activity\n\
                 github_search_repositories({\n\
                   \"query\": \"language:rust topic:async stars:>100 pushed:>2024-01-01\",\n\
                   \"sort\": \"updated\"\n\
                 })\n\
                 Analysis: Filter for actively maintained projects\n\n\
                 Step 3: Check documentation and examples\n\
                 github_search_repositories({\n\
                   \"query\": \"language:rust topic:async topic:example\"\n\
                 })\n\
                 Analysis: Find learning resources\n\n\
                 Decision Criteria:\n\
                 - Stars: Community adoption\n\
                 - Forks: Developer interest\n\
                 - Recent pushes: Active maintenance\n\
                 - Topics: Feature coverage\n\
                 - License: Legal compatibility\n\n\
                 WORKFLOW 2: FINDING ALTERNATIVES\n\
                 Goal: Compare different implementations of similar functionality\n\n\
                 Step 1: Find popular options\n\
                 github_search_repositories({\n\
                   \"query\": \"http server language:rust stars:>500\",\n\
                   \"sort\": \"stars\"\n\
                 })\n\n\
                 Step 2: Check newer alternatives\n\
                 github_search_repositories({\n\
                   \"query\": \"http server language:rust created:>2023-01-01 stars:>50\",\n\
                   \"sort\": \"stars\"\n\
                 })\n\n\
                 Step 3: Compare by specific features\n\
                 github_search_repositories({\n\
                   \"query\": \"http server language:rust topic:websocket\"\n\
                 })\n\n\
                 Comparison Points:\n\
                 - Star count: Popularity\n\
                 - Fork count: Developer activity\n\
                 - Last pushed: Maintenance status\n\
                 - Topics: Feature set\n\
                 - Size: Complexity\n\
                 - License: Usage constraints\n\n\
                 WORKFLOW 3: DEPENDENCY SELECTION\n\
                 Goal: Choose reliable dependencies for a project\n\n\
                 Step 1: Find stable, popular options\n\
                 github_search_repositories({\n\
                   \"query\": \"json parser language:rust stars:>500 archived:false\",\n\
                   \"sort\": \"stars\"\n\
                 })\n\n\
                 Step 2: Verify active maintenance\n\
                 github_search_repositories({\n\
                   \"query\": \"json parser language:rust pushed:>2024-01-01 stars:>500\"\n\
                 })\n\n\
                 Step 3: Check license compatibility\n\
                 github_search_repositories({\n\
                   \"query\": \"json parser language:rust license:mit stars:>500\"\n\
                 })\n\n\
                 Selection Criteria:\n\
                 - High star count: Proven quality\n\
                 - Recent activity: Ongoing support\n\
                 - Compatible license: Legal safety\n\
                 - Good documentation: Easy integration\n\
                 - Active issues: Community engagement\n\n\
                 WORKFLOW 4: FINDING CONTRIBUTION OPPORTUNITIES\n\
                 Goal: Find open source projects to contribute to\n\n\
                 Step 1: Find beginner-friendly projects\n\
                 github_search_repositories({\n\
                   \"query\": \"language:rust good-first-issues:>3 stars:>100\"\n\
                 })\n\n\
                 Step 2: Find projects seeking help\n\
                 github_search_repositories({\n\
                   \"query\": \"language:rust help-wanted-issues:>5 stars:>50\"\n\
                 })\n\n\
                 Step 3: Find active communities\n\
                 github_search_repositories({\n\
                   \"query\": \"language:rust pushed:>2024-11-01 forks:>50\",\n\
                   \"sort\": \"updated\"\n\
                 })\n\n\
                 BEST PRACTICES:\n\
                 - Start broad, narrow progressively\n\
                 - Use stars as quality indicator (>100 for libraries, >500 for frameworks)\n\
                 - Check pushed date for vitality (within 3 months for active projects)\n\
                 - Review topics for features\n\
                 - Verify license compatibility\n\
                 - Compare multiple options before deciding",
            ),
        },
    ]
}
