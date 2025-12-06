//! Prompt messages for git_clone tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitClonePromptArgs;

/// Prompt provider for git_clone tool
///
/// This is the ONLY way to provide prompts for git_clone - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ClonePrompts;

impl PromptProvider for ClonePrompts {
    type PromptArgs = GitClonePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("shallow") => prompt_shallow(),
            Some("branch") => prompt_branch(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show: basic (default), shallow, branch".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO CLONE GIT REPOSITORIES
// ============================================================================

/// Basic repository cloning with different URL formats
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I clone a Git repository using the git_clone tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_clone tool downloads a complete Git repository from a remote URL to your local filesystem. Here's how to use it:\n\n\
                 URL FORMATS:\n\n\
                 HTTPS (public repos):\n\
                 git_clone({\"url\": \"https://github.com/user/repo.git\", \"path\": \"/projects/repo\"})\n\
                 - No SSH key setup required\n\
                 - Works through firewalls\n\
                 - Best for public repositories\n\n\
                 SSH (contributors):\n\
                 git_clone({\"url\": \"git@github.com:user/repo.git\", \"path\": \"/projects/repo\"})\n\
                 - No password prompts after key setup\n\
                 - Preferred for active development\n\
                 - More secure for frequent operations\n\n\
                 Also supported: git:// protocol and local paths (file:///path/to/repo.git)\n\n\
                 RESPONSE FORMAT:\n\
                 {\"path\": \"/projects/repo\", \"branch\": \"main\", \"success\": true}\n\
                 - path: Absolute path where repository was cloned\n\
                 - branch: Default branch checked out\n\
                 - success: true if clone completed\n\n\
                 WHAT GETS CLONED:\n\
                 - All branches (as remote refs), full commit history, all tags\n\
                 - Checks out default branch (usually main or master)\n\
                 - Remote 'origin' automatically configured\n\
                 - Ready for: git_status, git_log, git_branch_list, git_pull\n\n\
                 EXAMPLES:\n\n\
                 Clone public repository:\n\
                 git_clone({\"url\": \"https://github.com/torvalds/linux.git\", \"path\": \"/projects/linux\"})\n\n\
                 Clone your project via SSH:\n\
                 git_clone({\"url\": \"git@github.com:myuser/myproject.git\", \"path\": \"/workspace/myproject\"})\n\n\
                 TROUBLESHOOTING:\n\
                 - \"path exists\": Directory already exists and not empty\n\
                 - \"authentication failed\": Configure credentials for private repos\n\
                 - \"repository not found\": URL incorrect or repo doesn't exist",
            ),
        },
    ]
}

/// Shallow clones for faster downloads and reduced disk usage
fn prompt_shallow() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I clone a repository faster with shallow clones?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Shallow cloning downloads only recent commit history, making clones much faster and using less disk space. Perfect for CI/CD and quick testing.\n\n\
                 DEPTH PARAMETER EXAMPLES:\n\n\
                 Latest only (fastest):\n\
                 git_clone({\"url\": \"https://github.com/user/repo.git\", \"path\": \"/projects/repo\", \"depth\": 1})\n\
                 - Only most recent commit\n\
                 - Perfect for build/deploy pipelines\n\n\
                 Recent history:\n\
                 git_clone({\"url\": \"https://github.com/user/repo.git\", \"path\": \"/projects/repo\", \"depth\": 10})\n\
                 - Last 10 commits\n\
                 - Good for code review\n\n\
                 Full clone (no depth):\n\
                 git_clone({\"url\": \"https://github.com/user/repo.git\", \"path\": \"/projects/repo\"})\n\
                 - Complete commit history\n\
                 - For development work\n\n\
                 SHALLOW BENEFITS:\n\
                 - 10x-100x faster download for large repos\n\
                 - Less disk space (gigabytes vs megabytes)\n\
                 - Lower bandwidth usage\n\
                 - Ideal for CI/CD pipelines\n\n\
                 DEPTH GUIDE:\n\
                 | Depth | Use Case | Impact |\n\
                 |-------|----------|--------|\n\
                 | 1 | CI/CD builds, deployment | Fastest, minimal size |\n\
                 | 10-50 | Testing, code review | Recent history |\n\
                 | 100+ | Analysis, archaeology | Extensive history |\n\
                 | none | Development, maintenance | Complete clone |\n\n\
                 REAL-WORLD EXAMPLE (Linux kernel):\n\
                 Full clone: 3.5 GB, 20+ minutes\n\
                 git_clone({\"url\": \"https://github.com/torvalds/linux.git\", \"path\": \"./linux\", \"depth\": 1})\n\
                 Shallow clone: 200 MB, 2 minutes\n\n\
                 COMPLETE WORKFLOW:\n\
                 1. Shallow clone for CI/CD:\n\
                    git_clone({\"url\": \"https://github.com/company/product.git\", \"path\": \"/build\", \"depth\": 1})\n\
                 2. Build project from latest code\n\
                 3. Deploy without needing full history\n\
                 Result: 90% faster than full clone\n\n\
                 LIMITATIONS:\n\
                 - Cannot view full git history beyond depth\n\
                 - git_log shows limited commits\n\
                 - Some git operations fail on old commits\n\
                 - Use full clone for development work\n\n\
                 BEST PRACTICES:\n\
                 - Use depth: 1 for automated builds\n\
                 - Use full clone for development\n\
                 - Combine with branch parameter for feature testing",
            ),
        },
    ]
}

