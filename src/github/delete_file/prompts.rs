//! Prompt messages for github_delete_file tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GithubDeleteFilePromptArgs;

/// Prompt provider for github_delete_file tool
///
/// This is the ONLY way to provide prompts for github_delete_file - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GithubDeleteFilePrompts;

impl PromptProvider for GithubDeleteFilePrompts {
    type PromptArgs = GithubDeleteFilePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("safety") => prompt_safety(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, safety, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO DELETE FILES FROM GITHUB
// ============================================================================

/// Basic file deletion examples
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I delete files from a GitHub repository using github_delete_file?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_delete_file tool removes files from GitHub repositories via the API without requiring a local clone. Here's how to use it:\n\n\
                 BASIC FILE DELETION:\n\
                 1. Delete a file (requires SHA):\n\
                    github_delete_file({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"old_file.txt\",\n\
                        \"message\": \"Remove old file\",\n\
                        \"sha\": \"abc1234567890...\"\n\
                    })\n\n\
                 2. Delete file from subdirectory:\n\
                    github_delete_file({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"docs/deprecated.md\",\n\
                        \"message\": \"Remove deprecated documentation\",\n\
                        \"sha\": \"def5678901234...\"\n\
                    })\n\n\
                 3. Delete file from specific branch:\n\
                    github_delete_file({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"config/old.json\",\n\
                        \"message\": \"Remove old config\",\n\
                        \"sha\": \"ghi9012345678...\",\n\
                        \"branch\": \"develop\"\n\
                    })\n\n\
                 SHA REQUIREMENT (CRITICAL!):\n\
                 - Every deletion REQUIRES the current file SHA\n\
                 - SHA ensures you're deleting the exact version you expect\n\
                 - Prevents race conditions and accidental overwrites\n\
                 - Get SHA from github_get_file_contents before deleting\n\n\
                 HOW TO GET FILE SHA:\n\
                 1. First, get file info:\n\
                    github_get_file_contents({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"file.txt\"\n\
                    })\n\
                 2. Response includes sha field:\n\
                    {\n\
                        \"name\": \"file.txt\",\n\
                        \"path\": \"file.txt\",\n\
                        \"sha\": \"abc1234567890...\",\n\
                        \"content\": \"...\"\n\
                    }\n\
                 3. Use that SHA in delete_file call\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                     \"commit\": {\n\
                         \"sha\": \"xyz7890123456...\",\n\
                         \"message\": \"Remove old file\",\n\
                         \"author\": {\n\
                             \"name\": \"Your Name\",\n\
                             \"email\": \"you@example.com\"\n\
                         }\n\
                     },\n\
                     \"content\": null\n\
                 }\n\
                 - commit.sha: New commit created by deletion\n\
                 - commit.message: Your deletion commit message\n\
                 - content: Always null (file no longer exists)\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or org)\n\
                 - repo (required): Repository name\n\
                 - path (required): File path in repository\n\
                 - message (required): Commit message for deletion\n\
                 - sha (required): Current file SHA (from get_file_contents)\n\
                 - branch (optional): Target branch (default: default branch)\n\
                 - committer (optional): Override committer info\n\
                 - author (optional): Override author info\n\n\
                 COMMON MISTAKES:\n\
                 1. Forgetting to get SHA first:\n\
                    WRONG: github_delete_file({...})  // Missing SHA!\n\
                    RIGHT: Get SHA with get_file_contents first\n\n\
                 2. Using wrong or outdated SHA:\n\
                    - Always get fresh SHA immediately before deletion\n\
                    - Don't reuse old SHAs from previous calls\n\n\
                 3. Trying to delete directories:\n\
                    - Can only delete individual files\n\
                    - Delete each file separately for directory cleanup\n\n\
                 4. Poor commit messages:\n\
                    BAD: \"delete\"\n\
                    GOOD: \"Remove deprecated config file\"\n\n\
                 PATH FORMATS:\n\
                 - Use forward slashes: \"src/components/old.tsx\"\n\
                 - No leading slash: \"file.txt\" not \"/file.txt\"\n\
                 - Case-sensitive paths (match exactly)\n\
                 - Unicode characters supported in paths\n\n\
                 EXAMPLES BY FILE TYPE:\n\
                 1. Delete source code:\n\
                    github_delete_file({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"app\",\n\
                        \"path\": \"src/deprecated.js\",\n\
                        \"message\": \"Remove deprecated module\",\n\
                        \"sha\": \"...\"\n\
                    })\n\n\
                 2. Delete config file:\n\
                    github_delete_file({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"app\",\n\
                        \"path\": \".env.example.old\",\n\
                        \"message\": \"Remove old environment example\",\n\
                        \"sha\": \"...\"\n\
                    })\n\n\
                 3. Delete documentation:\n\
                    github_delete_file({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"app\",\n\
                        \"path\": \"docs/outdated-guide.md\",\n\
                        \"message\": \"Remove outdated documentation\",\n\
                        \"sha\": \"...\"\n\
                    })\n\n\
                 WORKFLOW PATTERN:\n\
                 Step 1: Get file SHA\n\
                 Step 2: Verify it's the right file (optional but recommended)\n\
                 Step 3: Delete with that SHA\n\
                 Step 4: Verify deletion success from response\n\n\
                 ERROR HANDLING:\n\
                 - 404: File not found or already deleted\n\
                 - 409: SHA mismatch (file changed since you got SHA)\n\
                 - 422: Invalid parameters or cannot delete\n\
                 - If SHA mismatch: Get fresh SHA and try again",
            ),
        },
    ]
}

