//! Prompt messages for git_worktree_lock tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitWorktreeLockPromptArgs;

/// Prompt provider for git_worktree_lock tool
pub struct WorktreeLockPrompts;

impl PromptProvider for WorktreeLockPrompts {
    type PromptArgs = GitWorktreeLockPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("prevent") => prompt_prevent(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: basic, prevent".to_string()),
                required: Some(false),
            }
        ]
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I lock a worktree?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Lock a worktree to prevent automatic removal during git maintenance operations.\\n\\n\\\
                 BASIC EXAMPLE:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/repo\\\", \\\"worktree_path\\\": \\\"/repo-feature\\\", \\\"reason\\\": \\\"In use on server\\\"}\\n\\\
                 ```\\n\\\
                 Locked worktrees are protected from git worktree prune commands.\\n\\n\\\
                 REMOVABLE DRIVE:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/media/usb/repo\\\", \\\"worktree_path\\\": \\\"/media/usb/repo-work\\\", \\\"reason\\\": \\\"USB drive may be disconnected\\\"}\\n\\\
                 ```\\n\\\
                 Lock prevents git from removing stale references when drive is absent.\\n\\n\\\
                 NETWORK MOUNT:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/mnt/nfs/repo\\\", \\\"worktree_path\\\": \\\"/mnt/nfs/repo-share\\\", \\\"reason\\\": \\\"Network storage\\\"}\\n\\\
                 ```\\n\\\
                 Network latency or intermittent disconnections won't trigger auto-pruning.\\n\\n\\\
                 DEPLOYMENT:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/var/repo\\\", \\\"worktree_path\\\": \\\"/var/live-deploy\\\", \\\"reason\\\": \\\"Active production deployment\\\"}\\n\\\
                 ```\\n\\\
                 Production worktrees remain safe from accidental cleanup operations.\\n\\n\\\
                 PARAMETERS:\\n\\\
                 - path: Main repository directory (required)\\n\\\
                 - worktree_path: Absolute path to the worktree to lock (required)\\n\\\
                 - reason: Human-readable reason for lock, logged in .git/worktrees/<name>/locked (optional)\\n\\n\\\
                 LOCKING MECHANISM:\\n\\\
                 Creates .git/worktrees/<worktree-name>/locked file. Git skips this worktree during prune operations and reports it as locked when listing."
            ),
        },
    ]
}

fn prompt_prevent() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("Why would I lock a worktree?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Lock worktrees to prevent git from automatically removing them during maintenance operations.\\n\\n\\\
                 REMOVABLE STORAGE:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/media/usb/code/repo\\\", \\\"worktree_path\\\": \\\"/media/usb/code/temp-work\\\", \\\"reason\\\": \\\"USB drive may be disconnected\\\"}\\n\\\
                 ```\\n\\\
                 Without lock: Worktree reference deleted when drive offline for extended time.\\n\\\
                 With lock: Reference preserved; you can reconnect and resume work.\\n\\n\\\
                 NETWORK MOUNT:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/mnt/shared-repo\\\", \\\"worktree_path\\\": \\\"/mnt/shared-repo-team\\\", \\\"reason\\\": \\\"Team worktree on NFS share\\\"}\\n\\\
                 ```\\n\\\
                 Without lock: Network latency or brief disconnection triggers removal of worktree index.\\n\\\
                 With lock: Worktree survives temporary network issues.\\n\\n\\\
                 EPHEMERAL FILESYSTEM:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/tmp/build-repo\\\", \\\"worktree_path\\\": \\\"/tmp/build-work\\\", \\\"reason\\\": \\\"Temporary build directory\\\"}\\n\\\
                 ```\\n\\\
                 Without lock: System cleanup (tmpwatch, systemd-tmpfiles) may remove directory; git loses worktree metadata.\\n\\\
                 With lock: Git won't prune the reference even if directory is missing.\\n\\n\\\
                 CRITICAL DEPLOYMENT:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/var/www/repo\\\", \\\"worktree_path\\\": \\\"/var/www/live-app\\\", \\\"reason\\\": \\\"Production application serving traffic\\\"}\\n\\\
                 ```\\n\\\
                 Without lock: Maintenance script that prunes unused worktrees could remove active deployment.\\n\\\
                 With lock: Production worktree is protected from accidental removal.\\n\\n\\\
                 CI/CD PIPELINE:\\n\\\
                 ```json\\n\\\
                 {\\\"path\\\": \\\"/opt/ci/repo\\\", \\\"worktree_path\\\": \\\"/opt/ci/build-worker-1\\\", \\\"reason\\\": \\\"Active CI build process\\\"}\\n\\\
                 ```\\n\\\
                 Without lock: Long-running builds could be interrupted by repository cleanup.\\n\\\
                 With lock: Worktree remains stable even during scheduled maintenance windows.\\n\\n\\\
                 GIT BEHAVIOR:\\n\\\
                 - git worktree prune: Skips locked worktrees entirely\\n\\\
                 - git worktree list: Shows locked status with reason\\n\\\
                 - git worktree unlock <path>: Required to remove lock before pruning\\n\\\
                 - Lock file: .git/worktrees/<name>/locked (exists as marker; content is reason or empty)\\n\\n\\\
                 WHEN TO LOCK:\\n\\\
                 Lock when worktree is on storage that might disconnect, when directory uses ephemeral filesystem, when worktree serves active production or CI process, when multiple users share worktree access, or when maintenance scripts run frequently that could trigger pruning."
            ),
        },
    ]
}

