//! Schema types for git_revert tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_GIT, GIT_REVERT};
use crate::{ToolArgs, tool_metadata};
use super::prompts::RevertPrompts;

// ============================================================================
// GIT_REVERT TOOL
// ============================================================================

/// Arguments for `git_revert` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitRevertArgs {
    /// Path to repository
    pub path: String,

    /// Single commit hash to revert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,

    /// Multiple commit hashes to revert
    /// 
    /// Accepts both single string and array: `commits: "abc1234"` or `commits: ["abc1234", "def5678"]`
    #[serde(default, skip_serializing_if = "Vec::is_empty", deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub commits: Vec<String>,

    /// Range of commits to revert (e.g., "abc1234..def5678")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,

    /// Stage changes but don't commit (allows combining multiple reverts)
    #[serde(default, skip_serializing_if = "is_false")]
    pub no_commit: bool,

    /// Custom commit message (overrides default "Revert" message)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Parent number for merge commits (1 or 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mainline: Option<u32>,

    /// Continue revert after resolving conflicts
    #[serde(default, skip_serializing_if = "is_false")]
    pub r#continue: bool,

    /// Abort revert operation
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

/// Output from `git_revert` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRevertOutput {
    /// Whether the operation succeeded
    pub success: bool,

    /// Original commit hash that was reverted (for single commit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverted: Option<String>,

    /// New commit hash created by revert (for single commit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_commit: Option<String>,

    /// Commit message of the revert commit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Multiple reverts info (for multiple commits)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverts: Option<Vec<RevertCommitInfo>>,

    /// Files with conflicts (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<String>>,

    /// Current revert state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Mainline parent used (for merge commits)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mainline_used: Option<u32>,

    /// Whether changes are staged but not committed (no_commit mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staged: Option<bool>,

    /// Error message if operation failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Information about a single reverted commit
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RevertCommitInfo {
    /// Original commit hash that was reverted
    pub original: String,

    /// New revert commit hash (if successfully applied)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert_commit: Option<String>,

    /// Whether this revert succeeded
    pub success: bool,

    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[tool_metadata(
    description = "Create new commits that undo changes from previous commits"
)]
impl ToolArgs for GitRevertArgs {
    type Output = GitRevertOutput;
    type Prompts = RevertPrompts;

    const NAME: &'static str = GIT_REVERT;
    const CATEGORY: &'static str = CATEGORY_GIT;
    const DESCRIPTION: &'static str = "Create new commits that undo changes from previous commits";
}
