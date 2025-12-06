//! Prompt messages for web_search tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::WebSearchPromptArgs;

/// Prompt provider for web_search tool
///
/// This is the ONLY way to provide prompts for web_search - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct WebSearchPrompts;

impl PromptProvider for WebSearchPrompts {
    type PromptArgs = WebSearchPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic_search(),
            Some("research") => prompt_research_search(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Search scenario: basic, research".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS WEB SEARCH WORKFLOWS
// ============================================================================

/// Basic DuckDuckGo web search
fn prompt_basic_search() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I search the web using DuckDuckGo to find relevant documentation?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The web_search tool performs DuckDuckGo searches and returns structured results with titles, URLs, and snippets.\n\n\
                 BASIC SEARCH USAGE:\n\
                 web_search({\"query\": \"rust async await tutorial\"})\n\
                 Returns:\n\
                 {\n\
                   \"success\": true,\n\
                   \"query\": \"rust async await tutorial\",\n\
                   \"results_count\": 10,\n\
                   \"results\": [\n\
                     {\n\
                       \"rank\": 1,\n\
                       \"title\": \"Asynchronous Programming in Rust\",\n\
                       \"url\": \"https://rust-lang.github.io/async-book/\",\n\
                       \"snippet\": \"Learn how to use async/await in Rust for concurrent programming...\"\n\
                     },\n\
                     {\n\
                       \"rank\": 2,\n\
                       \"title\": \"Tokio Tutorial - Asynchronous Rust\",\n\
                       \"url\": \"https://tokio.rs/tokio/tutorial\",\n\
                       \"snippet\": \"A guide to async Rust with the Tokio runtime...\"\n\
                     },\n\
                     {\n\
                       \"rank\": 3,\n\
                       \"title\": \"Async/await - The Rust Programming Language\",\n\
                       \"url\": \"https://doc.rust-lang.org/book/ch16-04-async-await.html\",\n\
                       \"snippet\": \"Rust's async/await syntax for writing asynchronous code...\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 KEY FEATURES:\n\
                 - Simple query parameter (just search terms)\n\
                 - Returns 10 results by default\n\
                 - Structured output: rank, title, url, snippet\n\
                 - Fast (typically 3-5 seconds)\n\
                 - No rate limiting needed\n\
                 - Privacy-focused (DuckDuckGo doesn't track)\n\n\
                 RESULT FIELDS:\n\
                 - rank: Position in results (1-10)\n\
                 - title: Page title\n\
                 - url: Direct link to page\n\
                 - snippet: Description/preview text\n\n\
                 COMMON SEARCH PATTERNS:\n\
                 1. Find documentation:\n\
                    web_search({\"query\": \"python asyncio documentation\"})\n\
                 2. Discover libraries:\n\
                    web_search({\"query\": \"rust web framework comparison\"})\n\
                 3. Research topics:\n\
                    web_search({\"query\": \"oauth2 security best practices\"})\n\
                 4. Find tutorials:\n\
                    web_search({\"query\": \"react hooks tutorial beginner\"})\n\
                 5. Locate API docs:\n\
                    web_search({\"query\": \"stripe api reference\"})\n\n\
                 EFFECTIVE QUERY TIPS:\n\
                 - Be specific: \"rust error handling\" vs \"error handling\"\n\
                 - Include keywords: \"tutorial\", \"documentation\", \"guide\", \"reference\"\n\
                 - Add technology names: \"python\", \"rust\", \"javascript\"\n\
                 - Use quotes for exact phrases: \"\"async/await\"\"\n\
                 - Combine terms: \"tokio async runtime tutorial\"\n\n\
                 WHEN TO USE WEB_SEARCH:\n\
                 - Quick URL discovery before crawling\n\
                 - Finding documentation sites\n\
                 - Researching unfamiliar topics\n\
                 - Discovering relevant resources\n\
                 - Locating official documentation\n\n\
                 WHEN TO USE SCRAPE_URL INSTEAD:\n\
                 - Need full page content (not just snippets)\n\
                 - Want offline access to documentation\n\
                 - Building searchable knowledge base\n\
                 - Need to search within site content\n\
                 - Archiving for repeated access\n\n\
                 TYPICAL WORKFLOW:\n\
                 1. Use web_search to find relevant URLs\n\
                 2. Review results and pick best matches\n\
                 3. Use scrape_url to crawl those sites\n\
                 4. Build searchable knowledge base",
            ),
        },
    ]
}

