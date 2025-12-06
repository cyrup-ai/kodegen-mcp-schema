//! Prompt messages for git_config_get tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitConfigGetPromptArgs;

/// Prompt provider for git_config_get tool
pub struct ConfigGetPrompts;

impl PromptProvider for ConfigGetPrompts {
    type PromptArgs = GitConfigGetPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("user") => prompt_user(),
            Some("repo") => prompt_repo(),
            Some("scopes") => prompt_scopes(),
            Some("list") => prompt_list(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: user, repo, scopes, list".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// SCENARIO FUNCTIONS - TEACH AI AGENTS HOW TO READ GIT CONFIG
// ============================================================================

/// User identity settings
fn prompt_user() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I check my Git user identity settings?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "USER IDENTITY SETTINGS:\n\n\
                 1. Get user name:\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"key\": \"user.name\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"key\": \"user.name\",\n\
                   \"value\": \"John Doe\"\n\
                 }\n\n\
                 2. Get user email:\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"key\": \"user.email\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"key\": \"user.email\",\n\
                   \"value\": \"john@example.com\"\n\
                 }\n\n\
                 3. Check signing key:\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"key\": \"user.signingkey\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"key\": \"user.signingkey\",\n\
                   \"value\": \"0x1234567890ABCDEF\"\n\
                 }\n\n\
                 IDENTITY KEYS:\n\
                 - user.name: Display name for commits\n\
                 - user.email: Email address for commits\n\
                 - user.signingkey: GPG key ID for signing commits\n\n\
                 WHY CHECK:\n\
                 - Verify correct identity for project\n\
                 - Ensure commits attributed correctly\n\
                 - Check GPG signing setup\n\
                 - Confirm work/personal identity separation\n\n\
                 COMMON SCENARIOS:\n\
                 - Before first commit in new repository\n\
                 - When switching between work and personal projects\n\
                 - After setting up new development machine\n\
                 - When commits show wrong author information",
            ),
        },
    ]
}

/// Repository-specific settings
fn prompt_repo() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I check repository-specific Git configuration?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "REPOSITORY SETTINGS:\n\n\
                 1. Remote URL:\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"key\": \"remote.origin.url\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"key\": \"remote.origin.url\",\n\
                   \"value\": \"git@github.com:user/repo.git\"\n\
                 }\n\n\
                 2. Default branch:\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"key\": \"init.defaultBranch\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"key\": \"init.defaultBranch\",\n\
                   \"value\": \"main\"\n\
                 }\n\n\
                 3. Push behavior:\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"key\": \"push.default\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"key\": \"push.default\",\n\
                   \"value\": \"simple\"\n\
                 }\n\n\
                 4. Pull behavior:\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"key\": \"pull.rebase\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"key\": \"pull.rebase\",\n\
                   \"value\": \"false\"\n\
                 }\n\n\
                 COMMON REPO SETTINGS:\n\
                 - remote.origin.url: Where to push/pull from\n\
                 - remote.origin.fetch: Fetch refspec for remote\n\
                 - branch.main.remote: Which remote tracks main branch\n\
                 - branch.main.merge: Which branch to merge on pull\n\
                 - core.autocrlf: Line ending handling (true/false/input)\n\
                 - core.filemode: Track file permission changes (true/false)\n\
                 - core.ignorecase: Case sensitivity (true/false)\n\
                 - core.bare: Is this a bare repository (true/false)\n\n\
                 PUSH DEFAULT VALUES:\n\
                 - simple: Push current branch to upstream (recommended)\n\
                 - current: Push current branch to branch with same name\n\
                 - upstream: Push current branch to its upstream\n\
                 - matching: Push all matching branches\n\n\
                 PULL REBASE VALUES:\n\
                 - false: Merge on pull (default)\n\
                 - true: Rebase on pull\n\
                 - interactive: Interactive rebase on pull\n\n\
                 WHY CHECK THESE:\n\
                 - Verify correct remote URL before pushing\n\
                 - Understand push/pull behavior\n\
                 - Debug synchronization issues\n\
                 - Ensure correct line ending handling\n\
                 - Check branch tracking configuration",
            ),
        },
    ]
}

