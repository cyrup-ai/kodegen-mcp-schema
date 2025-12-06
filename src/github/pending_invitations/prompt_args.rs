//! Prompt argument types for github_pending_invitations tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for github_pending_invitations tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubPendingInvitationsPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Listing invitations
    /// - "workflows": Invitation management workflows
    /// - "filtering": Filtering and organizing invitations
    /// - "integration": Integration with other GitHub tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
