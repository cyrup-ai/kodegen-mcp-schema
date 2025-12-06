//! Prompt messages for github_search_code tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::SearchCodePromptArgs;

/// Prompt provider for search_code tool
///
/// This is the ONLY way to provide prompts for search_code - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct SearchCodePrompts;

impl PromptProvider for SearchCodePrompts {
    type PromptArgs = SearchCodePromptArgs;

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
                description: Some("Scenario to show: basic (default), syntax, workflows".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO SEARCH GITHUB CODE
// ============================================================================

/// Basic code search operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I search for code on GitHub using github_search_code?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_search_code tool searches code across GitHub repositories using GitHub's powerful code search API.\n\n\
                 BASIC SEARCHING:\n\
                 Simple keyword search:\n\
                 github_search_code({\"query\": \"async fn process\"})\n\n\
                 Search in specific repository:\n\
                 github_search_code({\"query\": \"async fn process repo:rust-lang/rust\"})\n\n\
                 PARAMETERS:\n\
                 - query (required): Search query string with GitHub code search syntax\n\
                 - sort (optional): \"indexed\" (sorts by last indexed time)\n\
                 - order (optional): \"asc\" or \"desc\" (default: \"desc\")\n\
                 - page (optional): Page number for pagination (default: 1)\n\
                 - per_page (optional): Results per page, max 100 (default: 30)\n\
                 - enrich_stars (optional): Include repository star counts (default: false)\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"query\": \"async fn process\",\n\
                   \"total_count\": 150,\n\
                   \"items\": [\n\
                     {\n\
                       \"name\": \"processor.rs\",\n\
                       \"path\": \"src/processor.rs\",\n\
                       \"sha\": \"abc123...\",\n\
                       \"repository_full_name\": \"example/project\",\n\
                       \"repository_owner\": \"example\",\n\
                       \"repository_name\": \"project\",\n\
                       \"html_url\": \"https://github.com/example/project/blob/main/src/processor.rs\",\n\
                       \"git_url\": \"https://api.github.com/repos/example/project/git/blobs/abc123\",\n\
                       \"star_count\": 1500\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 AUTHENTICATION:\n\
                 REQUIRED: Set GITHUB_TOKEN environment variable with a valid GitHub personal access token.\n\n\
                 Token requirements:\n\
                 - Any scope works (read:repo, public_repo, or no scope)\n\
                 - Token must be valid and not expired\n\
                 - Public repository access is sufficient for code search\n\n\
                 RATE LIMITING:\n\
                 WARNING: Code search has STRICT rate limiting!\n\n\
                 Limits:\n\
                 - 30 requests per minute (very strict)\n\
                 - Much stricter than general GitHub API (5,000 requests/hour)\n\
                 - Rate limit is shared across all code searches\n\n\
                 Strategies:\n\
                 - Use specific queries to minimize searches\n\
                 - Set per_page to 100 to get more results per request\n\
                 - Cache results locally when possible\n\
                 - Space out searches to avoid hitting the limit\n\
                 - Check X-RateLimit-Remaining header in responses\n\n\
                 COMMON PATTERNS:\n\
                 Find function implementations:\n\
                 github_search_code({\"query\": \"fn new language:rust\"})\n\n\
                 Search for API usage:\n\
                 github_search_code({\"query\": \"tokio::spawn repo:tokio-rs/tokio\"})\n\n\
                 Find configuration examples:\n\
                 github_search_code({\"query\": \"serde(default) extension:rs\"})\n\n\
                 Search in organization:\n\
                 github_search_code({\"query\": \"impl Error org:rust-lang\"})\n\n\
                 PAGINATION:\n\
                 For large result sets:\n\
                 github_search_code({\"query\": \"async fn\", \"per_page\": 100, \"page\": 1})\n\
                 github_search_code({\"query\": \"async fn\", \"per_page\": 100, \"page\": 2})\n\n\
                 Maximum 1,000 results per query (10 pages × 100 per_page).\n\
                 If total_count > 1,000, refine your query to be more specific.\n\n\
                 BEST PRACTICES:\n\
                 - Start with specific queries to reduce results\n\
                 - Use language filters to narrow scope (language:rust, language:python)\n\
                 - Add repo or org qualifiers when possible to limit scope\n\
                 - Set per_page to 100 for batch processing\n\
                 - Check total_count in response before paginating\n\
                 - Use enrich_stars to find popular repositories\n\
                 - ALWAYS respect the 30 requests/minute rate limit",
            ),
        },
    ]
}

