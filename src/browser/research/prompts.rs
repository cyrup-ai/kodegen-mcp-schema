//! Prompt messages for browser_research tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::BrowserResearchPromptArgs;

/// Prompt provider for browser_research tool
///
/// This is the ONLY way to provide prompts for browser_research - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ResearchPrompts;

impl PromptProvider for ResearchPrompts {
    type PromptArgs = BrowserResearchPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("deep_research") => prompt_deep_research(),
            Some("technical_docs") => prompt_technical_docs(),
            Some("comparison") => prompt_comparison(),
            Some("monitoring") => prompt_monitoring(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, deep_research, technical_docs, comparison, monitoring)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE BROWSER_RESEARCH
// ============================================================================

/// Basic research queries and when to use them
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use browser_research for basic web research?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The browser_research tool conducts autonomous web research, searching multiple pages and synthesizing findings. Here's how to use it for basic research:\n\n\
                 BASIC RESEARCH QUERIES:\n\n\
                 1. Simple topic research:\n\
                    browser_research({\"action\": \"RESEARCH\", \"query\": \"Rust async runtime comparison tokio vs async-std\", \"max_pages\": 5})\n\n\
                 2. Quick answer lookup:\n\
                    browser_research({\"action\": \"RESEARCH\", \"query\": \"how to fix rust borrow checker error E0382\", \"max_pages\": 3, \"max_depth\": 1})\n\n\
                 3. Technology overview:\n\
                    browser_research({\"action\": \"RESEARCH\", \"query\": \"MCP model context protocol architecture overview\"})\n\n\
                 WHEN TO USE BROWSER_RESEARCH:\n\
                 - Need synthesized information from multiple sources\n\
                 - Research questions requiring context from several pages\n\
                 - Technical topics with scattered documentation\n\
                 - Comparative analysis needs\n\
                 - Want AI-generated summary of findings\n\n\
                 WHEN TO USE SIMPLER TOOLS:\n\
                 - Single page lookup: browser_navigate + browser_extract_text\n\
                 - Known URL: scrape_url\n\
                 - Quick search: web_search (just returns links)\n\
                 - Need full control: Use browser tools directly\n\n\
                 DEFAULT PARAMETERS:\n\
                 - max_pages: 5 (total pages to visit)\n\
                 - max_depth: 2 (how many link levels to follow)\n\
                 - search_engine: \"google\"\n\
                 - extract_tables: true\n\
                 - include_links: true\n\
                 - await_completion_ms: 300000 (5 minutes, then timeout)\n\n\
                 BASIC WORKFLOW:\n\
                 1. Start research:\n\
                    browser_research({\"action\": \"RESEARCH\", \"query\": \"your topic\"})\n\
                 2. Returns session info with initial progress\n\
                 3. If await_completion_ms not set to 0, waits for completion\n\
                 4. Returns comprehensive report with sources\n\n\
                 RESPONSE INCLUDES:\n\
                 - Synthesized summary of findings\n\
                 - Source URLs for verification\n\
                 - Extracted content from multiple pages\n\
                 - Progress status (pages visited, time elapsed)\n\n\
                 PRACTICAL EXAMPLES:\n\n\
                 Quick overview (3 pages, shallow):\n\
                 browser_research({\"action\": \"RESEARCH\", \"query\": \"WebAssembly use cases\", \"max_pages\": 3, \"max_depth\": 1})\n\n\
                 Standard research (default settings):\n\
                 browser_research({\"action\": \"RESEARCH\", \"query\": \"Rust web framework ecosystem 2024\"})\n\n\
                 Focused topic:\n\
                 browser_research({\"action\": \"RESEARCH\", \"query\": \"implementing OAuth2 in Rust actix-web\", \"max_pages\": 5})",
            ),
        },
    ]
}

