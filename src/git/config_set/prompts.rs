//! Prompt messages for git_config_set tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitConfigSetPromptArgs;

/// Prompt provider for git_config_set tool
pub struct ConfigSetPrompts;

impl PromptProvider for ConfigSetPrompts {
    type PromptArgs = GitConfigSetPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("identity") => prompt_identity(),
            Some("behavior") => prompt_behavior(),
            Some("aliases") => prompt_aliases(),
            Some("repo_specific") => prompt_repo_specific(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: identity, behavior, aliases, repo_specific".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Setting user identity
fn prompt_identity() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I set my user identity with git_config_set?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Configure user identity for commits:\n\n\
                 SETTING USER IDENTITY:\n\n\
                 1. Set user name:\n\
                 ```json\n\
                 {\n\
                   \"path\": \"/project\",\n\
                   \"key\": \"user.name\",\n\
                   \"value\": \"John Doe\"\n\
                 }\n\
                 ```\n\n\
                 2. Set user email:\n\
                 ```json\n\
                 {\n\
                   \"path\": \"/project\",\n\
                   \"key\": \"user.email\",\n\
                   \"value\": \"john@example.com\"\n\
                 }\n\
                 ```\n\n\
                 3. Set globally (all repositories):\n\
                 ```json\n\
                 {\n\
                   \"key\": \"user.name\",\n\
                   \"value\": \"John Doe\",\n\
                   \"scope\": \"global\"\n\
                 }\n\
                 ```\n\
                 No path needed for global scope\n\n\
                 4. Work email for specific repo:\n\
                 ```json\n\
                 {\n\
                   \"path\": \"/work/project\",\n\
                   \"key\": \"user.email\",\n\
                   \"value\": \"john@company.com\",\n\
                   \"scope\": \"local\"\n\
                 }\n\
                 ```\n\n\
                 IDENTITY USE CASES:\n\
                 - Global personal identity: Set once with scope: \"global\"\n\
                 - Work email for work repos: Set per-repo with scope: \"local\"\n\
                 - Different names for open source: Override per-project\n\
                 - Multiple identities: Use local scope to override global\n\n\
                 SCOPE BEHAVIOR:\n\
                 - \"global\": ~/.gitconfig (user-wide default)\n\
                 - \"local\": .git/config (repository-specific, overrides global)\n\
                 - \"system\": /etc/gitconfig (machine-wide, rarely used)\n\
                 - No scope specified: Defaults to local if path provided\n\n\
                 COMPLETE IDENTITY SETUP:\n\
                 ```json\n\
                 // 1. Set global defaults\n\
                 {\"key\": \"user.name\", \"value\": \"John Doe\", \"scope\": \"global\"}\n\
                 {\"key\": \"user.email\", \"value\": \"john@personal.com\", \"scope\": \"global\"}\n\
                 \n\
                 // 2. Override for work projects\n\
                 {\"path\": \"/work/project\", \"key\": \"user.email\", \"value\": \"john@work.com\", \"scope\": \"local\"}\n\
                 ```\n\n\
                 VERIFICATION:\n\
                 After setting, use git_config_get to verify:\n\
                 ```json\n\
                 {\"path\": \"/project\", \"key\": \"user.name\"}\n\
                 {\"path\": \"/project\", \"key\": \"user.email\"}\n\
                 ```"
            ),
        },
    ]
}

