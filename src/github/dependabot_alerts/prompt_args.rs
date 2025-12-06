//! Prompt argument types for github_dependabot_alerts tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_dependabot_alerts tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubDependabotAlertsPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Listing alerts
    /// - "filtering": Filter by severity
    /// - "remediation": Fixing vulnerabilities
    /// - "workflows": Security workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
