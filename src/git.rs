//! Git tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical name for git_add tool
pub const GIT_ADD: &str = "git_add";

/// Canonical name for git_branch_create tool
pub const GIT_BRANCH_CREATE: &str = "git_branch_create";

/// Canonical name for git_branch_delete tool
pub const GIT_BRANCH_DELETE: &str = "git_branch_delete";

/// Canonical name for git_branch_list tool
pub const GIT_BRANCH_LIST: &str = "git_branch_list";

/// Canonical name for git_branch_rename tool
pub const GIT_BRANCH_RENAME: &str = "git_branch_rename";

/// Canonical name for git_checkout tool
pub const GIT_CHECKOUT: &str = "git_checkout";

/// Canonical name for git_clone tool
pub const GIT_CLONE: &str = "git_clone";

/// Canonical name for git_commit tool
pub const GIT_COMMIT: &str = "git_commit";

/// Canonical name for git_diff tool
pub const GIT_DIFF: &str = "git_diff";

/// Canonical name for git_discover tool
pub const GIT_DISCOVER: &str = "git_discover";

/// Canonical name for git_fetch tool
pub const GIT_FETCH: &str = "git_fetch";

/// Canonical name for git_init tool
pub const GIT_INIT: &str = "git_init";

/// Canonical name for git_log tool
pub const GIT_LOG: &str = "git_log";

/// Canonical name for git_merge tool
pub const GIT_MERGE: &str = "git_merge";

/// Canonical name for git_open tool
pub const GIT_OPEN: &str = "git_open";

/// Canonical name for git_pull tool
pub const GIT_PULL: &str = "git_pull";

/// Canonical name for git_push tool
pub const GIT_PUSH: &str = "git_push";

/// Canonical name for git_worktree_add tool
pub const GIT_WORKTREE_ADD: &str = "git_worktree_add";

/// Canonical name for git_worktree_list tool
pub const GIT_WORKTREE_LIST: &str = "git_worktree_list";

/// Canonical name for git_worktree_lock tool
pub const GIT_WORKTREE_LOCK: &str = "git_worktree_lock";

/// Canonical name for git_worktree_prune tool
pub const GIT_WORKTREE_PRUNE: &str = "git_worktree_prune";

/// Canonical name for git_worktree_remove tool
pub const GIT_WORKTREE_REMOVE: &str = "git_worktree_remove";

/// Canonical name for git_worktree_unlock tool
pub const GIT_WORKTREE_UNLOCK: &str = "git_worktree_unlock";

/// Canonical name for git_remote_add tool
pub const GIT_REMOTE_ADD: &str = "git_remote_add";

/// Canonical name for git_remote_list tool
pub const GIT_REMOTE_LIST: &str = "git_remote_list";

/// Canonical name for git_remote_remove tool
pub const GIT_REMOTE_REMOVE: &str = "git_remote_remove";

/// Canonical name for git_reset tool
pub const GIT_RESET: &str = "git_reset";

/// Canonical name for git_status tool
pub const GIT_STATUS: &str = "git_status";

/// Canonical name for git_stash tool
pub const GIT_STASH: &str = "git_stash";

/// Canonical name for git_tag tool
pub const GIT_TAG: &str = "git_tag";

// ============================================================================
// GIT INIT
// ============================================================================

/// Arguments for `git_init` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitInitArgs {
    /// Path where to initialize the repository
    pub path: String,

    /// Create a bare repository (no working directory)
    #[serde(default)]
    pub bare: bool,

    /// Name of the initial branch (informational only, gix uses default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_branch: Option<String>,
}

/// Prompt arguments for `git_init` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitInitPromptArgs {}

// ============================================================================
// GIT OPEN
// ============================================================================

/// Arguments for `git_open` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitOpenArgs {
    /// Path to the existing repository
    pub path: String,
}

/// Prompt arguments for `git_open` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitOpenPromptArgs {}

// ============================================================================
// GIT CLONE
// ============================================================================

/// Arguments for `git_clone` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitCloneArgs {
    /// Git URL to clone from (https:// or git://)
    pub url: String,

    /// Local path to clone into
    pub path: String,

    /// Specific branch to checkout (defaults to repository default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,

    /// Shallow clone depth (minimum: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
}

/// Prompt arguments for `git_clone` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitClonePromptArgs {}

// ============================================================================
// GIT DISCOVER
// ============================================================================

/// Arguments for `git_discover` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitDiscoverArgs {
    /// Path to search from (can be subdirectory within a repo)
    pub path: String,
}

