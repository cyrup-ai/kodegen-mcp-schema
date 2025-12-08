//! Schema types for secret_scanning_alerts tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GITHUB, GITHUB_SECRET_SCANNING_ALERTS};

use crate::{ToolArgs, tool_metadata};
use super::prompts::SecretScanningAlertsPrompts;

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for secret_scanning_alerts tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SecretScanningAlertsArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Filter by state: "open" or "resolved" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Filter by secret type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_type: Option<String>,
    /// Filter by resolution: "false_positive", "wont_fix", "revoked", "used_in_tests" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_secret_scanning_alerts` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubSecretScanningAlertsOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub count: usize,
    pub alerts: Vec<GitHubSecretScanningAlert>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubSecretScanningAlert {
    pub number: u64,
    pub state: String,
    pub secret_type: String,
    pub resolution: Option<String>,
    pub created_at: String,
    pub html_url: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "List secret scanning alerts"
)]
impl ToolArgs for SecretScanningAlertsArgs {
    type Output = GitHubSecretScanningAlertsOutput;
    type Prompts = SecretScanningAlertsPrompts;

    const NAME: &'static str = GITHUB_SECRET_SCANNING_ALERTS;
    const CATEGORY: &'static str = CATEGORY_GITHUB;
    const DESCRIPTION: &'static str = "List secret scanning alerts";
}
