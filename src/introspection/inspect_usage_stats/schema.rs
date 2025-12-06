//! Schema types for inspect_usage_stats tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::InspectUsageStatsPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical name for the inspect_usage_stats tool
pub const INSPECT_USAGE_STATS: &str = "inspect_usage_stats";

// ============================================================================
// TOOL ARGUMENTS
// ============================================================================

/// Arguments for `inspect_usage_stats` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct InspectUsageStatsArgs {
    // No arguments needed
}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

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
// TOOL ARGS TRAIT IMPLEMENTATION (Args→Output Binding)
// ============================================================================

#[tool_metadata(
    name = "inspect_usage_stats",
    category = "introspection",
    description = "Analyze aggregated tool usage statistics including call counts, success rates, and performance metrics. Essential for optimization and performance analysis"
)]
impl ToolArgs for InspectUsageStatsArgs {
    type Output = InspectUsageOutput;
    type Prompts = InspectUsageStatsPrompts;

    const NAME: &'static str = INSPECT_USAGE_STATS;
    const CATEGORY: &'static str = "introspection";
    const DESCRIPTION: &'static str = "Analyze aggregated tool usage statistics including call counts, success rates, and performance metrics. Essential for optimization and performance analysis";
}
