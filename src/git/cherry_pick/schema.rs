//! Schema types for git_cherry_pick tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::CherryPickPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANT
// ============================================================================

/// Canonical tool name for git_cherry_pick
pub const GIT_CHERRY_PICK: &str = "git_cherry_pick";

// ============================================================================
// GIT_CHERRY_PICK TOOL
// ============================================================================

/// Arguments for `git_cherry_pick` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitCherryPickArgs {
    /// Path to repository
    pub path: String,

    /// Single commit hash to cherry-pick
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,

    /// Multiple commit hashes to cherry-pick
    /// 
    /// Accepts both single string and array: `commits: "abc1234"` or `commits: ["abc1234", "def5678"]`
    #[serde(default, skip_serializing_if = "Vec::is_empty", deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub commits: Vec<String>,

    /// Range of commits to cherry-pick (e.g., "abc1234..def5678")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,

    /// Stage changes but don't commit (allows modification before committing)
    #[serde(default, skip_serializing_if = "is_false")]
    pub no_commit: bool,

    /// Custom commit message (overrides original)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Edit the commit message before committing
    #[serde(default, skip_serializing_if = "is_false")]
    pub edit: bool,

    /// Parent number for merge commits (1 or 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mainline: Option<u32>,

    /// Continue cherry-pick after resolving conflicts
    #[serde(default, skip_serializing_if = "is_false")]
    pub r#continue: bool,

    /// Abort cherry-pick operation
    #[serde(default, skip_serializing_if = "is_false")]
    pub abort: bool,

    /// Skip current commit and continue with next
    #[serde(default, skip_serializing_if = "is_false")]
    pub skip: bool,
}

// Helper function for skip_serializing_if
fn is_false(b: &bool) -> bool {
    !b
}

// ============================================================================
// OUTPUT TYPE
// ============================================================================

/// Output from `git_cherry_pick` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitCherryPickOutput {
    /// Whether the operation succeeded
    pub success: bool,

    /// Original commit hash (for single commit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,

    /// New commit hash created by cherry-pick (for single commit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_commit: Option<String>,

    /// Multiple commits info (for multiple cherry-picks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<CherryPickCommitInfo>>,

    /// Files with conflicts (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<String>>,

    /// Current cherry-pick state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Error message if operation failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Information about a single cherry-picked commit
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CherryPickCommitInfo {
    /// Original commit hash
    pub original: String,

    /// New commit hash (if successfully applied)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new: Option<String>,

    /// Whether this commit succeeded
    pub success: bool,

    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[tool_metadata(
    name = "git_cherry_pick",
    category = "git",
    description = "Apply changes from specific commits to current branch"
)]
impl ToolArgs for GitCherryPickArgs {
    type Output = GitCherryPickOutput;
    type Prompts = CherryPickPrompts;

    const NAME: &'static str = GIT_CHERRY_PICK;
    const CATEGORY: &'static str = "git";
    const DESCRIPTION: &'static str = "Apply changes from specific commits to current branch";
}
