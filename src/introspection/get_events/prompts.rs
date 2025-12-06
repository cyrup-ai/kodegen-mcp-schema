//! Prompt messages for introspection_get_events tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::IntrospectionGetEventsPromptArgs;

/// Prompt provider for introspection_get_events tool
///
/// This is the ONLY way to provide prompts for introspection_get_events - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct IntrospectionGetEventsPrompts;

impl PromptProvider for IntrospectionGetEventsPrompts {
    type PromptArgs = IntrospectionGetEventsPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("filtering") => prompt_filtering(),
            Some("analysis") => prompt_analysis(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, filtering, analysis)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE INTROSPECTION_GET_EVENTS
// ============================================================================

/// Basic event retrieval and understanding event structure
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I get tool execution events and history using introspection_get_events?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The introspection_get_events tool retrieves execution history and events for all tools that have been called. Here's how to use it:\n\n\
                 GETTING EVENTS:\n\
                 1. Get recent events:\n\
                    introspection_get_events({})\n\
                    Returns all recent tool execution events\n\n\
                 2. Get all events (no filters):\n\
                    introspection_get_events({})\n\
                    Shows complete execution history\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"events\": [\n\
                     {\n\
                       \"tool\": \"fs_read_file\",\n\
                       \"timestamp\": \"2024-01-15T10:30:00Z\",\n\
                       \"duration_ms\": 50,\n\
                       \"success\": true,\n\
                       \"input\": {...},\n\
                       \"output\": {...}\n\
                     },\n\
                     {\n\
                       \"tool\": \"fs_search\",\n\
                       \"timestamp\": \"2024-01-15T10:30:05Z\",\n\
                       \"duration_ms\": 125,\n\
                       \"success\": true,\n\
                       \"input\": {...},\n\
                       \"output\": {...}\n\
                     }\n\
                   ],\n\
                   \"total_count\": 2,\n\
                   \"has_more\": false\n\
                 }\n\n\
                 EVENT FIELDS EXPLAINED:\n\
                 - tool: Which tool was called (e.g., \"fs_read_file\", \"terminal\", \"git_clone\")\n\
                 - timestamp: When the tool was executed (ISO 8601 format)\n\
                 - duration_ms: How long the tool took to execute in milliseconds\n\
                 - success: Whether the tool execution succeeded (true) or failed (false)\n\
                 - input: The arguments passed to the tool (optional in some views)\n\
                 - output: The result returned by the tool (optional in some views)\n\
                 - error: Error message if success is false (optional)\n\n\
                 COMMON USE CASES:\n\
                 1. Review what tools were recently used:\n\
                    introspection_get_events({})\n\
                    Useful for understanding workflow history\n\n\
                 2. Check if operations completed:\n\
                    introspection_get_events({})\n\
                    Review success field for each event\n\n\
                 3. Audit tool usage:\n\
                    introspection_get_events({})\n\
                    See complete history of all tool calls\n\n\
                 4. Debug workflow issues:\n\
                    introspection_get_events({})\n\
                    Check sequence of tool calls and their results\n\n\
                 UNDERSTANDING THE OUTPUT:\n\
                 - Events are typically ordered by timestamp (most recent first)\n\
                 - total_count: Total number of events matching filters\n\
                 - has_more: If true, there are more events available (use pagination)\n\
                 - Empty events array means no tool executions have been recorded\n\n\
                 BASIC INTERPRETATION:\n\
                 - success: true → Tool executed successfully\n\
                 - success: false → Tool encountered an error (check error field)\n\
                 - duration_ms: Performance indicator (higher = slower)\n\
                 - timestamp: When the operation happened (useful for sequencing)\n\n\
                 EXAMPLE WORKFLOW:\n\
                 // After running several tools, check what happened:\n\
                 introspection_get_events({})\n\
                 // Review the events array to see:\n\
                 // - Which tools were called\n\
                 // - In what order (via timestamps)\n\
                 // - Whether they succeeded\n\
                 // - How long each took\n\n\
                 RESPONSE PAGINATION:\n\
                 If has_more is true, you can use limit and offset parameters:\n\
                 introspection_get_events({\"limit\": 50, \"offset\": 0})\n\
                 Then: introspection_get_events({\"limit\": 50, \"offset\": 50})\n\n\
                 NO EVENTS SCENARIO:\n\
                 If events array is empty:\n\
                 - No tools have been executed yet in this session\n\
                 - Or all event history has been cleared\n\
                 - Or filters are too restrictive (no matches)",
            ),
        },
    ]
}

