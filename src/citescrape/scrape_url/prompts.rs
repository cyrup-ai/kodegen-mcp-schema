//! Prompt messages for scrape_url tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ScrapeUrlPromptArgs;

/// Prompt provider for scrape_url tool
///
/// This is the ONLY way to provide prompts for scrape_url - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ScrapeUrlPrompts;

impl PromptProvider for ScrapeUrlPrompts {
    type PromptArgs = ScrapeUrlPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("search") => prompt_search(),
            Some("background") => prompt_background(),
            _ => prompt_crawling(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (crawling, search, background)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE SCRAPE_URL
// ============================================================================

/// Multi-page crawling and depth control
fn prompt_crawling() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I crawl multiple pages and follow links?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use the scrape_url tool with crawling parameters to follow links and extract content from multiple pages.\n\n\
                 MULTI-PAGE CRAWLING:\n\n\
                 1. Crawl documentation site:\n\
                    scrape_url({\n\
                        \"action\": \"CRAWL\",\n\
                        \"url\": \"https://docs.rs/serde/latest/serde/\",\n\
                        \"max_depth\": 2,\n\
                        \"limit\": 20\n\
                    })\n\n\
                 2. Crawl with subdomain access:\n\
                    scrape_url({\n\
                        \"action\": \"CRAWL\",\n\
                        \"url\": \"https://example.com/docs\",\n\
                        \"max_depth\": 3,\n\
                        \"limit\": 50,\n\
                        \"allow_subdomains\": true\n\
                    })\n\n\
                 3. Control crawl rate:\n\
                    scrape_url({\n\
                        \"action\": \"CRAWL\",\n\
                        \"url\": \"https://api.example.com/docs\",\n\
                        \"crawl_rate_rps\": 1,\n\
                        \"max_depth\": 2\n\
                    })\n\n\
                 CRAWL PARAMETERS:\n\
                 - max_depth: How deep to follow links (default: 3)\n\
                 - limit: Max pages to crawl (default: unbounded)\n\
                 - allow_subdomains: Follow subdomain links\n\
                 - crawl_rate_rps: Requests per second (default: 2)\n\n\
                 CRAWL BEHAVIOR:\n\
                 - Follows links within domain\n\
                 - Respects depth limit\n\
                 - Extracts text content\n\
                 - Converts to markdown\n\n\
                 DEPTH EXPLANATION:\n\
                 - Depth 0: Only the starting URL\n\
                 - Depth 1: Starting URL + direct links\n\
                 - Depth 2: Above + links from depth 1 pages\n\
                 - Depth 3: Above + links from depth 2 pages\n\n\
                 RATE LIMITING:\n\
                 - crawl_rate_rps controls politeness\n\
                 - Default: 2 requests per second\n\
                 - Lower for slower, more polite crawling\n\
                 - Higher for faster crawling (be careful)\n\n\
                 BEST PRACTICES:\n\
                 1. Set reasonable limits (50-100 pages)\n\
                 2. Use depth 2-3 for most sites\n\
                 3. Lower crawl_rate_rps for polite crawling\n\
                 4. Enable search for later queries\n\
                 5. Use background for large crawls",
            ),
        },
    ]
}

/// Searching crawled content
fn prompt_search() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I search through content I've already crawled?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "After crawling a site, use the SEARCH action to perform full-text search on the indexed content.\n\n\
                 SEARCHING CRAWLED CONTENT:\n\n\
                 1. First crawl the site:\n\
                    scrape_url({\n\
                        \"action\": \"CRAWL\",\n\
                        \"url\": \"https://docs.rs/tokio\",\n\
                        \"max_depth\": 2,\n\
                        \"enable_search\": true\n\
                    })\n\n\
                 2. Search indexed content:\n\
                    scrape_url({\n\
                        \"action\": \"SEARCH\",\n\
                        \"crawl_id\": 0,\n\
                        \"query\": \"spawn blocking\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"results\": [\n\
                     {\n\
                       \"url\": \"https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html\",\n\
                       \"title\": \"spawn_blocking - Tokio\",\n\
                       \"snippet\": \"Runs the provided closure on a thread...\",\n\
                       \"score\": 0.95\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 3. Search with pagination:\n\
                    scrape_url({\n\
                        \"action\": \"SEARCH\",\n\
                        \"crawl_id\": 0,\n\
                        \"query\": \"async\",\n\
                        \"search_limit\": 20,\n\
                        \"search_offset\": 0\n\
                    })\n\n\
                 SEARCH PARAMETERS:\n\
                 - enable_search: Enable indexing (default: true)\n\
                 - search_limit: Max results (default: 10)\n\
                 - search_offset: Pagination offset\n\
                 - search_highlight: Highlight matches (default: true)\n\n\
                 HOW SEARCH WORKS:\n\
                 - Uses Tantivy full-text search engine\n\
                 - Indexes content during crawl\n\
                 - Returns relevance-scored results\n\
                 - Supports pagination for large result sets\n\
                 - Highlights matching text in snippets\n\n\
                 SEARCH WORKFLOW:\n\
                 1. Crawl with enable_search: true (default)\n\
                 2. Wait for crawl to complete\n\
                 3. Use SEARCH action with queries\n\
                 4. Paginate through results if needed\n\n\
                 PAGINATION:\n\
                 - search_limit: Number of results per page\n\
                 - search_offset: Skip first N results\n\
                 - Example: offset=0, limit=10 (first 10)\n\
                 - Example: offset=10, limit=10 (next 10)\n\n\
                 SEARCH TIPS:\n\
                 - Use specific keywords for better results\n\
                 - Multiple words search for all terms\n\
                 - Results ordered by relevance score\n\
                 - Snippets show context around matches",
            ),
        },
    ]
}