/// Configuring git behavior
fn prompt_behavior() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I configure git behavior with git_config_set?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Configure git behavior and preferences:\n\n\
                 CONFIGURING BEHAVIOR:\n\n\
                 1. Set default branch name:\n\
                 ```json\n\
                 {\n\
                   \"key\": \"init.defaultBranch\",\n\
                   \"value\": \"main\",\n\
                   \"scope\": \"global\"\n\
                 }\n\
                 ```\n\
                 New repositories will use 'main' instead of 'master'\n\n\
                 2. Configure pull behavior:\n\
                 ```json\n\
                 {\n\
                   \"key\": \"pull.rebase\",\n\
                   \"value\": \"true\",\n\
                   \"scope\": \"global\"\n\
                 }\n\
                 ```\n\
                 Rebase instead of merge when pulling\n\n\
                 3. Configure push behavior:\n\
                 ```json\n\
                 {\n\
                   \"key\": \"push.default\",\n\
                   \"value\": \"current\",\n\
                   \"scope\": \"global\"\n\
                 }\n\
                 ```\n\
                 Push current branch to same name on remote\n\n\
                 4. Configure line endings:\n\
                 ```json\n\
                 {\n\
                   \"path\": \"/project\",\n\
                   \"key\": \"core.autocrlf\",\n\
                   \"value\": \"input\"\n\
                 }\n\
                 ```\n\
                 Convert CRLF to LF on commit (Unix-style)\n\n\
                 COMMON BEHAVIOR SETTINGS:\n\n\
                 pull.rebase:\n\
                 - \"true\": Rebase instead of merge on pull (cleaner history)\n\
                 - \"false\": Merge on pull (default)\n\
                 ```json\n\
                 {\"key\": \"pull.rebase\", \"value\": \"true\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 push.default:\n\
                 - \"current\": Push current branch to same name\n\
                 - \"simple\": Push to upstream branch\n\
                 - \"upstream\": Push to upstream branch\n\
                 - \"matching\": Push all matching branches\n\
                 ```json\n\
                 {\"key\": \"push.default\", \"value\": \"current\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 core.autocrlf:\n\
                 - \"input\": Convert CRLF to LF on commit (Unix)\n\
                 - \"true\": Convert both ways (Windows)\n\
                 - \"false\": No conversion\n\
                 ```json\n\
                 {\"path\": \"/project\", \"key\": \"core.autocrlf\", \"value\": \"input\"}\n\
                 ```\n\n\
                 core.editor:\n\
                 - Set preferred editor for commit messages\n\
                 ```json\n\
                 {\"key\": \"core.editor\", \"value\": \"vim\", \"scope\": \"global\"}\n\
                 {\"key\": \"core.editor\", \"value\": \"code --wait\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 merge.ff:\n\
                 - \"true\": Fast-forward when possible (default)\n\
                 - \"false\": Always create merge commit\n\
                 - \"only\": Only allow fast-forward merges\n\
                 ```json\n\
                 {\"key\": \"merge.ff\", \"value\": \"false\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 RECOMMENDED GLOBAL SETTINGS:\n\
                 ```json\n\
                 // Modern defaults\n\
                 {\"key\": \"init.defaultBranch\", \"value\": \"main\", \"scope\": \"global\"}\n\
                 {\"key\": \"pull.rebase\", \"value\": \"true\", \"scope\": \"global\"}\n\
                 {\"key\": \"push.default\", \"value\": \"current\", \"scope\": \"global\"}\n\
                 {\"key\": \"core.autocrlf\", \"value\": \"input\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 PROJECT-SPECIFIC SETTINGS:\n\
                 ```json\n\
                 // Windows project needs CRLF\n\
                 {\"path\": \"/windows-project\", \"key\": \"core.autocrlf\", \"value\": \"true\"}\n\
                 \n\
                 // This project requires merge commits\n\
                 {\"path\": \"/project\", \"key\": \"merge.ff\", \"value\": \"false\"}\n\
                 ```"
            ),
        },
    ]
}

