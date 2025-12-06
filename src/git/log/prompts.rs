//! Prompt messages for git_log tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitLogPromptArgs;

/// Prompt provider for git_log tool
///
/// This is the ONLY way to provide prompts for git_log - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct LogPrompts;

impl PromptProvider for LogPrompts {
    type PromptArgs = GitLogPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("filtering") => prompt_filtering(),
            Some("code_archaeology") => prompt_code_archaeology(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (filtering, code_archaeology)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT LOG
// ============================================================================

/// Basic commit history viewing with pagination
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I view commit history using the git_log tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_log tool displays commit history from a Git repository. Here's how to use it:\n\n\
                 BASIC COMMIT HISTORY:\n\
                 1. View all commits:\n\
                    git_log({\"path\": \"/project\"})\n\n\
                 2. View recent commits (limit to 10):\n\
                    git_log({\"path\": \"/project\", \"max_count\": 10})\n\n\
                 3. View latest commit:\n\
                    git_log({\"path\": \"/project\", \"max_count\": 1})\n\n\
                 PARAMETERS:\n\
                 - path (required): Path to Git repository\n\
                 - max_count (optional): Maximum number of commits to return\n\
                 - skip (optional): Number of commits to skip for pagination\n\
                 - path_filter (optional): Filter commits by file/directory path\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"commits\": [\n\
                     {\n\
                       \"id\": \"abc123...\",\n\
                       \"author\": {\"name\": \"John Doe\", \"email\": \"john@example.com\", \"time\": \"2024-01-15T10:30:00Z\"},\n\
                       \"summary\": \"Add user authentication\",\n\
                       \"time\": \"2024-01-15T10:30:00Z\"\n\
                     }\n\
                   ],\n\
                   \"count\": 10\n\
                 }\n\n\
                 READING THE OUTPUT:\n\
                 - id: Full commit SHA hash (use for git_diff)\n\
                 - author: Person who made the commit\n\
                 - summary: Commit message subject line\n\
                 - count: Total number of commits returned\n\n\
                 COMMIT ORDER:\n\
                 - Most recent commits appear FIRST\n\
                 - Reverse chronological order (newest to oldest)\n\
                 - First commit in array is the latest\n\n\
                 PAGINATION BASICS:\n\
                 Use skip and max_count together to paginate:\n\n\
                 Page 1 (commits 1-10):\n\
                 git_log({\"path\": \"/repo\", \"max_count\": 10, \"skip\": 0})\n\n\
                 Page 2 (commits 11-20):\n\
                 git_log({\"path\": \"/repo\", \"max_count\": 10, \"skip\": 10})\n\n\
                 Page 3 (commits 21-30):\n\
                 git_log({\"path\": \"/repo\", \"max_count\": 10, \"skip\": 20})\n\n\
                 PAGINATION FORMULA:\n\
                 skip = (page - 1) × page_size\n\n\
                 DETECT LAST PAGE:\n\
                 If count < max_count, this is the final page\n\n\
                 COMMON USE CASES:\n\
                 1. Review recent work:\n\
                    git_log({\"path\": \"/repo\", \"max_count\": 10})\n\n\
                 2. Quick check of latest change:\n\
                    git_log({\"path\": \"/repo\", \"max_count\": 1})\n\n\
                 3. Browse large repository:\n\
                    Use pagination to navigate thousands of commits\n\n\
                 PERFORMANCE TIPS:\n\
                 - Use max_count on large repositories to limit results\n\
                 - Start with 10-20 commits, increase if needed\n\
                 - Large repositories can have thousands of commits\n\
                 - Pagination helps avoid overwhelming data",
            ),
        },
    ]
}