/// Filtering events by tool name, time range, and success status
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I filter tool execution events to find specific information?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The introspection_get_events tool supports powerful filtering to help you find exactly what you need. Here's how to filter events:\n\n\
                 FILTERING BY TOOL:\n\
                 1. Single tool filter:\n\
                    introspection_get_events({\"tool\": \"fs_search\"})\n\
                    Returns only fs_search events\n\n\
                 2. Check specific tool usage:\n\
                    introspection_get_events({\"tool\": \"terminal\"})\n\
                    See all terminal tool executions\n\n\
                 3. Filter filesystem operations:\n\
                    introspection_get_events({\"tool\": \"fs_read_file\"})\n\
                    Review all file read operations\n\n\
                 FILTERING BY TIME RANGE:\n\
                 1. Events since a specific time:\n\
                    introspection_get_events({\n\
                        \"since\": \"2024-01-15T00:00:00Z\"\n\
                    })\n\
                    All events from that timestamp onward\n\n\
                 2. Events before a specific time:\n\
                    introspection_get_events({\n\
                        \"before\": \"2024-01-15T23:59:59Z\"\n\
                    })\n\
                    All events up to that timestamp\n\n\
                 3. Events in a specific time window:\n\
                    introspection_get_events({\n\
                        \"since\": \"2024-01-15T00:00:00Z\",\n\
                        \"before\": \"2024-01-15T23:59:59Z\"\n\
                    })\n\
                    All events within that day\n\n\
                 4. Recent events (last hour example):\n\
                    introspection_get_events({\n\
                        \"since\": \"2024-01-15T10:00:00Z\"\n\
                    })\n\
                    Events from the last hour (adjust timestamp)\n\n\
                 FILTERING BY SUCCESS STATUS:\n\
                 1. Only successful events:\n\
                    introspection_get_events({\"success\": true})\n\
                    See what worked\n\n\
                 2. Only failed events (errors):\n\
                    introspection_get_events({\"success\": false})\n\
                    Debug what went wrong\n\n\
                 3. Error investigation workflow:\n\
                    introspection_get_events({\"success\": false})\n\
                    Review error field for each failed event\n\n\
                 COMBINING FILTERS:\n\
                 1. Failed filesystem operations:\n\
                    introspection_get_events({\n\
                        \"tool\": \"fs_read_file\",\n\
                        \"success\": false\n\
                    })\n\
                    Find all failed file reads\n\n\
                 2. Recent terminal failures:\n\
                    introspection_get_events({\n\
                        \"tool\": \"terminal\",\n\
                        \"success\": false,\n\
                        \"since\": \"2024-01-15T10:00:00Z\"\n\
                    })\n\
                    Recent terminal errors only\n\n\
                 3. Successful searches in time range:\n\
                    introspection_get_events({\n\
                        \"tool\": \"fs_search\",\n\
                        \"success\": true,\n\
                        \"since\": \"2024-01-15T00:00:00Z\",\n\
                        \"before\": \"2024-01-15T23:59:59Z\"\n\
                    })\n\
                    All successful searches on that day\n\n\
                 PAGINATION FILTERS:\n\
                 1. Limit results:\n\
                    introspection_get_events({\"limit\": 10})\n\
                    Get only first 10 events\n\n\
                 2. Skip to next page:\n\
                    introspection_get_events({\"limit\": 10, \"offset\": 10})\n\
                    Get events 11-20\n\n\
                 3. Combine with other filters:\n\
                    introspection_get_events({\n\
                        \"tool\": \"fs_search\",\n\
                        \"limit\": 20,\n\
                        \"offset\": 0\n\
                    })\n\
                    First 20 fs_search events\n\n\
                 COMMON FILTERING PATTERNS:\n\
                 1. \"What failed recently?\":\n\
                    introspection_get_events({\n\
                        \"success\": false,\n\
                        \"since\": \"2024-01-15T10:00:00Z\",\n\
                        \"limit\": 20\n\
                    })\n\n\
                 2. \"How many times did I use this tool?\":\n\
                    introspection_get_events({\"tool\": \"git_clone\"})\n\
                    Check total_count in response\n\n\
                 3. \"What happened during this time window?\":\n\
                    introspection_get_events({\n\
                        \"since\": \"2024-01-15T10:00:00Z\",\n\
                        \"before\": \"2024-01-15T11:00:00Z\"\n\
                    })\n\n\
                 4. \"Show me only errors for debugging\":\n\
                    introspection_get_events({\"success\": false})\n\
                    Focus on problems only\n\n\
                 FILTER PARAMETER SUMMARY:\n\
                 - tool: Filter by tool name (exact match)\n\
                 - success: Filter by success status (true/false)\n\
                 - since: Events after this timestamp (ISO 8601)\n\
                 - before: Events before this timestamp (ISO 8601)\n\
                 - limit: Maximum events to return (pagination)\n\
                 - offset: Skip this many events (pagination)\n\n\
                 BEST PRACTICES:\n\
                 1. Use tool filter when investigating specific tool behavior\n\
                 2. Use success: false for error investigation\n\
                 3. Use time filters for temporal analysis\n\
                 4. Combine filters for precise queries\n\
                 5. Use pagination for large result sets\n\
                 6. Check has_more in response for additional results",
            ),
        },
    ]
}

