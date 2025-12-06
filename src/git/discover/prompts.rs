//! Prompt messages for git_discover tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitDiscoverPromptArgs;

/// Prompt provider for git_discover tool
pub struct DiscoverPrompts;

impl PromptProvider for DiscoverPrompts {
    type PromptArgs = GitDiscoverPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I find the Git repository for a file or directory?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_discover to find the repository root from any path inside it:\n\n\
                 BASIC USAGE:\n\
                 ```json\n\
                 {\"path\": \"/home/user/project/src/components/Button.tsx\"}\n\
                 ```\n\n\
                 This searches upward through parent directories to find the .git directory.\n\n\
                 RESPONSE STRUCTURE:\n\
                 - `success`: Whether repository was found\n\
                 - `searched_from`: The path where search started\n\
                 - `repo_root`: Absolute path to repository root (if found)\n\
                 - `message`: Human-readable status\n\n\
                 Example: Searching from /home/user/project/src/components/Button.tsx\n\
                 Returns repo_root: /home/user/project\n\n\
                 HOW IT WORKS:\n\
                 1. Starts at the given path\n\
                 2. Checks for .git directory or file\n\
                 3. If not found, moves to parent directory\n\
                 4. Repeats until .git found or filesystem root reached\n\
                 5. Returns repo_root if found, or error if search exhausted\n\n\
                 USE CASES:\n\
                 - You have a file path and need the repository root for git operations\n\
                 - Working in a deeply nested subdirectory, need to find project root\n\
                 - Building tools that accept any path within a repository\n\
                 - Validating whether a path is inside a Git repository\n\n\
                 WORKFLOW PATTERN:\n\
                 ```\n\
                 # 1. User provides a path to any file or directory\n\
                 user_path = \"/projects/myapp/backend/src/services/auth.rs\"\n\
                 \n\
                 # 2. Discover the repository root\n\
                 result = git_discover({\"path\": user_path})\n\
                 \n\
                 # 3. Use repo_root for subsequent git operations\n\
                 if result.success:\n\
                     repo_root = result.repo_root  # \"/projects/myapp\"\n\
                     # Now use repo_root with other git tools:\n\
                     git_status({\"path\": repo_root})\n\
                     git_log({\"path\": repo_root, \"max_count\": 10})\n\
                 else:\n\
                     print(f\"Not a git repository: {result.message}\")\n\
                 ```\n\n\
                 KEY POINTS:\n\
                 - Works with files, directories, and symlinks\n\
                 - Handles nested repositories (stops at first .git found)\n\
                 - Returns absolute paths for consistency\n\
                 - Fails gracefully if no repository found\n\
                 - Related tools: git_open, git_status, git_log (all require repo root)"
            ),
        },
    ]
}


