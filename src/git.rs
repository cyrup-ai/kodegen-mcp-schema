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

/// Canonical name for git_history tool
pub const GIT_HISTORY: &str = "git_history";

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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
// GIT HISTORY
// ============================================================================

/// Arguments for `git_history` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GitHistoryArgs {
    /// Path to repository
    pub path: String,

    /// File path to investigate
    pub file: String,

    /// Maximum number of commits to return (default: 20)
    #[serde(default = "default_history_limit")]
    pub limit: usize,

    /// Optional regex pattern to filter diffs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Start revision (default: HEAD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,

    /// End revision (for range mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

fn default_history_limit() -> usize {
    20
}

/// Prompt arguments for `git_history` tool
#[derive(Deserialize, JsonSchema)]
pub struct GitHistoryPromptArgs {}

// ============================================================================
// GIT DIFF
// ============================================================================

/// Arguments for `git_diff` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
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

// ============================================================================
// OUTPUT TYPES
// ============================================================================

use crate::ToolArgs;

// ----------------------------------------------------------------------------
// Shared Types
// ----------------------------------------------------------------------------

/// Commit information returned by git_log
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitCommitInfo {
    /// Full commit hash
    pub id: String,
    /// Author information
    pub author: GitAuthorInfo,
    /// Commit summary (first line of message)
    pub summary: String,
    /// Commit timestamp in RFC3339 format
    pub time: String,
}

/// Author information for commits
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitAuthorInfo {
    /// Author name
    pub name: String,
    /// Author email
    pub email: String,
    /// Authoring time in RFC3339 format
    pub time: String,
}

/// File change information for diffs
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitDiffFile {
    /// File path
    pub path: String,
    /// Type of change
    pub change_type: String,
    /// Number of lines added
    pub additions: u32,
    /// Number of lines deleted
    pub deletions: u32,
}

/// Worktree information
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeInfo {
    /// Path to the worktree
    pub path: String,
    /// Path to the .git directory
    pub git_dir: String,
    /// Whether this is the main worktree
    pub is_main: bool,
    /// Whether this is a bare repository
    pub is_bare: bool,
    /// HEAD commit hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_commit: Option<String>,
    /// Checked-out branch name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_branch: Option<String>,
    /// Whether the worktree is locked
    pub is_locked: bool,
    /// Lock reason if locked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_reason: Option<String>,
    /// Whether HEAD is detached
    pub is_detached: bool,
}

/// Remote repository information
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRemoteInfo {
    /// Remote name (e.g., "origin")
    pub name: String,
    /// Fetch URL
    pub fetch_url: String,
    /// Push URL
    pub push_url: String,
}

/// Tag information
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitTagInfo {
    /// Tag name
    pub name: String,
    /// Whether this is an annotated tag
    pub is_annotated: bool,
    /// Target commit hash
    pub target_commit: String,
    /// Tag message (for annotated tags)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Tag timestamp in RFC3339 format
    pub timestamp: String,
}

// ----------------------------------------------------------------------------
// Git Init Output
// ----------------------------------------------------------------------------

/// Output from `git_init` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitInitOutput {
    pub success: bool,
    pub path: String,
    pub bare: bool,
    pub message: String,
}

impl ToolArgs for GitInitArgs {
    type Output = GitInitOutput;
}

// ----------------------------------------------------------------------------
// Git Open Output
// ----------------------------------------------------------------------------

/// Output from `git_open` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitOpenOutput {
    pub success: bool,
    pub path: String,
    pub branch: String,
    pub is_clean: bool,
    pub message: String,
}

impl ToolArgs for GitOpenArgs {
    type Output = GitOpenOutput;
}

// ----------------------------------------------------------------------------
// Git Clone Output
// ----------------------------------------------------------------------------

/// Output from `git_clone` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitCloneOutput {
    pub success: bool,
    pub url: String,
    pub path: String,
    pub branch: String,
    pub shallow: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
    pub message: String,
}

impl ToolArgs for GitCloneArgs {
    type Output = GitCloneOutput;
}

// ----------------------------------------------------------------------------
// Git Discover Output
// ----------------------------------------------------------------------------

/// Output from `git_discover` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitDiscoverOutput {
    pub success: bool,
    pub searched_from: String,
    pub repo_root: String,
    pub message: String,
}

impl ToolArgs for GitDiscoverArgs {
    type Output = GitDiscoverOutput;
}

// ----------------------------------------------------------------------------
// Git Add Output
// ----------------------------------------------------------------------------

/// Output from `git_add` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitAddOutput {
    pub success: bool,
    pub all: bool,
    pub paths: Vec<String>,
    pub count: usize,
}

impl ToolArgs for GitAddArgs {
    type Output = GitAddOutput;
}

// ----------------------------------------------------------------------------
// Git Commit Output
// ----------------------------------------------------------------------------

/// Output from `git_commit` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitCommitOutput {
    pub success: bool,
    pub commit_id: String,
    pub message: String,
    pub file_count: usize,
}

