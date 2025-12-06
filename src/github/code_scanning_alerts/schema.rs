//! Schema types for code_scanning_alerts tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::CodeScanningAlertsPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for code scanning alerts
pub const GITHUB_CODE_SCANNING_ALERTS: &str = "github_code_scanning_alerts";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for code_scanning_alerts tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CodeScanningAlertsArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Filter by state: "open", "closed", or "dismissed" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Filter by branch/ref (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    /// Filter by tool name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    /// Filter by severity: "critical", "high", "medium", "low", "warning", "note", "error" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_code_scanning_alerts` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCodeScanningAlertsOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub count: usize,
    pub alerts: Vec<GitHubCodeScanningAlert>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCodeScanningAlert {
    pub number: u64,
    pub state: String,
    pub severity: String,
    pub rule_id: String,
    pub rule_description: String,
    pub tool_name: String,
    pub created_at: String,
    pub html_url: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_code_scanning_alerts",
    category = "github",
    description = "Get code scanning alerts for a repository"
)]
impl ToolArgs for CodeScanningAlertsArgs {
    type Output = GitHubCodeScanningAlertsOutput;
    type Prompts = CodeScanningAlertsPrompts;

    const NAME: &'static str = GITHUB_CODE_SCANNING_ALERTS;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Get code scanning alerts for a repository";
}
