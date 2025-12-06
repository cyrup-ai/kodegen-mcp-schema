//! Prompt messages for browser_agent tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::BrowserAgentPromptArgs;

/// Prompt provider for browser_agent tool
///
/// This is the ONLY way to provide prompts for browser_agent - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct AgentPrompts;

impl PromptProvider for AgentPrompts {
    type PromptArgs = BrowserAgentPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("autonomous") => prompt_autonomous(),
            Some("monitoring") => prompt_monitoring(),
            Some("management") => prompt_management(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: basic (PROMPT action), autonomous (multi-step), monitoring (READ action), management (KILL action)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE BROWSER_AGENT
// ============================================================================

/// Basic browser automation - PROMPT action for spawning tasks
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I spawn and execute browser agent tasks?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The browser_agent PROMPT action spawns autonomous browser automation tasks with AI-driven navigation and interaction. Here's how to use it:\n\n\
                 BASIC TASK SPAWNING:\n\
                 1. Simple documentation lookup:\n\
                    browser_agent({\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Navigate to docs.rs and search for 'tokio runtime'. Summarize the main runtime configuration options.\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"agent\": 0,\n\
                   \"status\": \"completed\",\n\
                   \"result\": \"Found tokio runtime documentation. Main configuration options: thread count, worker threads, blocking threads...\"\n\
                 }\n\n\
                 2. Start with specific URL:\n\
                    browser_agent({\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Extract the pricing tiers and features listed on this page as structured JSON.\",\n\
                        \"start_url\": \"https://example.com/pricing\"\n\
                    })\n\n\
                 3. Search and extraction:\n\
                    browser_agent({\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Search GitHub for 'rust mcp' and list the top 5 repositories with star counts and descriptions.\",\n\
                        \"start_url\": \"https://github.com\",\n\
                        \"max_steps\": 8\n\
                    })\n\n\
                 4. Form interaction (controlled):\n\
                    browser_agent({\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Fill the registration form with: name='Test User', email='test@example.com', password='TestPass123'. DO NOT submit the form.\",\n\
                        \"start_url\": \"https://example.com/register\"\n\
                    })\n\n\
                 5. Multi-page research:\n\
                    browser_agent({\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Navigate to the API documentation, find the authentication section, and extract all supported OAuth2 grant types.\",\n\
                        \"start_url\": \"https://docs.example.com\",\n\
                        \"max_steps\": 12,\n\
                        \"additional_info\": \"Focus only on OAuth2, skip API key methods\"\n\
                    })\n\n\
                 KEY PARAMETERS:\n\
                 - task (required): Clear description of what to accomplish\n\
                 - start_url (optional): Initial page to load (agent can navigate if omitted)\n\
                 - max_steps (optional): Maximum navigation steps (default: 10)\n\
                 - max_actions_per_step (optional): Actions per step (default: 3)\n\
                 - max_tokens (optional): Response length (default: 2048)\n\
                 - temperature (optional): Creativity level 0-1 (default: 0.7)\n\
                 - additional_info (optional): Extra context or constraints\n\
                 - await_completion_ms (optional): Timeout in ms (default: 600000 = 10 min)\n\n\
                 OUTPUT INTERPRETATION:\n\
                 - agent: Agent number (0, 1, 2... for multiple agents)\n\
                 - status: null/\"pending\" (not started), \"running\" (executing), \"completed\" (finished)\n\
                 - result: Task output when completed\n\
                 - exit_code: 0 (success), non-zero (error), null (running/pending)\n\n\
                 TASK WRITING GUIDELINES:\n\
                 - Be specific about the objective\n\
                 - Specify output format (JSON, markdown, plain text)\n\
                 - Include constraints (don't submit, don't click ads, only first 3 results)\n\
                 - Provide context in additional_info for complex scenarios\n\n\
                 Good: \"Extract the pricing table as JSON with fields: tier, price, features\"\n\
                 Bad: \"Get the pricing info\"\n\n\
                 Good: \"Search for 'rust web frameworks' and list the top 3 with descriptions\"\n\
                 Bad: \"Find some Rust frameworks\"\n\n\
                 CONFIGURATION TIPS:\n\
                 - Simple tasks: Default max_steps (10) is sufficient\n\
                 - Multi-page workflows: Increase max_steps (15-20)\n\
                 - Deep navigation: Use max_steps: 25+ for complex site traversal\n\
                 - Quick extraction: Reduce max_steps to 5 if starting at target URL\n\n\
                 TIMEOUT CONTROL:\n\
                 - Default: 600000ms (10 minutes) - waits for completion\n\
                 - Custom timeout: Set await_completion_ms to desired milliseconds\n\
                 - Fire-and-forget: Set await_completion_ms: 0 (returns immediately)\n\n\
                 When await_completion_ms is 0, use READ action to check progress (see monitoring scenario).",
            ),
        },
    ]
}

