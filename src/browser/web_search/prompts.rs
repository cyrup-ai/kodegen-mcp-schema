//! Prompt messages for browser_web_search tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::BrowserWebSearchPromptArgs;

/// Prompt provider for browser_web_search tool
///
/// This is the ONLY way to provide prompts for browser_web_search - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct WebSearchPrompts;

impl PromptProvider for WebSearchPrompts {
    type PromptArgs = BrowserWebSearchPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("comparison") => prompt_comparison(),
            _ => prompt_comparison(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, comparison)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO SEARCH THE WEB
// ============================================================================

/// Basic web search usage
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I perform basic web searches using browser_web_search?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The browser_web_search tool performs web searches and returns structured results with titles, URLs, and snippets. It's fast and returns just search results.\n\n\
                 BASIC WEB SEARCH:\n\n\
                 1. Simple query:\n\
                    browser_web_search({\"query\": \"rust async programming\"})\n\n\
                 2. Question format:\n\
                    browser_web_search({\"query\": \"how to implement linked list in rust\"})\n\n\
                 3. Specific topic:\n\
                    browser_web_search({\"query\": \"tokio vs async-std comparison\"})\n\n\
                 4. Documentation lookup:\n\
                    browser_web_search({\"query\": \"serde deserialize documentation\"})\n\n\
                 5. Library search:\n\
                    browser_web_search({\"query\": \"best rust http client 2024\"})\n\n\
                 RESPONSE STRUCTURE:\n\
                 Returns array of results:\n\
                 [\n\
                   {\n\
                     \"title\": \"Page Title\",\n\
                     \"url\": \"https://example.com/page\",\n\
                     \"snippet\": \"Brief description from search result...\"\n\
                   },\n\
                   ...\n\
                 ]\n\n\
                 Each result contains:\n\
                 - title: The page title from search results\n\
                 - url: Full URL to the page\n\
                 - snippet: Brief description/preview text\n\n\
                 TYPICAL WORKFLOW:\n\
                 1. Search for topic using browser_web_search\n\
                 2. Review result titles and snippets\n\
                 3. Navigate to promising URLs with browser_navigate\n\
                 4. Or use browser_research for deeper analysis\n\n\
                 QUERY FORMULATION TIPS:\n\
                 - Be specific: \"tokio spawn task example\" vs \"tokio\"\n\
                 - Include language: \"rust error handling\" vs \"error handling\"\n\
                 - Include year for current info: \"rust 2024 features\"\n\
                 - Use natural questions: \"how to parse JSON in rust\"\n\
                 - Combine key terms: \"actix-web middleware tutorial\"\n\n\
                 COMMON USE CASES:\n\
                 - Finding documentation links\n\
                 - Looking up error messages\n\
                 - Searching for code examples\n\
                 - Finding tutorials and guides\n\
                 - Discovering libraries and crates\n\
                 - Looking up API references\n\n\
                 RESPONSE CHARACTERISTICS:\n\
                 - Fast: Returns immediately with search results\n\
                 - Structured: Each result has title, URL, snippet\n\
                 - Ordered: Results ranked by relevance\n\
                 - Multiple sources: Get diverse perspectives\n\
                 - No content extraction: Just links and descriptions\n\n\
                 EXAMPLE QUERIES:\n\
                 ```\n\
                 // Finding documentation\n\
                 browser_web_search({\"query\": \"tokio runtime documentation\"})\n\
                 \n\
                 // Looking for tutorials\n\
                 browser_web_search({\"query\": \"rust web scraping tutorial\"})\n\
                 \n\
                 // Error troubleshooting\n\
                 browser_web_search({\"query\": \"rust error E0382 cannot move\"})\n\
                 \n\
                 // Library comparison\n\
                 browser_web_search({\"query\": \"axum vs actix-web performance\"})\n\
                 \n\
                 // Best practices\n\
                 browser_web_search({\"query\": \"rust async best practices 2024\"})\n\
                 ```",
            ),
        },
    ]
}

