//! Introspection tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// TOOL NAME CONSTANTS
// ============================================================================

/// Canonical name for the inspect_tool_calls tool
pub const INSPECT_TOOL_CALLS: &str = "inspect_tool_calls";

/// Canonical name for the inspect_usage_stats tool
pub const INSPECT_USAGE_STATS: &str = "inspect_usage_stats";

// ============================================================================
// INSPECT USAGE STATS
// ============================================================================

/// Arguments for `inspect_usage_stats` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct InspectUsageStatsArgs {
    // No arguments needed
}

/// Prompt arguments for `inspect_usage_stats` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct InspectUsageStatsPromptArgs {
    // No arguments needed
}

// ============================================================================
// INSPECT TOOL CALLS
// ============================================================================

/// Default value for max_results
fn default_max_results() -> usize {
    50
}

/// Arguments for `inspect_tool_calls` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct InspectToolCallsArgs {
    /// Maximum number of results to return (default: 50, max: 1000)
    /// Ignored when offset is negative
    #[serde(default = "default_max_results")]
    pub max_results: usize,

    /// Offset for pagination (default: 0)
    /// Positive: Start from result N (0-based, oldest to newest)
    /// Negative: Read last N results from end (tail behavior, most recent)
    #[serde(default)]
    pub offset: i64,

    /// Filter by specific tool name (optional)
    #[serde(default)]
    pub tool_name: Option<String>,

    /// Only return calls since this timestamp (ISO 8601 format)
    #[serde(default)]
    pub since: Option<String>,
}

/// Prompt arguments for `inspect_tool_calls` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct InspectToolCallsPromptArgs {
    // No arguments needed
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `inspect_tool_calls` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InspectToolCallsOutput {
    /// Whether the operation succeeded
    pub success: bool,
    /// Number of calls returned
    pub count: usize,
    /// Total entries in memory
    pub total_entries_in_memory: usize,
    /// Tool call records
    pub calls: Vec<ToolCallRecord>,
    /// Filter applied (tool name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_tool_name: Option<String>,
    /// Filter applied (since timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_since: Option<String>,
    /// Offset used for pagination
    pub offset: i64,
    /// Max results requested
    pub max_results: usize,
}

/// A single tool call record
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolCallRecord {
    /// Tool name that was called
    pub tool_name: String,
    /// Timestamp of the call (ISO 8601)
    pub timestamp: String,
    /// Duration in milliseconds (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    /// JSON string of the arguments (serialized from original typed args)
    pub args_json: String,
    /// JSON string of the output (serialized from original typed output)
    pub output_json: String,
}

/// Output from `inspect_usage_stats` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InspectUsageOutput {
    /// Whether the operation succeeded
    pub success: bool,
    /// Total number of tool calls
    pub total_calls: usize,
    /// Number of unique tools used
    pub tools_used: usize,
    /// Per-tool usage statistics
    pub tool_usage: Vec<ToolUsageStats>,
    /// Session duration in milliseconds
    pub session_duration_ms: u64,
    /// Success rate percentage
    pub success_rate: f64,
    /// Number of successful calls
    pub successful_calls: usize,
    /// Number of failed calls
    pub failed_calls: usize,
}

/// Usage statistics for a single tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolUsageStats {
    /// Tool name
    pub tool_name: String,
    /// Number of times called
    pub call_count: usize,
    /// Total duration in milliseconds
    pub total_duration_ms: u64,
    /// Average duration in milliseconds
    pub avg_duration_ms: u64,
}

// ============================================================================
// TOOL ARGS TRAIT IMPLEMENTATIONS
// ============================================================================

impl crate::ToolArgs for InspectToolCallsArgs {
    type Output = InspectToolCallsOutput;
}

impl crate::ToolArgs for InspectUsageStatsArgs {
    type Output = InspectUsageOutput;
}
