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

fn prompt_programming() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I effectively search for programming content using browser_web_search?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Browser_web_search is excellent for finding programming resources. Here are effective patterns for common programming search tasks.\n\n\
                 SEARCHING FOR PROGRAMMING CONTENT:\n\n\
                 1. Documentation search:\n\
                    browser_web_search({\"query\": \"tokio spawn async task docs.rs\"})\n\
                    browser_web_search({\"query\": \"serde deserialize derive documentation\"})\n\
                    browser_web_search({\"query\": \"actix-web middleware docs\"})\n\
                    browser_web_search({\"query\": \"std::fs::File rust documentation\"})\n\n\
                 2. Error message search:\n\
                    browser_web_search({\"query\": \"rust error E0382 cannot move out of\"})\n\
                    browser_web_search({\"query\": \"borrow of moved value rust fix\"})\n\
                    browser_web_search({\"query\": \"lifetime annotation may not live long enough\"})\n\
                    browser_web_search({\"query\": \"cannot infer type for type parameter\"})\n\n\
                 3. Code example search:\n\
                    browser_web_search({\"query\": \"rust async file read example\"})\n\
                    browser_web_search({\"query\": \"actix-web middleware example\"})\n\
                    browser_web_search({\"query\": \"tokio select macro example code\"})\n\
                    browser_web_search({\"query\": \"serde custom deserialize example\"})\n\n\
                 4. Library/crate search:\n\
                    browser_web_search({\"query\": \"rust crate json parsing\"})\n\
                    browser_web_search({\"query\": \"best rust http client library 2024\"})\n\
                    browser_web_search({\"query\": \"rust async database driver\"})\n\
                    browser_web_search({\"query\": \"rust web framework comparison\"})\n\n\
                 5. Stack Overflow specific:\n\
                    browser_web_search({\"query\": \"rust lifetime annotation site:stackoverflow.com\"})\n\
                    browser_web_search({\"query\": \"tokio runtime panic site:stackoverflow.com\"})\n\
                    browser_web_search({\"query\": \"async await syntax site:stackoverflow.com\"})\n\n\
                 6. GitHub code search:\n\
                    browser_web_search({\"query\": \"tokio::spawn example site:github.com\"})\n\
                    browser_web_search({\"query\": \"actix-web application site:github.com\"})\n\
                    browser_web_search({\"query\": '\"impl Future\" site:github.com language:rust'})\n\n\
                 7. API reference search:\n\
                    browser_web_search({\"query\": \"std::collections::HashMap api\"})\n\
                    browser_web_search({\"query\": \"tokio::sync::Mutex methods\"})\n\
                    browser_web_search({\"query\": \"serde_json::Value api reference\"})\n\n\
                 8. Tutorial and guide search:\n\
                    browser_web_search({\"query\": \"rust async programming tutorial 2024\"})\n\
                    browser_web_search({\"query\": \"tokio getting started guide\"})\n\
                    browser_web_search({\"query\": \"rust web development beginner tutorial\"})\n\n\
                 9. Best practices search:\n\
                    browser_web_search({\"query\": \"rust async best practices\"})\n\
                    browser_web_search({\"query\": \"tokio runtime configuration best practices\"})\n\
                    browser_web_search({\"query\": \"rust error handling patterns\"})\n\n\
                 10. Version-specific search:\n\
                     browser_web_search({\"query\": \"tokio 1.0 migration guide\"})\n\
                     browser_web_search({\"query\": \"rust edition 2021 changes\"})\n\
                     browser_web_search({\"query\": \"actix-web 4.0 breaking changes\"})\n\n\
                 EFFECTIVE PROGRAMMING QUERIES:\n\n\
                 Include language name:\n\
                 - \"rust async programming\" not \"async programming\"\n\
                 - \"python pandas dataframe\" not \"pandas dataframe\"\n\
                 - \"typescript interface\" not \"interface\"\n\n\
                 Include framework/library:\n\
                 - \"tokio spawn task\" not \"spawn task\"\n\
                 - \"actix-web middleware\" not \"middleware\"\n\
                 - \"serde deserialize\" not \"deserialize\"\n\n\
                 Include year for current info:\n\
                 - \"rust web frameworks 2024\"\n\
                 - \"best practices 2024\"\n\
                 - \"tutorial 2025\"\n\n\
                 Include \"example\" or \"tutorial\" for learning:\n\
                 - \"tokio spawn example\"\n\
                 - \"rust async tutorial\"\n\
                 - \"actix-web getting started\"\n\n\
                 Include error codes for troubleshooting:\n\
                 - \"rust error E0382\"\n\
                 - \"error E0597 borrowed value\"\n\
                 - \"panic at 'index out of bounds'\"\n\n\
                 PROGRAMMING SEARCH PATTERNS:\n\n\
                 Pattern: \"how to X in Y\"\n\
                 browser_web_search({\"query\": \"how to parse JSON in rust\"})\n\
                 browser_web_search({\"query\": \"how to create async function in rust\"})\n\n\
                 Pattern: \"X vs Y\"\n\
                 browser_web_search({\"query\": \"tokio vs async-std\"})\n\
                 browser_web_search({\"query\": \"actix-web vs axum performance\"})\n\n\
                 Pattern: \"X error Y\"\n\
                 browser_web_search({\"query\": \"rust borrow checker error\"})\n\
                 browser_web_search({\"query\": \"tokio runtime error cannot start\"})\n\n\
                 Pattern: \"X example Y\"\n\
                 browser_web_search({\"query\": \"rust async file read example\"})\n\
                 browser_web_search({\"query\": \"tokio tcp server example\"})\n\n\
                 Pattern: \"best X for Y\"\n\
                 browser_web_search({\"query\": \"best rust web framework for api\"})\n\
                 browser_web_search({\"query\": \"best database driver for rust\"})\n\n\
                 SITE-SPECIFIC PROGRAMMING SEARCHES:\n\n\
                 Official documentation:\n\
                 browser_web_search({\"query\": \"site:doc.rust-lang.org Iterator\"})\n\
                 browser_web_search({\"query\": \"site:docs.rs tokio\"})\n\n\
                 Community Q&A:\n\
                 browser_web_search({\"query\": \"site:stackoverflow.com [rust] async\"})\n\
                 browser_web_search({\"query\": \"site:users.rust-lang.org borrow checker\"})\n\n\
                 Code repositories:\n\
                 browser_web_search({\"query\": \"site:github.com tokio example\"})\n\
                 browser_web_search({\"query\": \"site:gitlab.com rust project\"})\n\n\
                 Crate registry:\n\
                 browser_web_search({\"query\": \"site:crates.io async http\"})\n\n\
                 Blog posts and articles:\n\
                 browser_web_search({\"query\": \"inurl:blog rust async\"})\n\
                 browser_web_search({\"query\": \"inurl:article tokio tutorial\"})\n\n\
                 COMMON PROGRAMMING SEARCH WORKFLOWS:\n\n\
                 Learning a new library:\n\
                 1. browser_web_search({\"query\": \"tokio documentation\"})\n\
                 2. browser_web_search({\"query\": \"tokio getting started tutorial\"})\n\
                 3. browser_web_search({\"query\": \"tokio examples site:github.com\"})\n\n\
                 Debugging an error:\n\
                 1. browser_web_search({\"query\": \"rust error E0382\"})\n\
                 2. browser_web_search({\"query\": \"cannot move out of borrowed content site:stackoverflow.com\"})\n\
                 3. browser_web_search({\"query\": \"rust borrow checker fix\"})\n\n\
                 Finding the right library:\n\
                 1. browser_web_search({\"query\": \"rust http client library 2024\"})\n\
                 2. browser_web_search({\"query\": \"reqwest vs ureq comparison\"})\n\
                 3. browser_web_search({\"query\": \"reqwest documentation\"})\n\n\
                 Understanding a concept:\n\
                 1. browser_web_search({\"query\": \"rust async await explanation\"})\n\
                 2. browser_web_search({\"query\": \"async await tutorial rust\"})\n\
                 3. browser_web_search({\"query\": \"async programming best practices rust\"})\n\n\
                 TIPS FOR PROGRAMMING SEARCHES:\n\
                 - Always include the programming language name\n\
                 - Be specific with library/framework names\n\
                 - Include version numbers when relevant\n\
                 - Use site: to target authoritative sources\n\
                 - Include year for current best practices\n\
                 - Copy exact error messages when debugging\n\
                 - Use \"example\" to find code samples\n\
                 - Use \"tutorial\" for step-by-step guides\n\
                 - Combine language + framework + concept for precision",
            ),
        },
    ]
}