/// Event analysis patterns for debugging and performance monitoring
fn prompt_analysis() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I analyze tool execution events for debugging and performance monitoring?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use introspection_get_events for powerful debugging and performance analysis. Here are key analysis patterns:\n\n\
                 PERFORMANCE ANALYSIS:\n\
                 1. Find slow operations:\n\
                    introspection_get_events({})\n\
                    Review duration_ms field for each event\n\
                    Look for values > 1000ms (1 second) or higher\n\n\
                 2. Compare tool performance:\n\
                    // Get all fs_search events\n\
                    introspection_get_events({\"tool\": \"fs_search\"})\n\
                    // Calculate average duration_ms\n\
                    // Compare with other tools\n\n\
                 3. Identify performance bottlenecks:\n\
                    introspection_get_events({})\n\
                    Sort by duration_ms (highest first)\n\
                    Focus optimization on slowest operations\n\n\
                 4. Track performance over time:\n\
                    introspection_get_events({\n\
                        \"tool\": \"terminal\",\n\
                        \"since\": \"2024-01-15T00:00:00Z\"\n\
                    })\n\
                    Compare duration_ms across time periods\n\n\
                 ERROR ANALYSIS:\n\
                 1. Debug recent failures:\n\
                    introspection_get_events({\"success\": false})\n\
                    Review error messages in each event\n\n\
                 2. Identify problematic tools:\n\
                    introspection_get_events({\"success\": false})\n\
                    Group by tool name\n\
                    See which tools fail most often\n\n\
                 3. Error pattern detection:\n\
                    introspection_get_events({\n\
                        \"tool\": \"fs_read_file\",\n\
                        \"success\": false\n\
                    })\n\
                    Look for common error patterns\n\
                    (e.g., file not found, permission denied)\n\n\
                 4. Temporal error analysis:\n\
                    introspection_get_events({\n\
                        \"success\": false,\n\
                        \"since\": \"2024-01-15T00:00:00Z\"\n\
                    })\n\
                    See when errors started occurring\n\n\
                 WORKFLOW ANALYSIS:\n\
                 1. Reconstruct operation sequence:\n\
                    introspection_get_events({})\n\
                    Review timestamps to see order of operations\n\
                    Understand workflow execution flow\n\n\
                 2. Find missing steps:\n\
                    introspection_get_events({})\n\
                    Check if expected tools were called\n\
                    Identify gaps in workflow\n\n\
                 3. Verify execution order:\n\
                    introspection_get_events({\n\
                        \"since\": \"2024-01-15T10:00:00Z\"\n\
                    })\n\
                    Confirm tools ran in correct sequence\n\n\
                 4. Detect retry patterns:\n\
                    introspection_get_events({\"tool\": \"git_clone\"})\n\
                    Look for multiple calls with same input\n\
                    Identify retry attempts\n\n\
                 USAGE AUDITING:\n\
                 1. Tool usage frequency:\n\
                    introspection_get_events({\"tool\": \"fs_search\"})\n\
                    Check total_count for usage statistics\n\n\
                 2. Most used tools:\n\
                    introspection_get_events({})\n\
                    Group events by tool name\n\
                    Count occurrences\n\n\
                 3. Success rate calculation:\n\
                    // Get all events for a tool\n\
                    introspection_get_events({\"tool\": \"terminal\"})\n\
                    // Count success: true vs false\n\
                    // Calculate success percentage\n\n\
                 4. Time-based usage patterns:\n\
                    introspection_get_events({})\n\
                    Group by time period (hour, day)\n\
                    Identify peak usage times\n\n\
                 DEBUGGING WORKFLOWS:\n\
                 1. \"Why did this fail?\":\n\
                    introspection_get_events({\n\
                        \"tool\": \"fs_write_file\",\n\
                        \"success\": false\n\
                    })\n\
                    Review error and input fields\n\n\
                 2. \"What happened before the error?\":\n\
                    introspection_get_events({\n\
                        \"before\": \"2024-01-15T10:30:00Z\",\n\
                        \"limit\": 10\n\
                    })\n\
                    See events leading up to failure\n\n\
                 3. \"Which operations succeeded?\":\n\
                    introspection_get_events({\"success\": true})\n\
                    Verify expected operations completed\n\n\
                 4. \"Is this tool working correctly?\":\n\
                    introspection_get_events({\"tool\": \"git_clone\"})\n\
                    Check success rate and error patterns\n\n\
                 ANALYSIS USE CASES:\n\
                 1. Performance optimization:\n\
                    - Identify slowest tools (high duration_ms)\n\
                    - Focus optimization efforts\n\
                    - Track improvements over time\n\n\
                 2. Error investigation:\n\
                    - Find all failures\n\
                    - Identify error patterns\n\
                    - Debug tool issues\n\
                    - Validate fixes\n\n\
                 3. Workflow verification:\n\
                    - Confirm expected sequence\n\
                    - Find missing operations\n\
                    - Detect anomalies\n\
                    - Validate automation\n\n\
                 4. Usage auditing:\n\
                    - Track tool usage\n\
                    - Calculate success rates\n\
                    - Identify trends\n\
                    - Monitor system health\n\n\
                 ANALYSIS TIPS:\n\
                 - Sort by duration_ms to find performance issues\n\
                 - Filter by success: false for error investigation\n\
                 - Use time filters for temporal analysis\n\
                 - Compare before/after timestamps for sequencing\n\
                 - Group by tool for usage statistics\n\
                 - Check error field for detailed failure information\n\
                 - Use input field to understand what was attempted\n\
                 - Use output field to see what was returned\n\n\
                 METRICS TO TRACK:\n\
                 - Average duration_ms per tool\n\
                 - Success rate (% of success: true)\n\
                 - Error frequency by tool\n\
                 - Peak usage times\n\
                 - Retry attempts (duplicate inputs)\n\
                 - Workflow completion rate",
            ),
        },
    ]
}

