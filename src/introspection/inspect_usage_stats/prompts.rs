//! Prompt messages for inspect_usage_stats tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::InspectUsageStatsPromptArgs;

/// Prompt provider for inspect_usage_stats tool
pub struct InspectUsageStatsPrompts;

impl PromptProvider for InspectUsageStatsPrompts {
    type PromptArgs = InspectUsageStatsPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_health_check()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

fn prompt_health_check() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Is this session healthy? Show me the usage statistics."
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "```typescript\n\
                 // inspect_usage_stats - Session usage statistics and health monitoring\n\
                 // Analyzes tool usage across the current session to identify patterns,\n\
                 // success rates, and potential issues. Takes no required parameters.\n\
                 // Returns InspectUsageOutput with comprehensive metrics.\n\n\
                 // Basic usage\n\
                 inspect_usage_stats({})\n\n\
                 // Example response\n\
                 {\n\
                 //   success_rate: 94.2,\n\
                 //   successful_calls: 85,\n\
                 //   failed_calls: 5,\n\
                 //   total_calls: 90,\n\
                 //   session_duration_ms: 45000,\n\
                 //   tools_used: 12,\n\
                 //   tool_usage: [/* per-tool stats */]\n\
                 }\n\n\
                 // Response structure\n\
                 // - success_rate: Percentage of successful calls (0-100)\n\
                 //   Values above 90% indicate healthy session\n\
                 // - successful_calls: Count of operations that completed without error\n\
                 // - failed_calls: Count of operations that threw errors\n\
                 //   High values warrant investigation\n\
                 // - total_calls: Sum of successful_calls + failed_calls\n\
                 // - session_duration_ms: Time elapsed since first tool invocation\n\
                 // - tools_used: Number of distinct tools called\n\
                 // - tool_usage: Array of ToolUsageStats, one per tool\n\
                 //   Each entry contains: tool_name, call_count, total_duration_ms, avg_duration_ms\n\n\
                 // When to use\n\
                 // - After running multiple tools, check if session is healthy\n\
                 // - When debugging repeated failures, identify which tools are problematic\n\
                 // - To monitor success rates during long-running sessions\n\
                 // - For performance analysis, find slow or frequently-called tools\n\n\
                 // Common pattern: health check investigation\n\
                 // 1. Run multiple tools during your session\n\
                 //    Example: fs_read_file, fs_search, terminal, etc.\n\n\
                 // 2. Call inspect_usage_stats to get session overview\n\
                 const stats = inspect_usage_stats({});\n\n\
                 // 3. Check success_rate\n\
                 if (stats.success_rate < 90) {\n\
                 //   Low success rate detected\n\n\
                 //   4. Inspect tool_usage array to find problematic tools\n\
                 //   Look for tools with high call_count but contributing to failed_calls\n\
                 const problematicTools = stats.tool_usage.filter(\n\
                 //     tool => /* high failure rate */\n\
                 //   );\n\n\
                 //   5. Use inspect_tool_calls for detailed failure information\n\
                 //   Get call history with error messages\n\
                 const details = inspect_tool_calls({\n\
                 //     tool_name: problematicTools[0].tool_name\n\
                 //   });\n\
                 }\n\n\
                 // Quick reference\n\
                 // Command: inspect_usage_stats({})\n\
                 // Parameters: None required\n\
                 // Returns: InspectUsageOutput with success_rate, calls, duration, tool breakdown\n\
                 // Related tools:\n\
                 //   - inspect_tool_calls: Detailed call history with parameters and errors\n\
                 //   - get_events: System events and lifecycle information\n\
                 ```"
            ),
        },
    ]
}
