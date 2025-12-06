//! Schema types for request_copilot_review tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::RequestCopilotReviewPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for requesting Copilot review
pub const GITHUB_REQUEST_COPILOT_REVIEW: &str = "github_request_copilot_review";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for `request_copilot_review` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequestCopilotReviewArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pull_number: u64,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_request_copilot_review` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubRequestCopilotReviewOutput {
    pub success: bool,
    pub owner: String,
    pub repo: String,
    pub pr_number: u64,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_request_copilot_review",
    category = "github",
    description = "Request Copilot review for pull request"
)]
impl ToolArgs for RequestCopilotReviewArgs {
    type Output = GitHubRequestCopilotReviewOutput;
    type Prompts = RequestCopilotReviewPrompts;

    const NAME: &'static str = GITHUB_REQUEST_COPILOT_REVIEW;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Request Copilot review for pull request";
}
