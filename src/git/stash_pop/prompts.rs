//! Prompt messages for git_stash_pop tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitStashPopPromptArgs;

/// Prompt provider for git_stash_pop tool
///
/// This is the ONLY way to provide prompts for git_stash_pop - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GitStashPopPrompts;

impl PromptProvider for GitStashPopPrompts {
    type PromptArgs = GitStashPopPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("specific") => prompt_specific(),
            Some("conflicts") => prompt_conflicts(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show examples for: basic, specific, conflicts, workflows".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT_STASH_POP
// ============================================================================

/// Basic stash pop operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I pop stashed changes and remove them from the stash list?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_stash_pop to apply stashed changes and automatically remove the stash entry. This is a one-step operation for restoring and cleaning up.\n\n\
                 POPPING STASH:\n\n\
                 1. Pop latest stash:\n\
                    git_stash_pop({\n\
                        \"path\": \"/project\"\n\
                    })\n\
                    // Applies stash@{0} and removes it from the stash list\n\n\
                 2. Check result:\n\
                    Response shows:\n\
                    - stash: Which stash was popped (e.g., \"stash@{0}\")\n\
                    - files_restored: List of files modified\n\
                    - stash_removed: true (stash is gone)\n\
                    - success: true if operation succeeded\n\n\
                 EXAMPLE RESPONSE:\n\
                 {\n\
                   \"stash\": \"stash@{0}\",\n\
                   \"files_restored\": [\"src/main.rs\", \"Cargo.toml\"],\n\
                   \"stash_removed\": true,\n\
                   \"success\": true\n\
                 }\n\n\
                 TYPICAL WORKFLOW:\n\
                 1. Save work:\n\
                    git_stash_save({\"path\": \"/project\", \"message\": \"WIP: feature\"})\n\
                 2. Switch context:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\
                 3. Do something on main (fix bug, review code, etc.)\n\
                 4. Switch back:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature\"})\n\
                 5. Restore and cleanup:\n\
                    git_stash_pop({\"path\": \"/project\"})\n\
                    // Work restored, stash entry removed\n\n\
                 POP vs APPLY:\n\
                 - pop: Applies changes AND removes stash entry (one-step cleanup)\n\
                 - apply: Applies changes but KEEPS stash entry (safe testing)\n\
                 - Use pop when you're done with the stash\n\
                 - Use apply when you want to test before removing\n\n\
                 WHAT HAPPENS:\n\
                 1. Git applies the stashed changes to your working directory\n\
                 2. If successful, the stash entry is automatically removed\n\
                 3. Your working directory now has the changes back\n\
                 4. The stash list is one entry shorter\n\n\
                 WHEN TO USE POP:\n\
                 - You're done with the stashed work and want to continue it\n\
                 - You want to clean up the stash list automatically\n\
                 - You're confident the pop will succeed without conflicts\n\
                 - You don't need to apply the same stash multiple times\n\n\
                 DIRECTORY STATE:\n\
                 Before pop:\n\
                 - Working directory: Clean (no uncommitted changes)\n\
                 - Stash list: Contains stash@{0}\n\
                 After pop:\n\
                 - Working directory: Has restored changes (uncommitted)\n\
                 - Stash list: stash@{0} is removed, other stashes renumbered\n\n\
                 PARAMETERS:\n\
                 - path (required): Repository path\n\
                 - stash (optional): Specific stash to pop (default: stash@{0})\n\
                 - index (optional): Also restore staged changes (default: false)\n\n\
                 BEST PRACTICES:\n\
                 1. Ensure working directory is clean before popping\n\
                 2. Use git_status first to check for uncommitted changes\n\
                 3. If unsure, use git_stash_apply first to test\n\
                 4. Check response for conflicts before continuing\n\
                 5. Use git_stash_list to see available stashes",
            ),
        },
    ]
}