/// Comparison with other search tools
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using browser_web_search effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The browser_web_search tool performs web searches and returns structured results. It's fast and ideal for discovering relevant URLs.\n\n\
                 =============================================================================\n\
                 WHAT IS browser_web_search?\n\
                 =============================================================================\n\n\
                 PURPOSE:\n\
                 - Searches the web for your query\n\
                 - Returns structured results (title, URL, snippet)\n\
                 - Fast - returns immediately with search results\n\
                 - Unlike browser_research which crawls pages, this just returns search results\n\n\
                 WHEN TO USE:\n\
                 - Finding documentation links\n\
                 - Discovering relevant pages\n\
                 - Getting list of resources\n\
                 - Quick URL lookup\n\
                 - Initial discovery phase\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 Simple query:\n\
                 browser_web_search({\"query\": \"rust async programming\"})\n\n\
                 Question format:\n\
                 browser_web_search({\"query\": \"how to implement linked list in rust\"})\n\n\
                 Specific topic:\n\
                 browser_web_search({\"query\": \"tokio vs async-std comparison\"})\n\n\
                 Documentation lookup:\n\
                 browser_web_search({\"query\": \"serde deserialize documentation\"})\n\n\
                 Library search:\n\
                 browser_web_search({\"query\": \"best rust http client 2024\"})\n\n\
                 RESPONSE STRUCTURE:\n\
                 [\n\
                   {\n\
                     \"title\": \"Page Title\",\n\
                     \"url\": \"https://example.com/page\",\n\
                     \"snippet\": \"Brief description from search result...\"\n\
                   },\n\
                   ...\n\
                 ]\n\n\
                 =============================================================================\n\
                 ADVANCED SEARCH OPERATORS\n\
                 =============================================================================\n\n\
                 SITE-SPECIFIC SEARCH (site:):\n\
                 browser_web_search({\"query\": \"async await site:rust-lang.org\"})\n\
                 browser_web_search({\"query\": \"error handling site:stackoverflow.com\"})\n\
                 browser_web_search({\"query\": \"tokio tutorial site:docs.rs\"})\n\n\
                 EXACT PHRASE (\"quotes\"):\n\
                 browser_web_search({\"query\": '\"borrow checker\" rust'})\n\
                 browser_web_search({\"query\": '\"async fn\" syntax'})\n\
                 browser_web_search({\"query\": '\"impl Trait\" return type'})\n\n\
                 EXCLUDE TERMS (-minus):\n\
                 browser_web_search({\"query\": \"rust programming -gaming\"})\n\
                 browser_web_search({\"query\": \"python tutorial -django -flask\"})\n\n\
                 FILE TYPE (filetype:):\n\
                 browser_web_search({\"query\": \"rust cheat sheet filetype:pdf\"})\n\
                 browser_web_search({\"query\": \"api documentation filetype:md\"})\n\n\
                 COMBINE WITH OR:\n\
                 browser_web_search({\"query\": \"tokio tutorial site:github.com OR site:docs.rs\"})\n\
                 browser_web_search({\"query\": \"rust async OR futures\"})\n\n\
                 TITLE SEARCH (intitle:):\n\
                 browser_web_search({\"query\": \"intitle:rust intitle:tutorial async\"})\n\
                 browser_web_search({\"query\": \"intitle:guide tokio\"})\n\n\
                 URL SEARCH (inurl:):\n\
                 browser_web_search({\"query\": \"inurl:blog rust async\"})\n\
                 browser_web_search({\"query\": \"inurl:tutorial web framework\"})\n\n\
                 OPERATOR REFERENCE:\n\
                 - site:domain.com - Search within specific site\n\
                 - \"exact phrase\" - Match exact phrase  \n\
                 - -term - Exclude term from results\n\
                 - filetype:ext - Specific file type\n\
                 - OR - Either term (must be uppercase)\n\
                 - intitle:term - Term must be in title\n\
                 - inurl:term - Term must be in URL\n\n\
                 =============================================================================\n\
                 PROGRAMMING SEARCH PATTERNS\n\
                 =============================================================================\n\n\
                 DOCUMENTATION SEARCH:\n\
                 browser_web_search({\"query\": \"tokio spawn async task docs.rs\"})\n\
                 browser_web_search({\"query\": \"serde deserialize derive documentation\"})\n\
                 browser_web_search({\"query\": \"actix-web middleware docs\"})\n\n\
                 ERROR MESSAGE SEARCH:\n\
                 browser_web_search({\"query\": \"rust error E0382 cannot move out of\"})\n\
                 browser_web_search({\"query\": \"borrow of moved value rust fix\"})\n\
                 browser_web_search({\"query\": \"lifetime annotation may not live long enough\"})\n\n\
                 CODE EXAMPLE SEARCH:\n\
                 browser_web_search({\"query\": \"rust async file read example\"})\n\
                 browser_web_search({\"query\": \"actix-web middleware example\"})\n\
                 browser_web_search({\"query\": \"tokio select macro example code\"})\n\n\
                 LIBRARY/CRATE SEARCH:\n\
                 browser_web_search({\"query\": \"rust crate json parsing\"})\n\
                 browser_web_search({\"query\": \"best rust http client library 2024\"})\n\
                 browser_web_search({\"query\": \"rust async database driver\"})\n\n\
                 STACK OVERFLOW:\n\
                 browser_web_search({\"query\": \"rust lifetime annotation site:stackoverflow.com\"})\n\
                 browser_web_search({\"query\": \"tokio runtime panic site:stackoverflow.com\"})\n\n\
                 GITHUB CODE SEARCH:\n\
                 browser_web_search({\"query\": \"tokio::spawn example site:github.com\"})\n\
                 browser_web_search({\"query\": \"actix-web application site:github.com\"})\n\n\
                 EFFECTIVE PROGRAMMING QUERIES:\n\
                 - Include language name: \"rust\", \"python\", \"typescript\"\n\
                 - Include framework/library: \"tokio\", \"actix\", \"serde\"\n\
                 - Include year for current info: \"2024\"\n\
                 - Include \"example\" or \"tutorial\" for learning\n\
                 - Include error codes for troubleshooting\n\n\
                 =============================================================================\n\
                 COMPARISON WITH OTHER TOOLS\n\
                 =============================================================================\n\n\
                 browser_web_search:\n\
                 - Quick lookup of URLs\n\
                 - Fast, returns just search results\n\
                 - Good for: \"I need to find pages about X\"\n\
                 - Speed: Seconds\n\
                 - Depth: Surface (just links)\n\n\
                 browser_research:\n\
                 - Deep research on topic\n\
                 - Synthesized information from multiple pages\n\
                 - Good for: \"I need to understand X\"\n\
                 - Speed: Minutes\n\
                 - Depth: Comprehensive (full analysis)\n\n\
                 web_search:\n\
                 - Similar to browser_web_search\n\
                 - May have different search engine\n\
                 - Check which is available\n\n\
                 scrape_url:\n\
                 - When you already know the URL\n\
                 - Direct content extraction\n\
                 - Good for: \"I need content from this specific URL\"\n\
                 - Speed: Seconds\n\
                 - Depth: Single page (full content)\n\n\
                 DECISION FLOW:\n\
                 1. Know exact URL? → scrape_url or browser_navigate\n\
                 2. Need quick links? → browser_web_search\n\
                 3. Need synthesized research? → browser_research\n\
                 4. Need to interact with page? → browser_navigate + browser_click/type\n\n\
                 =============================================================================\n\
                 QUERY FORMULATION TIPS\n\
                 =============================================================================\n\n\
                 BE SPECIFIC:\n\
                 ✓ \"tokio spawn task example\"\n\
                 ✗ \"tokio\"\n\n\
                 INCLUDE LANGUAGE:\n\
                 ✓ \"rust error handling\"\n\
                 ✗ \"error handling\"\n\n\
                 INCLUDE YEAR:\n\
                 ✓ \"rust web frameworks 2024\"\n\
                 ✗ \"rust web frameworks\"\n\n\
                 USE NATURAL QUESTIONS:\n\
                 ✓ \"how to parse JSON in rust\"\n\
                 ✓ \"what is tokio runtime\"\n\n\
                 COMBINE KEY TERMS:\n\
                 ✓ \"actix-web middleware tutorial\"\n\
                 ✓ \"rust async file operations\"\n\n\
                 =============================================================================\n\
                 COMMON USE CASES\n\
                 =============================================================================\n\n\
                 Finding documentation:\n\
                 browser_web_search({\"query\": \"tokio runtime documentation\"})\n\
                 → Get official docs URL\n\n\
                 Looking up error messages:\n\
                 browser_web_search({\"query\": \"rust error E0382 cannot move\"})\n\
                 → Find solutions and explanations\n\n\
                 Searching for code examples:\n\
                 browser_web_search({\"query\": \"rust async file read example\"})\n\
                 → Discover example code and tutorials\n\n\
                 Finding tutorials:\n\
                 browser_web_search({\"query\": \"rust web scraping tutorial\"})\n\
                 → Locate learning resources\n\n\
                 Discovering libraries:\n\
                 browser_web_search({\"query\": \"rust crate json parsing\"})\n\
                 → Find suitable libraries\n\n\
                 Looking up API references:\n\
                 browser_web_search({\"query\": \"std::collections::HashMap api\"})\n\
                 → Access API documentation\n\n\
                 =============================================================================\n\
                 TYPICAL WORKFLOWS\n\
                 =============================================================================\n\n\
                 WORKFLOW 1: DISCOVERY → EXPLORATION\n\
                 1. Search for topic:\n\
                    browser_web_search({\"query\": \"rust web frameworks comparison 2024\"})\n\
                 2. Review result titles and snippets\n\
                 3. Navigate to promising URLs:\n\
                    scrape_url({\"url\": \"https://example.com/best-result\"})\n\n\
                 WORKFLOW 2: LEARNING A NEW LIBRARY\n\
                 1. Find documentation:\n\
                    browser_web_search({\"query\": \"tokio documentation\"})\n\
                 2. Find tutorials:\n\
                    browser_web_search({\"query\": \"tokio getting started tutorial\"})\n\
                 3. Find examples:\n\
                    browser_web_search({\"query\": \"tokio examples site:github.com\"})\n\n\
                 WORKFLOW 3: DEBUGGING AN ERROR\n\
                 1. Search error code:\n\
                    browser_web_search({\"query\": \"rust error E0382\"})\n\
                 2. Search error message:\n\
                    browser_web_search({\"query\": \"cannot move out of borrowed content site:stackoverflow.com\"})\n\
                 3. Search for fix:\n\
                    browser_web_search({\"query\": \"rust borrow checker fix\"})\n\n\
                 WORKFLOW 4: FINDING THE RIGHT LIBRARY\n\
                 1. Search for category:\n\
                    browser_web_search({\"query\": \"rust http client library 2024\"})\n\
                 2. Compare options:\n\
                    browser_web_search({\"query\": \"reqwest vs ureq comparison\"})\n\
                 3. Get documentation:\n\
                    browser_web_search({\"query\": \"reqwest documentation\"})\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 ✓ DO:\n\
                 - Be specific with your queries\n\
                 - Include language/framework names\n\
                 - Use search operators for precision\n\
                 - Include year for current information\n\
                 - Use site: for authoritative sources\n\
                 - Review titles and snippets before diving deeper\n\
                 - Combine with scrape_url for full content\n\n\
                 ✗ DON'T:\n\
                 - Use overly generic queries\n\
                 - Forget to include language name\n\
                 - Ignore search operators\n\
                 - Expect full page content (use scrape_url for that)\n\
                 - Use when you need deep research (use browser_research)\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Basic search:\n\
                 browser_web_search({\"query\": \"your search terms\"})\n\n\
                 Site-specific:\n\
                 browser_web_search({\"query\": \"topic site:example.com\"})\n\n\
                 Exact phrase:\n\
                 browser_web_search({\"query\": '\"exact phrase\" context'})\n\n\
                 Exclude terms:\n\
                 browser_web_search({\"query\": \"include -exclude\"})\n\n\
                 File type:\n\
                 browser_web_search({\"query\": \"topic filetype:pdf\"})\n\n\
                 Combined:\n\
                 browser_web_search({\"query\": 'site:docs.rs \"tokio\" tutorial'})\n\n\
                 Remember: browser_web_search is for FINDING pages, not reading them. Use scrape_url or browser_navigate to get actual content!",
            ),
        },
    ]
}