/// Creating command aliases
fn prompt_aliases() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create git aliases with git_config_set?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Create shortcuts for frequently used git commands:\n\n\
                 CREATING ALIASES:\n\n\
                 1. Short status:\n\
                 ```json\n\
                 {\n\
                   \"key\": \"alias.st\",\n\
                   \"value\": \"status -sb\",\n\
                   \"scope\": \"global\"\n\
                 }\n\
                 ```\n\
                 Use: git st (shows status in short format)\n\n\
                 2. Pretty log:\n\
                 ```json\n\
                 {\n\
                   \"key\": \"alias.lg\",\n\
                   \"value\": \"log --oneline --graph --decorate\",\n\
                   \"scope\": \"global\"\n\
                 }\n\
                 ```\n\
                 Use: git lg (shows visual commit graph)\n\n\
                 3. Undo last commit:\n\
                 ```json\n\
                 {\n\
                   \"key\": \"alias.undo\",\n\
                   \"value\": \"reset HEAD~1 --mixed\",\n\
                   \"scope\": \"global\"\n\
                 }\n\
                 ```\n\
                 Use: git undo (removes last commit, keeps changes)\n\n\
                 4. List branches by date:\n\
                 ```json\n\
                 {\n\
                   \"key\": \"alias.recent\",\n\
                   \"value\": \"branch --sort=-committerdate\",\n\
                   \"scope\": \"global\"\n\
                 }\n\
                 ```\n\
                 Use: git recent (shows most recently used branches)\n\n\
                 USEFUL ALIASES LIBRARY:\n\n\
                 Basic shortcuts:\n\
                 ```json\n\
                 {\"key\": \"alias.st\", \"value\": \"status -sb\", \"scope\": \"global\"}\n\
                 {\"key\": \"alias.co\", \"value\": \"checkout\", \"scope\": \"global\"}\n\
                 {\"key\": \"alias.br\", \"value\": \"branch\", \"scope\": \"global\"}\n\
                 {\"key\": \"alias.ci\", \"value\": \"commit\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 Enhanced log views:\n\
                 ```json\n\
                 // Simple one-line log with graph\n\
                 {\"key\": \"alias.lg\", \"value\": \"log --oneline --graph --decorate\", \"scope\": \"global\"}\n\
                 \n\
                 // Detailed log with dates\n\
                 {\"key\": \"alias.ll\", \"value\": \"log --pretty=format:'%h %ad | %s%d [%an]' --graph --date=short\", \"scope\": \"global\"}\n\
                 \n\
                 // Show all branches\n\
                 {\"key\": \"alias.lga\", \"value\": \"log --oneline --graph --decorate --all\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 Undo operations:\n\
                 ```json\n\
                 // Undo last commit, keep changes staged\n\
                 {\"key\": \"alias.undo\", \"value\": \"reset HEAD~1 --mixed\", \"scope\": \"global\"}\n\
                 \n\
                 // Undo last commit, keep changes unstaged\n\
                 {\"key\": \"alias.uncommit\", \"value\": \"reset HEAD~1 --soft\", \"scope\": \"global\"}\n\
                 \n\
                 // Discard all local changes (dangerous!)\n\
                 {\"key\": \"alias.nuke\", \"value\": \"reset --hard HEAD\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 Branch management:\n\
                 ```json\n\
                 // List branches by most recent\n\
                 {\"key\": \"alias.recent\", \"value\": \"branch --sort=-committerdate\", \"scope\": \"global\"}\n\
                 \n\
                 // Delete merged branches\n\
                 {\"key\": \"alias.cleanup\", \"value\": \"branch --merged | grep -v '*' | xargs -n 1 git branch -d\", \"scope\": \"global\"}\n\
                 \n\
                 // Show branches with last commit\n\
                 {\"key\": \"alias.branches\", \"value\": \"branch -v\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 Diff and show:\n\
                 ```json\n\
                 // Show changes for last commit\n\
                 {\"key\": \"alias.last\", \"value\": \"show HEAD\", \"scope\": \"global\"}\n\
                 \n\
                 // Diff staged changes\n\
                 {\"key\": \"alias.staged\", \"value\": \"diff --staged\", \"scope\": \"global\"}\n\
                 \n\
                 // Word-level diff\n\
                 {\"key\": \"alias.wdiff\", \"value\": \"diff --word-diff\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 Stash shortcuts:\n\
                 ```json\n\
                 {\"key\": \"alias.save\", \"value\": \"stash save\", \"scope\": \"global\"}\n\
                 {\"key\": \"alias.pop\", \"value\": \"stash pop\", \"scope\": \"global\"}\n\
                 {\"key\": \"alias.stashes\", \"value\": \"stash list\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 COMPLETE ALIAS SETUP:\n\
                 ```json\n\
                 // Most useful aliases for daily work\n\
                 {\"key\": \"alias.st\", \"value\": \"status -sb\", \"scope\": \"global\"}\n\
                 {\"key\": \"alias.co\", \"value\": \"checkout\", \"scope\": \"global\"}\n\
                 {\"key\": \"alias.lg\", \"value\": \"log --oneline --graph --decorate\", \"scope\": \"global\"}\n\
                 {\"key\": \"alias.undo\", \"value\": \"reset HEAD~1 --mixed\", \"scope\": \"global\"}\n\
                 {\"key\": \"alias.recent\", \"value\": \"branch --sort=-committerdate\", \"scope\": \"global\"}\n\
                 {\"key\": \"alias.staged\", \"value\": \"diff --staged\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 ALIAS NAMING TIPS:\n\
                 - Use short names for frequently used commands (st, co, br)\n\
                 - Use descriptive names for complex commands (recent, staged)\n\
                 - Avoid overriding existing git commands\n\
                 - Set scope to \"global\" for personal aliases"
            ),
        },
    ]
}

