//! Prompt argument types for git_branch_rename tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for `git_branch_rename` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitBranchRenamePromptArgs {
    /// Optional: Use case for customized examples
    /// - "basic": Renaming a branch
    /// - "current": Renaming the current branch
    /// - "conventions": Following naming conventions
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
