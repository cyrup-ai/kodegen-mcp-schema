//! Prompt argument types for github_get_pull_request_reviews tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestReviewsPromptArgs {
    /// Scenario to display: "basic" (default) or "review_analysis"
    pub scenario: Option<String>,
}