/// Multi-query research pattern
fn prompt_research_search() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use web_search as part of a research workflow to discover and crawl documentation?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "web_search is the first step in a comprehensive research workflow - discover URLs, then crawl with scrape_url.\n\n\
                 RESEARCH WORKFLOW (3 STEPS):\n\n\
                 STEP 1: DISCOVER - Find relevant documentation URLs\n\
                 web_search({\"query\": \"rust async programming tutorial\"})\n\
                 Returns top 10 results with URLs to potential documentation sites\n\n\
                 STEP 2: REVIEW - Analyze search results\n\
                 Look for:\n\
                 - Official documentation (doc.rust-lang.org, docs.rs)\n\
                 - Tutorial sites (tokio.rs/tutorial)\n\
                 - Reference materials (API docs, guides)\n\
                 - Community resources (blogs, examples)\n\n\
                 STEP 3: CRAWL - Archive documentation with scrape_url\n\
                 scrape_url({\"action\": \"CRAWL\", \"url\": \"https://docs.rs/tokio\", \"crawl_id\": 0})\n\
                 scrape_url({\"action\": \"CRAWL\", \"url\": \"https://rust-lang.github.io/async-book\", \"crawl_id\": 1})\n\n\
                 MULTI-QUERY RESEARCH PATTERN:\n\
                 When researching complex topics, use multiple searches:\n\
                 // Search for official docs\n\
                 web_search({\"query\": \"rust async official documentation\"})\n\
                 // Search for tutorials\n\
                 web_search({\"query\": \"rust async await tutorial beginner\"})\n\
                 // Search for advanced topics\n\
                 web_search({\"query\": \"rust async runtime internals\"})\n\
                 // Search for specific libraries\n\
                 web_search({\"query\": \"tokio async runtime guide\"})\n\n\
                 TOPIC EXPLORATION WORKFLOW:\n\
                 1. Start broad:\n\
                    web_search({\"query\": \"machine learning python\"})\n\
                 2. Narrow down:\n\
                    web_search({\"query\": \"scikit-learn tutorial\"})\n\
                 3. Get specific:\n\
                    web_search({\"query\": \"scikit-learn random forest classifier\"})\n\
                 4. Find references:\n\
                    web_search({\"query\": \"scikit-learn api documentation\"})\n\n\
                 COMPARISON RESEARCH:\n\
                 Research multiple alternatives:\n\
                 web_search({\"query\": \"rust web framework comparison 2024\"})\n\
                 web_search({\"query\": \"actix-web tutorial\"})\n\
                 web_search({\"query\": \"rocket rust framework guide\"})\n\
                 web_search({\"query\": \"axum web framework\"})\n\
                 Then crawl documentation for each framework\n\n\
                 COMPLETE RESEARCH SESSION EXAMPLE:\n\
                 Topic: \"Learn about OAuth2 security\"\n\n\
                 // Discovery phase\n\
                 web_search({\"query\": \"oauth2 security best practices\"})\n\
                 web_search({\"query\": \"oauth2 specification RFC\"})\n\
                 web_search({\"query\": \"oauth2 tutorial implementation\"})\n\n\
                 // Review results, identify key URLs:\n\
                 // - https://oauth.net/2/\n\
                 // - https://datatracker.ietf.org/doc/html/rfc6749\n\
                 // - https://auth0.com/docs/get-started/authentication-and-authorization-flow\n\n\
                 // Crawl phase - archive for offline study\n\
                 scrape_url({\"action\": \"CRAWL\", \"url\": \"https://oauth.net/2/\", \"crawl_id\": 0, \"max_depth\": 3})\n\
                 scrape_url({\"action\": \"CRAWL\", \"url\": \"https://auth0.com/docs\", \"crawl_id\": 1, \"max_depth\": 4})\n\n\
                 // Monitor crawls\n\
                 scrape_url({\"action\": \"LIST\"})\n\n\
                 // Search indexed content\n\
                 scrape_url({\"action\": \"SEARCH\", \"crawl_id\": 0, \"query\": \"authorization code flow\"})\n\
                 scrape_url({\"action\": \"SEARCH\", \"crawl_id\": 1, \"query\": \"client credentials\"})\n\n\
                 PROGRESSIVE REFINEMENT:\n\
                 Start general, get specific based on results:\n\
                 1. web_search({\"query\": \"error handling programming\"})\n\
                 2. Identify Rust is relevant → web_search({\"query\": \"rust error handling\"})\n\
                 3. See Result<T,E> mentioned → web_search({\"query\": \"rust Result type tutorial\"})\n\
                 4. Find anyhow library → web_search({\"query\": \"rust anyhow error handling\"})\n\
                 5. Crawl final resources\n\n\
                 FINDING OFFICIAL DOCUMENTATION:\n\
                 Add \"official\" or \"documentation\" to queries:\n\
                 web_search({\"query\": \"python official documentation\"})\n\
                 web_search({\"query\": \"react official tutorial\"})\n\
                 web_search({\"query\": \"kubernetes api reference\"})\n\n\
                 COMBINING WITH OTHER TOOLS:\n\
                 1. web_search: Discover URLs (fast, lightweight)\n\
                 2. scrape_url: Archive content (comprehensive, offline)\n\
                 3. scrape_url SEARCH: Query indexed content (fast, precise)\n\n\
                 WHEN NOT TO USE WEB_SEARCH:\n\
                 - You already know the URL → Use scrape_url directly\n\
                 - Need full page content → Use scrape_url CRAWL\n\
                 - Searching already-crawled content → Use scrape_url SEARCH\n\
                 - Deep research on single site → Use scrape_url only\n\n\
                 RESEARCH BEST PRACTICES:\n\
                 1. Start with web_search for discovery\n\
                 2. Use multiple queries to explore topic\n\
                 3. Review snippets to identify quality sources\n\
                 4. Extract URLs from top-ranked results\n\
                 5. Use scrape_url to crawl best matches\n\
                 6. Build searchable knowledge base\n\
                 7. Search indexed content offline\n\n\
                 ADVANTAGES OF WEB_SEARCH + SCRAPE_URL:\n\
                 - web_search: Fast discovery (3-5 seconds)\n\
                 - scrape_url: Comprehensive archiving (minutes)\n\
                 - Combined: Best of both worlds\n\
                 - Offline access after initial crawl\n\
                 - Full-text search via Tantivy\n\
                 - Time-point documentation snapshots",
            ),
        },
    ]
}

