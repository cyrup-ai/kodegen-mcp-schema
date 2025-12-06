//! Prompt argument types for github_accept_repo_invitation tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_accept_repo_invitation tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubAcceptRepoInvitationPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Accepting invitations
    /// - "workflows": Complete invitation workflows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