/// Autonomous multi-step navigation and complex workflows
fn prompt_autonomous() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use browser_agent for autonomous multi-step operations?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The browser_agent excels at autonomous multi-step operations using vision-based navigation and AI reasoning. Here's how to leverage its autonomous capabilities:\n\n\
                 MULTI-STEP NAVIGATION:\n\
                 The agent uses computer vision to see the page and decides which elements to interact with based on your task description.\n\n\
                 1. Multi-page form workflow:\n\
                    browser_agent({\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Navigate the multi-step checkout: fill shipping (123 Main St, New York, NY 10001), proceed to payment, stop before entering card details.\",\n\
                        \"start_url\": \"https://shop.example.com/cart\",\n\
                        \"max_steps\": 15\n\
                    })\n\n\
                 2. Research workflow with navigation chains:\n\
                    browser_agent({\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Go to Rust blog, find the most recent async/await article, read it, then navigate to the referenced RFC and extract the key motivation points.\",\n\
                        \"start_url\": \"https://blog.rust-lang.org\",\n\
                        \"max_steps\": 20,\n\
                        \"additional_info\": \"Follow breadcrumb links between blog post and RFC\"\n\
                    })\n\n\
                 3. Competitive analysis:\n\
                    browser_agent({\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Research competitors: visit their pricing pages, extract all plan tiers with features, return as structured JSON array.\",\n\
                        \"additional_info\": \"Competitors: example1.com/pricing, example2.com/pricing, example3.com/pricing\",\n\
                        \"max_steps\": 30,\n\
                        \"max_tokens\": 4096\n\
                    })\n\n\
                 4. Fallback handling:\n\
                    browser_agent({\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Search for 'machine learning courses'. If search fails, try navigation menu. Extract course titles and durations from first page.\",\n\
                        \"start_url\": \"https://university.example.com\",\n\
                        \"max_steps\": 12\n\
                    })\n\n\
                 WHEN TO USE MULTI-STEP vs SINGLE-STEP:\n\
                 - Single-step (max_steps: 3-5): Extracting data from known URL\n\
                 - Multi-step (max_steps: 10-15): Navigation + extraction, form workflows\n\
                 - Complex multi-step (max_steps: 20-30): Deep research, multi-page forms, comparison tasks\n\n\
                 AGENT REASONING:\n\
                 The agent:\n\
                 1. Takes screenshot of current page\n\
                 2. Uses vision model to identify interactive elements\n\
                 3. Reasons about which actions advance the task\n\
                 4. Executes actions (click, type, scroll, navigate)\n\
                 5. Repeats until task complete or max_steps reached\n\n\
                 VISION-BASED ELEMENT DETECTION:\n\
                 - Identifies buttons, links, forms, input fields visually\n\
                 - Understands page layout and content hierarchy\n\
                 - Handles dynamic content and JavaScript-rendered elements\n\
                 - Adapts to different page designs without explicit selectors\n\n\
                 HANDLING DYNAMIC CONTENT:\n\
                 - Pagination: \"Navigate through first 3 pages and collect all items\"\n\
                 - Infinite scroll: \"Scroll down 5 times and extract all visible products\"\n\
                 - Lazy loading: Agent automatically waits for content to appear\n\
                 - Popups/modals: \"Dismiss any popups or cookie banners first\"\n\n\
                 BEST PRACTICES FOR TASK CLARITY:\n\
                 1. State the objective clearly:\n\
                    Good: \"Find the API reference section and extract all endpoint URLs\"\n\
                    Bad: \"Look around the docs\"\n\n\
                 2. Specify output format:\n\
                    Good: \"Return as JSON array with fields: name, price, features\"\n\
                    Bad: \"Get the products\"\n\n\
                 3. Include stopping conditions:\n\
                    Good: \"Extract data from first 3 pages, then stop\"\n\
                    Bad: \"Get all the data\" (might run forever)\n\n\
                 4. Provide constraints:\n\
                    Good: \"Fill the form but DO NOT click submit button\"\n\
                    Bad: \"Complete the form\" (ambiguous)\n\n\
                 TIMEOUT AND STEP BUDGETING:\n\
                 - Set realistic max_steps for task complexity\n\
                 - Each step can have 1-3 actions (default: 3)\n\
                 - Total actions = max_steps × max_actions_per_step\n\
                 - For complex tasks, increase both max_steps and await_completion_ms\n\n\
                 Example budgeting:\n\
                 - Simple extraction: max_steps: 5, await_completion_ms: 120000 (2 min)\n\
                 - Form workflow: max_steps: 15, await_completion_ms: 300000 (5 min)\n\
                 - Deep research: max_steps: 30, await_completion_ms: 900000 (15 min)\n\n\
                 ERROR RECOVERY:\n\
                 If agent gets stuck or confused:\n\
                 - Make task more specific\n\
                 - Provide start_url closer to target\n\
                 - Add constraints in additional_info\n\
                 - Increase max_steps if hitting limit\n\
                 - Reduce scope of task (break into smaller tasks)",
            ),
        },
    ]
}