/// Config scope hierarchy
fn prompt_scopes() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I check Git configuration at different scopes?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "CONFIG SCOPES:\n\n\
                 1. Local config (repository only):\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"key\": \"user.email\",\n\
                        \"scope\": \"local\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"key\": \"user.email\",\n\
                   \"value\": \"work@company.com\",\n\
                   \"scope\": \"local\"\n\
                 }\n\n\
                 2. Global config (all repositories):\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"key\": \"user.email\",\n\
                        \"scope\": \"global\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"key\": \"user.email\",\n\
                   \"value\": \"personal@email.com\",\n\
                   \"scope\": \"global\"\n\
                 }\n\n\
                 3. System config (machine-wide):\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"key\": \"user.email\",\n\
                        \"scope\": \"system\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"key\": \"user.email\",\n\
                   \"value\": null\n\
                 }\n\n\
                 4. Effective config (no scope specified):\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"key\": \"user.email\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"key\": \"user.email\",\n\
                   \"value\": \"work@company.com\"\n\
                 }\n\
                 // Returns local value because it overrides global\n\n\
                 SCOPE HIERARCHY:\n\
                 1. local: .git/config (repository-specific)\n\
                    - Highest priority\n\
                    - Only affects current repository\n\
                    - Stored in repo/.git/config\n\
                    - Use for project-specific overrides\n\n\
                 2. global: ~/.gitconfig or ~/.config/git/config (user-wide)\n\
                    - Medium priority\n\
                    - Affects all repositories for current user\n\
                    - Stored in home directory\n\
                    - Use for personal preferences\n\n\
                 3. system: /etc/gitconfig (machine-wide)\n\
                    - Lowest priority\n\
                    - Affects all users on machine\n\
                    - Requires admin/root access to modify\n\
                    - Use for organization-wide policies\n\n\
                 OVERRIDE BEHAVIOR:\n\
                 Local overrides global overrides system\n\
                 - If key exists in local, that value is used\n\
                 - If not in local, check global\n\
                 - If not in global, check system\n\
                 - If nowhere, value is null/not set\n\n\
                 WHEN TO CHECK SCOPE:\n\
                 - Debugging unexpected configuration values\n\
                 - Verifying per-repository overrides\n\
                 - Understanding which config file to edit\n\
                 - Checking if value is inherited or overridden\n\
                 - Separating work and personal configurations\n\n\
                 EXAMPLE USE CASES:\n\
                 1. Work repository with company email:\n\
                    - Set global: user.email = personal@email.com\n\
                    - Set local: user.email = work@company.com\n\
                    - Effective in work repo: work@company.com\n\
                    - Effective in other repos: personal@email.com\n\n\
                 2. Custom editor for one project:\n\
                    - Set global: core.editor = vim\n\
                    - Set local: core.editor = code\n\
                    - Effective in that repo: code\n\
                    - Effective elsewhere: vim\n\n\
                 3. Organization-wide settings:\n\
                    - Set system: core.autocrlf = input\n\
                    - All users get this by default\n\
                    - Users can override in global or local",
            ),
        },
    ]
}