/// Complete search syntax reference
fn prompt_syntax() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What search syntax can I use with github_search_code?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "GitHub code search supports powerful search qualifiers to narrow and target your searches.\n\n\
                 SEARCH QUALIFIERS:\n\n\
                 REPOSITORY QUALIFIERS:\n\
                 - repo:owner/name\n\
                   Search specific repository\n\
                   Example: github_search_code({\"query\": \"fn new repo:tokio-rs/tokio\"})\n\n\
                 - user:username\n\
                   Search all repositories owned by user\n\
                   Example: github_search_code({\"query\": \"impl From user:rust-lang\"})\n\n\
                 - org:organization\n\
                   Search all repositories in organization\n\
                   Example: github_search_code({\"query\": \"async fn org:tokio-rs\"})\n\n\
                 LANGUAGE QUALIFIERS:\n\
                 - language:name\n\
                   Filter by programming language\n\
                   Example: github_search_code({\"query\": \"impl Iterator language:rust\"})\n\
                   Common languages: rust, python, javascript, typescript, go, java, c, cpp\n\n\
                 PATH QUALIFIERS:\n\
                 - path:directory/\n\
                   Search in specific directory\n\
                   Example: github_search_code({\"query\": \"async path:src/handlers\"})\n\n\
                 - filename:name\n\
                   Search in specific filename\n\
                   Example: github_search_code({\"query\": \"impl filename:lib.rs\"})\n\n\
                 - extension:ext\n\
                   Filter by file extension\n\
                   Example: github_search_code({\"query\": \"serde extension:rs\"})\n\n\
                 SIZE QUALIFIERS:\n\
                 - size:>n\n\
                   Files larger than n bytes\n\
                   Example: github_search_code({\"query\": \"impl size:>1000 language:rust\"})\n\n\
                 - size:<n\n\
                   Files smaller than n bytes\n\
                   Example: github_search_code({\"query\": \"fn main size:<500\"})\n\n\
                 - size:n..m\n\
                   Files between n and m bytes\n\
                   Example: github_search_code({\"query\": \"size:1000..5000\"})\n\n\
                 BOOLEAN OPERATORS:\n\
                 - AND (default)\n\
                   Both terms must be present\n\
                   Example: github_search_code({\"query\": \"async await\"})\n\
                   (automatically AND)\n\n\
                 - OR\n\
                   Either term can be present\n\
                   Example: github_search_code({\"query\": \"tokio OR async-std\"})\n\n\
                 - NOT\n\
                   Exclude term from results\n\
                   Example: github_search_code({\"query\": \"async NOT tokio\"})\n\n\
                 - \"exact phrase\"\n\
                   Search for exact phrase\n\
                   Example: github_search_code({\"query\": \"\\\"async fn main\\\"\"})\n\n\
                 COMBINING QUALIFIERS:\n\
                 You can combine multiple qualifiers for precise searches:\n\n\
                 Language + Repository:\n\
                 github_search_code({\n\
                   \"query\": \"impl From language:rust repo:rust-lang/rust\"\n\
                 })\n\n\
                 Path + Extension + Language:\n\
                 github_search_code({\n\
                   \"query\": \"error handling path:src/ extension:rs language:rust\"\n\
                 })\n\n\
                 Organization + Filename:\n\
                 github_search_code({\n\
                   \"query\": \"serde org:serde-rs filename:lib.rs\"\n\
                 })\n\n\
                 PRACTICAL EXAMPLES:\n\n\
                 Find specific function in repo:\n\
                 github_search_code({\n\
                   \"query\": \"fn spawn repo:tokio-rs/tokio path:src/\"\n\
                 })\n\n\
                 Search for test patterns:\n\
                 github_search_code({\n\
                   \"query\": \"#[test] filename:test.rs language:rust\"\n\
                 })\n\n\
                 Find error handling patterns:\n\
                 github_search_code({\n\
                   \"query\": \"unwrap_or_else language:rust extension:rs\"\n\
                 })\n\n\
                 SPECIAL CHARACTERS:\n\
                 - Use quotes for exact phrases: \"async fn main\"\n\
                 - Spaces are treated as AND operators\n\n\
                 SYNTAX TIPS:\n\
                 - Qualifiers are case-insensitive (language:Rust = language:rust)\n\
                 - Keywords are case-sensitive by default\n\
                 - Combine specific qualifiers to narrow results\n\
                 - Use language + extension for precision\n\
                 - Add repo/org to limit scope and save rate limit\n\n\
                 LIMITATIONS:\n\
                 - Cannot search file contents larger than 384 KB\n\
                 - Only searches default branches\n\
                 - Rate limited to 30 requests per minute\n\
                 - Maximum 1000 results per query\n\
                 - Some complex regex not supported",
            ),
        },
    ]
}