/// Popping specific stash entries
fn prompt_specific() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I pop a specific stash instead of the latest one?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "You can pop any stash by specifying its reference. First list stashes to find the one you want, then pop it specifically.\n\n\
                 POPPING SPECIFIC STASH:\n\n\
                 1. List available stashes:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // Response shows:\n\
                    // stash@{0}: WIP on main: abc123 Recent commit\n\
                    // stash@{1}: Feature work: def456 Another commit\n\
                    // stash@{2}: Experiment: ghi789 Old commit\n\n\
                 2. Pop specific stash by reference:\n\
                    git_stash_pop({\n\
                        \"path\": \"/project\",\n\
                        \"stash\": \"stash@{1}\"\n\
                    })\n\
                    // Pops stash@{1} (Feature work) and removes it\n\n\
                 3. Pop by index (alternative syntax):\n\
                    git_stash_pop({\n\
                        \"path\": \"/project\",\n\
                        \"stash\": \"1\"\n\
                    })\n\
                    // Same as stash@{1}\n\n\
                 STASH NUMBERING:\n\
                 - stash@{0}: Most recent stash (default)\n\
                 - stash@{1}: Second most recent\n\
                 - stash@{2}: Third most recent\n\
                 - And so on...\n\n\
                 IMPORTANT: INDEX SHIFTING AFTER POP:\n\
                 When you pop stash@{1}, the indices shift:\n\
                 - Old stash@{0} stays at stash@{0}\n\
                 - Old stash@{1} is removed (popped)\n\
                 - Old stash@{2} becomes stash@{1}\n\
                 - Old stash@{3} becomes stash@{2}\n\
                 - Etc.\n\n\
                 EXAMPLE WORKFLOW:\n\
                 Initial state:\n\
                 git_stash_list({\"path\": \"/project\"})\n\
                 // stash@{0}: WIP: Current work\n\
                 // stash@{1}: Feature: Login page\n\
                 // stash@{2}: Bugfix: CSS issue\n\n\
                 Pop the middle one:\n\
                 git_stash_pop({\"path\": \"/project\", \"stash\": \"stash@{1}\"})\n\
                 // Applies \"Feature: Login page\" and removes it\n\n\
                 New state:\n\
                 git_stash_list({\"path\": \"/project\"})\n\
                 // stash@{0}: WIP: Current work (unchanged)\n\
                 // stash@{1}: Bugfix: CSS issue (was stash@{2})\n\n\
                 WHY POP SPECIFIC STASH:\n\
                 - You have multiple unrelated stashes\n\
                 - You want to restore an older piece of work\n\
                 - You need to apply stashes out of order\n\
                 - You want to cherry-pick specific saved work\n\n\
                 IDENTIFYING THE RIGHT STASH:\n\
                 1. List all stashes:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                 2. Look at the messages to identify which one you need\n\
                 3. Note the stash@{N} reference\n\
                 4. Pop that specific one\n\n\
                 COMMON PATTERN:\n\
                 // Find the stash you want\n\
                 git_stash_list({\"path\": \"/project\"})\n\
                 // Review the list and pick the right one\n\
                 // Pop it specifically\n\
                 git_stash_pop({\"path\": \"/project\", \"stash\": \"stash@{2}\"})\n\n\
                 STASH REFERENCES:\n\
                 You can specify stash in three ways:\n\
                 1. Full reference: \"stash@{1}\" (most explicit)\n\
                 2. Just the index: \"1\" (shorthand)\n\
                 3. Omit for latest: defaults to stash@{0}\n\n\
                 RESPONSE FIELDS:\n\
                 {\n\
                   \"stash\": \"stash@{1}\",              // Which stash was popped\n\
                   \"files_restored\": [\"src/login.rs\"],  // Files affected\n\
                   \"stash_removed\": true,               // Stash entry deleted\n\
                   \"success\": true                      // Operation succeeded\n\
                 }\n\n\
                 BEST PRACTICES:\n\
                 1. Always list stashes first to see what's available\n\
                 2. Use descriptive messages when creating stashes\n\
                 3. Pop stashes in logical order when possible\n\
                 4. Check working directory is clean before popping\n\
                 5. Verify the correct stash was popped by checking files_restored\n\n\
                 CAUTION:\n\
                 - Once popped, the stash is removed (cannot undo)\n\
                 - If unsure, use git_stash_show first to preview changes\n\
                 - Or use git_stash_apply to test without removing",
            ),
        },
    ]
}