/// Safe deletion practices and patterns
fn prompt_safety() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are the safety best practices for deleting files from GitHub?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Safe file deletion requires careful verification and proper SHA handling to prevent accidents. Follow these practices:\n\n\
                 SAFE DELETION WORKFLOW:\n\
                 1. Get current file info and SHA:\n\
                    github_get_file_contents({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"file.txt\"\n\
                    })\n\
                    // Returns: {sha: \"abc123...\", content: \"...\"}\n\n\
                 2. Verify it's the correct file (RECOMMENDED):\n\
                    - Check file name matches expectation\n\
                    - Review content if sensitive deletion\n\
                    - Confirm path is exactly what you want to delete\n\n\
                 3. Delete using fresh SHA:\n\
                    github_delete_file({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"file.txt\",\n\
                        \"message\": \"Remove obsolete configuration file\",\n\
                        \"sha\": \"abc123...\"  // Use SHA from step 1\n\
                    })\n\n\
                 4. Verify deletion succeeded:\n\
                    - Check response for commit.sha\n\
                    - Response content is null (file deleted)\n\n\
                 WHY SHA IS REQUIRED:\n\
                 - Prevents race conditions (two people editing simultaneously)\n\
                 - Ensures you delete the exact version you reviewed\n\
                 - GitHub rejects deletion if file changed since you got SHA\n\
                 - Acts as confirmation you know what you're deleting\n\n\
                 SHA MISMATCH SCENARIO:\n\
                 Problem: Someone else modified file between your get_file_contents and delete_file\n\
                 1. You get SHA: \"abc123\"\n\
                 2. Someone else commits change: SHA becomes \"def456\"\n\
                 3. You try delete with \"abc123\": GitHub returns 409 Conflict\n\
                 4. Solution: Get fresh SHA and verify changes are acceptable, then delete again\n\n\
                 Example handling SHA mismatch:\n\
                 // First attempt\n\
                 github_get_file_contents({...})  // SHA: \"abc123\"\n\
                 github_delete_file({sha: \"abc123\"})  // Fails with 409\n\
                 // File was modified, get new SHA\n\
                 github_get_file_contents({...})  // SHA: \"def456\" (changed!)\n\
                 // Review changes, then delete with new SHA\n\
                 github_delete_file({sha: \"def456\"})\n\n\
                 SAFETY MEASURES:\n\
                 1. ALWAYS get fresh SHA immediately before deletion\n\
                    - Don't reuse SHAs from hours or days ago\n\
                    - Get SHA right before delete in same workflow\n\n\
                 2. Verify before delete:\n\
                    - Read file content to confirm it's what you think\n\
                    - Check file size if expecting specific file\n\
                    - Verify path doesn't contain typos\n\n\
                 3. Use descriptive commit messages:\n\
                    BAD: \"delete\", \"remove\", \"cleanup\"\n\
                    GOOD: \"Remove deprecated v1 API endpoints\"\n\
                    GOOD: \"Delete temporary test fixtures\"\n\
                    GOOD: \"Remove sensitive credentials file\"\n\n\
                 4. Consider branch safety:\n\
                    - Delete on feature branch first to test\n\
                    - Review changes before merging to main\n\
                    - Use branch parameter to target non-default branch\n\n\
                 LIMITATIONS AND CONSTRAINTS:\n\
                 1. Cannot delete directories directly:\n\
                    - Must delete each file individually\n\
                    - Directory automatically removed when empty\n\
                    - Use multiple delete_file calls for multiple files\n\n\
                 2. Cannot undo deletions (without reverting commit):\n\
                    - File is permanently removed from branch\n\
                    - History preserved in Git (can recover from previous commits)\n\
                    - Consider creating backup branch before bulk deletions\n\n\
                 3. Requires repository write access:\n\
                    - Must have push permissions\n\
                    - Cannot delete files in repositories you can't write to\n\n\
                 BRANCH-SPECIFIC SAFETY:\n\
                 1. Protect main branch by testing on feature branch:\n\
                    // Create and switch to test branch first (separate tool)\n\
                    github_delete_file({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"file.txt\",\n\
                        \"message\": \"Test deletion\",\n\
                        \"sha\": \"...\",\n\
                        \"branch\": \"test-deletion\"  // Test first!\n\
                    })\n\
                    // Verify on test branch, then merge if good\n\n\
                 2. Default branch behavior:\n\
                    // If branch not specified, uses repository's default branch\n\
                    github_delete_file({...})  // Deletes from main/master\n\n\
                 SENSITIVE FILE DELETION:\n\
                 For files with credentials or secrets:\n\
                 1. Delete file (creates commit)\n\
                 2. File is removed but HISTORY remains\n\
                 3. For true secret removal: Use git-filter-branch or BFG\n\
                 4. Consider rotating exposed credentials\n\
                 5. Deletion only removes from current branch state\n\n\
                 VERIFICATION CHECKLIST:\n\
                 Before deleting:\n\
                 □ Confirmed correct repository (owner/repo)\n\
                 □ Verified exact file path\n\
                 □ Obtained fresh SHA\n\
                 □ Reviewed file content if sensitive\n\
                 □ Written clear commit message\n\
                 □ Specified correct branch (if not default)\n\n\
                 After deleting:\n\
                 □ Response contains commit SHA\n\
                 □ Response content is null\n\
                 □ Commit message appears correctly\n\n\
                 RECOVERY FROM ACCIDENTAL DELETION:\n\
                 If you accidentally delete a file:\n\
                 1. File still exists in Git history\n\
                 2. Get previous commit SHA (before deletion)\n\
                 3. Use get_file_contents with ref parameter to retrieve old version\n\
                 4. Use create_or_update_file to restore it\n\n\
                 BEST PRACTICES SUMMARY:\n\
                 1. Get SHA immediately before deletion\n\
                 2. Verify file identity before deleting\n\
                 3. Use descriptive commit messages\n\
                 4. Test on feature branch for risky deletions\n\
                 5. Handle SHA mismatch by re-fetching\n\
                 6. Remember: deletion affects current branch only\n\
                 7. Git history preserves file for recovery if needed",
            ),
        },
    ]
}