/// Prompt arguments for `git_discover` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitDiscoverPromptArgs {}

// ============================================================================
// GIT ADD
// ============================================================================

/// Arguments for `git_add` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitAddArgs {
    /// Path to repository
    pub path: String,

    /// Specific file paths to stage
    /// 
    /// Accepts both single string and array: `paths: "file.rs"` or `paths: ["file1.rs", "file2.rs"]`
    #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub paths: Vec<String>,

    /// Stage all modified files
    #[serde(default)]
    pub all: bool,

    /// Force add files even if in .gitignore
    #[serde(default)]
    pub force: bool,
}

/// Prompt arguments for `git_add` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitAddPromptArgs {}

// ============================================================================
// GIT COMMIT
// ============================================================================

/// Arguments for `git_commit` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitCommitArgs {
    /// Path to repository
    pub path: String,

    /// Commit message
    pub message: String,

    /// Author name (optional, uses git config if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,

    /// Author email (optional, uses git config if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,

    /// Stage all modified tracked files before committing
    #[serde(default)]
    pub all: bool,
}

/// Prompt arguments for `git_commit` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitCommitPromptArgs {}

// ============================================================================
// GIT LOG
// ============================================================================

/// Arguments for `git_log` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitLogArgs {
    /// Path to repository
    pub path: String,

    /// Maximum number of commits to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<usize>,

    /// Number of commits to skip
    #[serde(default)]
    pub skip: usize,

    /// Filter commits by file path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_filter: Option<String>,
}

/// Prompt arguments for `git_log` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitLogPromptArgs {}

// ============================================================================
// GIT DIFF
// ============================================================================

/// Arguments for `git_diff` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitDiffArgs {
    /// Path to repository
    pub path: String,

    /// Source revision (commit hash, branch name, or 'HEAD')
    pub from: String,

    /// Target revision (optional, defaults to working directory)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// Prompt arguments for `git_diff` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitDiffPromptArgs {}

// ============================================================================
// GIT BRANCH CREATE
// ============================================================================

/// Arguments for `git_branch_create` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitBranchCreateArgs {
    /// Path to repository
    pub path: String,

    /// Name for new branch
    pub branch: String,

    /// Starting point (defaults to HEAD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_branch: Option<String>,

    /// Force creation (overwrite if exists)
    #[serde(default)]
    pub force: bool,

    /// Checkout the branch after creation
    #[serde(default)]
    pub checkout: bool,
}

/// Prompt arguments for `git_branch_create` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitBranchCreatePromptArgs {}

// ============================================================================
// GIT BRANCH DELETE
// ============================================================================

/// Arguments for `git_branch_delete` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitBranchDeleteArgs {
    /// Path to repository
    pub path: String,

    /// Name of branch to delete
    pub branch: String,

    /// Force deletion
    #[serde(default)]
    pub force: bool,
}

/// Prompt arguments for `git_branch_delete` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitBranchDeletePromptArgs {}

// ============================================================================
// GIT BRANCH LIST
// ============================================================================

/// Arguments for `git_branch_list` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitBranchListArgs {
    /// Path to repository
    pub path: String,
}

/// Prompt arguments for `git_branch_list` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitBranchListPromptArgs {}

// ============================================================================
// GIT BRANCH RENAME
// ============================================================================

/// Arguments for `git_branch_rename` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitBranchRenameArgs {
    /// Path to repository
    pub path: String,

    /// Current branch name
    pub old_name: String,

    /// New branch name
    pub new_name: String,

    /// Force rename (overwrite if new name exists)
    #[serde(default)]
    pub force: bool,
}

/// Prompt arguments for `git_branch_rename` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitBranchRenamePromptArgs {}

// ============================================================================
// GIT CHECKOUT
// ============================================================================

/// Arguments for `git_checkout` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitCheckoutArgs {
    /// Path to repository
    pub path: String,

    /// Target reference (branch, tag, or commit)
    pub target: String,

    /// Specific file paths to restore from the target reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,

    /// Create new branch before checking out
    #[serde(default)]
    pub create: bool,

    /// Force checkout (discard local changes)
    #[serde(default)]
    pub force: bool,
}

/// Prompt arguments for `git_checkout` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitCheckoutPromptArgs {}

// ============================================================================
// GIT FETCH
// ============================================================================

fn default_remote() -> String {
    "origin".to_string()
}