/// Deep research with multi-page crawling
fn prompt_deep_research() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I conduct deep, comprehensive research on a complex topic?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "For comprehensive research, tune max_pages and max_depth to explore more content and follow links deeper:\n\n\
                 DEEP RESEARCH CONFIGURATION:\n\n\
                 1. Comprehensive topic research:\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"implementing distributed consensus algorithms in Rust\",\n\
                      \"max_pages\": 15,\n\
                      \"max_depth\": 3,\n\
                      \"extract_tables\": true,\n\
                      \"include_links\": true\n\
                    })\n\n\
                 2. Academic/technical deep dive:\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"CRDT conflict-free replicated data types implementation\",\n\
                      \"max_pages\": 20,\n\
                      \"max_depth\": 3,\n\
                      \"timeout_seconds\": 120\n\
                    })\n\n\
                 3. Exhaustive topic exploration:\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"Rust memory model and unsafe abstractions\",\n\
                      \"max_pages\": 15,\n\
                      \"max_depth\": 2,\n\
                      \"extract_tables\": true,\n\
                      \"extract_images\": true\n\
                    })\n\n\
                 PARAMETERS FOR DEPTH:\n\
                 - max_pages: Total pages to visit (default: 5, increase for depth)\n\
                 - max_depth: Link-following depth from search results (default: 2)\n\
                 - timeout_seconds: Per-page timeout (default: 60)\n\
                 - await_completion_ms: Total research timeout (default: 300000 = 5 min)\n\n\
                 DEPTH VS BREADTH TRADEOFFS:\n\
                 - max_pages: 5, max_depth: 1 = Broad, shallow (quick overview)\n\
                 - max_pages: 3, max_depth: 3 = Narrow, deep (detailed single path)\n\
                 - max_pages: 15, max_depth: 2 = Balanced comprehensive (recommended)\n\
                 - max_pages: 20, max_depth: 3 = Exhaustive (may timeout, use monitoring)\n\n\
                 HOW DEPTH WORKS:\n\
                 - Depth 1: Only visit search result pages\n\
                 - Depth 2: Visit search results + linked pages from those results\n\
                 - Depth 3: Visit search results + linked pages + links from those pages\n\
                 - Higher depth = exponentially more pages, better coverage, longer time\n\n\
                 EXTRACTION OPTIONS:\n\
                 - extract_tables: true = Parse HTML tables (good for benchmarks, comparisons)\n\
                 - include_links: true = Capture hyperlinks (good for finding related resources)\n\
                 - extract_images: true = Get image URLs and alt text (usually not needed)\n\n\
                 TIMEOUT MANAGEMENT:\n\
                 Deep research takes time. Use await_completion_ms to control:\n\
                 - Default (5 min): Good for 5-10 pages\n\
                 - 600000 (10 min): Good for 10-20 pages\n\
                 - 0 (fire-and-forget): Start research, check later with READ\n\n\
                 MONITORING LONG RESEARCH:\n\
                 For very deep research (15+ pages, depth 3):\n\
                 1. Start with await_completion_ms: 0\n\
                 2. Check progress with READ action\n\
                 3. Wait for completion\n\
                 (See monitoring scenario for details)\n\n\
                 BEST PRACTICES:\n\
                 - Start with max_pages: 10, max_depth: 2 for balanced research\n\
                 - Increase max_pages for more sources (breadth)\n\
                 - Increase max_depth for deeper exploration (depth)\n\
                 - Use extract_tables for technical/benchmark content\n\
                 - Set longer timeout_seconds for slow pages\n\
                 - Use monitoring pattern for exhaustive research",
            ),
        },
    ]
}