/// Per-repository settings
fn prompt_repo_specific() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I configure repository-specific settings?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Configure settings for specific repositories:\n\n\
                 PER-REPOSITORY SETTINGS:\n\n\
                 1. Repository-specific email:\n\
                 ```json\n\
                 {\n\
                   \"path\": \"/opensource/project\",\n\
                   \"key\": \"user.email\",\n\
                   \"value\": \"me@personal.com\",\n\
                   \"scope\": \"local\"\n\
                 }\n\
                 ```\n\
                 Override global email for open source work\n\n\
                 2. Disable hooks for testing:\n\
                 ```json\n\
                 {\n\
                   \"path\": \"/project\",\n\
                   \"key\": \"core.hooksPath\",\n\
                   \"value\": \"/dev/null\",\n\
                   \"scope\": \"local\"\n\
                 }\n\
                 ```\n\
                 Skip git hooks in this repository\n\n\
                 3. Custom merge tool:\n\
                 ```json\n\
                 {\n\
                   \"path\": \"/project\",\n\
                   \"key\": \"merge.tool\",\n\
                   \"value\": \"vscode\",\n\
                   \"scope\": \"local\"\n\
                 }\n\
                 ```\n\
                 Use VS Code for merge conflicts\n\n\
                 4. Enable sparse checkout:\n\
                 ```json\n\
                 {\n\
                   \"path\": \"/monorepo\",\n\
                   \"key\": \"core.sparseCheckout\",\n\
                   \"value\": \"true\",\n\
                   \"scope\": \"local\"\n\
                 }\n\
                 ```\n\
                 Only checkout specific directories in monorepo\n\n\
                 USE CASES:\n\n\
                 Different identity per project:\n\
                 ```json\n\
                 // Personal projects - personal email\n\
                 {\"path\": \"/personal/blog\", \"key\": \"user.email\", \"value\": \"me@personal.com\", \"scope\": \"local\"}\n\
                 \n\
                 // Work projects - work email\n\
                 {\"path\": \"/work/product\", \"key\": \"user.email\", \"value\": \"me@company.com\", \"scope\": \"local\"}\n\
                 \n\
                 // Open source - public email\n\
                 {\"path\": \"/oss/contrib\", \"key\": \"user.email\", \"value\": \"me@opensource.org\", \"scope\": \"local\"}\n\
                 ```\n\n\
                 Project-specific tools:\n\
                 ```json\n\
                 // This project uses meld for diffs\n\
                 {\"path\": \"/project-a\", \"key\": \"diff.tool\", \"value\": \"meld\", \"scope\": \"local\"}\n\
                 \n\
                 // This project uses kdiff3 for merges\n\
                 {\"path\": \"/project-a\", \"key\": \"merge.tool\", \"value\": \"kdiff3\", \"scope\": \"local\"}\n\
                 ```\n\n\
                 Monorepo configuration:\n\
                 ```json\n\
                 // Enable sparse checkout\n\
                 {\"path\": \"/monorepo\", \"key\": \"core.sparseCheckout\", \"value\": \"true\", \"scope\": \"local\"}\n\
                 \n\
                 // Increase performance for large repos\n\
                 {\"path\": \"/monorepo\", \"key\": \"core.preloadIndex\", \"value\": \"true\", \"scope\": \"local\"}\n\
                 {\"path\": \"/monorepo\", \"key\": \"core.fscache\", \"value\": \"true\", \"scope\": \"local\"}\n\
                 ```\n\n\
                 Disable features for specific repo:\n\
                 ```json\n\
                 // Disable hooks during testing\n\
                 {\"path\": \"/test-repo\", \"key\": \"core.hooksPath\", \"value\": \"/dev/null\", \"scope\": \"local\"}\n\
                 \n\
                 // Disable GPG signing\n\
                 {\"path\": \"/test-repo\", \"key\": \"commit.gpgSign\", \"value\": \"false\", \"scope\": \"local\"}\n\
                 ```\n\n\
                 SCOPE PRIORITY:\n\
                 When the same key is set at multiple scopes:\n\
                 1. Local (.git/config) - highest priority\n\
                 2. Global (~/.gitconfig) - medium priority\n\
                 3. System (/etc/gitconfig) - lowest priority\n\n\
                 Local settings override global settings!\n\n\
                 WORKFLOW EXAMPLE:\n\
                 ```json\n\
                 // 1. Set global defaults\n\
                 {\"key\": \"user.name\", \"value\": \"John Doe\", \"scope\": \"global\"}\n\
                 {\"key\": \"user.email\", \"value\": \"john@personal.com\", \"scope\": \"global\"}\n\
                 \n\
                 // 2. Override for work repository\n\
                 {\"path\": \"/work/project\", \"key\": \"user.email\", \"value\": \"john@company.com\", \"scope\": \"local\"}\n\
                 \n\
                 // 3. Configure work repo specifics\n\
                 {\"path\": \"/work/project\", \"key\": \"commit.gpgSign\", \"value\": \"true\", \"scope\": \"local\"}\n\
                 {\"path\": \"/work/project\", \"key\": \"user.signingKey\", \"value\": \"ABCD1234\", \"scope\": \"local\"}\n\
                 ```\n\n\
                 VERIFICATION:\n\
                 Check effective configuration:\n\
                 ```json\n\
                 // See what value is actually used\n\
                 {\"path\": \"/work/project\", \"key\": \"user.email\"}\n\
                 // Returns: john@company.com (local overrides global)\n\
                 ```"
            ),
        },
    ]
}

