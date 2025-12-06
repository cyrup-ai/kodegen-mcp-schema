//! Prompt argument types for github_secret_scanning_alerts tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_secret_scanning_alerts tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SecretScanningAlertsPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Listing secret alerts and understanding alert data
    /// - "remediation": Step-by-step guide for fixing exposed secrets
    /// - "workflows": Security workflows and automation patterns
    /// - "audit": Regular security audits and compliance checks
    /// - "prevention": Best practices for preventing secret exposure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