/// Listing all configuration
fn prompt_list() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list all Git configuration values?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "LISTING ALL CONFIG:\n\n\
                 1. All effective config (merged scopes):\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"list\": true\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"values\": [\n\
                     {\"key\": \"user.name\", \"value\": \"John Doe\"},\n\
                     {\"key\": \"user.email\", \"value\": \"john@example.com\"},\n\
                     {\"key\": \"core.autocrlf\", \"value\": \"input\"},\n\
                     {\"key\": \"core.editor\", \"value\": \"vim\"},\n\
                     {\"key\": \"remote.origin.url\", \"value\": \"git@github.com:user/repo.git\"},\n\
                     {\"key\": \"branch.main.remote\", \"value\": \"origin\"},\n\
                     {\"key\": \"branch.main.merge\", \"value\": \"refs/heads/main\"},\n\
                     {\"key\": \"push.default\", \"value\": \"simple\"},\n\
                     {\"key\": \"pull.rebase\", \"value\": \"false\"}\n\
                   ]\n\
                 }\n\n\
                 2. Local config only (repository-specific):\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"list\": true,\n\
                        \"scope\": \"local\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"values\": [\n\
                     {\"key\": \"remote.origin.url\", \"value\": \"git@github.com:user/repo.git\"},\n\
                     {\"key\": \"branch.main.remote\", \"value\": \"origin\"},\n\
                     {\"key\": \"branch.main.merge\", \"value\": \"refs/heads/main\"}\n\
                   ]\n\
                 }\n\n\
                 3. Global config only (user-wide):\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"list\": true,\n\
                        \"scope\": \"global\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"values\": [\n\
                     {\"key\": \"user.name\", \"value\": \"John Doe\"},\n\
                     {\"key\": \"user.email\", \"value\": \"john@example.com\"},\n\
                     {\"key\": \"core.editor\", \"value\": \"vim\"},\n\
                     {\"key\": \"push.default\", \"value\": \"simple\"}\n\
                   ]\n\
                 }\n\n\
                 4. System config only (machine-wide):\n\
                    git_config_get({\n\
                        \"path\": \"/project\",\n\
                        \"list\": true,\n\
                        \"scope\": \"system\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"values\": [\n\
                     {\"key\": \"core.autocrlf\", \"value\": \"input\"}\n\
                   ]\n\
                 }\n\n\
                 USE CASES:\n\
                 1. Audit all settings:\n\
                    - Review complete configuration\n\
                    - Find unexpected values\n\
                    - Document current setup\n\n\
                 2. Debug configuration issues:\n\
                    - See all effective values\n\
                    - Identify conflicting settings\n\
                    - Trace where values come from\n\n\
                 3. Export config for backup:\n\
                    - Save configuration snapshot\n\
                    - Document environment setup\n\
                    - Replicate setup on another machine\n\n\
                 4. Compare repositories:\n\
                    - Check differences between repos\n\
                    - Ensure consistent settings\n\
                    - Identify project-specific overrides\n\n\
                 5. Verify scope separation:\n\
                    - List local to see repo-only settings\n\
                    - List global to see user-wide settings\n\
                    - List system to see machine-wide settings\n\
                    - Compare to understand inheritance\n\n\
                 COMMON CONFIG CATEGORIES:\n\
                 - user.*: Identity (name, email, signingkey)\n\
                 - core.*: Core behavior (editor, autocrlf, filemode)\n\
                 - remote.*: Remote repositories (url, fetch, push)\n\
                 - branch.*: Branch tracking (remote, merge, rebase)\n\
                 - push.*: Push behavior (default, followTags)\n\
                 - pull.*: Pull behavior (rebase, ff)\n\
                 - merge.*: Merge behavior (tool, conflictstyle)\n\
                 - diff.*: Diff settings (tool, algorithm)\n\
                 - color.*: Color output (ui, branch, diff, status)\n\
                 - alias.*: Custom command aliases\n\n\
                 FILTERING TIPS:\n\
                 - Use list to see all keys\n\
                 - Look for patterns in key names\n\
                 - Group related settings by prefix\n\
                 - Check both presence and absence of keys",
            ),
        },
    ]
}