/// Comprehensive guide covering all git_config_set usage
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using git_config_set."
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Set Git configuration values at different scopes:\n\n\
                 BASIC USAGE:\n\
                 ```json\n\
                 {\n\
                   \"path\": \"/repository\",\n\
                   \"key\": \"user.name\",\n\
                   \"value\": \"John Doe\"\n\
                 }\n\
                 ```\n\n\
                 PARAMETERS:\n\
                 - key (required): Configuration key (e.g., \"user.name\", \"core.editor\")\n\
                 - value (required): Value to set\n\
                 - path (optional): Repository path (not needed for global scope)\n\
                 - scope (optional): \"local\", \"global\", or \"system\" (default: local if path provided)\n\n\
                 THREE CONFIGURATION SCOPES:\n\n\
                 1. LOCAL (.git/config in repository):\n\
                 ```json\n\
                 {\n\
                   \"path\": \"/repo\",\n\
                   \"key\": \"user.email\",\n\
                   \"value\": \"me@example.com\",\n\
                   \"scope\": \"local\"\n\
                 }\n\
                 ```\n\
                 - Repository-specific\n\
                 - Highest priority (overrides global and system)\n\
                 - Requires path parameter\n\n\
                 2. GLOBAL (~/.gitconfig for user):\n\
                 ```json\n\
                 {\n\
                   \"key\": \"user.name\",\n\
                   \"value\": \"John Doe\",\n\
                   \"scope\": \"global\"\n\
                 }\n\
                 ```\n\
                 - User-wide defaults\n\
                 - Medium priority\n\
                 - No path parameter needed\n\n\
                 3. SYSTEM (/etc/gitconfig for all users):\n\
                 ```json\n\
                 {\n\
                   \"key\": \"core.editor\",\n\
                   \"value\": \"vim\",\n\
                   \"scope\": \"system\"\n\
                 }\n\
                 ```\n\
                 - Machine-wide settings\n\
                 - Lowest priority\n\
                 - Rarely used\n\
                 - May require admin permissions\n\n\
                 =============================================================================\n\
                 COMMON CONFIGURATION CATEGORIES\n\
                 =============================================================================\n\n\
                 USER IDENTITY:\n\
                 ```json\n\
                 // Set globally for all repos\n\
                 {\"key\": \"user.name\", \"value\": \"John Doe\", \"scope\": \"global\"}\n\
                 {\"key\": \"user.email\", \"value\": \"john@example.com\", \"scope\": \"global\"}\n\
                 \n\
                 // Override for specific repo\n\
                 {\"path\": \"/work/project\", \"key\": \"user.email\", \"value\": \"john@work.com\", \"scope\": \"local\"}\n\
                 ```\n\n\
                 GIT BEHAVIOR:\n\
                 ```json\n\
                 // Default branch for new repos\n\
                 {\"key\": \"init.defaultBranch\", \"value\": \"main\", \"scope\": \"global\"}\n\
                 \n\
                 // Rebase on pull instead of merge\n\
                 {\"key\": \"pull.rebase\", \"value\": \"true\", \"scope\": \"global\"}\n\
                 \n\
                 // Push current branch to same name\n\
                 {\"key\": \"push.default\", \"value\": \"current\", \"scope\": \"global\"}\n\
                 \n\
                 // Line ending conversion (Unix)\n\
                 {\"key\": \"core.autocrlf\", \"value\": \"input\", \"scope\": \"global\"}\n\
                 \n\
                 // Default editor\n\
                 {\"key\": \"core.editor\", \"value\": \"vim\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 ALIASES:\n\
                 ```json\n\
                 // Short status\n\
                 {\"key\": \"alias.st\", \"value\": \"status -sb\", \"scope\": \"global\"}\n\
                 \n\
                 // Visual log\n\
                 {\"key\": \"alias.lg\", \"value\": \"log --oneline --graph --decorate\", \"scope\": \"global\"}\n\
                 \n\
                 // Undo last commit\n\
                 {\"key\": \"alias.undo\", \"value\": \"reset HEAD~1 --mixed\", \"scope\": \"global\"}\n\
                 \n\
                 // Recent branches\n\
                 {\"key\": \"alias.recent\", \"value\": \"branch --sort=-committerdate\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 MERGE & DIFF:\n\
                 ```json\n\
                 // Always create merge commit\n\
                 {\"key\": \"merge.ff\", \"value\": \"false\", \"scope\": \"global\"}\n\
                 \n\
                 // Set merge tool\n\
                 {\"path\": \"/repo\", \"key\": \"merge.tool\", \"value\": \"vscode\", \"scope\": \"local\"}\n\
                 \n\
                 // Set diff tool\n\
                 {\"key\": \"diff.tool\", \"value\": \"meld\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 PERFORMANCE:\n\
                 ```json\n\
                 // Large repository optimizations\n\
                 {\"path\": \"/monorepo\", \"key\": \"core.preloadIndex\", \"value\": \"true\", \"scope\": \"local\"}\n\
                 {\"path\": \"/monorepo\", \"key\": \"core.fscache\", \"value\": \"true\", \"scope\": \"local\"}\n\
                 ```\n\n\
                 SECURITY:\n\
                 ```json\n\
                 // Enable GPG signing\n\
                 {\"key\": \"commit.gpgSign\", \"value\": \"true\", \"scope\": \"global\"}\n\
                 {\"key\": \"user.signingKey\", \"value\": \"ABCD1234\", \"scope\": \"global\"}\n\
                 \n\
                 // Disable hooks (testing only)\n\
                 {\"path\": \"/test-repo\", \"key\": \"core.hooksPath\", \"value\": \"/dev/null\", \"scope\": \"local\"}\n\
                 ```\n\n\
                 =============================================================================\n\
                 COMPLETE SETUP EXAMPLES\n\
                 =============================================================================\n\n\
                 NEW USER SETUP:\n\
                 ```json\n\
                 // 1. Identity\n\
                 {\"key\": \"user.name\", \"value\": \"John Doe\", \"scope\": \"global\"}\n\
                 {\"key\": \"user.email\", \"value\": \"john@example.com\", \"scope\": \"global\"}\n\
                 \n\
                 // 2. Modern defaults\n\
                 {\"key\": \"init.defaultBranch\", \"value\": \"main\", \"scope\": \"global\"}\n\
                 {\"key\": \"pull.rebase\", \"value\": \"true\", \"scope\": \"global\"}\n\
                 {\"key\": \"push.default\", \"value\": \"current\", \"scope\": \"global\"}\n\
                 \n\
                 // 3. Essential aliases\n\
                 {\"key\": \"alias.st\", \"value\": \"status -sb\", \"scope\": \"global\"}\n\
                 {\"key\": \"alias.lg\", \"value\": \"log --oneline --graph --decorate\", \"scope\": \"global\"}\n\
                 {\"key\": \"alias.co\", \"value\": \"checkout\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 WORK + PERSONAL SETUP:\n\
                 ```json\n\
                 // 1. Global personal identity\n\
                 {\"key\": \"user.name\", \"value\": \"John Doe\", \"scope\": \"global\"}\n\
                 {\"key\": \"user.email\", \"value\": \"john@personal.com\", \"scope\": \"global\"}\n\
                 \n\
                 // 2. Override for work projects\n\
                 {\"path\": \"/work/project1\", \"key\": \"user.email\", \"value\": \"john@company.com\", \"scope\": \"local\"}\n\
                 {\"path\": \"/work/project2\", \"key\": \"user.email\", \"value\": \"john@company.com\", \"scope\": \"local\"}\n\
                 \n\
                 // 3. Work repos require GPG signing\n\
                 {\"path\": \"/work/project1\", \"key\": \"commit.gpgSign\", \"value\": \"true\", \"scope\": \"local\"}\n\
                 {\"path\": \"/work/project2\", \"key\": \"commit.gpgSign\", \"value\": \"true\", \"scope\": \"local\"}\n\
                 ```\n\n\
                 MONOREPO OPTIMIZATION:\n\
                 ```json\n\
                 // Enable sparse checkout\n\
                 {\"path\": \"/monorepo\", \"key\": \"core.sparseCheckout\", \"value\": \"true\", \"scope\": \"local\"}\n\
                 \n\
                 // Performance optimizations\n\
                 {\"path\": \"/monorepo\", \"key\": \"core.preloadIndex\", \"value\": \"true\", \"scope\": \"local\"}\n\
                 {\"path\": \"/monorepo\", \"key\": \"core.fscache\", \"value\": \"true\", \"scope\": \"local\"}\n\
                 {\"path\": \"/monorepo\", \"key\": \"feature.manyFiles\", \"value\": \"true\", \"scope\": \"local\"}\n\
                 ```\n\n\
                 =============================================================================\n\
                 KEY FORMAT REFERENCE\n\
                 =============================================================================\n\n\
                 Configuration keys use dot notation: section.key or section.subsection.key\n\n\
                 Common sections:\n\
                 - user.* - User identity\n\
                 - core.* - Core git settings\n\
                 - init.* - Repository initialization\n\
                 - pull.* - Pull behavior\n\
                 - push.* - Push behavior\n\
                 - merge.* - Merge settings\n\
                 - diff.* - Diff settings\n\
                 - alias.* - Command aliases\n\
                 - commit.* - Commit settings\n\
                 - branch.* - Branch settings\n\
                 - remote.* - Remote settings\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. SCOPE SELECTION:\n\
                    - Use \"global\" for personal preferences\n\
                    - Use \"local\" for project-specific settings\n\
                    - Avoid \"system\" unless managing shared machines\n\n\
                 2. IDENTITY MANAGEMENT:\n\
                    - Set global identity as default\n\
                    - Override locally for work/open source\n\
                    - Verify with git_config_get before committing\n\n\
                 3. ALIASES:\n\
                    - Keep short for frequent commands\n\
                    - Set globally for personal productivity\n\
                    - Don't override existing git commands\n\n\
                 4. VERIFICATION:\n\
                    - Use git_config_get to verify settings\n\
                    - Check effective value in repository context\n\
                    - Remember: local > global > system priority\n\n\
                 5. SAFETY:\n\
                    - Be careful with --global scope changes\n\
                    - Test in single repo with --local first\n\
                    - Back up ~/.gitconfig before bulk changes\n\n\
                 =============================================================================\n\
                 TROUBLESHOOTING\n\
                 =============================================================================\n\n\
                 Wrong email in commits:\n\
                 ```json\n\
                 // Check current value\n\
                 {\"path\": \"/repo\", \"key\": \"user.email\"}\n\
                 \n\
                 // Set correct value\n\
                 {\"path\": \"/repo\", \"key\": \"user.email\", \"value\": \"correct@email.com\", \"scope\": \"local\"}\n\
                 ```\n\n\
                 Merge/pull behavior unexpected:\n\
                 ```json\n\
                 // Check current settings\n\
                 {\"key\": \"pull.rebase\"}\n\
                 {\"key\": \"merge.ff\"}\n\
                 \n\
                 // Set desired behavior\n\
                 {\"key\": \"pull.rebase\", \"value\": \"true\", \"scope\": \"global\"}\n\
                 ```\n\n\
                 Setting not taking effect:\n\
                 - Check scope priority (local > global > system)\n\
                 - Verify with git_config_get in repository\n\
                 - Ensure path is correct for local scope\n\n\
                 =============================================================================\n\
                 REMEMBER\n\
                 =============================================================================\n\n\
                 - Configuration is hierarchical: local overrides global overrides system\n\
                 - Use git_config_get to verify settings took effect\n\
                 - Global scope affects all repositories\n\
                 - Local scope only affects specified repository\n\
                 - Aliases are powerful productivity tools\n\
                 - Always verify identity before first commit in new repo"
            ),
        },
    ]
}