/// Researching technical documentation
fn prompt_technical_docs() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use browser_research to find and understand technical documentation?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Browser_research is excellent for gathering technical documentation from multiple sources and synthesizing it:\n\n\
                 RESEARCHING TECHNICAL DOCUMENTATION:\n\n\
                 1. API documentation:\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"tokio runtime spawn async task documentation\",\n\
                      \"search_engine\": \"google\",\n\
                      \"extract_tables\": true\n\
                    })\n\n\
                 2. Framework guides:\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"actix-web middleware authentication example\",\n\
                      \"max_pages\": 8\n\
                    })\n\n\
                 3. Error troubleshooting:\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"rust lifetime annotation expected named lifetime parameter\",\n\
                      \"max_pages\": 10\n\
                    })\n\n\
                 4. Library usage examples:\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"serde derive custom serialization Rust examples\",\n\
                      \"max_pages\": 8,\n\
                      \"include_links\": true\n\
                    })\n\n\
                 SEARCH ENGINE CHOICE:\n\
                 - \"google\": Best for general technical content (default)\n\
                 - \"bing\": Good alternative, sometimes different results\n\
                 - \"duckduckgo\": Privacy-focused, less personalized\n\n\
                 DOCUMENTATION EXTRACTION:\n\
                 - extract_tables: true = Capture API reference tables, parameter lists\n\
                 - include_links: true = Get links to related docs, examples, GitHub repos\n\
                 - extract_images: false = Usually not needed for docs (diagrams are rare)\n\n\
                 EFFECTIVE DOC QUERIES:\n\
                 - Include the technology name: \"tokio\", \"actix-web\", \"serde\"\n\
                 - Add specific feature: \"spawn\", \"middleware\", \"derive\"\n\
                 - Include context: \"documentation\", \"example\", \"tutorial\"\n\
                 - Be specific: \"tokio::spawn\" better than \"spawning tasks\"\n\n\
                 ERROR DOCUMENTATION:\n\
                 For compiler errors or runtime issues:\n\
                 - Include error code: \"E0382\", \"E0502\"\n\
                 - Include error message keywords: \"lifetime annotation expected\"\n\
                 - Add language: \"rust\" (helps filter results)\n\n\
                 FRAMEWORK PATTERNS:\n\
                 When learning a framework:\n\
                 1. Architecture overview:\n\
                    browser_research({\"query\": \"actix-web architecture application state\", \"max_pages\": 6})\n\
                 2. Specific feature:\n\
                    browser_research({\"query\": \"actix-web middleware request guards\", \"max_pages\": 5})\n\
                 3. Common patterns:\n\
                    browser_research({\"query\": \"actix-web best practices error handling\", \"max_pages\": 8})\n\n\
                 LIBRARY RESEARCH:\n\
                 Understanding a library:\n\
                 - Overview: \"library_name getting started tutorial\"\n\
                 - Features: \"library_name feature_name usage\"\n\
                 - Examples: \"library_name real world examples\"\n\
                 - Troubleshooting: \"library_name common issues\"\n\n\
                 BEST PRACTICES:\n\
                 - Use 6-10 pages for thorough doc coverage\n\
                 - Enable extract_tables for API references\n\
                 - Include library/framework name in query\n\
                 - Add \"example\" or \"tutorial\" for practical content\n\
                 - Use google for most complete results\n\
                 - Check include_links to find official repos and related resources",
            ),
        },
    ]
}