/// Background crawl management
fn prompt_background() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I run large crawls in the background and check their progress?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use await_completion_ms parameter to control timeout behavior. Set to 0 for fire-and-forget background crawling.\n\n\
                 BACKGROUND CRAWL MANAGEMENT:\n\n\
                 1. Start background crawl:\n\
                    scrape_url({\n\
                        \"action\": \"CRAWL\",\n\
                        \"url\": \"https://large-docs-site.com\",\n\
                        \"max_depth\": 3,\n\
                        \"limit\": 100,\n\
                        \"await_completion_ms\": 0\n\
                    })\n\
                    // Returns immediately, crawl continues\n\n\
                 2. Check progress:\n\
                    scrape_url({\n\
                        \"action\": \"READ\",\n\
                        \"crawl_id\": 0\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"crawl_id\": 0,\n\
                   \"status\": \"running\",\n\
                   \"pages_crawled\": 25,\n\
                   \"pages_queued\": 75,\n\
                   \"errors\": 2\n\
                 }\n\n\
                 3. List all crawls:\n\
                    scrape_url({\n\
                        \"action\": \"LIST\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"crawls\": [\n\
                     { \"crawl_id\": 0, \"status\": \"running\", \"url\": \"...\" },\n\
                     { \"crawl_id\": 1, \"status\": \"completed\", \"url\": \"...\" }\n\
                   ]\n\
                 }\n\n\
                 4. Kill crawl:\n\
                    scrape_url({\n\
                        \"action\": \"KILL\",\n\
                        \"crawl_id\": 0\n\
                    })\n\n\
                 TIMEOUT BEHAVIOR:\n\
                 - await_completion_ms: 0 = fire-and-forget\n\
                 - await_completion_ms: 60000 = wait up to 1 minute\n\
                 - Default: 600000 (10 minutes)\n\n\
                 WHEN TIMEOUT OCCURS:\n\
                 - Returns current progress\n\
                 - Crawl continues in background\n\
                 - Use READ action to check status\n\
                 - Can search once completed\n\n\
                 CRAWL STATUSES:\n\
                 - \"running\": Currently crawling pages\n\
                 - \"completed\": Finished successfully\n\
                 - \"failed\": Encountered fatal error\n\
                 - \"cancelled\": Stopped via KILL action\n\n\
                 MONITORING WORKFLOW:\n\
                 1. Start with await_completion_ms: 0\n\
                 2. Check with READ periodically\n\
                 3. When status is \"completed\", search is ready\n\
                 4. KILL if no longer needed\n\n\
                 MULTIPLE CRAWLS:\n\
                 Use different crawl_id for parallel crawls:\n\
                 scrape_url({\"crawl_id\": 0, \"url\": \"https://site1.com\", ...})\n\
                 scrape_url({\"crawl_id\": 1, \"url\": \"https://site2.com\", ...})\n\
                 Each crawl is independent and can run concurrently.\n\n\
                 BEST PRACTICES:\n\
                 1. Use background for crawls over 50 pages\n\
                 2. Check progress with READ action\n\
                 3. Use LIST to see all active crawls\n\
                 4. KILL crawls when no longer needed\n\
                 5. Each crawl_id has independent state",
            ),
        },
    ]
}
