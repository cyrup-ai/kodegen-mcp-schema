//! Prompt argument types for github_code_scanning_alerts tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_code_scanning_alerts tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CodeScanningAlertsPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Listing code scanning alerts
    /// - "filtering": Filter by severity, state
    /// - "analysis": Analyzing security results
    /// - "remediation": Fixing security issues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