fn prompt_comparison() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "When should I use browser_web_search vs other search and research tools?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Choose the right tool based on your needs. Each search and research tool has different strengths and use cases.\n\n\
                 WHEN TO USE EACH SEARCH TOOL:\n\n\
                 =============================================================================\n\
                 browser_web_search\n\
                 =============================================================================\n\
                 USE FOR:\n\
                 - Quick lookup of URLs\n\
                 - Finding documentation links\n\
                 - Getting list of resources\n\
                 - Discovering relevant pages\n\
                 - Fast, returns just search results\n\
                 - When you need links, not content\n\n\
                 GOOD FOR:\n\
                 \"I need to find pages about X\"\n\
                 \"Where is the documentation for Y?\"\n\
                 \"What are the top resources for learning Z?\"\n\n\
                 CHARACTERISTICS:\n\
                 - Very fast (seconds)\n\
                 - Returns structured results (title, URL, snippet)\n\
                 - No content extraction\n\
                 - Multiple results at once\n\
                 - Good for discovery phase\n\n\
                 EXAMPLE USAGE:\n\
                 browser_web_search({\"query\": \"tokio async runtime documentation\"})\n\
                 // Returns: List of relevant URLs with titles and snippets\n\
                 // You decide which links to explore further\n\n\
                 =============================================================================\n\
                 browser_research\n\
                 =============================================================================\n\
                 USE FOR:\n\
                 - Deep research on topic\n\
                 - Synthesized information from multiple pages\n\
                 - Comprehensive analysis\n\
                 - When you need understanding, not just links\n\
                 - Automatically crawls and summarizes\n\n\
                 GOOD FOR:\n\
                 \"I need to understand X\"\n\
                 \"What are the pros and cons of Y?\"\n\
                 \"Give me a comprehensive overview of Z\"\n\n\
                 CHARACTERISTICS:\n\
                 - Slower (minutes)\n\
                 - Crawls multiple pages automatically\n\
                 - Synthesizes information from sources\n\
                 - Returns comprehensive report with citations\n\
                 - Good for learning phase\n\n\
                 EXAMPLE USAGE:\n\
                 browser_research({\"query\": \"tokio async runtime architecture\"})\n\
                 // Returns: Detailed report synthesizing information from multiple sources\n\
                 // Includes summaries, comparisons, and citations\n\n\
                 =============================================================================\n\
                 web_search (tool outside browser package)\n\
                 =============================================================================\n\
                 USE FOR:\n\
                 - Similar to browser_web_search\n\
                 - May have different search engine backend\n\
                 - Check which is available in your environment\n\n\
                 COMPARISON WITH browser_web_search:\n\
                 - Same use case (getting search results)\n\
                 - May use different search provider\n\
                 - Check both to see which gives better results\n\
                 - Response format may differ slightly\n\n\
                 =============================================================================\n\
                 scrape_url / browser_navigate\n\
                 =============================================================================\n\
                 USE FOR:\n\
                 - When you already know the URL\n\
                 - Direct content extraction from specific page\n\
                 - No search involved\n\
                 - Getting actual page content\n\n\
                 GOOD FOR:\n\
                 \"I need content from this specific URL\"\n\
                 \"Extract the text from this page\"\n\
                 \"Get me the documentation from this link\"\n\n\
                 CHARACTERISTICS:\n\
                 - Fast (one page fetch)\n\
                 - Returns full page content\n\
                 - Requires knowing the URL beforehand\n\
                 - No search or discovery\n\n\
                 EXAMPLE USAGE:\n\
                 scrape_url({\"url\": \"https://docs.rs/tokio\"})\n\
                 // Returns: Full content of that specific page\n\n\
                 =============================================================================\n\
                 DECISION FLOW\n\
                 =============================================================================\n\n\
                 START: What do you need?\n\n\
                 Do you know the exact URL?\n\
                 ├─ YES → Use scrape_url or browser_navigate\n\
                 │         \"Get content from https://docs.rs/tokio\"\n\
                 │\n\
                 └─ NO → Do you need deep understanding?\n\
                     ├─ YES → Use browser_research\n\
                     │         \"Understand tokio async runtime architecture\"\n\
                     │\n\
                     └─ NO → Do you need quick links?\n\
                         ├─ YES → Use browser_web_search\n\
                         │         \"Find tokio documentation\"\n\
                         │\n\
                         └─ MAYBE → Do you need to interact with page?\n\
                             └─ YES → Use browser_navigate + browser_click/type\n\
                                       \"Navigate to docs.rs and search for tokio\"\n\n\
                 =============================================================================\n\
                 WORKFLOW EXAMPLES\n\
                 =============================================================================\n\n\
                 Workflow 1: DISCOVERY → EXPLORATION\n\
                 Step 1: Find relevant pages\n\
                 browser_web_search({\"query\": \"rust web frameworks comparison 2024\"})\n\
                 \n\
                 Step 2: Visit promising URLs\n\
                 scrape_url({\"url\": \"https://example.com/rust-web-frameworks\"})\n\n\
                 Workflow 2: RESEARCH → DEEP DIVE\n\
                 Step 1: Get comprehensive overview\n\
                 browser_research({\"query\": \"tokio vs async-std differences\"})\n\
                 \n\
                 Step 2: Explore specific aspects\n\
                 browser_web_search({\"query\": \"tokio performance benchmarks\"})\n\n\
                 Workflow 3: DIRECT ACCESS\n\
                 Step 1: Already know the URL\n\
                 scrape_url({\"url\": \"https://tokio.rs/tokio/tutorial\"})\n\n\
                 Workflow 4: INTERACTIVE\n\
                 Step 1: Navigate to site\n\
                 browser_navigate({\"url\": \"https://docs.rs\"})\n\
                 \n\
                 Step 2: Interact with page\n\
                 browser_type_text({\"text\": \"tokio\"})\n\
                 browser_click({\"selector\": \"button[type=submit]\"})\n\n\
                 =============================================================================\n\
                 SPEED vs DEPTH COMPARISON\n\
                 =============================================================================\n\n\
                 FASTEST → SLOWEST:\n\
                 1. scrape_url (if you know URL) - seconds\n\
                 2. browser_web_search - seconds  \n\
                 3. web_search - seconds\n\
                 4. browser_navigate - seconds per interaction\n\
                 5. browser_research - minutes (crawls multiple pages)\n\n\
                 SHALLOWEST → DEEPEST:\n\
                 1. browser_web_search - just search results\n\
                 2. web_search - just search results\n\
                 3. scrape_url - single page content\n\
                 4. browser_navigate - interactive exploration\n\
                 5. browser_research - multi-page synthesis\n\n\
                 =============================================================================\n\
                 CHOOSING THE RIGHT TOOL\n\
                 =============================================================================\n\n\
                 Use browser_web_search when:\n\
                 ✓ You need to find relevant URLs quickly\n\
                 ✓ You want to see what's available on a topic\n\
                 ✓ You're looking for documentation links\n\
                 ✓ You want multiple options to choose from\n\
                 ✓ Speed is important\n\
                 ✓ You'll decide which links to explore\n\n\
                 Use browser_research when:\n\
                 ✓ You need comprehensive understanding\n\
                 ✓ You want synthesized information\n\
                 ✓ You're researching a complex topic\n\
                 ✓ You need information from multiple sources\n\
                 ✓ Depth is more important than speed\n\
                 ✓ You want a detailed report with citations\n\n\
                 Use scrape_url when:\n\
                 ✓ You already have the URL\n\
                 ✓ You need content from specific page\n\
                 ✓ No search is needed\n\
                 ✓ You want full page content\n\
                 ✓ You're extracting specific information\n\n\
                 Use browser_navigate when:\n\
                 ✓ You need to interact with a website\n\
                 ✓ You need to click buttons or fill forms\n\
                 ✓ The content is behind interactions\n\
                 ✓ You need to navigate through multiple pages\n\
                 ✓ You're automating web workflows\n\n\
                 =============================================================================\n\
                 COMBINATION STRATEGIES\n\
                 =============================================================================\n\n\
                 Strategy 1: SEARCH → SCRAPE\n\
                 Best for: Finding and reading specific content\n\
                 1. browser_web_search - find relevant URLs\n\
                 2. scrape_url - get content from best URLs\n\n\
                 Strategy 2: RESEARCH → SEARCH → SCRAPE\n\
                 Best for: Deep dive on specific aspects\n\
                 1. browser_research - get overview\n\
                 2. browser_web_search - find specific details\n\
                 3. scrape_url - read detailed content\n\n\
                 Strategy 3: SEARCH → NAVIGATE → INTERACT\n\
                 Best for: Finding then interacting with sites\n\
                 1. browser_web_search - find the right site\n\
                 2. browser_navigate - go to the site\n\
                 3. browser_click/type - interact as needed\n\n\
                 REMEMBER:\n\
                 - browser_web_search: Fast, gives you options\n\
                 - browser_research: Slow, gives you answers\n\
                 - scrape_url: Fast, gives you content (need URL)\n\
                 - browser_navigate: Interactive, gives you control\n\
                 - web_search: Alternative to browser_web_search\n\n\
                 Choose based on: Do you need links, content, understanding, or interaction?",
            ),
        },
    ]
}