/// Handling conflicts when popping
fn prompt_conflicts() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What happens if popping a stash causes merge conflicts?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "When popping causes conflicts, Git applies partial changes but KEEPS the stash entry. You must resolve conflicts manually and clean up the stash yourself.\n\n\
                 HANDLING POP CONFLICTS:\n\n\
                 1. Attempt to pop:\n\
                    git_stash_pop({\"path\": \"/project\"})\n\
                    // Response indicates conflict:\n\
                    {\n\
                      \"stash\": \"stash@{0}\",\n\
                      \"files_restored\": [\"src/main.rs\"],\n\
                      \"conflicts\": [\"src/main.rs\"],\n\
                      \"stash_removed\": false,  // IMPORTANT: Stash NOT removed!\n\
                      \"success\": false\n\
                    }\n\n\
                 2. Check working directory status:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Shows conflict markers in affected files\n\
                    // Files are marked as \"both modified\" or similar\n\n\
                 3. Resolve conflicts manually:\n\
                    fs_read_file({\"path\": \"/project/src/main.rs\"})\n\
                    // Look for conflict markers:\n\
                    // <<<<<<< Updated upstream\n\
                    // Current code\n\
                    // =======\n\
                    // Stashed code\n\
                    // >>>>>>> Stashed changes\n\n\
                 4. Edit to resolve:\n\
                    fs_edit_block({\n\
                        \"path\": \"/project/src/main.rs\",\n\
                        \"old_string\": \"<<<<<<< Updated upstream\\nCurrent code\\n=======\\nStashed code\\n>>>>>>> Stashed changes\",\n\
                        \"new_string\": \"Merged code\"\n\
                    })\n\n\
                 5. Stage resolved files:\n\
                    git_add({\"path\": \"/project\", \"all\": true})\n\
                    // Or add specific files\n\n\
                 6. Manually drop the stash:\n\
                    git_stash_drop({\"path\": \"/project\"})\n\
                    // Now the stash is removed\n\n\
                 CRITICAL: STASH PRESERVED ON CONFLICT:\n\
                 - When pop conflicts, stash entry is NOT removed\n\
                 - This is a safety feature to prevent data loss\n\
                 - You must manually drop the stash after resolving\n\
                 - If you don't drop it, the stash remains in the list\n\n\
                 WHY CONFLICTS HAPPEN:\n\
                 - Same file modified in both stash and current branch\n\
                 - Different changes to the same lines of code\n\
                 - Files renamed or moved since stashing\n\
                 - Binary files changed in incompatible ways\n\n\
                 CONFLICT RESOLUTION WORKFLOW:\n\
                 1. Pop stash (fails with conflicts)\n\
                 2. Check status to see conflicted files\n\
                 3. Read each conflicted file\n\
                 4. Manually merge the changes\n\
                 5. Stage the resolved files\n\
                 6. Drop the stash manually\n\
                 7. Continue working\n\n\
                 AVOIDING CONFLICTS:\n\
                 1. Keep stashes short-lived (pop soon after saving)\n\
                 2. Don't modify the same files before popping\n\
                 3. Pop stashes on the same branch they were created\n\
                 4. Use git_stash_apply first to test for conflicts\n\n\
                 SAFE APPROACH WITH APPLY:\n\
                 1. Test with apply first:\n\
                    git_stash_apply({\"path\": \"/project\"})\n\
                    // Keeps stash, applies changes\n\n\
                 2. If no conflicts:\n\
                    git_stash_drop({\"path\": \"/project\"})\n\
                    // Manually remove stash\n\n\
                 3. If conflicts occur:\n\
                    git_reset({\"path\": \"/project\", \"mode\": \"hard\"})\n\
                    // Abort and try different approach\n\n\
                 CONFLICT MARKERS EXPLAINED:\n\
                 <<<<<<< Updated upstream\n\
                 This is the current code in your working directory\n\
                 =======\n\
                 This is the stashed code you're trying to apply\n\
                 >>>>>>> Stashed changes\n\
                 \n\
                 You must:\n\
                 - Choose one version, or\n\
                 - Merge both versions, or\n\
                 - Write something completely different\n\
                 - Remove the conflict markers\n\n\
                 RESPONSE FIELDS ON CONFLICT:\n\
                 {\n\
                   \"stash\": \"stash@{0}\",\n\
                   \"files_restored\": [\"src/lib.rs\"],      // Partially applied\n\
                   \"conflicts\": [\"src/main.rs\"],          // Files with conflicts\n\
                   \"stash_removed\": false,                 // STASH STILL EXISTS\n\
                   \"success\": false,                       // Operation failed\n\
                   \"error_message\": \"Merge conflicts...\"  // Explanation\n\
                 }\n\n\
                 POST-CONFLICT CLEANUP:\n\
                 After resolving all conflicts:\n\
                 1. Verify all files are correct:\n\
                    git_status({\"path\": \"/project\"})\n\
                 2. Stage all resolved files:\n\
                    git_add({\"path\": \"/project\", \"all\": true})\n\
                 3. Drop the stash manually:\n\
                    git_stash_drop({\"path\": \"/project\"})\n\
                 4. Optionally commit:\n\
                    git_commit({\"path\": \"/project\", \"message\": \"Resolved stash conflicts\"})\n\n\
                 IMPORTANT NOTES:\n\
                 - Stash NOT removed on conflict (safety feature)\n\
                 - Manual cleanup required after resolution\n\
                 - Use apply instead if unsure about conflicts\n\
                 - Check working directory is clean before popping\n\
                 - Save important work before attempting risky pops\n\n\
                 ALTERNATIVE: APPLY INSTEAD OF POP:\n\
                 If you're concerned about conflicts:\n\
                 - Use git_stash_apply (keeps stash on failure)\n\
                 - Test if it applies cleanly\n\
                 - Manually drop after confirming success\n\
                 - This gives you more control and safety",
            ),
        },
    ]
}

