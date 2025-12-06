//! Prompt argument types for github_get_pull_request_status tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_get_pull_request_status tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestStatusPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Retrieving pull request status and checks
    /// - "merge_decision": Evaluating merge readiness and workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