impl ToolArgs for GitCommitArgs {
    type Output = GitCommitOutput;
}

// ----------------------------------------------------------------------------
// Git Log Output
// ----------------------------------------------------------------------------

/// Output from `git_log` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitLogOutput {
    pub success: bool,
    pub commits: Vec<GitCommitInfo>,
    pub count: usize,
}

impl ToolArgs for GitLogArgs {
    type Output = GitLogOutput;
}

// ----------------------------------------------------------------------------
// Git History Output
// ----------------------------------------------------------------------------

/// A commit in the history with its diff
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHistoryCommit {
    /// Short commit hash (7 chars)
    pub id: String,
    /// Commit summary message
    pub summary: String,
    /// Commit timestamp (RFC3339)
    pub time: String,
    /// Number of lines added
    pub additions: u32,
    /// Number of lines deleted
    pub deletions: u32,
    /// Unified diff content
    pub diff: String,
}

/// Output from `git_history` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHistoryOutput {
    pub success: bool,
    /// File path that was analyzed
    pub file: String,
    /// Mode: "commits" or "range"
    pub mode: String,
    /// Total commits examined (commits mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_examined: Option<usize>,
    /// List of commits with diffs (commits mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<GitHistoryCommit>>,
    /// Start revision (range mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// End revision (range mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    /// Total additions (range mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions: Option<u32>,
    /// Total deletions (range mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletions: Option<u32>,
    /// Cumulative diff (range mode only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
}

impl ToolArgs for GitHistoryArgs {
    type Output = GitHistoryOutput;
}

// ----------------------------------------------------------------------------
// Git Diff Output
// ----------------------------------------------------------------------------

/// Output from `git_diff` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitDiffOutput {
    pub success: bool,
    pub from: String,
    pub to: String,
    pub files_changed: u32,
    pub insertions: u32,
    pub deletions: u32,
    pub files: Vec<GitDiffFile>,
}

impl ToolArgs for GitDiffArgs {
    type Output = GitDiffOutput;
}

// ----------------------------------------------------------------------------
// Git Branch Create Output
// ----------------------------------------------------------------------------

/// Output from `git_branch_create` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitBranchCreateOutput {
    pub success: bool,
    pub branch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_branch: Option<String>,
    pub message: String,
}

impl ToolArgs for GitBranchCreateArgs {
    type Output = GitBranchCreateOutput;
}

// ----------------------------------------------------------------------------
// Git Branch Delete Output
// ----------------------------------------------------------------------------

/// Output from `git_branch_delete` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitBranchDeleteOutput {
    pub success: bool,
    pub branch: String,
    pub message: String,
}

impl ToolArgs for GitBranchDeleteArgs {
    type Output = GitBranchDeleteOutput;
}

// ----------------------------------------------------------------------------
// Git Branch List Output
// ----------------------------------------------------------------------------

/// Output from `git_branch_list` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitBranchListOutput {
    pub success: bool,
    pub branches: Vec<String>,
    pub count: usize,
}

impl ToolArgs for GitBranchListArgs {
    type Output = GitBranchListOutput;
}

// ----------------------------------------------------------------------------
// Git Branch Rename Output
// ----------------------------------------------------------------------------

/// Output from `git_branch_rename` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitBranchRenameOutput {
    pub success: bool,
    pub old_name: String,
    pub new_name: String,
    pub message: String,
}

impl ToolArgs for GitBranchRenameArgs {
    type Output = GitBranchRenameOutput;
}

// ----------------------------------------------------------------------------
// Git Checkout Output
// ----------------------------------------------------------------------------

/// Output from `git_checkout` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitCheckoutOutput {
    pub success: bool,
    pub target: String,
    pub created: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    pub message: String,
}

impl ToolArgs for GitCheckoutArgs {
    type Output = GitCheckoutOutput;
}

// ----------------------------------------------------------------------------
// Git Fetch Output
// ----------------------------------------------------------------------------

/// Output from `git_fetch` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitFetchOutput {
    pub success: bool,
    pub remote: String,
    pub pruned: bool,
}

impl ToolArgs for GitFetchArgs {
    type Output = GitFetchOutput;
}

// ----------------------------------------------------------------------------
// Git Merge Output
// ----------------------------------------------------------------------------

/// Output from `git_merge` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitMergeOutput {
    pub success: bool,
    pub merge_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    pub message: String,
}

impl ToolArgs for GitMergeArgs {
    type Output = GitMergeOutput;
}

// ----------------------------------------------------------------------------
// Git Worktree Add Output
// ----------------------------------------------------------------------------

/// Output from `git_worktree_add` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeAddOutput {
    pub success: bool,
    pub worktree_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    pub message: String,
}

impl ToolArgs for GitWorktreeAddArgs {
    type Output = GitWorktreeAddOutput;
}

// ----------------------------------------------------------------------------
// Git Worktree List Output
// ----------------------------------------------------------------------------

