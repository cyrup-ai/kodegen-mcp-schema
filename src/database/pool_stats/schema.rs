//! Schema types for db_pool_stats tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::super::types::{ConnectionStats, PoolConfiguration, PoolHealth};
use super::prompts::PoolStatsPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Tool name: Get connection pool statistics
pub const DB_POOL_STATS: &str = "db_pool_stats";

// ============================================================================
// TOOL ARGUMENTS
// ============================================================================

/// Arguments for `db_pool_stats` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPoolStatsArgs {}

// ============================================================================
// TOOL OUTPUT
// ============================================================================

/// Output from `db_pool_stats` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPoolStatsOutput {
    pub database_type: String,
    pub connections: ConnectionStats,
    pub configuration: PoolConfiguration,
    pub health: PoolHealth,
}

// ============================================================================
// TOOL ARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "db_pool_stats",
    category = "database",
    description = "Get connection pool statistics including active/idle connections, utilization, and health metrics."
)]
impl ToolArgs for GetPoolStatsArgs {
    type Output = GetPoolStatsOutput;
    type Prompts = PoolStatsPrompts;

    const NAME: &'static str = DB_POOL_STATS;
    const CATEGORY: &'static str = "database";
    const DESCRIPTION: &'static str = "Get connection pool statistics including active/idle connections, utilization, and health metrics.";
}