/// Monitoring agent progress - READ action
fn prompt_monitoring() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I monitor browser agent progress?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The READ action checks browser agent progress and status without executing new tasks. Use it to monitor long-running tasks or background agents.\n\n\
                 BASIC READ SYNTAX:\n\
                 browser_agent({\n\
                     \"action\": \"READ\",\n\
                     \"agent\": 0\n\
                 })\n\n\
                 RESPONSE FIELDS:\n\
                 - agent: Agent number being monitored\n\
                 - status: null/\"pending\" (not started), \"running\" (executing), \"completed\" (finished)\n\
                 - current_url: Current page URL (if available)\n\
                 - steps_completed: Number of steps executed so far\n\
                 - progress: Human-readable progress description\n\
                 - exit_code: null (running), 0 (success), non-zero (error)\n\
                 - result: Task output (populated when completed)\n\n\
                 FIRE-AND-FORGET PATTERN:\n\
                 1. Start background task (await_completion_ms: 0):\n\
                    browser_agent({\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Research competitor pricing and features across 5 websites. Return comprehensive comparison.\",\n\
                        \"await_completion_ms\": 0,\n\
                        \"max_steps\": 40\n\
                    })\n\n\
                 Returns immediately:\n\
                 {\n\
                   \"agent\": 0,\n\
                   \"status\": \"pending\"\n\
                 }\n\n\
                 2. Check progress periodically:\n\
                    browser_agent({ \"action\": \"READ\", \"agent\": 0 })\n\n\
                 Response while running:\n\
                 {\n\
                   \"agent\": 0,\n\
                   \"status\": \"running\",\n\
                   \"current_url\": \"https://competitor2.com/pricing\",\n\
                   \"steps_completed\": 12,\n\
                   \"progress\": \"Extracting pricing data from second competitor...\"\n\
                 }\n\n\
                 Response when completed:\n\
                 {\n\
                   \"agent\": 0,\n\
                   \"status\": \"completed\",\n\
                   \"exit_code\": 0,\n\
                   \"result\": \"Comparison of 5 competitors: [detailed data]...\"\n\
                 }\n\n\
                 POLLING STRATEGY:\n\
                 For long-running tasks:\n\
                 1. Start with await_completion_ms: 0\n\
                 2. Poll with READ every 30-60 seconds\n\
                 3. Check status field for completion\n\
                 4. When status=\"completed\", extract result\n\n\
                 MULTIPLE AGENTS MONITORING:\n\
                 Run parallel tasks and monitor each:\n\n\
                 // Start 3 agents\n\
                 browser_agent({ \"agent\": 0, \"action\": \"PROMPT\", \"task\": \"Task A\", \"await_completion_ms\": 0 })\n\
                 browser_agent({ \"agent\": 1, \"action\": \"PROMPT\", \"task\": \"Task B\", \"await_completion_ms\": 0 })\n\
                 browser_agent({ \"agent\": 2, \"action\": \"PROMPT\", \"task\": \"Task C\", \"await_completion_ms\": 0 })\n\n\
                 // Monitor all three\n\
                 browser_agent({ \"action\": \"READ\", \"agent\": 0 })\n\
                 browser_agent({ \"action\": \"READ\", \"agent\": 1 })\n\
                 browser_agent({ \"action\": \"READ\", \"agent\": 2 })\n\n\
                 TIMEOUT DETECTION:\n\
                 When PROMPT times out (reaches await_completion_ms), the task continues in background:\n\n\
                 browser_agent({\n\
                     \"action\": \"PROMPT\",\n\
                     \"task\": \"Complex research task...\",\n\
                     \"await_completion_ms\": 60000  // 1 minute timeout\n\
                 })\n\n\
                 If task not done in 1 minute, returns:\n\
                 {\n\
                   \"agent\": 0,\n\
                   \"status\": \"running\",\n\
                   \"steps_completed\": 8,\n\
                   \"progress\": \"Still working...\"\n\
                 }\n\n\
                 Use READ to check:\n\
                 browser_agent({ \"action\": \"READ\", \"agent\": 0 })\n\n\
                 STATUS INTERPRETATION:\n\
                 - null or \"pending\": Agent hasn't started (task queued)\n\
                 - \"running\": Agent actively executing steps\n\
                 - \"completed\": Task finished\n\
                   - exit_code: 0 = success\n\
                   - exit_code: non-zero = error occurred\n\
                   - Check result field for output\n\n\
                 TAIL PARAMETER:\n\
                 Limit output size when reading:\n\n\
                 browser_agent({\n\
                     \"action\": \"READ\",\n\
                     \"agent\": 0,\n\
                     \"tail\": 100  // Only last 100 lines of output\n\
                 })\n\n\
                 Useful for:\n\
                 - Large result sets\n\
                 - Progress monitoring (recent output only)\n\
                 - Reducing response payload\n\n\
                 BEST PRACTICES:\n\
                 1. Fire-and-forget for tasks > 2 minutes:\n\
                    Set await_completion_ms: 0, poll with READ\n\n\
                 2. Don't spam READ:\n\
                    Poll every 30-60 seconds, not every second\n\n\
                 3. Monitor multiple agents efficiently:\n\
                    Read all agents in parallel calls\n\n\
                 4. Check exit_code on completion:\n\
                    exit_code: 0 = success, non-zero = investigate error\n\n\
                 5. Use tail for large outputs:\n\
                    Prevents overwhelming responses",
            ),
        },
    ]
}