/// Filtering commits by file and directory
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I filter git log results by file or directory?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use the path_filter parameter to narrow commit history to specific files or directories:\n\n\
                 FILTER BY FILE:\n\
                 1. Track changes to specific file:\n\
                    git_log({\"path\": \"/project\", \"path_filter\": \"src/main.rs\"})\n\
                    // Returns only commits that modified src/main.rs\n\n\
                 2. File with recent changes:\n\
                    git_log({\"path\": \"/project\", \"path_filter\": \"README.md\", \"max_count\": 5})\n\
                    // Last 5 commits that touched README.md\n\n\
                 3. Track configuration changes:\n\
                    git_log({\"path\": \"/project\", \"path_filter\": \"config.toml\"})\n\
                    // All commits affecting config.toml\n\n\
                 4. Monitor test file:\n\
                    git_log({\"path\": \"/project\", \"path_filter\": \"tests/integration.rs\"})\n\
                    // History of integration test changes\n\n\
                 FILTER BY DIRECTORY:\n\
                 1. Track module changes:\n\
                    git_log({\"path\": \"/project\", \"path_filter\": \"src/auth/\"})\n\
                    // All commits affecting files in src/auth/ directory\n\n\
                 2. Recent feature work:\n\
                    git_log({\"path\": \"/project\", \"path_filter\": \"src/features/payments/\", \"max_count\": 20})\n\
                    // Last 20 commits in payment feature directory\n\n\
                 3. Documentation changes:\n\
                    git_log({\"path\": \"/project\", \"path_filter\": \"docs/\"})\n\
                    // All documentation updates\n\n\
                 4. Test suite history:\n\
                    git_log({\"path\": \"/project\", \"path_filter\": \"tests/\", \"max_count\": 15})\n\
                    // Last 15 test-related commits\n\n\
                 PATH_FILTER SYNTAX:\n\
                 - Uses Git's pathspec syntax\n\
                 - Relative to repository root\n\
                 - Glob patterns work: \"*.rs\", \"src/**/*.toml\"\n\
                 - Directory paths: \"src/models/\" (trailing slash optional)\n\
                 - Exact file paths: \"src/main.rs\"\n\n\
                 COMBINE FILTERING WITH LIMITS:\n\
                 1. Recent changes to specific file:\n\
                    git_log({\"path\": \"/project\", \"path_filter\": \"Cargo.toml\", \"max_count\": 10})\n\n\
                 2. Latest work in directory:\n\
                    git_log({\"path\": \"/project\", \"path_filter\": \"src/handlers/\", \"max_count\": 20})\n\n\
                 3. Quick file history check:\n\
                    git_log({\"path\": \"/project\", \"path_filter\": \"src/lib.rs\", \"max_count\": 5})\n\n\
                 USE CASES:\n\n\
                 DEBUGGING:\n\
                 Find when bug was introduced:\n\
                 git_log({\"path\": \"/project\", \"path_filter\": \"src/buggy_module.rs\"})\n\
                 // Review commits to identify problematic change\n\n\
                 CODE ARCHAEOLOGY:\n\
                 Understand file evolution:\n\
                 git_log({\"path\": \"/project\", \"path_filter\": \"src/core.rs\"})\n\
                 // See how core module developed over time\n\n\
                 FEATURE TRACKING:\n\
                 Review feature development:\n\
                 git_log({\"path\": \"/project\", \"path_filter\": \"src/features/auth/\"})\n\
                 // All commits in authentication feature\n\n\
                 WHO CHANGED WHAT:\n\
                 Find who modified file:\n\
                 git_log({\"path\": \"/project\", \"path_filter\": \"config.json\"})\n\
                 // Review author information for each commit",
            ),
        },
    ]
}

/// Code archaeology and file history tracking
fn prompt_code_archaeology() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I track file history and perform code archaeology with git_log?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_log for code archaeology to understand when, why, and by whom changes were made:\n\n\
                 BASIC FILE TRACKING:\n\
                 See all commits that modified a file:\n\
                 git_log({\"path\": \"/project\", \"path_filter\": \"src/main.rs\"})\n\
                 // Complete history of src/main.rs\n\n\
                 Recent file changes:\n\
                 git_log({\"path\": \"/project\", \"path_filter\": \"src/auth.rs\", \"max_count\": 10})\n\
                 // Last 10 changes to authentication module\n\n\
                 Find contributors:\n\
                 git_log({\"path\": \"/project\", \"path_filter\": \"src/api.rs\"})\n\
                 // Review author.name and author.email fields\n\n\
                 USE CASES:\n\n\
                 BUG INVESTIGATION (10-step workflow):\n\
                 1. Find commits to problematic file:\n\
                    git_log({\"path\": \"/project\", \"path_filter\": \"src/buggy_module.rs\"})\n\
                 2. Review commit messages for suspicious changes\n\
                 3. Look for \"refactor\", \"optimize\", \"cleanup\" messages\n\
                 4. Use git_diff to inspect specific commits:\n\
                    git_diff({\"path\": \"/project\", \"from\": \"commit_id^\", \"to\": \"commit_id\"})\n\
                 5. Identify the problematic commit\n\n\
                 FEATURE TRACKING:\n\
                 See how feature evolved:\n\
                 git_log({\"path\": \"/project\", \"path_filter\": \"src/features/payments/\"})\n\
                 // All commits in payment feature directory\n\n\
                 Recent feature work:\n\
                 git_log({\"path\": \"/project\", \"path_filter\": \"src/features/analytics/\", \"max_count\": 30})\n\n\
                 MODULE EVOLUTION:\n\
                 Track module changes over time:\n\
                 git_log({\"path\": \"/project\", \"path_filter\": \"src/database/\"})\n\
                 // See how database module developed\n\n\
                 CONFIGURATION CHANGES:\n\
                 Track config modifications:\n\
                 git_log({\"path\": \"/project\", \"path_filter\": \"config.toml\"})\n\
                 // Who changed configuration and when\n\n\
                 DEPENDENCY TRACKING:\n\
                 Monitor dependency changes:\n\
                 git_log({\"path\": \"/project\", \"path_filter\": \"Cargo.toml\"})\n\
                 // All dependency updates and version changes\n\n\
                 CODE ARCHAEOLOGY WORKFLOW:\n\
                 1. Identify the file or module\n\
                 2. Get complete history:\n\
                    git_log({\"path\": \"/repo\", \"path_filter\": \"src/problem.rs\"})\n\
                 3. Review commit messages for patterns\n\
                 4. Narrow suspects with max_count:\n\
                    git_log({\"path\": \"/repo\", \"path_filter\": \"src/problem.rs\", \"max_count\": 20})\n\
                 5. Use git_diff on suspicious commits\n\
                 6. Identify root cause\n\n\
                 EXTRACTING INSIGHTS:\n\
                 - author.name: Who made the change\n\
                 - author.email: Contact information\n\
                 - author.time: When change was made\n\
                 - summary: Commit message describing intent\n\
                 - id: Full SHA hash for git_diff comparisons",
            ),
        },
    ]
}