/// Arguments for `git_fetch` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitFetchArgs {
    /// Path to repository
    pub path: String,

    /// Remote name (defaults to "origin")
    #[serde(default = "default_remote")]
    pub remote: String,

    /// Refspecs to fetch (e.g., ["refs/heads/main:refs/remotes/origin/main"]).
    /// If empty, uses repository's configured refspecs for the remote.
    /// 
    /// Accepts both single string and array: `refspecs: "main"` or `refspecs: ["main", "develop"]`
    #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub refspecs: Vec<String>,

    /// Prune remote-tracking branches that no longer exist on remote (default: false)
    #[serde(default)]
    pub prune: bool,
}

/// Prompt arguments for `git_fetch` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitFetchPromptArgs {}

// ============================================================================
// GIT MERGE
// ============================================================================

fn default_true() -> bool {
    true
}

/// Arguments for `git_merge` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitMergeArgs {
    /// Path to repository
    pub path: String,

    /// Branch or commit to merge into current branch
    pub branch: String,

    /// Allow fast-forward merges when possible (default: true).
    /// When false, always creates a merge commit even if fast-forward is possible.
    #[serde(default = "default_true")]
    pub fast_forward: bool,

    /// Automatically create merge commit (default: true).
    /// When false, performs merge but leaves changes staged for manual commit.
    #[serde(default = "default_true")]
    pub auto_commit: bool,
}

/// Prompt arguments for `git_merge` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitMergePromptArgs {}

// ============================================================================
// GIT WORKTREE ADD
// ============================================================================

/// Arguments for `git_worktree_add` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitWorktreeAddArgs {
    /// Path to repository
    pub path: String,

    /// Path where the new worktree will be created
    pub worktree_path: String,

    /// Branch or commit to checkout in the worktree (optional, defaults to HEAD).
    /// Can be a branch name, tag, or commit SHA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,

    /// Force creation even if worktree path already exists (default: false)
    #[serde(default)]
    pub force: bool,
}

/// Prompt arguments for `git_worktree_add` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitWorktreeAddPromptArgs {}

// ============================================================================
// GIT WORKTREE LIST
// ============================================================================

/// Arguments for `git_worktree_list` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitWorktreeListArgs {
    /// Path to repository
    pub path: String,
}

/// Prompt arguments for `git_worktree_list` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitWorktreeListPromptArgs {}

// ============================================================================
// GIT WORKTREE LOCK
// ============================================================================

/// Arguments for `git_worktree_lock` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitWorktreeLockArgs {
    /// Path to repository
    pub path: String,

    /// Path to the worktree to lock (prevents deletion)
    pub worktree_path: String,

    /// Optional reason for locking (e.g., "On removable drive").
    /// Stored in the lock file for documentation purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Prompt arguments for `git_worktree_lock` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitWorktreeLockPromptArgs {}

// ============================================================================
// GIT WORKTREE UNLOCK
// ============================================================================

/// Arguments for `git_worktree_unlock` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitWorktreeUnlockArgs {
    /// Path to repository
    pub path: String,

    /// Path to worktree to unlock
    pub worktree_path: String,
}

/// Prompt arguments for `git_worktree_unlock` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitWorktreeUnlockPromptArgs {}

// ============================================================================
// GIT WORKTREE PRUNE
// ============================================================================

/// Arguments for `git_worktree_prune` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitWorktreePruneArgs {
    /// Path to repository
    pub path: String,
}

/// Prompt arguments for `git_worktree_prune` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitWorktreePrunePromptArgs {}

// ============================================================================
// GIT WORKTREE REMOVE
// ============================================================================

/// Arguments for `git_worktree_remove` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitWorktreeRemoveArgs {
    /// Path to repository
    pub path: String,

    /// Path to the worktree to remove (both working directory and admin files)
    pub worktree_path: String,

    /// Force removal even if worktree is locked (default: false)
    #[serde(default)]
    pub force: bool,
}

/// Prompt arguments for `git_worktree_remove` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitWorktreeRemovePromptArgs {}

// ============================================================================
// GIT PULL
// ============================================================================

/// Arguments for `git_pull` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitPullArgs {
    /// Path to repository
    pub path: String,

    /// Remote name (defaults to "origin")
    #[serde(default = "default_remote")]
    pub remote: String,

    /// Allow fast-forward merges (default: true)
    #[serde(default = "default_true")]
    pub fast_forward: bool,

    /// Automatically create merge commit (default: true)
    #[serde(default = "default_true")]
    pub auto_commit: bool,
}

