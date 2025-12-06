//! Schema types for create_repository tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ToolArgs, tool_metadata};
use super::prompts::CreateRepositoryPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for creating a repository
pub const GITHUB_CREATE_REPOSITORY: &str = "github_create_repository";

// ============================================================================
// ARGS STRUCT
// ============================================================================

/// Arguments for creating a repository
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateRepositoryArgs {
    /// Repository name
    pub name: String,
    /// Repository description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub description: Option<String>,
    /// Make repository private (optional, default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub private: Option<bool>,
    /// Initialize with README (optional, default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub auto_init: Option<bool>,
    /// .gitignore template name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub gitignore_template: Option<String>,
    /// License template name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub license_template: Option<String>,
    /// Allow squash merging (optional, default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub allow_squash_merge: Option<bool>,
    /// Allow merge commits (optional, default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub allow_merge_commit: Option<bool>,
    /// Allow rebase merging (optional, default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub allow_rebase_merge: Option<bool>,
    /// Automatically delete head branches after merge (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub delete_branch_on_merge: Option<bool>,
    /// Enable issues (optional, default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub has_issues: Option<bool>,
    /// Enable projects (optional, default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub has_projects: Option<bool>,
    /// Enable wiki (optional, default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub has_wiki: Option<bool>,
}

// ============================================================================
// OUTPUT STRUCT
// ============================================================================

/// Output from `github_create_repository` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubCreateRepoOutput {
    pub success: bool,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub clone_url: String,
    pub message: String,
}

// ============================================================================
// TOOLARGS IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    name = "github_create_repository",
    category = "github",
    description = "Create a new GitHub repository"
)]
impl ToolArgs for CreateRepositoryArgs {
    type Output = GitHubCreateRepoOutput;
    type Prompts = CreateRepositoryPrompts;

    const NAME: &'static str = GITHUB_CREATE_REPOSITORY;
    const CATEGORY: &'static str = "github";
    const DESCRIPTION: &'static str = "Create a new GitHub repository";
}