/// Output from `git_worktree_list` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeListOutput {
    pub success: bool,
    pub worktrees: Vec<GitWorktreeInfo>,
    pub count: usize,
}

impl ToolArgs for GitWorktreeListArgs {
    type Output = GitWorktreeListOutput;
}

// ----------------------------------------------------------------------------
// Git Worktree Lock Output
// ----------------------------------------------------------------------------

/// Output from `git_worktree_lock` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeLockOutput {
    pub success: bool,
    pub worktree_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub message: String,
}

impl ToolArgs for GitWorktreeLockArgs {
    type Output = GitWorktreeLockOutput;
}

// ----------------------------------------------------------------------------
// Git Worktree Unlock Output
// ----------------------------------------------------------------------------

/// Output from `git_worktree_unlock` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeUnlockOutput {
    pub success: bool,
    pub worktree_path: String,
    pub message: String,
}

impl ToolArgs for GitWorktreeUnlockArgs {
    type Output = GitWorktreeUnlockOutput;
}

// ----------------------------------------------------------------------------
// Git Worktree Prune Output
// ----------------------------------------------------------------------------

/// Output from `git_worktree_prune` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreePruneOutput {
    pub success: bool,
    pub pruned_count: usize,
    pub message: String,
}

impl ToolArgs for GitWorktreePruneArgs {
    type Output = GitWorktreePruneOutput;
}

// ----------------------------------------------------------------------------
// Git Worktree Remove Output
// ----------------------------------------------------------------------------

/// Output from `git_worktree_remove` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitWorktreeRemoveOutput {
    pub success: bool,
    pub worktree_path: String,
    pub message: String,
}

impl ToolArgs for GitWorktreeRemoveArgs {
    type Output = GitWorktreeRemoveOutput;
}

// ----------------------------------------------------------------------------
// Git Pull Output
// ----------------------------------------------------------------------------

/// Output from `git_pull` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitPullOutput {
    pub success: bool,
    pub remote: String,
    pub merge_outcome: String,
}

impl ToolArgs for GitPullArgs {
    type Output = GitPullOutput;
}

// ----------------------------------------------------------------------------
// Git Push Output
// ----------------------------------------------------------------------------

/// Output from `git_push` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitPushOutput {
    pub success: bool,
    pub remote: String,
    pub refs_pushed: u32,
    pub tags_pushed: u32,
    pub force: bool,
    pub warnings: Vec<String>,
}

impl ToolArgs for GitPushArgs {
    type Output = GitPushOutput;
}

// ----------------------------------------------------------------------------
// Git Remote Add Output
// ----------------------------------------------------------------------------

/// Output from `git_remote_add` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRemoteAddOutput {
    pub success: bool,
    pub name: String,
    pub url: String,
    pub message: String,
}

impl ToolArgs for GitRemoteAddArgs {
    type Output = GitRemoteAddOutput;
}

// ----------------------------------------------------------------------------
// Git Remote List Output
// ----------------------------------------------------------------------------

/// Output from `git_remote_list` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRemoteListOutput {
    pub success: bool,
    pub count: usize,
    pub remotes: Vec<GitRemoteInfo>,
}

impl ToolArgs for GitRemoteListArgs {
    type Output = GitRemoteListOutput;
}

// ----------------------------------------------------------------------------
// Git Remote Remove Output
// ----------------------------------------------------------------------------

/// Output from `git_remote_remove` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitRemoteRemoveOutput {
    pub success: bool,
    pub name: String,
    pub message: String,
}

impl ToolArgs for GitRemoteRemoveArgs {
    type Output = GitRemoteRemoveOutput;
}

// ----------------------------------------------------------------------------
// Git Reset Output
// ----------------------------------------------------------------------------

/// Output from `git_reset` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitResetOutput {
    pub success: bool,
    pub mode: String,
    pub target: String,
}

impl ToolArgs for GitResetArgs {
    type Output = GitResetOutput;
}

// ----------------------------------------------------------------------------
// Git Status Output
// ----------------------------------------------------------------------------

/// Output from `git_status` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitStatusOutput {
    pub success: bool,
    pub branch: String,
    pub commit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ahead: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behind: Option<u32>,
    pub is_clean: bool,
    pub is_detached: bool,
}

impl ToolArgs for GitStatusArgs {
    type Output = GitStatusOutput;
}

// ----------------------------------------------------------------------------
// Git Stash Output
// ----------------------------------------------------------------------------

/// Output from `git_stash` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitStashOutput {
    pub success: bool,
    pub operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_hash: Option<String>,
}

impl ToolArgs for GitStashArgs {
    type Output = GitStashOutput;
}

// ----------------------------------------------------------------------------
// Git Tag Output
// ----------------------------------------------------------------------------

/// Output from `git_tag` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitTagOutput {
    pub success: bool,
    pub operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_annotated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GitTagInfo>>,
}

impl ToolArgs for GitTagArgs {
    type Output = GitTagOutput;
}
