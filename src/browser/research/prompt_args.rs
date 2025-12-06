//! Prompt argument types for browser_research tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for browser_research tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserResearchPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Simple research queries
    /// - "deep_research": Multi-page in-depth research
    /// - "technical_docs": Researching technical documentation
    /// - "comparison": Comparing multiple solutions/products
    /// - "monitoring": Long-running research management
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
