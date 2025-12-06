//! Prompt argument types for db_pool_stats tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for db_pool_stats tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPoolStatsPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple pool status check
    /// - "troubleshooting": Diagnosing connection issues
    /// - "monitoring": Understanding pool health
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