/// Research workflows and advanced techniques
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are effective research workflows using github_search_code?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Here are proven workflows for effective code research using github_search_code.\n\n\
                 WORKFLOW 1: LEARNING A NEW LIBRARY\n\n\
                 Step 1: Find basic usage examples\n\
                 github_search_code({\n\
                   \"query\": \"use tokio::sync::mpsc language:rust\",\n\
                   \"per_page\": 100,\n\
                   \"enrich_stars\": true\n\
                 })\n\
                 - Get overview of how library is used\n\
                 - Star counts indicate quality projects\n\
                 - Review initialization patterns\n\n\
                 Step 2: Find specific API usage\n\
                 github_search_code({\n\
                   \"query\": \"tokio::sync::mpsc::channel language:rust\"\n\
                 })\n\
                 - See how channels are created\n\
                 - Learn configuration options\n\
                 - Understand error handling\n\n\
                 Step 3: Study production code\n\
                 github_search_code({\n\
                   \"query\": \"tokio::sync::mpsc repo:rust-lang/rust-analyzer\",\n\
                   \"per_page\": 50\n\
                 })\n\
                 - See how experts use the library\n\
                 - Learn advanced patterns\n\
                 - Understand real-world architecture\n\n\
                 Step 4: Find error patterns\n\
                 github_search_code({\n\
                   \"query\": \"tokio::sync::mpsc try_recv language:rust\"\n\
                 })\n\
                 - Learn error handling approaches\n\
                 - See timeout handling\n\
                 - Understand edge cases\n\n\
                 WORKFLOW 2: DISCOVERING PATTERNS\n\n\
                 Step 1: Broad pattern search\n\
                 github_search_code({\n\
                   \"query\": \"async fn main language:rust\",\n\
                   \"per_page\": 100\n\
                 })\n\
                 - Discover common async patterns\n\
                 - See different runtime choices\n\
                 - Learn initialization approaches\n\n\
                 Step 2: Narrow to specific runtime\n\
                 github_search_code({\n\
                   \"query\": \"#[tokio::main] language:rust\"\n\
                 })\n\
                 - Focus on tokio-specific patterns\n\
                 - See attribute usage\n\
                 - Understand runtime configuration\n\n\
                 Step 3: Find configuration patterns\n\
                 github_search_code({\n\
                   \"query\": \"Builder::new tokio language:rust\"\n\
                 })\n\
                 - Learn advanced configuration\n\
                 - See builder pattern usage\n\
                 - Understand customization options\n\n\
                 WORKFLOW 3: SECURITY AUDIT\n\n\
                 Step 1: Find authentication code\n\
                 github_search_code({\n\
                   \"query\": \"password hash org:myorg\"\n\
                 })\n\
                 - Audit password handling\n\
                 - Check for plaintext storage\n\
                 - Verify hashing algorithms\n\n\
                 Step 2: Find token usage\n\
                 github_search_code({\n\
                   \"query\": \"api_key org:myorg extension:rs\"\n\
                 })\n\
                 - Check for hardcoded secrets\n\
                 - Verify secure storage\n\
                 - Find potential leaks\n\n\
                 Step 3: Find SQL queries\n\
                 github_search_code({\n\
                   \"query\": \"execute sql org:myorg\"\n\
                 })\n\
                 - Check for SQL injection risks\n\
                 - Verify parameterized queries\n\
                 - Audit database access\n\n\
                 ADVANCED TECHNIQUES:\n\n\
                 Incremental refinement:\n\
                 - Start with broad query, add qualifiers based on results\n\
                 - Narrow scope iteratively until results are focused\n\n\
                 Popularity-based learning:\n\
                 - Use enrich_stars: true to find quality examples\n\
                 - Learn from popular projects and community standards\n\n\
                 BEST PRACTICES:\n\
                 - Start broad, narrow with qualifiers\n\
                 - Use enrich_stars for quality signals\n\
                 - Respect rate limits (30/minute)\n\
                 - Save queries for common searches\n\
                 - Review code context on GitHub\n\
                 - Verify pattern applicability",
            ),
        },
    ]
}