/// Comprehensive guide covering all features and patterns
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using introspection_get_events for tool execution monitoring.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The introspection_get_events tool provides comprehensive visibility into tool execution history, performance, and errors.\n\n\
                 BASIC USAGE:\n\
                 Get all events:\n\
                 introspection_get_events({})\n\n\
                 =============================================================================\n\
                 FILTERING OPTIONS\n\
                 =============================================================================\n\n\
                 BY TOOL:\n\
                 introspection_get_events({\"tool\": \"fs_search\"})\n\
                 introspection_get_events({\"tool\": \"terminal\"})\n\
                 introspection_get_events({\"tool\": \"git_clone\"})\n\n\
                 BY SUCCESS STATUS:\n\
                 introspection_get_events({\"success\": true})   // Only successful\n\
                 introspection_get_events({\"success\": false})  // Only failures\n\n\
                 BY TIME RANGE:\n\
                 introspection_get_events({\"since\": \"2024-01-15T00:00:00Z\"})\n\
                 introspection_get_events({\"before\": \"2024-01-15T23:59:59Z\"})\n\
                 introspection_get_events({\n\
                     \"since\": \"2024-01-15T00:00:00Z\",\n\
                     \"before\": \"2024-01-15T23:59:59Z\"\n\
                 })\n\n\
                 COMBINED FILTERS:\n\
                 introspection_get_events({\n\
                     \"tool\": \"fs_read_file\",\n\
                     \"success\": false,\n\
                     \"since\": \"2024-01-15T10:00:00Z\"\n\
                 })\n\n\
                 PAGINATION:\n\
                 introspection_get_events({\"limit\": 50, \"offset\": 0})\n\
                 introspection_get_events({\"limit\": 50, \"offset\": 50})\n\n\
                 =============================================================================\n\
                 EVENT STRUCTURE\n\
                 =============================================================================\n\n\
                 Each event contains:\n\
                 {\n\
                   \"tool\": \"fs_search\",                    // Tool name\n\
                   \"timestamp\": \"2024-01-15T10:30:00Z\",   // When executed\n\
                   \"duration_ms\": 125,                     // How long it took\n\
                   \"success\": true,                        // Success/failure\n\
                   \"input\": {...},                         // Tool arguments\n\
                   \"output\": {...},                        // Tool result\n\
                   \"error\": \"error message\"               // If success: false\n\
                 }\n\n\
                 =============================================================================\n\
                 ANALYSIS PATTERNS\n\
                 =============================================================================\n\n\
                 PERFORMANCE MONITORING:\n\
                 1. Find slow operations:\n\
                    introspection_get_events({})\n\
                    Look for high duration_ms values\n\n\
                 2. Tool performance comparison:\n\
                    introspection_get_events({\"tool\": \"fs_search\"})\n\
                    Calculate average duration_ms\n\n\
                 ERROR DEBUGGING:\n\
                 1. Recent failures:\n\
                    introspection_get_events({\"success\": false})\n\n\
                 2. Tool-specific errors:\n\
                    introspection_get_events({\n\
                        \"tool\": \"terminal\",\n\
                        \"success\": false\n\
                    })\n\n\
                 WORKFLOW ANALYSIS:\n\
                 1. Execution sequence:\n\
                    introspection_get_events({})\n\
                    Review timestamps for order\n\n\
                 2. Missing operations:\n\
                    Check if expected tools appear in events\n\n\
                 USAGE AUDITING:\n\
                 1. Tool frequency:\n\
                    introspection_get_events({\"tool\": \"fs_search\"})\n\
                    Check total_count\n\n\
                 2. Success rates:\n\
                    Compare success: true vs false counts\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 1. DEBUG FAILURES:\n\
                 introspection_get_events({\"success\": false})\n\
                 Review error field for each failed event\n\n\
                 2. PERFORMANCE AUDIT:\n\
                 introspection_get_events({})\n\
                 Sort by duration_ms to find slow operations\n\n\
                 3. TOOL USAGE STATS:\n\
                 introspection_get_events({\"tool\": \"terminal\"})\n\
                 Count events and calculate metrics\n\n\
                 4. WORKFLOW VERIFICATION:\n\
                 introspection_get_events({\n\
                     \"since\": \"2024-01-15T10:00:00Z\"\n\
                 })\n\
                 Verify expected sequence of operations\n\n\
                 5. ERROR INVESTIGATION:\n\
                 introspection_get_events({\n\
                     \"tool\": \"fs_write_file\",\n\
                     \"success\": false\n\
                 })\n\
                 Find patterns in failures\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. FILTERING:\n\
                    - Use tool filter for specific tool analysis\n\
                    - Use success: false for error investigation\n\
                    - Use time filters for temporal analysis\n\
                    - Combine filters for precise queries\n\n\
                 2. PAGINATION:\n\
                    - Use limit for large result sets\n\
                    - Check has_more for additional results\n\
                    - Use offset for next pages\n\n\
                 3. PERFORMANCE:\n\
                    - Focus on high duration_ms values\n\
                    - Track trends over time\n\
                    - Compare across tools\n\n\
                 4. DEBUGGING:\n\
                    - Start with success: false filter\n\
                    - Review error messages\n\
                    - Check input for problematic patterns\n\
                    - Look at events before failure\n\n\
                 5. AUDITING:\n\
                    - Track tool usage frequency\n\
                    - Calculate success rates\n\
                    - Monitor error patterns\n\
                    - Identify usage trends\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 All events: introspection_get_events({})\n\
                 By tool: introspection_get_events({\"tool\": \"fs_search\"})\n\
                 Errors only: introspection_get_events({\"success\": false})\n\
                 Time range: introspection_get_events({\"since\": \"...\", \"before\": \"...\"})\n\
                 Paginated: introspection_get_events({\"limit\": 50, \"offset\": 0})\n\n\
                 Remember: Events provide complete visibility into tool execution history. Use filters to find exactly what you need, analyze duration_ms for performance, and review error fields for debugging!",
            ),
        },
    ]
}