/// Agent lifecycle and cleanup - KILL action
fn prompt_management() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I manage agent lifecycle and cleanup?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The KILL action gracefully shuts down browser agents and cleans up resources. Use it to manage agent lifecycle and free resources when tasks are complete.\n\n\
                 BASIC KILL SYNTAX:\n\
                 browser_agent({\n\
                     \"action\": \"KILL\",\n\
                     \"agent\": 0\n\
                 })\n\n\
                 LIFECYCLE PATTERN:\n\
                 1. PROMPT → spawn task\n\
                 2. READ → monitor progress (optional)\n\
                 3. KILL → cleanup when done\n\n\
                 CLEANUP PATTERNS:\n\n\
                 1. Simple cleanup after completion:\n\
                    // Start task\n\
                    browser_agent({\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Extract pricing data...\"\n\
                    })\n\
                    // Task completes, returns result\n\
                    // Clean up resources\n\
                    browser_agent({ \"action\": \"KILL\", \"agent\": 0 })\n\n\
                 2. Multiple agent cleanup:\n\
                    // Start parallel agents\n\
                    browser_agent({ \"agent\": 0, \"action\": \"PROMPT\", \"task\": \"Task A\" })\n\
                    browser_agent({ \"agent\": 1, \"action\": \"PROMPT\", \"task\": \"Task B\" })\n\
                    browser_agent({ \"agent\": 2, \"action\": \"PROMPT\", \"task\": \"Task C\" })\n\n\
                    // Clean all when done\n\
                    browser_agent({ \"action\": \"KILL\", \"agent\": 0 })\n\
                    browser_agent({ \"action\": \"KILL\", \"agent\": 1 })\n\
                    browser_agent({ \"action\": \"KILL\", \"agent\": 2 })\n\n\
                 3. Error recovery cleanup:\n\
                    // Start task\n\
                    browser_agent({\n\
                        \"agent\": 0,\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Complex task...\",\n\
                        \"await_completion_ms\": 0\n\
                    })\n\n\
                    // Check status\n\
                    browser_agent({ \"action\": \"READ\", \"agent\": 0 })\n\
                    // If error or stuck, kill it\n\
                    browser_agent({ \"action\": \"KILL\", \"agent\": 0 })\n\n\
                 4. Background service teardown:\n\
                    // Start long-running research\n\
                    browser_agent({\n\
                        \"agent\": 1,\n\
                        \"action\": \"PROMPT\",\n\
                        \"task\": \"Monitor competitor site for changes...\",\n\
                        \"await_completion_ms\": 0,\n\
                        \"max_steps\": 100\n\
                    })\n\n\
                    // Do other work...\n\
                    // When done with monitoring, clean up\n\
                    browser_agent({ \"action\": \"KILL\", \"agent\": 1 })\n\n\
                 GRACEFUL SHUTDOWN:\n\
                 KILL action:\n\
                 - Stops any running browser navigation\n\
                 - Closes browser instance\n\
                 - Releases memory and resources\n\
                 - Removes agent from active pool\n\
                 - Cannot be undone\n\n\
                 WHEN TO KILL:\n\
                 ✓ Task completed successfully\n\
                 ✓ Task failed and won't recover\n\
                 ✓ No longer need the agent\n\
                 ✓ Freeing resources for new agents\n\
                 ✓ Stopping stuck or infinite tasks\n\n\
                 WHEN NOT TO KILL:\n\
                 ✗ While task still running and needed\n\
                 ✗ Before reading final results\n\
                 ✗ If you plan to reuse agent (can't reuse agent numbers)\n\n\
                 CONCURRENCY PATTERNS:\n\
                 Each agent (0, 1, 2...) runs independently:\n\n\
                 // Agent 0: Documentation research\n\
                 browser_agent({\n\
                     \"agent\": 0,\n\
                     \"action\": \"PROMPT\",\n\
                     \"task\": \"Research Rust async patterns in official docs\",\n\
                     \"await_completion_ms\": 0\n\
                 })\n\n\
                 // Agent 1: Competitor analysis\n\
                 browser_agent({\n\
                     \"agent\": 1,\n\
                     \"action\": \"PROMPT\",\n\
                     \"task\": \"Extract competitor feature lists\",\n\
                     \"await_completion_ms\": 0\n\
                 })\n\n\
                 // Monitor both\n\
                 browser_agent({ \"action\": \"READ\", \"agent\": 0 })\n\
                 browser_agent({ \"action\": \"READ\", \"agent\": 1 })\n\n\
                 // Kill finished agents\n\
                 browser_agent({ \"action\": \"KILL\", \"agent\": 0 })  // Done first\n\
                 browser_agent({ \"action\": \"KILL\", \"agent\": 1 })  // Done second\n\n\
                 RESOURCE MANAGEMENT:\n\
                 Browser agents use significant resources:\n\
                 - Memory: ~100-500MB per agent\n\
                 - CPU: Active during navigation\n\
                 - Network: For page loading\n\n\
                 Best practices:\n\
                 1. Kill agents when done\n\
                 2. Don't accumulate idle agents\n\
                 3. Reuse agent numbers after killing (0, 1, 2...)\n\
                 4. Limit concurrent agents (recommend 3-5 max)\n\n\
                 TYPICAL WORKFLOW:\n\
                 // Background research pattern\n\
                 browser_agent({ \"agent\": 0, \"action\": \"PROMPT\", \"task\": \"...\", \"await_completion_ms\": 0 })\n\
                 // Do other work\n\
                 browser_agent({ \"action\": \"READ\", \"agent\": 0 })  // Check progress\n\
                 // More work\n\
                 browser_agent({ \"action\": \"READ\", \"agent\": 0 })  // Check again\n\
                 // If completed: use results, then cleanup\n\
                 browser_agent({ \"action\": \"KILL\", \"agent\": 0 })",
            ),
        },
    ]
}

