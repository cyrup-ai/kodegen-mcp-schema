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