/// Prompt arguments for `git_pull` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitPullPromptArgs {}

// ============================================================================
// GIT PUSH
// ============================================================================

/// Arguments for `git_push` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitPushArgs {
    /// Path to repository
    pub path: String,

    /// Remote name (defaults to "origin")
    #[serde(default = "default_remote")]
    pub remote: String,

    /// Refspecs to push (e.g., ["refs/heads/main", "refs/tags/v1.0"]).
    /// Empty list pushes the current branch to the remote.
    /// 
    /// Accepts both single string and array: `refspecs: "main"` or `refspecs: ["main", "develop"]`
    #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub refspecs: Vec<String>,

    /// Force push (use with caution in shared repositories)
    #[serde(default)]
    pub force: bool,

    /// Push all tags to the remote
    #[serde(default)]
    pub tags: bool,

    /// Push operation timeout in seconds (default: 300 for 5 minutes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u64>,
}

/// Prompt arguments for `git_push` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitPushPromptArgs {}

// ============================================================================
// GIT REMOTE ADD
// ============================================================================

/// Arguments for `git_remote_add` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitRemoteAddArgs {
    /// Path to repository
    pub path: String,

    /// Remote name (e.g., "origin", "upstream")
    pub name: String,

    /// Remote URL (https, git, ssh, or file URL)
    pub url: String,

    /// Force add (overwrite existing remote with same name)
    #[serde(default)]
    pub force: bool,
}

/// Prompt arguments for `git_remote_add` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitRemoteAddPromptArgs {}

// ============================================================================
// GIT REMOTE LIST
// ============================================================================

/// Arguments for `git_remote_list` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitRemoteListArgs {
    /// Path to repository
    pub path: String,
}

/// Prompt arguments for `git_remote_list` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitRemoteListPromptArgs {}

// ============================================================================
// GIT REMOTE REMOVE
// ============================================================================

/// Arguments for `git_remote_remove` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitRemoteRemoveArgs {
    /// Path to repository
    pub path: String,

    /// Remote name to remove (e.g., "origin", "upstream")
    pub name: String,
}

/// Prompt arguments for `git_remote_remove` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitRemoteRemovePromptArgs {}

// ============================================================================
// GIT RESET
// ============================================================================

/// Reset mode for git reset operation
#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResetMode {
    /// Soft reset - move HEAD only
    Soft,
    /// Mixed reset - move HEAD and reset index
    Mixed,
    /// Hard reset - move HEAD, reset index, and reset working directory
    Hard,
}

fn default_reset_mode() -> ResetMode {
    ResetMode::Mixed
}

/// Arguments for `git_reset` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitResetArgs {
    /// Path to repository
    pub path: String,

    /// Target commit (hash, ref, or symbolic name like "HEAD~1")
    pub target: String,

    /// Reset mode: soft, mixed, or hard (default: mixed)
    #[serde(default = "default_reset_mode")]
    pub mode: ResetMode,
}

/// Prompt arguments for `git_reset` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitResetPromptArgs {}

// ============================================================================
// GIT STATUS
// ============================================================================

/// Arguments for `git_status` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitStatusArgs {
    /// Path to repository
    pub path: String,
}

/// Prompt arguments for `git_status` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitStatusPromptArgs {}

// ============================================================================
// GIT TAG
// ============================================================================

/// Arguments for `git_tag` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitTagArgs {
    /// Path to repository
    pub path: String,

    /// Operation: "create", "delete", or "list" (default: "list")
    #[serde(default = "default_tag_operation")]
    pub operation: String,

    /// Tag name (required for create and delete operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Tag message for annotated tags (if provided, creates annotated tag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Target commit (defaults to HEAD if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Force create/delete (overwrite if exists)
    #[serde(default)]
    pub force: bool,
}

fn default_tag_operation() -> String {
    "list".to_string()
}

/// Prompt arguments for `git_tag` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitTagPromptArgs {}

// ============================================================================
// GIT STASH
// ============================================================================

/// Arguments for `git_stash` tool
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GitStashArgs {
    /// Path to repository
    pub path: String,

    /// Operation: "save" or "pop" (default: "save")
    #[serde(default = "default_stash_operation")]
    pub operation: String,

    /// Optional stash message (for save operation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Include untracked files (default: true)
    #[serde(default = "default_true")]
    pub include_untracked: bool,
}

fn default_stash_operation() -> String {
    "save".to_string()
}

/// Prompt arguments for `git_stash` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitStashPromptArgs {}