/// Comparative research and analysis
fn prompt_comparison() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I research and compare multiple technologies or solutions?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Browser_research excels at comparative analysis by gathering multiple perspectives and synthesizing comparisons:\n\n\
                 COMPARATIVE RESEARCH:\n\n\
                 1. Technology comparison:\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"PostgreSQL vs MySQL vs SQLite performance comparison 2024\",\n\
                      \"max_pages\": 12\n\
                    })\n\n\
                 2. Library comparison:\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"serde vs rkyv serialization benchmark Rust\",\n\
                      \"max_pages\": 10,\n\
                      \"extract_tables\": true\n\
                    })\n\n\
                 3. Framework comparison:\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"Axum vs Actix-web vs Rocket framework comparison features\",\n\
                      \"max_pages\": 15,\n\
                      \"extract_tables\": true\n\
                    })\n\n\
                 4. Approach comparison:\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"async runtime tokio vs async-std ecosystem 2024\",\n\
                      \"max_pages\": 12\n\
                    })\n\n\
                 EFFECTIVE COMPARISON QUERIES:\n\
                 - Use \"vs\" or \"versus\": \"technology1 vs technology2 vs technology3\"\n\
                 - Add \"comparison\": Makes intent clear to search engines\n\
                 - Include year: \"2024\" or \"2025\" for current information\n\
                 - Add specific aspects: \"performance\", \"features\", \"ecosystem\", \"benchmarks\"\n\
                 - Be specific: \"serialization benchmark\" not just \"comparison\"\n\n\
                 COMPARISON DIMENSIONS:\n\
                 Performance comparison:\n\
                 browser_research({\"query\": \"database performance benchmark PostgreSQL MySQL 2024\", \"extract_tables\": true})\n\n\
                 Feature comparison:\n\
                 browser_research({\"query\": \"Rust web framework features comparison Axum Actix Rocket\", \"max_pages\": 12})\n\n\
                 Ecosystem comparison:\n\
                 browser_research({\"query\": \"React vs Vue ecosystem library support 2024\", \"max_pages\": 10})\n\n\
                 Use case comparison:\n\
                 browser_research({\"query\": \"when to use SQLite vs PostgreSQL use cases\", \"max_pages\": 8})\n\n\
                 SYNTHESIS FEATURES:\n\
                 The research output synthesizes:\n\
                 - Key differentiators from multiple sources\n\
                 - Performance benchmarks if available (especially with extract_tables: true)\n\
                 - Community sentiment and adoption trends\n\
                 - Use case recommendations from various experts\n\
                 - Pros and cons from different perspectives\n\n\
                 TABLE EXTRACTION:\n\
                 Enable extract_tables for comparisons:\n\
                 - Benchmark tables (performance numbers)\n\
                 - Feature matrices (which supports what)\n\
                 - Compatibility tables (version support)\n\
                 - Pricing tables (for commercial tools)\n\n\
                 PAGE COUNT GUIDELINES:\n\
                 - 2 technologies: 8-10 pages\n\
                 - 3 technologies: 12-15 pages\n\
                 - 4+ technologies: 15-20 pages\n\
                 More pages = more perspectives = better synthesis\n\n\
                 YEAR QUALIFIERS:\n\
                 Always include current year in queries:\n\
                 - Technology evolves rapidly\n\
                 - \"2024\" or \"2025\" ensures recent information\n\
                 - Filters out outdated comparisons\n\
                 - Gets latest benchmarks and features\n\n\
                 DECISION-MAKING QUERIES:\n\
                 Help choose between options:\n\
                 browser_research({\"query\": \"should I use Axum or Actix-web for REST API\", \"max_pages\": 10})\n\
                 browser_research({\"query\": \"when to choose async-std over tokio\", \"max_pages\": 8})\n\
                 browser_research({\"query\": \"PostgreSQL vs MySQL for read-heavy application\", \"max_pages\": 12})\n\n\
                 BEST PRACTICES:\n\
                 - Use 10-15 pages for thorough comparison coverage\n\
                 - Enable extract_tables for benchmark data\n\
                 - Include year for current information\n\
                 - Use specific aspect keywords (performance, features, ecosystem)\n\
                 - List all items being compared in query\n\
                 - Review sources to understand different perspectives",
            ),
        },
    ]
}