/// Common stash pop workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows where I should use git_stash_pop?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Git stash pop is essential for temporarily saving work during context switches. Here are the most common workflows.\n\n\
                 POP WORKFLOWS:\n\n\
                 1. QUICK CONTEXT SWITCH:\n\
                    // Working on feature, urgent bug appears\n\
                    git_stash_save({\"path\": \"/project\", \"message\": \"WIP: feature\"})\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"hotfix\"})\n\
                    // Fix bug, commit, push\n\
                    git_commit({\"path\": \"/project\", \"message\": \"Fix critical bug\"})\n\
                    git_push({\"path\": \"/project\"})\n\
                    // Switch back to feature\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature\"})\n\
                    git_stash_pop({\"path\": \"/project\"})\n\
                    // Continue working on feature\n\n\
                 2. PULL WITH LOCAL CHANGES:\n\
                    // You have uncommitted changes, need to pull\n\
                    git_stash_save({\"path\": \"/project\"})\n\
                    git_pull({\"path\": \"/project\"})\n\
                    git_stash_pop({\"path\": \"/project\"})\n\
                    // Resolve any conflicts if they occur\n\n\
                 3. REBASE WITH CHANGES:\n\
                    // Clean working directory required for rebase\n\
                    git_stash_save({\"path\": \"/project\"})\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"upstream\": \"origin/main\"\n\
                    })\n\
                    git_stash_pop({\"path\": \"/project\"})\n\
                    // Your changes are now on top of latest main\n\n\
                 4. BRANCH SWITCHING WITH WIP:\n\
                    // Working on feature-a, need to check feature-b\n\
                    git_stash_save({\"path\": \"/project\", \"message\": \"WIP: feature-a\"})\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature-b\"})\n\
                    // Review or test feature-b\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature-a\"})\n\
                    git_stash_pop({\"path\": \"/project\"})\n\n\
                 5. CLEAN WORKSPACE FOR TESTING:\n\
                    // Save experimental changes, test clean build\n\
                    git_stash_save({\"path\": \"/project\", \"message\": \"Experiment\"})\n\
                    // Run tests on clean codebase\n\
                    terminal({\"command\": \"cargo test\"})\n\
                    // Restore experiment\n\
                    git_stash_pop({\"path\": \"/project\"})\n\n\
                 6. MERGE PREPARATION:\n\
                    // Clear working directory for merge\n\
                    git_stash_save({\"path\": \"/project\"})\n\
                    git_merge({\"path\": \"/project\", \"branch\": \"develop\"})\n\
                    git_stash_pop({\"path\": \"/project\"})\n\n\
                 COMMON PATTERN:\n\
                 The standard stash-operation-pop pattern:\n\
                 1. Save current work with git_stash_save\n\
                 2. Do git operation requiring clean directory\n\
                 3. Restore work with git_stash_pop\n\n\
                 WHEN TO USE POP vs APPLY:\n\
                 Use POP when:\n\
                 - You're done with the stashed work\n\
                 - You want automatic cleanup\n\
                 - You won't need the stash again\n\
                 - You're confident it will apply cleanly\n\
                 \n\
                 Use APPLY when:\n\
                 - You want to test the changes first\n\
                 - You might need the stash on multiple branches\n\
                 - You're unsure about conflicts\n\
                 - You want to keep the stash as a backup\n\n\
                 MULTI-STASH WORKFLOW:\n\
                 // Save multiple pieces of work\n\
                 git_stash_save({\"path\": \"/project\", \"message\": \"UI changes\"})\n\
                 // Do some work\n\
                 git_stash_save({\"path\": \"/project\", \"message\": \"API changes\"})\n\
                 // List to see both\n\
                 git_stash_list({\"path\": \"/project\"})\n\
                 // stash@{0}: API changes\n\
                 // stash@{1}: UI changes\n\
                 // Pop specific one\n\
                 git_stash_pop({\"path\": \"/project\", \"stash\": \"stash@{1}\"})\n\n\
                 SAFE POP WORKFLOW:\n\
                 1. Check status first:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Ensure working directory is clean\n\n\
                 2. List available stashes:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // Verify the stash you want exists\n\n\
                 3. Preview stash contents:\n\
                    git_stash_show({\"path\": \"/project\", \"stash\": \"stash@{0}\"})\n\
                    // See what changes will be applied\n\n\
                 4. Pop the stash:\n\
                    git_stash_pop({\"path\": \"/project\"})\n\n\
                 5. Verify result:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Check that expected files are modified\n\n\
                 EMERGENCY RECOVERY:\n\
                 If pop goes wrong:\n\
                 1. Don't panic - stash is preserved on conflict\n\
                 2. Check what happened:\n\
                    git_status({\"path\": \"/project\"})\n\
                 3. Reset if needed:\n\
                    git_reset({\"path\": \"/project\", \"mode\": \"hard\"})\n\
                 4. Try different approach:\n\
                    git_stash_apply({\"path\": \"/project\"})\n\n\
                 TEAM WORKFLOWS:\n\
                 Working with others:\n\
                 1. Stash before pulling team changes:\n\
                    git_stash_save({\"path\": \"/project\"})\n\
                    git_pull({\"path\": \"/project\"})\n\
                    git_stash_pop({\"path\": \"/project\"})\n\n\
                 2. Stash before reviewing PR:\n\
                    git_stash_save({\"path\": \"/project\"})\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"pr-branch\"})\n\
                    // Review code\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"my-branch\"})\n\
                    git_stash_pop({\"path\": \"/project\"})\n\n\
                 AUTOMATION PATTERN:\n\
                 For scripts and automation:\n\
                 1. Save work: git_stash_save\n\
                 2. Do automated task\n\
                 3. Restore: git_stash_pop\n\
                 4. Check success field in response\n\
                 5. Handle conflicts if success is false\n\n\
                 BEST PRACTICES:\n\
                 1. Use descriptive stash messages\n\
                 2. Pop stashes soon after creating (don't let them pile up)\n\
                 3. Verify working directory is clean before popping\n\
                 4. Check response success field\n\
                 5. Have a plan for handling conflicts\n\
                 6. Keep stashes short-lived and focused\n\
                 7. Use git_stash_list to track multiple stashes\n\n\
                 DEBUGGING STASH ISSUES:\n\
                 If pop isn't working as expected:\n\
                 1. List stashes: git_stash_list\n\
                 2. Show stash contents: git_stash_show\n\
                 3. Check working directory: git_status\n\
                 4. Try apply first: git_stash_apply\n\
                 5. Review for conflicts carefully",
            ),
        },
    ]
}