/// Deletion workflows and common use cases
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows and use cases for deleting files from GitHub?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "File deletion supports various workflows from cleanup to refactoring. Here are common patterns:\n\n\
                 WORKFLOW 1: CLEANUP TEMPORARY FILES\n\
                 Use case: Remove debug logs, temp files, or test artifacts\n\
                 // Step 1: Get file info\n\
                 github_get_file_contents({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"temp/debug.log\"\n\
                 })\n\
                 // Step 2: Delete\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"temp/debug.log\",\n\
                     \"message\": \"Clean up debug logs from development\",\n\
                     \"sha\": \"abc123...\"\n\
                 })\n\n\
                 WORKFLOW 2: REMOVE DEPRECATED CODE\n\
                 Use case: Delete outdated implementations after refactoring\n\
                 // Delete old implementation\n\
                 github_get_file_contents({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"src/legacy/old_api.js\"\n\
                 })\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"src/legacy/old_api.js\",\n\
                     \"message\": \"Remove deprecated v1 API implementation (replaced by v2)\",\n\
                     \"sha\": \"def456...\"\n\
                 })\n\n\
                 WORKFLOW 3: BRANCH-SPECIFIC DELETION\n\
                 Use case: Remove files from development branch\n\
                 github_get_file_contents({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"experiments/prototype.py\",\n\
                     \"ref\": \"develop\"  // Get from specific branch\n\
                 })\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"experiments/prototype.py\",\n\
                     \"message\": \"Remove experimental prototype\",\n\
                     \"sha\": \"ghi789...\",\n\
                     \"branch\": \"develop\"  // Delete from develop branch\n\
                 })\n\n\
                 WORKFLOW 4: BULK FILE DELETION\n\
                 Use case: Remove multiple related files\n\
                 // Delete each file separately (no batch delete)\n\
                 const files = [\"old1.txt\", \"old2.txt\", \"old3.txt\"];\n\
                 for (const file of files) {\n\
                     // Get SHA for each file\n\
                     const info = github_get_file_contents({\n\
                         \"owner\": \"user\",\n\
                         \"repo\": \"project\",\n\
                         \"path\": file\n\
                     });\n\
                     // Delete with that file's SHA\n\
                     github_delete_file({\n\
                         \"owner\": \"user\",\n\
                         \"repo\": \"project\",\n\
                         \"path\": file,\n\
                         \"message\": `Remove obsolete file: ${file}`,\n\
                         \"sha\": info.sha\n\
                     });\n\
                 }\n\n\
                 WORKFLOW 5: CONFIGURATION CLEANUP\n\
                 Use case: Remove old config files after migration\n\
                 // Remove old config\n\
                 github_get_file_contents({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"config/database.old.yml\"\n\
                 })\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"config/database.old.yml\",\n\
                     \"message\": \"Remove old database config (migrated to new format)\",\n\
                     \"sha\": \"jkl012...\"\n\
                 })\n\n\
                 WORKFLOW 6: DOCUMENTATION UPDATES\n\
                 Use case: Remove outdated documentation\n\
                 github_get_file_contents({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"docs/v1-guide.md\"\n\
                 })\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"docs/v1-guide.md\",\n\
                     \"message\": \"Remove v1 documentation (v1 EOL)\",\n\
                     \"sha\": \"mno345...\"\n\
                 })\n\n\
                 WORKFLOW 7: SENSITIVE DATA REMOVAL\n\
                 Use case: Remove accidentally committed secrets\n\
                 // Remove from current branch (history still contains it!)\n\
                 github_get_file_contents({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \".env.production\"\n\
                 })\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \".env.production\",\n\
                     \"message\": \"Remove accidentally committed production secrets\",\n\
                     \"sha\": \"pqr678...\"\n\
                 })\n\
                 // IMPORTANT: Also rotate exposed credentials!\n\
                 // Consider git-filter-branch to remove from history\n\n\
                 WORKFLOW 8: TEST FILE CLEANUP\n\
                 Use case: Remove obsolete test files\n\
                 github_get_file_contents({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"tests/deprecated_test.js\"\n\
                 })\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"tests/deprecated_test.js\",\n\
                     \"message\": \"Remove tests for deleted feature\",\n\
                     \"sha\": \"stu901...\"\n\
                 })\n\n\
                 WORKFLOW 9: ASSET REMOVAL\n\
                 Use case: Delete unused images or media files\n\
                 github_get_file_contents({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"assets/old-logo.png\"\n\
                 })\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"assets/old-logo.png\",\n\
                     \"message\": \"Remove old logo after rebranding\",\n\
                     \"sha\": \"vwx234...\"\n\
                 })\n\n\
                 WORKFLOW 10: DEPENDENCY CLEANUP\n\
                 Use case: Remove unused dependency files\n\
                 github_get_file_contents({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"vendor/unused-lib.js\"\n\
                 })\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"vendor/unused-lib.js\",\n\
                     \"message\": \"Remove unused vendor library\",\n\
                     \"sha\": \"yza567...\"\n\
                 })\n\n\
                 USE CASES SUMMARY:\n\
                 1. Cleanup workflows:\n\
                    - Remove temporary/debug files\n\
                    - Delete test artifacts\n\
                    - Clean up build outputs\n\n\
                 2. Code refactoring:\n\
                    - Remove deprecated implementations\n\
                    - Delete obsolete tests\n\
                    - Clean up after migrations\n\n\
                 3. Configuration management:\n\
                    - Remove old config files\n\
                    - Delete environment-specific configs\n\
                    - Clean up after config migrations\n\n\
                 4. Documentation maintenance:\n\
                    - Remove outdated guides\n\
                    - Delete superseded documentation\n\
                    - Clean up old examples\n\n\
                 5. Security:\n\
                    - Remove accidentally committed secrets\n\
                    - Delete sensitive data files\n\
                    - Clean up credentials (remember: also rotate!)\n\n\
                 6. Asset management:\n\
                    - Remove unused images\n\
                    - Delete old media files\n\
                    - Clean up after redesigns\n\n\
                 7. Dependency management:\n\
                    - Remove unused libraries\n\
                    - Delete obsolete vendor files\n\
                    - Clean up after dependency updates\n\n\
                 ANTI-PATTERNS TO AVOID:\n\
                 1. Deleting without getting fresh SHA:\n\
                    BAD: Reusing old SHA from cache\n\
                    GOOD: Get SHA immediately before delete\n\n\
                 2. Batch deleting without verification:\n\
                    BAD: Loop and delete without checking each file\n\
                    GOOD: Verify each file before deletion\n\n\
                 3. Generic commit messages:\n\
                    BAD: \"delete files\"\n\
                    GOOD: \"Remove deprecated v1 API endpoints\"\n\n\
                 4. Deleting on main without testing:\n\
                    BAD: Direct deletion on main branch\n\
                    GOOD: Test on feature branch first\n\n\
                 5. Ignoring SHA mismatch errors:\n\
                    BAD: Retrying with same old SHA\n\
                    GOOD: Get fresh SHA and review changes\n\n\
                 WORKFLOW BEST PRACTICES:\n\
                 1. Plan deletions (list files to delete)\n\
                 2. Create feature branch for major cleanups\n\
                 3. Get fresh SHA for each file\n\
                 4. Use descriptive commit messages\n\
                 5. Verify deletions succeeded\n\
                 6. Review changes before merging to main\n\
                 7. For sensitive files: Also rotate credentials",
            ),
        },
    ]
}