/// Comprehensive guide covering all actions and patterns
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using browser_agent effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "COMPLETE BROWSER_AGENT GUIDE:\n\n\
                 The browser_agent provides autonomous web automation with AI-driven navigation, vision-based element detection, and multi-step task execution.\n\n\
                 THREE ACTIONS:\n\
                 1. PROMPT: Spawn new browser automation task\n\
                 2. READ: Check current agent progress/status\n\
                 3. KILL: Gracefully shutdown agent and cleanup\n\n\
                 =============================================================================\n\
                 ACTION 1 - PROMPT (Spawn Tasks)\n\
                 =============================================================================\n\n\
                 Start autonomous browser task:\n\
                 browser_agent({\n\
                     \"action\": \"PROMPT\",\n\
                     \"task\": \"Clear description of what to accomplish\"\n\
                 })\n\n\
                 CONFIGURATION PARAMETERS:\n\
                 - task (required): What the agent should accomplish\n\
                 - start_url (optional): Initial page (agent can navigate if omitted)\n\
                 - max_steps (optional): Maximum navigation steps (default: 10)\n\
                 - max_actions_per_step (optional): Actions per step (default: 3)\n\
                 - await_completion_ms (optional): Timeout in ms (default: 600000 = 10 min)\n\
                 - max_tokens (optional): Response length (default: 2048)\n\
                 - temperature (optional): Creativity 0-1 (default: 0.7)\n\
                 - additional_info (optional): Extra context/hints\n\n\
                 RESPONSE STRUCTURE:\n\
                 - agent: Agent number (0, 1, 2...)\n\
                 - status: \"pending\", \"running\", \"completed\"\n\
                 - result: Task output (when completed)\n\
                 - exit_code: 0 (success), non-zero (error), null (running)\n\n\
                 TIMEOUT CONTROL:\n\
                 - Default (600000ms): Waits up to 10 minutes for completion\n\
                 - Custom timeout: Set await_completion_ms to desired milliseconds\n\
                 - Fire-and-forget: Set await_completion_ms: 0 (returns immediately)\n\n\
                 Fire-and-forget example:\n\
                 browser_agent({\n\
                     \"action\": \"PROMPT\",\n\
                     \"task\": \"Long research task...\",\n\
                     \"await_completion_ms\": 0\n\
                 })\n\
                 // Returns immediately, use READ to check progress\n\n\
                 =============================================================================\n\
                 ACTION 2 - READ (Monitor Progress)\n\
                 =============================================================================\n\n\
                 Check agent status:\n\
                 browser_agent({\n\
                     \"action\": \"READ\",\n\
                     \"agent\": 0\n\
                 })\n\n\
                 WHEN TO USE READ:\n\
                 - After fire-and-forget (await_completion_ms: 0)\n\
                 - When PROMPT times out before completing\n\
                 - Periodic monitoring of long-running tasks\n\
                 - Checking multiple parallel agents\n\n\
                 RESPONSE FIELDS:\n\
                 - status: Current state (pending/running/completed)\n\
                 - current_url: Current page URL\n\
                 - steps_completed: Steps executed so far\n\
                 - progress: Human-readable progress description\n\
                 - result: Output (populated when completed)\n\
                 - exit_code: null (running), 0 (success), non-zero (error)\n\n\
                 POLLING PATTERN:\n\
                 1. Start background task: browser_agent({ \"action\": \"PROMPT\", \"await_completion_ms\": 0 })\n\
                 2. Poll periodically: browser_agent({ \"action\": \"READ\", \"agent\": 0 })\n\
                 3. Check status field for \"completed\"\n\
                 4. Extract result when done\n\n\
                 =============================================================================\n\
                 ACTION 3 - KILL (Cleanup)\n\
                 =============================================================================\n\n\
                 Shutdown agent:\n\
                 browser_agent({\n\
                     \"action\": \"KILL\",\n\
                     \"agent\": 0\n\
                 })\n\n\
                 LIFECYCLE:\n\
                 PROMPT → spawn task\n\
                 READ → monitor (optional)\n\
                 KILL → cleanup resources\n\n\
                 WHEN TO KILL:\n\
                 - Task completed and results extracted\n\
                 - Task failed and won't recover\n\
                 - Freeing resources for new agents\n\
                 - Stopping stuck/infinite tasks\n\n\
                 Cleanup example:\n\
                 browser_agent({ \"agent\": 0, \"action\": \"PROMPT\", \"task\": \"...\" })\n\
                 // Use results\n\
                 browser_agent({ \"action\": \"KILL\", \"agent\": 0 })\n\n\
                 =============================================================================\n\
                 DECISION TREE - Which Action?\n\
                 =============================================================================\n\n\
                 Need to RUN task → PROMPT\n\
                 - Quick: browser_agent({ \"action\": \"PROMPT\", \"task\": \"...\" })\n\
                 - Background: browser_agent({ \"action\": \"PROMPT\", \"task\": \"...\", \"await_completion_ms\": 0 })\n\n\
                 Need to CHECK progress → READ\n\
                 - browser_agent({ \"action\": \"READ\", \"agent\": 0 })\n\n\
                 Need to STOP agent → KILL\n\
                 - browser_agent({ \"action\": \"KILL\", \"agent\": 0 })\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 1. Documentation research:\n\
                 browser_agent({ \"action\": \"PROMPT\", \"task\": \"Find async/await docs, summarize concepts\", \"start_url\": \"https://doc.rust-lang.org\" })\n\n\
                 2. Data extraction:\n\
                 browser_agent({ \"action\": \"PROMPT\", \"task\": \"Extract pricing as JSON\", \"start_url\": \"https://example.com/pricing\" })\n\n\
                 3. Multi-page form:\n\
                 browser_agent({ \"action\": \"PROMPT\", \"task\": \"Fill form: shipping, payment (stop before card)\", \"start_url\": \"https://shop.com/checkout\", \"max_steps\": 15 })\n\n\
                 4. Background research (fire-and-forget):\n\
                 browser_agent({ \"agent\": 0, \"action\": \"PROMPT\", \"task\": \"Research competitors\", \"await_completion_ms\": 0 })\n\
                 browser_agent({ \"action\": \"READ\", \"agent\": 0 })  // Poll\n\
                 browser_agent({ \"action\": \"KILL\", \"agent\": 0 })  // Cleanup\n\n\
                 =============================================================================\n\
                 CONCURRENCY PATTERNS\n\
                 =============================================================================\n\n\
                 Run parallel tasks with independent agents (numbered 0, 1, 2...):\n\
                 browser_agent({ \"agent\": 0, \"action\": \"PROMPT\", \"task\": \"Task A\", \"await_completion_ms\": 0 })\n\
                 browser_agent({ \"agent\": 1, \"action\": \"PROMPT\", \"task\": \"Task B\", \"await_completion_ms\": 0 })\n\
                 browser_agent({ \"agent\": 2, \"action\": \"PROMPT\", \"task\": \"Task C\", \"await_completion_ms\": 0 })\n\n\
                 Monitor all agents:\n\
                 browser_agent({ \"action\": \"READ\", \"agent\": 0 })\n\
                 browser_agent({ \"action\": \"READ\", \"agent\": 1 })\n\
                 browser_agent({ \"action\": \"READ\", \"agent\": 2 })\n\n\
                 Cleanup when done:\n\
                 browser_agent({ \"action\": \"KILL\", \"agent\": 0 })\n\
                 browser_agent({ \"action\": \"KILL\", \"agent\": 1 })\n\
                 browser_agent({ \"action\": \"KILL\", \"agent\": 2 })\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. Write clear, specific task descriptions with output format\n\
                 2. Set appropriate max_steps (5=simple, 15=form, 25-30=research)\n\
                 3. Use start_url when known to save navigation steps\n\
                 4. Fire-and-forget for long tasks (await_completion_ms: 0)\n\
                 5. KILL agents after completion to free resources\n\
                 6. Limit concurrent agents (3-5 recommended)",
            ),
        },
    ]
}