/// Managing long-running research sessions
fn prompt_monitoring() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I manage long-running research that might take a while to complete?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "For extensive research, use the monitoring pattern with background execution and progress checking:\n\n\
                 MANAGING LONG-RUNNING RESEARCH:\n\n\
                 1. Start research (fire-and-forget):\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"comprehensive guide to Rust memory safety\",\n\
                      \"max_pages\": 20,\n\
                      \"await_completion_ms\": 0\n\
                    })\n\
                    Returns immediately with session info, research runs in background\n\n\
                 2. Check progress:\n\
                    browser_research({\n\
                      \"action\": \"READ\",\n\
                      \"session\": 0\n\
                    })\n\
                    Shows: pages visited, content extracted, time elapsed, completion status\n\n\
                 3. List all research sessions:\n\
                    browser_research({\"action\": \"LIST\"})\n\
                    Returns array of all active sessions with their status\n\n\
                 4. Kill stuck or unwanted session:\n\
                    browser_research({\n\
                      \"action\": \"KILL\",\n\
                      \"session\": 0\n\
                    })\n\
                    Gracefully terminates research and cleans up resources\n\n\
                 TIMEOUT STRATEGIES:\n\
                 - await_completion_ms: 0 → Fire-and-forget, use READ to check progress\n\
                 - await_completion_ms: 300000 → Wait up to 5 minutes, return if done sooner\n\
                 - await_completion_ms: 600000 → Wait up to 10 minutes for deep research\n\
                 - await_completion_ms: default (300000) → Standard 5 minute wait\n\n\
                 TIMEOUT BEHAVIOR:\n\
                 When research times out:\n\
                 - Returns current progress snapshot\n\
                 - Research continues in background\n\
                 - Use READ to check completion later\n\
                 - Results accumulate in session\n\n\
                 MONITORING WORKFLOW:\n\
                 1. Start extensive research:\n\
                    browser_research({\n\
                      \"action\": \"RESEARCH\",\n\
                      \"query\": \"distributed systems consensus algorithms survey\",\n\
                      \"max_pages\": 20,\n\
                      \"max_depth\": 3,\n\
                      \"await_completion_ms\": 0\n\
                    })\n\
                    Response: {\"session\": 0, \"status\": \"running\", \"pages_visited\": 0}\n\n\
                 2. Do other work (research runs in background)\n\n\
                 3. Periodically check progress:\n\
                    browser_research({\"action\": \"READ\", \"session\": 0})\n\
                    Response shows progress: {\"pages_visited\": 12, \"pages_total\": 20, \"completed\": false}\n\n\
                 4. When complete, final READ shows all results:\n\
                    browser_research({\"action\": \"READ\", \"session\": 0})\n\
                    Response: {\"completed\": true, \"summary\": \"...\", \"sources\": [...]}\n\n\
                 SESSION MANAGEMENT:\n\
                 Multiple parallel research sessions:\n\
                 browser_research({\"action\": \"RESEARCH\", \"query\": \"topic1\", \"session\": 0, \"await_completion_ms\": 0})\n\
                 browser_research({\"action\": \"RESEARCH\", \"query\": \"topic2\", \"session\": 1, \"await_completion_ms\": 0})\n\
                 browser_research({\"action\": \"RESEARCH\", \"query\": \"topic3\", \"session\": 2, \"await_completion_ms\": 0})\n\
                 \n\
                 Check all sessions:\n\
                 browser_research({\"action\": \"LIST\"})\n\
                 Returns: [{\"session\": 0, \"status\": \"complete\"}, {\"session\": 1, \"status\": \"running\"}, ...]\n\n\
                 WHEN TO USE FIRE-AND-FORGET:\n\
                 - Research with 15+ pages\n\
                 - max_depth: 3 (exponential page growth)\n\
                 - Multiple parallel research tasks\n\
                 - Research while doing other work\n\
                 - Don't want to block on long operation\n\n\
                 WHEN TO USE TIMEOUT:\n\
                 - Quick research (5-10 pages)\n\
                 - Want results immediately if fast\n\
                 - Okay to wait a few minutes\n\
                 - Single research task\n\n\
                 READ ACTION RESPONSE:\n\
                 While running:\n\
                 {\n\
                   \"session\": 0,\n\
                   \"completed\": false,\n\
                   \"pages_visited\": 8,\n\
                   \"pages_remaining\": 12,\n\
                   \"time_elapsed_ms\": 45000,\n\
                   \"current_url\": \"https://...\"\n\
                 }\n\n\
                 When complete:\n\
                 {\n\
                   \"session\": 0,\n\
                   \"completed\": true,\n\
                   \"summary\": \"Research findings...\",\n\
                   \"sources\": [{\"url\": \"...\", \"title\": \"...\"}],\n\
                   \"total_pages\": 20,\n\
                   \"time_elapsed_ms\": 180000\n\
                 }\n\n\
                 CLEANUP:\n\
                 Kill sessions when done:\n\
                 browser_research({\"action\": \"KILL\", \"session\": 0})\n\
                 - Stops any running research\n\
                 - Frees resources\n\
                 - Removes session from LIST\n\n\
                 BEST PRACTICES:\n\
                 1. Use await_completion_ms: 0 for research with 15+ pages\n\
                 2. Use READ to check progress periodically\n\
                 3. Use LIST to see all active sessions\n\
                 4. KILL sessions when you have results\n\
                 5. Use different session numbers for parallel research\n\
                 6. Don't spam READ - give research time to progress",
            ),
        },
    ]
}