/// Comprehensive guide covering all aspects of file deletion
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to deleting files from GitHub repositories.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_delete_file tool removes files from GitHub repositories via the API without requiring a local clone. Here's everything you need to know:\n\n\
                 =============================================================================\n\
                 OVERVIEW\n\
                 =============================================================================\n\n\
                 WHAT IT DOES:\n\
                 - Deletes individual files from GitHub repositories\n\
                 - Creates a commit for the deletion\n\
                 - Works via API (no local clone needed)\n\
                 - Requires file SHA for safety\n\
                 - Supports branch-specific operations\n\n\
                 WHAT IT CANNOT DO:\n\
                 - Cannot delete directories directly (delete files individually)\n\
                 - Cannot batch delete (one file per call)\n\
                 - Cannot delete without write access\n\
                 - Cannot undo (must revert commit to restore)\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 SIMPLE FILE DELETION:\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",       // Required: repo owner\n\
                     \"repo\": \"project\",         // Required: repo name\n\
                     \"path\": \"file.txt\",        // Required: file path\n\
                     \"message\": \"Remove file\",  // Required: commit message\n\
                     \"sha\": \"abc123...\"         // Required: current file SHA\n\
                 })\n\n\
                 ALL PARAMETERS:\n\
                 - owner (string, required): Repository owner (user or org)\n\
                 - repo (string, required): Repository name\n\
                 - path (string, required): File path in repository\n\
                 - message (string, required): Commit message\n\
                 - sha (string, required): Current file SHA (from get_file_contents)\n\
                 - branch (string, optional): Target branch (default: repo default branch)\n\
                 - committer (object, optional): Override committer info\n\
                 - author (object, optional): Override author info\n\n\
                 =============================================================================\n\
                 SHA REQUIREMENT (CRITICAL!)\n\
                 =============================================================================\n\n\
                 WHY SHA IS REQUIRED:\n\
                 - Ensures you delete the exact version you reviewed\n\
                 - Prevents race conditions (concurrent modifications)\n\
                 - Acts as confirmation you know what you're deleting\n\
                 - GitHub rejects deletion if SHA doesn't match current file\n\n\
                 HOW TO GET SHA:\n\
                 Step 1: Call github_get_file_contents:\n\
                 github_get_file_contents({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"file.txt\"\n\
                 })\n\n\
                 Step 2: Extract SHA from response:\n\
                 Response: {\n\
                     \"name\": \"file.txt\",\n\
                     \"path\": \"file.txt\",\n\
                     \"sha\": \"abc1234567890...\",  // Use this!\n\
                     \"size\": 1234,\n\
                     \"content\": \"...\"\n\
                 }\n\n\
                 Step 3: Use SHA in delete call:\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"file.txt\",\n\
                     \"message\": \"Remove file\",\n\
                     \"sha\": \"abc1234567890...\"  // SHA from step 2\n\
                 })\n\n\
                 SHA MISMATCH (409 Error):\n\
                 Occurs when file changed between get_file_contents and delete_file:\n\
                 1. You get SHA: \"abc123\"\n\
                 2. Someone else modifies file: SHA becomes \"def456\"\n\
                 3. You try delete with \"abc123\": GitHub returns 409 Conflict\n\
                 4. Solution: Get fresh SHA and verify changes are acceptable\n\n\
                 ALWAYS:\n\
                 - Get SHA immediately before deletion\n\
                 - Don't reuse old SHAs from previous operations\n\
                 - Get fresh SHA if deletion fails with 409\n\n\
                 =============================================================================\n\
                 STANDARD WORKFLOW\n\
                 =============================================================================\n\n\
                 COMPLETE DELETION WORKFLOW:\n\
                 1. Get file information and SHA:\n\
                    const fileInfo = github_get_file_contents({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"file.txt\"\n\
                    });\n\
                    // Returns: {sha: \"abc123...\", content: \"...\"}\n\n\
                 2. Verify it's the correct file (RECOMMENDED):\n\
                    - Check file.name matches expectation\n\
                    - Review content if sensitive\n\
                    - Confirm path is exactly right\n\n\
                 3. Delete using the SHA:\n\
                    const result = github_delete_file({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"file.txt\",\n\
                        \"message\": \"Remove obsolete configuration\",\n\
                        \"sha\": fileInfo.sha\n\
                    });\n\n\
                 4. Verify deletion succeeded:\n\
                    - result.commit.sha exists (new commit created)\n\
                    - result.content is null (file deleted)\n\n\
                 =============================================================================\n\
                 RESPONSE STRUCTURE\n\
                 =============================================================================\n\n\
                 SUCCESSFUL DELETION RESPONSE:\n\
                 {\n\
                     \"commit\": {\n\
                         \"sha\": \"xyz7890...\",           // New commit SHA\n\
                         \"message\": \"Remove file\",      // Your message\n\
                         \"author\": {\n\
                             \"name\": \"Your Name\",\n\
                             \"email\": \"you@example.com\",\n\
                             \"date\": \"2025-12-05T...\"\n\
                         },\n\
                         \"committer\": {\n\
                             \"name\": \"GitHub\",\n\
                             \"email\": \"noreply@github.com\",\n\
                             \"date\": \"2025-12-05T...\"\n\
                         },\n\
                         \"tree\": {\"sha\": \"...\"},\n\
                         \"parents\": [{\"sha\": \"...\"}]\n\
                     },\n\
                     \"content\": null  // Always null after deletion\n\
                 }\n\n\
                 KEY FIELDS:\n\
                 - commit.sha: New commit created by deletion\n\
                 - commit.message: Your commit message\n\
                 - content: null (file no longer exists)\n\n\
                 =============================================================================\n\
                 PATH FORMATS\n\
                 =============================================================================\n\n\
                 CORRECT PATH FORMATS:\n\
                 ✓ \"file.txt\"                    (root file)\n\
                 ✓ \"src/main.js\"                (subdirectory)\n\
                 ✓ \"docs/api/reference.md\"      (nested)\n\
                 ✓ \".env.example\"               (hidden file)\n\
                 ✓ \"config/app.config.js\"       (multiple dots)\n\n\
                 INCORRECT PATH FORMATS:\n\
                 ✗ \"/file.txt\"                  (no leading slash)\n\
                 ✗ \"src\\\\main.js\"               (use forward slash)\n\
                 ✗ \"./file.txt\"                 (no ./ prefix)\n\
                 ✗ \"src/\"                       (no trailing slash)\n\n\
                 PATH RULES:\n\
                 - Use forward slashes (/) not backslashes (\\)\n\
                 - No leading slash\n\
                 - No trailing slash\n\
                 - Case-sensitive (match exact capitalization)\n\
                 - Unicode characters supported\n\n\
                 =============================================================================\n\
                 BRANCH OPERATIONS\n\
                 =============================================================================\n\n\
                 DEFAULT BRANCH DELETION:\n\
                 // Deletes from repository's default branch (usually main/master)\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"file.txt\",\n\
                     \"message\": \"Remove file\",\n\
                     \"sha\": \"...\"\n\
                     // branch not specified = default branch\n\
                 })\n\n\
                 SPECIFIC BRANCH DELETION:\n\
                 // Delete from a specific branch\n\
                 // Step 1: Get SHA from that branch\n\
                 github_get_file_contents({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"file.txt\",\n\
                     \"ref\": \"develop\"  // Get from develop branch\n\
                 })\n\
                 // Step 2: Delete from that branch\n\
                 github_delete_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"file.txt\",\n\
                     \"message\": \"Remove file from develop\",\n\
                     \"sha\": \"...\",\n\
                     \"branch\": \"develop\"  // Delete from develop\n\
                 })\n\n\
                 BRANCH SAFETY:\n\
                 - Test deletions on feature branch first\n\
                 - Review changes before merging to main\n\
                 - Use branch parameter to protect main branch\n\n\
                 =============================================================================\n\
                 COMMON USE CASES\n\
                 =============================================================================\n\n\
                 1. REMOVE DEPRECATED CODE:\n\
                 github_delete_file({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"app\",\n\
                     \"path\": \"src/deprecated/old_api.js\",\n\
                     \"message\": \"Remove deprecated v1 API (replaced by v2)\",\n\
                     \"sha\": \"...\"\n\
                 })\n\n\
                 2. CLEAN UP TEMPORARY FILES:\n\
                 github_delete_file({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"app\",\n\
                     \"path\": \"temp/debug.log\",\n\
                     \"message\": \"Remove debug logs\",\n\
                     \"sha\": \"...\"\n\
                 })\n\n\
                 3. REMOVE OLD CONFIGURATION:\n\
                 github_delete_file({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"app\",\n\
                     \"path\": \"config/old-settings.yml\",\n\
                     \"message\": \"Remove old config (migrated to new format)\",\n\
                     \"sha\": \"...\"\n\
                 })\n\n\
                 4. DELETE DOCUMENTATION:\n\
                 github_delete_file({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"app\",\n\
                     \"path\": \"docs/outdated-guide.md\",\n\
                     \"message\": \"Remove outdated documentation\",\n\
                     \"sha\": \"...\"\n\
                 })\n\n\
                 5. REMOVE TEST FILES:\n\
                 github_delete_file({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"app\",\n\
                     \"path\": \"tests/obsolete_test.js\",\n\
                     \"message\": \"Remove tests for deleted feature\",\n\
                     \"sha\": \"...\"\n\
                 })\n\n\
                 =============================================================================\n\
                 SAFETY AND BEST PRACTICES\n\
                 =============================================================================\n\n\
                 SAFETY CHECKLIST:\n\
                 Before deletion:\n\
                 □ Confirmed correct repository (owner/repo)\n\
                 □ Verified exact file path (no typos)\n\
                 □ Obtained fresh SHA\n\
                 □ Reviewed file content if sensitive\n\
                 □ Written clear, descriptive commit message\n\
                 □ Specified correct branch (if not default)\n\
                 □ Tested on feature branch (for risky deletions)\n\n\
                 After deletion:\n\
                 □ Response contains commit.sha\n\
                 □ Response content is null\n\
                 □ Commit message correct\n\
                 □ Changes reviewed before merging\n\n\
                 BEST PRACTICES:\n\
                 1. ALWAYS get fresh SHA immediately before deletion\n\
                 2. Verify file identity before deleting\n\
                 3. Use descriptive commit messages\n\
                 4. Test on feature branch for important deletions\n\
                 5. Handle SHA mismatch by re-fetching\n\
                 6. Remember: deletion affects current branch only\n\
                 7. Git history preserves file for recovery\n\n\
                 COMMIT MESSAGE QUALITY:\n\
                 BAD: \"delete\", \"remove\", \"cleanup\"\n\
                 GOOD: \"Remove deprecated v1 API endpoints\"\n\
                 GOOD: \"Delete temporary test fixtures after migration\"\n\
                 GOOD: \"Remove sensitive credentials file\"\n\n\
                 =============================================================================\n\
                 ERROR HANDLING\n\
                 =============================================================================\n\n\
                 COMMON ERRORS:\n\
                 1. 404 Not Found:\n\
                    - File doesn't exist at specified path\n\
                    - Repository doesn't exist\n\
                    - No access to repository\n\
                    Solution: Verify owner/repo/path are correct\n\n\
                 2. 409 Conflict (SHA mismatch):\n\
                    - File changed since you got SHA\n\
                    - Someone else modified or deleted file\n\
                    Solution: Get fresh SHA and try again\n\n\
                 3. 422 Unprocessable Entity:\n\
                    - Invalid parameters\n\
                    - Missing required field\n\
                    - Malformed SHA\n\
                    Solution: Check all required parameters\n\n\
                 4. 403 Forbidden:\n\
                    - No write access to repository\n\
                    - Repository is archived\n\
                    - Branch protection prevents deletion\n\
                    Solution: Check permissions and branch protection\n\n\
                 HANDLING SHA MISMATCH:\n\
                 if (error.status === 409) {\n\
                     // File changed, get fresh SHA\n\
                     const updated = github_get_file_contents({...});\n\
                     // Review changes to ensure still want to delete\n\
                     // Then retry with new SHA\n\
                     github_delete_file({sha: updated.sha, ...});\n\
                 }\n\n\
                 =============================================================================\n\
                 LIMITATIONS AND CONSTRAINTS\n\
                 =============================================================================\n\n\
                 CANNOT DO:\n\
                 1. Cannot delete directories:\n\
                    - Must delete each file individually\n\
                    - Directory removed automatically when empty\n\n\
                 2. Cannot batch delete:\n\
                    - One file per API call\n\
                    - For multiple files: call delete_file for each\n\n\
                 3. Cannot undo directly:\n\
                    - Creates permanent commit\n\
                    - Recovery requires reverting commit or restoring from history\n\n\
                 4. Requires write access:\n\
                    - Must have push permissions\n\
                    - Cannot delete in read-only repos\n\n\
                 5. History remains:\n\
                    - File deleted from current branch state\n\
                    - Still exists in Git history\n\
                    - For complete removal: use git-filter-branch or BFG\n\n\
                 =============================================================================\n\
                 RECOVERY FROM ACCIDENTAL DELETION\n\
                 =============================================================================\n\n\
                 IF YOU ACCIDENTALLY DELETE A FILE:\n\
                 1. File still exists in Git history\n\
                 2. Find commit before deletion (use list_commits or get commit SHA from deletion response's parent)\n\
                 3. Get file from previous commit:\n\
                    github_get_file_contents({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"file.txt\",\n\
                        \"ref\": \"<commit-sha-before-deletion>\"\n\
                    })\n\
                 4. Restore file using create_or_update_file:\n\
                    github_create_or_update_file({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"file.txt\",\n\
                        \"message\": \"Restore accidentally deleted file\",\n\
                        \"content\": \"<base64-content-from-step-3>\"\n\
                    })\n\n\
                 OR: Revert the deletion commit using Git\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Basic deletion:\n\
                 github_delete_file({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"file.txt\",\n\
                     \"message\": \"Remove file\",\n\
                     \"sha\": \"abc123...\"\n\
                 })\n\n\
                 Branch-specific:\n\
                 github_delete_file({..., \"branch\": \"develop\"})\n\n\
                 Get SHA first:\n\
                 github_get_file_contents({owner, repo, path})\n\n\
                 Multiple files:\n\
                 for (const file of files) {\n\
                     const info = github_get_file_contents({path: file});\n\
                     github_delete_file({path: file, sha: info.sha});\n\
                 }\n\n\
                 Remember:\n\
                 - Get fresh SHA before every deletion\n\
                 - Verify file before deleting\n\
                 - Use descriptive commit messages\n\
                 - Test on feature branch for risky deletions\n\
                 - Handle 409 errors by re-fetching SHA\n\
                 - Git history preserves file for recovery",
            ),
        },
    ]
}