/// Cloning specific branches directly
fn prompt_branch() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I clone a specific branch or tag?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Clone a repository and immediately check out a specific branch or tag. Useful for working on feature branches or specific releases.\n\n\
                 BRANCH CLONING EXAMPLES:\n\n\
                 Clone develop branch:\n\
                 git_clone({\"url\": \"https://github.com/user/repo.git\", \"path\": \"/projects/repo\", \"branch\": \"develop\"})\n\
                 - Checks out 'develop' branch immediately\n\
                 - All branches still available as remote refs\n\n\
                 Clone release tag:\n\
                 git_clone({\"url\": \"https://github.com/user/repo.git\", \"path\": \"/projects/v1.0.0\", \"branch\": \"v1.0.0\"})\n\
                 - Fixed point in history, reproducible builds\n\
                 - Repository in 'detached HEAD' state\n\n\
                 Branch vs Tag:\n\
                 - Branch: Continues to evolve, can use git_pull\n\
                 - Tag: Immutable snapshot, never changes\n\n\
                 Shallow + branch combo:\n\
                 git_clone({\"url\": \"https://github.com/user/repo.git\", \"path\": \"/projects/repo\", \"branch\": \"feature/api\", \"depth\": 1})\n\
                 - Fastest way to get specific branch\n\
                 - Ideal for CI testing of feature branches\n\n\
                 BRANCH BEHAVIOR:\n\
                 - Specified branch is checked out in working directory\n\
                 - All branches still available as remote-tracking refs\n\
                 - Can switch later with git_checkout\n\
                 - If branch doesn't exist, clone operation fails\n\
                 - Use git_branch_list to see all branches\n\n\
                 USE CASES:\n\n\
                 Feature branch work:\n\
                 git_clone({\"url\": \"git@github.com:team/project.git\", \"path\": \"./feature-work\", \"branch\": \"feature/user-auth\"})\n\
                 - Immediately ready to work on feature\n\
                 - No extra checkout needed\n\n\
                 Release tag building:\n\
                 git_clone({\"url\": \"https://github.com/project/repo.git\", \"path\": \"./build-v2.1.0\", \"branch\": \"v2.1.0\"})\n\
                 - Reproducible builds from tagged releases\n\
                 - Guaranteed specific version\n\n\
                 PR branch review:\n\
                 git_clone({\"url\": \"https://github.com/user/repo.git\", \"path\": \"./review-pr\", \"branch\": \"pr/123\"})\n\
                 - Clone specific PR branch for code review\n\n\
                 Development testing:\n\
                 git_clone({\"url\": \"https://github.com/project/repo.git\", \"path\": \"./test\", \"branch\": \"develop\", \"depth\": 1})\n\
                 - Quick testing of latest development code\n\
                 - Fast download with shallow clone\n\n\
                 BRANCH NAMING CONVENTIONS:\n\
                 - feature/user-login (feature work)\n\
                 - hotfix/crash-bug (urgent fixes)\n\
                 - release/v2.0 (release preparation)\n\
                 - v1.0.0 (version tags)\n\n\
                 WORKFLOW EXAMPLE:\n\
                 1. Clone feature branch: git_clone({...branch: \"feature/api\"})\n\
                 2. Make changes and commit\n\
                 3. Push to remote: git_push\n\
                 4. Create PR for review\n\n\
                 BEST PRACTICES:\n\
                 - Clone main/master for stable code\n\
                 - Clone develop for latest development\n\
                 - Clone tags for building releases\n\
                 - Combine with depth:1 for CI/CD speed\n\
                 - Use descriptive paths matching branch names",
            ),
        },
    ]
}

