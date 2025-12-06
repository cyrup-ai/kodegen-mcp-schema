//! Git category module

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// SHARED TYPES (used by multiple tools)
// ============================================================================

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

// ============================================================================
// TOOL MODULES
// ============================================================================

pub mod init;
pub mod open;
pub mod clone;
pub mod discover;
pub mod add;
pub mod commit;
pub mod log;
pub mod history;
pub mod diff;
pub mod branch_create;
pub mod branch_delete;
pub mod branch_list;
pub mod branch_rename;
pub mod checkout;
pub mod cherry_pick;
pub mod config_get;
pub mod config_set;
pub mod fetch;
pub mod merge;
pub mod rebase;
pub mod worktree_add;
pub mod worktree_list;
pub mod worktree_lock;
pub mod worktree_unlock;
pub mod worktree_prune;
pub mod worktree_remove;
pub mod pull;
pub mod push;
pub mod remote_add;
pub mod remote_list;
pub mod remote_remove;
pub mod reset;
pub mod revert;
pub mod status;
pub mod stash;
pub mod stash_apply;
pub mod stash_list;
pub mod stash_pop;
pub mod tag;
pub mod tag_create;
pub mod tag_list;
pub mod show;

// ============================================================================
// RE-EXPORTS
// ============================================================================

// Re-export init tool
pub use init::{
    GIT_INIT,
    GitInitArgs,
    GitInitOutput,
    GitInitPromptArgs,
    InitPrompts,
};

// Re-export open tool
pub use open::{
    GIT_OPEN,
    GitOpenArgs,
    GitOpenOutput,
    GitOpenPromptArgs,
    OpenPrompts,
};

// Re-export clone tool
pub use clone::{
    GIT_CLONE,
    GitCloneArgs,
    GitCloneOutput,
    GitClonePromptArgs,
    ClonePrompts,
};

// Re-export discover tool
pub use discover::{
    GIT_DISCOVER,
    GitDiscoverArgs,
    GitDiscoverOutput,
    GitDiscoverPromptArgs,
    DiscoverPrompts,
};

// Re-export add tool
pub use add::{
    GIT_ADD,
    GitAddArgs,
    GitAddOutput,
    GitAddPromptArgs,
    AddPrompts,
};

// Re-export commit tool
pub use commit::{
    GIT_COMMIT,
    GitCommitArgs,
    GitCommitOutput,
    GitCommitPromptArgs,
    CommitPrompts,
};

// Re-export log tool
pub use log::{
    GIT_LOG,
    GitLogArgs,
    GitLogOutput,
    GitLogPromptArgs,
    LogPrompts,
};

// Re-export history tool
pub use history::{
    GIT_HISTORY,
    GitHistoryArgs,
    GitHistoryOutput,
    GitHistoryPromptArgs,
    HistoryPrompts,
};

// Re-export diff tool
pub use diff::{
    GIT_DIFF,
    GitDiffArgs,
    GitDiffOutput,
    GitDiffPromptArgs,
    DiffPrompts,
};

// Re-export branch_create tool
pub use branch_create::{
    GIT_BRANCH_CREATE,
    GitBranchCreateArgs,
    GitBranchCreateOutput,
    GitBranchCreatePromptArgs,
    BranchCreatePrompts,
};

// Re-export branch_delete tool
pub use branch_delete::{
    GIT_BRANCH_DELETE,
    GitBranchDeleteArgs,
    GitBranchDeleteOutput,
    GitBranchDeletePromptArgs,
    BranchDeletePrompts,
};

// Re-export branch_list tool
pub use branch_list::{
    GIT_BRANCH_LIST,
    GitBranchListArgs,
    GitBranchListOutput,
    GitBranchListPromptArgs,
    BranchListPrompts,
};

// Re-export branch_rename tool
pub use branch_rename::{
    GIT_BRANCH_RENAME,
    GitBranchRenameArgs,
    GitBranchRenameOutput,
    GitBranchRenamePromptArgs,
    BranchRenamePrompts,
};

// Re-export checkout tool
pub use checkout::{
    GIT_CHECKOUT,
    GitCheckoutArgs,
    GitCheckoutOutput,
    GitCheckoutPromptArgs,
    GitCheckoutPrompts,
};

// Re-export cherry_pick tool
pub use cherry_pick::{
    GIT_CHERRY_PICK,
    GitCherryPickArgs,
    GitCherryPickOutput,
    GitCherryPickPromptArgs,
    CherryPickPrompts,
};

// Re-export config_get tool
pub use config_get::{
    GIT_CONFIG_GET,
    GitConfigGetArgs,
    GitConfigGetOutput,
    GitConfigGetPromptArgs,
    ConfigGetPrompts,
    ConfigValue,
};

// Re-export config_set tool
pub use config_set::{
    GIT_CONFIG_SET,
    GitConfigSetArgs,
    GitConfigSetOutput,
    GitConfigSetPromptArgs,
    ConfigSetPrompts,
};

// Re-export fetch tool
pub use fetch::{
    GIT_FETCH,
    GitFetchArgs,
    GitFetchOutput,
    GitFetchPromptArgs,
    FetchPrompts,
};