/// Comprehensive guide to git_stash_pop
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using git_stash_pop effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_stash_pop tool applies stashed changes to your working directory and automatically removes the stash entry. This is a one-step restore-and-cleanup operation.\n\n\
                 =============================================================================\n\
                 WHAT IS GIT_STASH_POP?\n\
                 =============================================================================\n\n\
                 Git stash pop combines two operations:\n\
                 1. Apply stashed changes to working directory (like git_stash_apply)\n\
                 2. Remove the stash from the stash list (like git_stash_drop)\n\n\
                 This is the most common way to restore stashed work because it:\n\
                 - Restores your changes\n\
                 - Cleans up the stash list automatically\n\
                 - Saves you from manual stash management\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 1. Pop latest stash (most common):\n\
                    git_stash_pop({\"path\": \"/project\"})\n\n\
                 2. Pop specific stash:\n\
                    git_stash_pop({\n\
                        \"path\": \"/project\",\n\
                        \"stash\": \"stash@{1}\"\n\
                    })\n\n\
                 3. Pop with index restoration:\n\
                    git_stash_pop({\n\
                        \"path\": \"/project\",\n\
                        \"index\": true\n\
                    })\n\
                    // Also restores staged changes\n\n\
                 PARAMETERS:\n\
                 - path (required): Repository path\n\
                 - stash (optional): Which stash to pop (default: \"stash@{0}\")\n\
                 - index (optional): Restore staged state (default: false)\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"stash\": \"stash@{0}\",\n\
                   \"files_restored\": [\"src/main.rs\", \"Cargo.toml\"],\n\
                   \"stash_removed\": true,\n\
                   \"success\": true,\n\
                   \"conflicts\": []  // Empty if no conflicts\n\
                 }\n\n\
                 =============================================================================\n\
                 POP vs APPLY vs DROP\n\
                 =============================================================================\n\n\
                 git_stash_pop = git_stash_apply + git_stash_drop\n\n\
                 POP:\n\
                 - Applies changes AND removes stash\n\
                 - One-step operation\n\
                 - Use when done with stash\n\
                 - Automatic cleanup\n\n\
                 APPLY:\n\
                 - Applies changes but KEEPS stash\n\
                 - Use for testing\n\
                 - Use when applying to multiple branches\n\
                 - Manual cleanup required\n\n\
                 DROP:\n\
                 - Removes stash without applying\n\
                 - Use to delete unwanted stash\n\
                 - Permanent deletion\n\n\
                 DECISION TREE:\n\
                 - Want changes back and done with stash? → POP\n\
                 - Want to test changes first? → APPLY then DROP\n\
                 - Want to apply to multiple branches? → APPLY multiple times, then DROP\n\
                 - Want to delete without applying? → DROP\n\n\
                 =============================================================================\n\
                 STASH REFERENCES\n\
                 =============================================================================\n\n\
                 Three ways to specify which stash:\n\n\
                 1. Full reference:\n\
                    git_stash_pop({\"path\": \"/project\", \"stash\": \"stash@{1}\"})\n\n\
                 2. Index only:\n\
                    git_stash_pop({\"path\": \"/project\", \"stash\": \"1\"})\n\n\
                 3. Omit for latest:\n\
                    git_stash_pop({\"path\": \"/project\"})\n\
                    // Defaults to stash@{0}\n\n\
                 STASH NUMBERING:\n\
                 - stash@{0}: Most recent (default)\n\
                 - stash@{1}: Second most recent\n\
                 - stash@{2}: Third most recent\n\
                 - Etc.\n\n\
                 INDEX SHIFTING:\n\
                 After popping stash@{1}:\n\
                 - stash@{0} stays at 0\n\
                 - stash@{1} is removed\n\
                 - Old stash@{2} becomes stash@{1}\n\
                 - Old stash@{3} becomes stash@{2}\n\
                 - All higher stashes shift down\n\n\
                 =============================================================================\n\
                 HANDLING CONFLICTS\n\
                 =============================================================================\n\n\
                 When pop causes conflicts:\n\
                 1. Changes are partially applied\n\
                 2. Stash is NOT removed (safety feature)\n\
                 3. Response shows success: false\n\
                 4. Conflict markers added to files\n\n\
                 CONFLICT RESPONSE:\n\
                 {\n\
                   \"stash\": \"stash@{0}\",\n\
                   \"files_restored\": [\"src/lib.rs\"],\n\
                   \"conflicts\": [\"src/main.rs\"],\n\
                   \"stash_removed\": false,  // IMPORTANT!\n\
                   \"success\": false\n\
                 }\n\n\
                 RESOLUTION WORKFLOW:\n\
                 1. Check status:\n\
                    git_status({\"path\": \"/project\"})\n\n\
                 2. Read conflicted files:\n\
                    fs_read_file({\"path\": \"/project/src/main.rs\"})\n\n\
                 3. Look for conflict markers:\n\
                    <<<<<<< Updated upstream\n\
                    Current code\n\
                    =======\n\
                    Stashed code\n\
                    >>>>>>> Stashed changes\n\n\
                 4. Edit to resolve:\n\
                    fs_edit_block({...})\n\n\
                 5. Stage resolved files:\n\
                    git_add({\"path\": \"/project\", \"all\": true})\n\n\
                 6. Manually drop stash:\n\
                    git_stash_drop({\"path\": \"/project\"})\n\n\
                 WHY STASH IS PRESERVED:\n\
                 - Prevents data loss on conflict\n\
                 - Gives you chance to abort (git reset --hard)\n\
                 - Allows multiple resolution attempts\n\
                 - You control when stash is removed\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 1. CONTEXT SWITCHING:\n\
                    git_stash_save({\"path\": \"/project\"})\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"other-branch\"})\n\
                    // Do work\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"original-branch\"})\n\
                    git_stash_pop({\"path\": \"/project\"})\n\n\
                 2. PULL WITH LOCAL CHANGES:\n\
                    git_stash_save({\"path\": \"/project\"})\n\
                    git_pull({\"path\": \"/project\"})\n\
                    git_stash_pop({\"path\": \"/project\"})\n\n\
                 3. REBASE WORKFLOW:\n\
                    git_stash_save({\"path\": \"/project\"})\n\
                    git_rebase({\"path\": \"/project\", \"upstream\": \"origin/main\"})\n\
                    git_stash_pop({\"path\": \"/project\"})\n\n\
                 4. TESTING CLEAN BUILD:\n\
                    git_stash_save({\"path\": \"/project\"})\n\
                    terminal({\"command\": \"cargo test\"})\n\
                    git_stash_pop({\"path\": \"/project\"})\n\n\
                 5. MULTIPLE STASHES:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    git_stash_pop({\"path\": \"/project\", \"stash\": \"stash@{2}\"})\n\n\
                 =============================================================================\n\
                 SAFE POPPING PRACTICES\n\
                 =============================================================================\n\n\
                 1. VERIFY CLEAN WORKING DIRECTORY:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Should show \"nothing to commit, working tree clean\"\n\n\
                 2. LIST AVAILABLE STASHES:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // Verify stash exists\n\n\
                 3. PREVIEW STASH CONTENTS:\n\
                    git_stash_show({\"path\": \"/project\"})\n\
                    // See what will be applied\n\n\
                 4. POP THE STASH:\n\
                    git_stash_pop({\"path\": \"/project\"})\n\n\
                 5. CHECK RESULT:\n\
                    // Check response success field\n\
                    // Verify files_restored contains expected files\n\
                    // Check for conflicts array\n\n\
                 6. VERIFY WORKING DIRECTORY:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Confirm expected changes are present\n\n\
                 =============================================================================\n\
                 AVOIDING CONFLICTS\n\
                 =============================================================================\n\n\
                 1. Pop soon after stashing (don't let stashes age)\n\
                 2. Pop on the same branch where stash was created\n\
                 3. Avoid modifying same files before popping\n\
                 4. Keep stashes focused and small\n\
                 5. Use descriptive messages to track stashes\n\
                 6. Test with apply first if unsure\n\n\
                 =============================================================================\n\
                 ALTERNATIVE: APPLY-THEN-DROP\n\
                 =============================================================================\n\n\
                 If you want more control:\n\n\
                 1. Apply without removing:\n\
                    git_stash_apply({\"path\": \"/project\"})\n\n\
                 2. Test the changes\n\n\
                 3. If satisfied, drop stash:\n\
                    git_stash_drop({\"path\": \"/project\"})\n\n\
                 4. If not satisfied, reset:\n\
                    git_reset({\"path\": \"/project\", \"mode\": \"hard\"})\n\n\
                 This gives you a chance to abort without affecting the stash.\n\n\
                 =============================================================================\n\
                 STASH INDEX PARAMETER\n\
                 =============================================================================\n\n\
                 The index parameter controls whether staged changes are restored:\n\n\
                 Without index (default):\n\
                 git_stash_pop({\"path\": \"/project\"})\n\
                 - All changes restored as unstaged\n\
                 - Simple and common\n\n\
                 With index:\n\
                 git_stash_pop({\"path\": \"/project\", \"index\": true})\n\
                 - Staged changes restored as staged\n\
                 - Unstaged changes restored as unstaged\n\
                 - Preserves exact state before stash\n\n\
                 Use index: true when:\n\
                 - You carefully staged specific changes\n\
                 - You want exact state restoration\n\
                 - You're about to commit after popping\n\n\
                 =============================================================================\n\
                 ERROR HANDLING\n\
                 =============================================================================\n\n\
                 Always check the response:\n\n\
                 if response[\"success\"] == true:\n\
                     // Pop succeeded\n\
                     // Stash removed (stash_removed == true)\n\
                     // Files restored (check files_restored)\n\
                     // Continue working\n\
                 else:\n\
                     // Pop failed (likely conflicts)\n\
                     // Stash preserved (stash_removed == false)\n\
                     // Check conflicts array\n\
                     // Resolve conflicts manually\n\
                     // Drop stash after resolving\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Pop latest:\n\
                 git_stash_pop({\"path\": \"/project\"})\n\n\
                 Pop specific:\n\
                 git_stash_pop({\"path\": \"/project\", \"stash\": \"stash@{1}\"})\n\n\
                 Pop with index:\n\
                 git_stash_pop({\"path\": \"/project\", \"index\": true})\n\n\
                 List before popping:\n\
                 git_stash_list({\"path\": \"/project\"})\n\n\
                 Check status:\n\
                 git_status({\"path\": \"/project\"})\n\n\
                 Handle conflicts:\n\
                 1. git_status\n\
                 2. fs_edit_block (resolve)\n\
                 3. git_add\n\
                 4. git_stash_drop\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. Verify working directory is clean before popping\n\
                 2. Use descriptive messages when creating stashes\n\
                 3. Pop stashes soon after creating them\n\
                 4. Check response success field\n\
                 5. Handle conflicts properly (don't ignore them)\n\
                 6. Use apply first if unsure about conflicts\n\
                 7. Keep stashes focused and small\n\
                 8. Don't let stashes pile up in the list\n\
                 9. Pop on the same branch where stash was created\n\
                 10. Manually drop stash after resolving conflicts\n\n\
                 =============================================================================\n\
                 REMEMBER\n\
                 =============================================================================\n\n\
                 - Pop = Apply + Drop (one-step operation)\n\
                 - Stash is preserved on conflict (safety feature)\n\
                 - Always check success field in response\n\
                 - Use git_stash_list to see available stashes\n\
                 - Conflicts require manual resolution\n\
                 - Working directory should be clean before popping\n\
                 - Pop is the most common way to restore stashed work",
            ),
        },
    ]
}