/// Comprehensive guide covering all aspects
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to reading Git configuration.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_config_get tool reads Git configuration values from repositories. It supports reading individual keys, listing all values, and understanding the scope hierarchy.\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 Read a single config value:\n\
                 git_config_get({\n\
                     \"path\": \"/project\",\n\
                     \"key\": \"user.name\"\n\
                 })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"key\": \"user.name\",\n\
                   \"value\": \"John Doe\"\n\
                 }\n\n\
                 If key doesn't exist:\n\
                 {\n\
                   \"key\": \"user.name\",\n\
                   \"value\": null\n\
                 }\n\n\
                 =============================================================================\n\
                 PARAMETERS\n\
                 =============================================================================\n\n\
                 - path (required): Repository path or any path within the repository\n\
                 - key (optional): Config key to read (e.g., \"user.name\")\n\
                 - scope (optional): Config scope (\"local\", \"global\", \"system\")\n\
                 - list (optional): Set to true to list all config values\n\n\
                 Note: Either provide 'key' for single value OR 'list: true' for all values\n\n\
                 =============================================================================\n\
                 USER IDENTITY\n\
                 =============================================================================\n\n\
                 Check who you are in commits:\n\n\
                 User name:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"user.name\"})\n\
                 → {\"value\": \"John Doe\"}\n\n\
                 User email:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"user.email\"})\n\
                 → {\"value\": \"john@example.com\"}\n\n\
                 Signing key:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"user.signingkey\"})\n\
                 → {\"value\": \"0x1234567890ABCDEF\"}\n\n\
                 WHY CHECK:\n\
                 - Before first commit in new repo\n\
                 - When commits show wrong author\n\
                 - Switching between work/personal\n\
                 - Setting up GPG signing\n\n\
                 =============================================================================\n\
                 REPOSITORY SETTINGS\n\
                 =============================================================================\n\n\
                 Remote URL (where to push/pull):\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"remote.origin.url\"})\n\
                 → {\"value\": \"git@github.com:user/repo.git\"}\n\n\
                 Default branch name:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"init.defaultBranch\"})\n\
                 → {\"value\": \"main\"}\n\n\
                 Push behavior:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"push.default\"})\n\
                 → {\"value\": \"simple\"}\n\
                 Values: simple (recommended), current, upstream, matching\n\n\
                 Pull behavior:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"pull.rebase\"})\n\
                 → {\"value\": \"false\"}\n\
                 Values: false (merge), true (rebase), interactive\n\n\
                 Line endings:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"core.autocrlf\"})\n\
                 → {\"value\": \"input\"}\n\
                 Values: true (Windows), false (Unix), input (commit Unix, checkout as-is)\n\n\
                 File permissions:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"core.filemode\"})\n\
                 → {\"value\": \"true\"}\n\
                 Values: true (track), false (ignore)\n\n\
                 =============================================================================\n\
                 CONFIG SCOPES\n\
                 =============================================================================\n\n\
                 Git has three configuration scopes:\n\n\
                 1. LOCAL (repository-specific):\n\
                    - File: .git/config\n\
                    - Priority: Highest (overrides global and system)\n\
                    - Use for: Project-specific settings\n\
                    - Example: Work email for company repo\n\n\
                 2. GLOBAL (user-wide):\n\
                    - File: ~/.gitconfig or ~/.config/git/config\n\
                    - Priority: Medium (overrides system)\n\
                    - Use for: Personal preferences\n\
                    - Example: Your name and default editor\n\n\
                 3. SYSTEM (machine-wide):\n\
                    - File: /etc/gitconfig\n\
                    - Priority: Lowest\n\
                    - Use for: Organization policies\n\
                    - Example: Consistent line ending handling\n\n\
                 READING SCOPES:\n\n\
                 Effective value (merged hierarchy):\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"user.email\"})\n\
                 → Returns the value that would actually be used\n\n\
                 Local only:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"user.email\", \"scope\": \"local\"})\n\
                 → Only checks .git/config\n\n\
                 Global only:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"user.email\", \"scope\": \"global\"})\n\
                 → Only checks ~/.gitconfig\n\n\
                 System only:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"user.email\", \"scope\": \"system\"})\n\
                 → Only checks /etc/gitconfig\n\n\
                 =============================================================================\n\
                 LISTING CONFIG\n\
                 =============================================================================\n\n\
                 List all effective config:\n\
                 git_config_get({\"path\": \"/project\", \"list\": true})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"values\": [\n\
                     {\"key\": \"user.name\", \"value\": \"John Doe\"},\n\
                     {\"key\": \"user.email\", \"value\": \"john@example.com\"},\n\
                     {\"key\": \"core.editor\", \"value\": \"vim\"},\n\
                     {\"key\": \"remote.origin.url\", \"value\": \"git@github.com:user/repo.git\"},\n\
                     ...\n\
                   ]\n\
                 }\n\n\
                 List local config only:\n\
                 git_config_get({\"path\": \"/project\", \"list\": true, \"scope\": \"local\"})\n\
                 → Only values from .git/config\n\n\
                 List global config only:\n\
                 git_config_get({\"path\": \"/project\", \"list\": true, \"scope\": \"global\"})\n\
                 → Only values from ~/.gitconfig\n\n\
                 =============================================================================\n\
                 COMMON CONFIG KEYS\n\
                 =============================================================================\n\n\
                 USER IDENTITY:\n\
                 - user.name: Your name\n\
                 - user.email: Your email\n\
                 - user.signingkey: GPG key for signing\n\n\
                 CORE BEHAVIOR:\n\
                 - core.editor: Text editor for Git messages\n\
                 - core.autocrlf: Line ending conversion\n\
                 - core.filemode: Track file permission changes\n\
                 - core.ignorecase: Case sensitivity in file names\n\
                 - core.quotepath: Quote non-ASCII characters in paths\n\n\
                 REMOTES:\n\
                 - remote.origin.url: Remote repository URL\n\
                 - remote.origin.fetch: Fetch refspec\n\
                 - remote.origin.push: Push refspec\n\n\
                 BRANCHES:\n\
                 - branch.<name>.remote: Which remote tracks this branch\n\
                 - branch.<name>.merge: Which remote branch to merge\n\
                 - branch.<name>.rebase: Whether to rebase on pull\n\n\
                 PUSH/PULL:\n\
                 - push.default: Default push behavior\n\
                 - push.followTags: Push tags with commits\n\
                 - pull.rebase: Default rebase behavior on pull\n\
                 - pull.ff: Fast-forward behavior on pull\n\n\
                 MERGE/DIFF:\n\
                 - merge.tool: Merge tool to use\n\
                 - merge.conflictstyle: Conflict marker style\n\
                 - diff.tool: Diff tool to use\n\
                 - diff.algorithm: Diff algorithm (myers, minimal, patience, histogram)\n\n\
                 COLOR:\n\
                 - color.ui: Enable color output (auto, always, never)\n\
                 - color.branch: Color branch output\n\
                 - color.diff: Color diff output\n\
                 - color.status: Color status output\n\n\
                 ALIASES:\n\
                 - alias.<name>: Custom command shortcuts\n\
                 - Example: alias.co = checkout\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 1. VERIFY IDENTITY BEFORE COMMIT:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"user.name\"})\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"user.email\"})\n\
                 → Ensure correct identity for this project\n\n\
                 2. CHECK REMOTE BEFORE PUSH:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"remote.origin.url\"})\n\
                 → Verify pushing to correct repository\n\n\
                 3. DEBUG PUSH/PULL BEHAVIOR:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"push.default\"})\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"pull.rebase\"})\n\
                 → Understand how commands behave\n\n\
                 4. AUDIT COMPLETE CONFIGURATION:\n\
                 git_config_get({\"path\": \"/project\", \"list\": true})\n\
                 → See all effective settings\n\n\
                 5. COMPARE SCOPES:\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"user.email\"})\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"user.email\", \"scope\": \"local\"})\n\
                 git_config_get({\"path\": \"/project\", \"key\": \"user.email\", \"scope\": \"global\"})\n\
                 → See where value comes from and what overrides what\n\n\
                 6. FIND PROJECT OVERRIDES:\n\
                 git_config_get({\"path\": \"/project\", \"list\": true, \"scope\": \"local\"})\n\
                 → See only repository-specific settings\n\n\
                 =============================================================================\n\
                 TROUBLESHOOTING\n\
                 =============================================================================\n\n\
                 WRONG AUTHOR IN COMMITS:\n\
                 1. Check effective identity:\n\
                    git_config_get({\"path\": \"/repo\", \"key\": \"user.name\"})\n\
                    git_config_get({\"path\": \"/repo\", \"key\": \"user.email\"})\n\
                 2. Check which scope it comes from:\n\
                    git_config_get({\"path\": \"/repo\", \"key\": \"user.email\", \"scope\": \"local\"})\n\
                    git_config_get({\"path\": \"/repo\", \"key\": \"user.email\", \"scope\": \"global\"})\n\
                 3. Set local override if needed (use git_config_set)\n\n\
                 PUSH GOES TO WRONG PLACE:\n\
                 1. Check remote URL:\n\
                    git_config_get({\"path\": \"/repo\", \"key\": \"remote.origin.url\"})\n\
                 2. Check push default:\n\
                    git_config_get({\"path\": \"/repo\", \"key\": \"push.default\"})\n\n\
                 UNEXPECTED LINE ENDING CHANGES:\n\
                 1. Check autocrlf setting:\n\
                    git_config_get({\"path\": \"/repo\", \"key\": \"core.autocrlf\"})\n\
                 2. Check at all scopes to find where it's set\n\n\
                 PERMISSION CHANGES SHOWING UP:\n\
                 1. Check filemode:\n\
                    git_config_get({\"path\": \"/repo\", \"key\": \"core.filemode\"})\n\
                 2. May need to disable on filesystems that don't support it\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. CHECK BEFORE COMMIT:\n\
                    Always verify user.name and user.email before first commit\n\n\
                 2. VERIFY REMOTES:\n\
                    Check remote.origin.url before pushing\n\n\
                 3. UNDERSTAND SCOPES:\n\
                    Know which scope a setting comes from\n\
                    Use local for project-specific overrides\n\
                    Use global for personal defaults\n\n\
                 4. LIST FOR AUDIT:\n\
                    Use list mode to review all settings periodically\n\n\
                 5. SCOPE-SPECIFIC CHECKS:\n\
                    Use scope parameter when debugging inheritance\n\n\
                 6. DOCUMENT SETUP:\n\
                    List and save config when setting up new machine\n\n\
                 =============================================================================\n\
                 RESPONSE FORMAT\n\
                 =============================================================================\n\n\
                 Single key response:\n\
                 {\n\
                   \"key\": \"user.name\",\n\
                   \"value\": \"John Doe\",\n\
                   \"scope\": \"global\"  // Optional, included if scope specified\n\
                 }\n\n\
                 List response:\n\
                 {\n\
                   \"values\": [\n\
                     {\"key\": \"user.name\", \"value\": \"John Doe\"},\n\
                     {\"key\": \"user.email\", \"value\": \"john@example.com\"},\n\
                     ...\n\
                   ]\n\
                 }\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Single value: git_config_get({\"path\": \"/repo\", \"key\": \"user.name\"})\n\
                 With scope: git_config_get({\"path\": \"/repo\", \"key\": \"user.email\", \"scope\": \"local\"})\n\
                 List all: git_config_get({\"path\": \"/repo\", \"list\": true})\n\
                 List scope: git_config_get({\"path\": \"/repo\", \"list\": true, \"scope\": \"global\"})\n\n\
                 Remember: This tool reads config. Use git_config_set to modify values!",
            ),
        },
    ]
}