// Re-export merge tool
pub use merge::{
    GIT_MERGE,
    GitMergeArgs,
    GitMergeOutput,
    GitMergePromptArgs,
    MergePrompts,
};

// Re-export rebase tool
pub use rebase::{
    GIT_REBASE,
    GitRebaseArgs,
    GitRebaseOutput,
    GitRebasePromptArgs,
    RebasePrompts,
};

// Re-export worktree_add tool
pub use worktree_add::{
    GIT_WORKTREE_ADD,
    GitWorktreeAddArgs,
    GitWorktreeAddOutput,
    GitWorktreeAddPromptArgs,
    WorktreeAddPrompts,
};

// Re-export worktree_list tool
pub use worktree_list::{
    GIT_WORKTREE_LIST,
    GitWorktreeListArgs,
    GitWorktreeListOutput,
    GitWorktreeListPromptArgs,
    WorktreeListPrompts,
};

// Re-export worktree_lock tool
pub use worktree_lock::{
    GIT_WORKTREE_LOCK,
    GitWorktreeLockArgs,
    GitWorktreeLockOutput,
    GitWorktreeLockPromptArgs,
    WorktreeLockPrompts,
};

// Re-export worktree_unlock tool
pub use worktree_unlock::{
    GIT_WORKTREE_UNLOCK,
    GitWorktreeUnlockArgs,
    GitWorktreeUnlockOutput,
    GitWorktreeUnlockPromptArgs,
    WorktreeUnlockPrompts,
};

// Re-export worktree_prune tool
pub use worktree_prune::{
    GIT_WORKTREE_PRUNE,
    GitWorktreePruneArgs,
    GitWorktreePruneOutput,
    GitWorktreePrunePromptArgs,
    WorktreePrunePrompts,
};

// Re-export worktree_remove tool
pub use worktree_remove::{
    GIT_WORKTREE_REMOVE,
    GitWorktreeRemoveArgs,
    GitWorktreeRemoveOutput,
    GitWorktreeRemovePromptArgs,
    WorktreeRemovePrompts,
};

// Re-export pull tool
pub use pull::{
    GIT_PULL,
    GitPullArgs,
    GitPullOutput,
    GitPullPromptArgs,
    PullPrompts,
};

// Re-export push tool
pub use push::{
    GIT_PUSH,
    GitPushArgs,
    GitPushOutput,
    GitPushPromptArgs,
    PushPrompts,
};

// Re-export remote_add tool
pub use remote_add::{
    GIT_REMOTE_ADD,
    GitRemoteAddArgs,
    GitRemoteAddOutput,
    GitRemoteAddPromptArgs,
    RemoteAddPrompts,
};

// Re-export remote_list tool
pub use remote_list::{
    GIT_REMOTE_LIST,
    GitRemoteListArgs,
    GitRemoteListOutput,
    GitRemoteListPromptArgs,
    RemoteListPrompts,
};

// Re-export remote_remove tool
pub use remote_remove::{
    GIT_REMOTE_REMOVE,
    GitRemoteRemoveArgs,
    GitRemoteRemoveOutput,
    GitRemoteRemovePromptArgs,
    RemoteRemovePrompts,
};

// Re-export reset tool
pub use reset::{
    GIT_RESET,
    GitResetArgs,
    GitResetOutput,
    GitResetPromptArgs,
    ResetPrompts,
};

// Re-export revert tool
pub use revert::{
    GIT_REVERT,
    GitRevertArgs,
    GitRevertOutput,
    GitRevertPromptArgs,
    RevertPrompts,
};

// Re-export status tool
pub use status::{
    GIT_STATUS,
    GitStatusArgs,
    GitStatusOutput,
    GitStatusPromptArgs,
    StatusPrompts,
};

// Re-export stash tool
pub use stash::{
    GIT_STASH,
    GitStashArgs,
    GitStashOutput,
    GitStashSavePromptArgs,
    StashPrompts,
};

// Re-export stash_apply tool
pub use stash_apply::{
    GIT_STASH_APPLY,
    GitStashApplyArgs,
    GitStashApplyOutput,
    GitStashApplyPromptArgs,
    StashApplyPrompts,
};

// Re-export stash_list tool
pub use stash_list::{
    GIT_STASH_LIST,
    GitStashListArgs,
    GitStashListOutput,
    GitStashListPromptArgs,
    StashListPrompts,
    StashEntry,
};

// Re-export stash_pop tool
pub use stash_pop::{
    GitStashPopPromptArgs,
    GitStashPopPrompts,
};

// Re-export tag tool
pub use tag::{
    GIT_TAG,
    GitTagArgs,
    GitTagOutput,
    GitTagPromptArgs,
    TagPrompts,
};

// Re-export tag_create tool
pub use tag_create::{
    GIT_TAG_CREATE,
    GitTagCreateArgs,
    GitTagCreateOutput,
    GitTagCreatePromptArgs,
    TagCreatePrompts,
};

// Re-export tag_list tool
pub use tag_list::{
    GitTagListPromptArgs,
    GitTagListPrompts,
};

// Re-export show tool
pub use show::{
    GitShowPromptArgs,
    ShowPrompts,
};