/// Comprehensive guide to web search
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using web_search effectively for research and discovery.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "web_search performs DuckDuckGo searches and returns structured results - perfect for discovering documentation URLs before crawling.\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 SIMPLE SEARCH:\n\
                 web_search({\"query\": \"rust async programming\"})\n\n\
                 Returns:\n\
                 {\n\
                   \"success\": true,\n\
                   \"query\": \"rust async programming\",\n\
                   \"results_count\": 10,\n\
                   \"results\": [\n\
                     {\n\
                       \"rank\": 1,\n\
                       \"title\": \"Asynchronous Programming in Rust\",\n\
                       \"url\": \"https://rust-lang.github.io/async-book/\",\n\
                       \"snippet\": \"Learn async/await in Rust...\"\n\
                     },\n\
                     // ... 9 more results\n\
                   ]\n\
                 }\n\n\
                 PARAMETERS:\n\
                 - query (required): Search terms as a string\n\
                 - No other parameters needed!\n\n\
                 OUTPUT STRUCTURE:\n\
                 - success: true if search succeeded\n\
                 - query: Your search terms (echoed back)\n\
                 - results_count: Number of results returned (typically 10)\n\
                 - results: Array of search results\n\n\
                 RESULT FIELDS:\n\
                 - rank: Position in results (1-10)\n\
                 - title: Page title from search result\n\
                 - url: Direct link to page\n\
                 - snippet: Description/preview text (may be None)\n\n\
                 =============================================================================\n\
                 QUERY CRAFTING\n\
                 =============================================================================\n\n\
                 EFFECTIVE QUERIES:\n\
                 1. Be specific:\n\
                    GOOD: \"rust tokio async runtime tutorial\"\n\
                    BAD: \"async\"\n\n\
                 2. Include technology names:\n\
                    GOOD: \"python fastapi documentation\"\n\
                    BAD: \"api framework\"\n\n\
                 3. Add context keywords:\n\
                    - \"tutorial\" for learning resources\n\
                    - \"documentation\" for official docs\n\
                    - \"guide\" for step-by-step instructions\n\
                    - \"reference\" for API documentation\n\
                    - \"examples\" for code samples\n\
                    - \"best practices\" for expert guidance\n\n\
                 4. Use natural language:\n\
                    GOOD: \"how to handle errors in rust\"\n\
                    GOOD: \"rust error handling patterns\"\n\n\
                 5. Combine terms:\n\
                    \"react hooks useState useEffect tutorial\"\n\
                    \"kubernetes deployment yaml configuration\"\n\n\
                 =============================================================================\n\
                 COMMON USE CASES\n\
                 =============================================================================\n\n\
                 1. FIND OFFICIAL DOCUMENTATION:\n\
                    web_search({\"query\": \"rust official documentation\"})\n\
                    web_search({\"query\": \"python stdlib reference\"})\n\
                    web_search({\"query\": \"react official tutorial\"})\n\n\
                 2. DISCOVER LIBRARIES:\n\
                    web_search({\"query\": \"rust http client library\"})\n\
                    web_search({\"query\": \"python async web framework\"})\n\
                    web_search({\"query\": \"javascript state management\"})\n\n\
                 3. RESEARCH TOPICS:\n\
                    web_search({\"query\": \"oauth2 security best practices\"})\n\
                    web_search({\"query\": \"microservices architecture patterns\"})\n\
                    web_search({\"query\": \"database indexing optimization\"})\n\n\
                 4. FIND TUTORIALS:\n\
                    web_search({\"query\": \"tokio async rust tutorial beginner\"})\n\
                    web_search({\"query\": \"react native tutorial 2024\"})\n\
                    web_search({\"query\": \"docker compose tutorial\"})\n\n\
                 5. LOCATE API DOCS:\n\
                    web_search({\"query\": \"stripe api documentation\"})\n\
                    web_search({\"query\": \"github api reference\"})\n\
                    web_search({\"query\": \"openai api documentation\"})\n\n\
                 6. COMPARE OPTIONS:\n\
                    web_search({\"query\": \"rust web framework comparison 2024\"})\n\
                    web_search({\"query\": \"nosql database comparison\"})\n\
                    web_search({\"query\": \"ci/cd tools comparison\"})\n\n\
                 =============================================================================\n\
                 INTEGRATION WITH SCRAPE_URL\n\
                 =============================================================================\n\n\
                 WORKFLOW: DISCOVER → CRAWL → SEARCH\n\n\
                 STEP 1: DISCOVER URLs with web_search\n\
                 web_search({\"query\": \"rust async programming tutorial\"})\n\
                 // Returns URLs to crawl\n\n\
                 STEP 2: CRAWL selected sites with scrape_url\n\
                 scrape_url({\"action\": \"CRAWL\", \"url\": \"https://docs.rs/tokio\", \"crawl_id\": 0})\n\
                 scrape_url({\"action\": \"CRAWL\", \"url\": \"https://rust-lang.github.io/async-book\", \"crawl_id\": 1})\n\n\
                 STEP 3: SEARCH indexed content\n\
                 scrape_url({\"action\": \"SEARCH\", \"crawl_id\": 0, \"query\": \"runtime\"})\n\n\
                 =============================================================================\n\
                 DECISION TREE: WEB_SEARCH VS SCRAPE_URL\n\
                 =============================================================================\n\n\
                 Use WEB_SEARCH when:\n\
                 - Need to discover URLs quickly\n\
                 - Researching unfamiliar topic\n\
                 - Want quick preview (snippets)\n\
                 - Don't know which sites to visit\n\
                 - Exploring options/alternatives\n\
                 - Need fast results (3-5 seconds)\n\n\
                 Use SCRAPE_URL when:\n\
                 - Already know the URL\n\
                 - Need full page content\n\
                 - Want offline access\n\
                 - Building knowledge base\n\
                 - Need searchable archive\n\
                 - Deep research on specific site\n\n\
                 Use BOTH when:\n\
                 - Research workflow: discover → crawl → search\n\
                 - Building comprehensive knowledge base\n\
                 - Comparing multiple sources\n\
                 - Long-term research project\n\n\
                 =============================================================================\n\
                 COMPLETE RESEARCH EXAMPLE\n\
                 =============================================================================\n\n\
                 SCENARIO: Research Kubernetes deployment strategies\n\n\
                 // Phase 1: Discovery\n\
                 web_search({\"query\": \"kubernetes deployment strategies\"})\n\
                 web_search({\"query\": \"kubernetes official documentation\"})\n\
                 web_search({\"query\": \"kubernetes best practices 2024\"})\n\n\
                 // Phase 2: Review results, identify top sources:\n\
                 // - kubernetes.io/docs\n\
                 // - kubernetes.io/blog\n\
                 // - CNCF resources\n\n\
                 // Phase 3: Crawl for offline access\n\
                 scrape_url({\"action\": \"CRAWL\", \"url\": \"https://kubernetes.io/docs\", \"crawl_id\": 0, \"max_depth\": 4})\n\
                 scrape_url({\"action\": \"CRAWL\", \"url\": \"https://kubernetes.io/blog\", \"crawl_id\": 1, \"max_depth\": 3})\n\n\
                 // Phase 4: Monitor crawls\n\
                 scrape_url({\"action\": \"LIST\"})\n\n\
                 // Phase 5: Search indexed content\n\
                 scrape_url({\"action\": \"SEARCH\", \"crawl_id\": 0, \"query\": \"rolling update\"})\n\
                 scrape_url({\"action\": \"SEARCH\", \"crawl_id\": 0, \"query\": \"deployment strategies\"})\n\
                 scrape_url({\"action\": \"SEARCH\", \"crawl_id\": 1, \"query\": \"blue green deployment\"})\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. QUERY QUALITY:\n\
                    - Be specific with technology names\n\
                    - Include context keywords (tutorial, docs, guide)\n\
                    - Use natural language\n\
                    - Avoid overly broad queries\n\n\
                 2. RESULT REVIEW:\n\
                    - Check rank (top 3 usually most relevant)\n\
                    - Read snippets for quality assessment\n\
                    - Look for official sources (org, .io, .com)\n\
                    - Prefer recent results (check URL/snippet for dates)\n\n\
                 3. PROGRESSIVE REFINEMENT:\n\
                    - Start broad, refine based on results\n\
                    - Use multiple queries for complex topics\n\
                    - Follow up on interesting results\n\n\
                 4. INTEGRATION:\n\
                    - Use web_search for discovery\n\
                    - Use scrape_url for deep research\n\
                    - Combine for comprehensive workflows\n\n\
                 5. URL EXTRACTION:\n\
                    - Extract URLs from top-ranked results\n\
                    - Pass to scrape_url for crawling\n\
                    - Build searchable knowledge base\n\n\
                 =============================================================================\n\
                 KEY CHARACTERISTICS\n\
                 =============================================================================\n\n\
                 SPEED: 3-5 seconds per query\n\
                 RESULTS: Typically 10 results\n\
                 SOURCE: DuckDuckGo (privacy-focused)\n\
                 OUTPUT: Structured JSON (easy to parse)\n\
                 RATE LIMITS: None (reasonable use)\n\
                 PRIVACY: No tracking or history\n\n\
                 Remember: web_search is for discovery. Use it to find URLs, then use scrape_url to build comprehensive, searchable knowledge bases!",
            ),
        },
    ]
}
