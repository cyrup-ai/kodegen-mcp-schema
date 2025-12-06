//! Prompt messages for git_stash_apply tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitStashApplyPromptArgs;

/// Prompt provider for git_stash_apply tool
///
/// This is the ONLY way to provide prompts for git_stash_apply - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct StashApplyPrompts;

impl PromptProvider for StashApplyPrompts {
    type PromptArgs = GitStashApplyPromptArgs;

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
                description: Some("Scenario to show (basic, specific, conflicts, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO APPLY STASHED CHANGES
// ============================================================================

/// Basic stash apply operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I apply stashed changes without removing them from the stash?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Apply stashed changes while keeping the stash entry for potential reuse. This is useful when you want to test changes on multiple branches or keep a backup.\n\n\
                 APPLYING STASH:\n\n\
                 1. Apply latest stash:\n\
                    git_stash_apply({\n\
                        \"path\": \"/project\"\n\
                    })\n\
                    // Applies stash@{0} - most recent stash\n\n\
                 2. Verify changes restored:\n\
                    git_stash_apply({\"path\": \"/project\"})\n\
                    git_status({\"path\": \"/project\"})\n\
                    // See restored changes in working directory\n\n\
                 3. Check stash still exists:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // Stash entry remains available\n\n\
                 RESPONSE FORMAT:\n\
                 {\n\
                   \"stash\": \"stash@{0}\",\n\
                   \"files_restored\": [\"src/main.rs\", \"src/lib.rs\"],\n\
                   \"success\": true\n\
                 }\n\n\
                 APPLY vs POP:\n\
                 - apply: Keeps stash entry (can reapply later)\n\
                 - pop: Removes stash entry after applying\n\
                 - Use apply when testing changes\n\
                 - Use pop when permanently restoring\n\n\
                 WHEN TO USE APPLY:\n\
                 - Testing changes on different branches\n\
                 - Keeping a backup while experimenting\n\
                 - Applying same changes to multiple locations\n\
                 - Uncertain if changes will work\n\
                 - Want option to discard and retry\n\n\
                 WORKFLOW AFTER APPLY:\n\
                 1. Apply stash to see changes\n\
                 2. Test/review the changes\n\
                 3. If good: commit the changes\n\
                 4. If bad: discard with git_reset or git_checkout\n\
                 5. Stash remains for another attempt\n\n\
                 DISCARDING APPLIED CHANGES:\n\
                 If applied changes don't work:\n\
                 git_checkout({\n\
                     \"path\": \"/project\",\n\
                     \"files\": [\".]\"  // Discard all changes\n\
                 })\n\
                 // Try applying to different branch or with modifications",
            ),
        },
    ]
}

/// Applying specific stash entries
fn prompt_specific() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I apply a specific stash instead of the most recent one?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Apply any stash by specifying its reference. This is essential when you have multiple stashes and need an older one.\n\n\
                 APPLYING SPECIFIC STASH:\n\n\
                 1. List available stashes first:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // Output:\n\
                    // stash@{0}: WIP on main: Latest work\n\
                    // stash@{1}: WIP on feature: Feature work\n\
                    // stash@{2}: WIP on bugfix: Old bugfix\n\n\
                 2. Apply specific stash by reference:\n\
                    git_stash_apply({\n\
                        \"path\": \"/project\",\n\
                        \"stash\": \"stash@{1}\"\n\
                    })\n\
                    // Applies the feature work, not latest\n\n\
                 3. Apply by index number:\n\
                    git_stash_apply({\n\
                        \"path\": \"/project\",\n\
                        \"stash\": \"1\"\n\
                    })\n\
                    // Same as stash@{1} - shorthand notation\n\n\
                 4. Apply oldest stash:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // If there are 5 stashes (0-4)\n\
                    git_stash_apply({\n\
                        \"path\": \"/project\",\n\
                        \"stash\": \"stash@{4}\"\n\
                    })\n\n\
                 STASH REFERENCES:\n\
                 - stash@{0}: Most recent (default)\n\
                 - stash@{1}: Second most recent\n\
                 - stash@{N}: N+1 stash from top\n\
                 - Can use just index: \"0\", \"1\", \"2\"\n\
                 - References don't change until you modify stash list\n\n\
                 COMMON SCENARIOS:\n\n\
                 Scenario 1: Apply old work\n\
                 git_stash_list({\"path\": \"/project\"})\n\
                 // Find the one you want\n\
                 git_stash_apply({\n\
                     \"path\": \"/project\",\n\
                     \"stash\": \"stash@{3}\"\n\
                 })\n\n\
                 Scenario 2: Apply by message pattern\n\
                 // First list to see messages\n\
                 git_stash_list({\"path\": \"/project\"})\n\
                 // stash@{1}: WIP on main: database migration\n\
                 git_stash_apply({\n\
                     \"path\": \"/project\",\n\
                     \"stash\": \"1\"\n\
                 })\n\n\
                 Scenario 3: Try multiple stashes\n\
                 // Apply first attempt\n\
                 git_stash_apply({\"path\": \"/project\", \"stash\": \"2\"})\n\
                 // If doesn't work, reset and try another\n\
                 git_reset({\"path\": \"/project\", \"mode\": \"hard\"})\n\
                 git_stash_apply({\"path\": \"/project\", \"stash\": \"3\"})\n\n\
                 STASH NUMBERING:\n\
                 - Numbers start at 0 (most recent)\n\
                 - Increase with age: 0, 1, 2, 3...\n\
                 - Creating new stash shifts all numbers up\n\
                 - Removing a stash renumbers remaining ones\n\
                 - Always list first to see current numbers\n\n\
                 RESPONSE WITH SPECIFIC STASH:\n\
                 {\n\
                   \"stash\": \"stash@{1}\",\n\
                   \"message\": \"WIP on feature: Feature work\",\n\
                   \"files_restored\": [\"src/feature.rs\"],\n\
                   \"success\": true\n\
                 }",
            ),
        },
    ]
}

/// Handling conflicts when applying stash
fn prompt_conflicts() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What do I do when applying a stash causes conflicts?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Conflicts occur when stashed changes overlap with current branch changes. Handle them systematically to safely restore your work.\n\n\
                 HANDLING APPLY CONFLICTS:\n\n\
                 1. Conflict occurs during apply:\n\
                    git_stash_apply({\"path\": \"/project\"})\n\
                    // Response indicates conflict:\n\
                    {\n\
                      \"success\": false,\n\
                      \"conflicts\": [\"src/main.rs\", \"src/lib.rs\"],\n\
                      \"message\": \"Merge conflicts in 2 files\"\n\
                    }\n\n\
                 2. Check what's conflicted:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Shows:\n\
                    // both modified:   src/main.rs\n\
                    // both modified:   src/lib.rs\n\n\
                 3. Read conflicted file:\n\
                    fs_read_file({\"path\": \"/project/src/main.rs\"})\n\
                    // Contains conflict markers:\n\
                    // <<<<<<< Updated upstream\n\
                    // Current branch code\n\
                    // =======\n\
                    // Stashed changes\n\
                    // >>>>>>> Stashed changes\n\n\
                 4. Resolve conflicts manually:\n\
                    fs_edit_block({\n\
                        \"path\": \"/project/src/main.rs\",\n\
                        \"old_string\": \"<<<<<<< Updated upstream\\nconflicting code\\n=======\\nstashed code\\n>>>>>>> Stashed changes\",\n\
                        \"new_string\": \"resolved code combining both\"\n\
                    })\n\n\
                 5. Stage resolved files:\n\
                    git_add({\n\
                        \"path\": \"/project\",\n\
                        \"files\": [\"src/main.rs\", \"src/lib.rs\"]\n\
                    })\n\
                    // Marks conflicts as resolved\n\n\
                 6. Stash still exists after conflict:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // Original stash remains available\n\
                    // Can try applying again if needed\n\n\
                 WHY CONFLICTS HAPPEN:\n\
                 - Branch changed since stash was created\n\
                 - Stashed lines overlap with new commits\n\
                 - Same functions modified in both\n\
                 - Files renamed or moved\n\
                 - Conflicting deletions or additions\n\n\
                 CONFLICT RESOLUTION STRATEGIES:\n\n\
                 Strategy 1: Accept current (discard stash)\n\
                 git_checkout({\n\
                     \"path\": \"/project\",\n\
                     \"files\": [\"src/main.rs\"]\n\
                 })\n\
                 // Discards stashed version, keeps current\n\n\
                 Strategy 2: Accept stashed (replace current)\n\
                 // Manually edit to keep only stashed version\n\
                 fs_edit_block({...})\n\
                 git_add({\"path\": \"/project\", \"files\": [\"src/main.rs\"]})\n\n\
                 Strategy 3: Merge both versions\n\
                 // Manually combine the best of both\n\
                 // Remove conflict markers\n\
                 // Keep needed code from both sides\n\
                 fs_edit_block({...})\n\
                 git_add({\"path\": \"/project\", \"files\": [\"src/main.rs\"]})\n\n\
                 Strategy 4: Abort and retry differently\n\
                 git_reset({\"path\": \"/project\", \"mode\": \"hard\"})\n\
                 // Start over, maybe apply to different branch\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"other-branch\"})\n\
                 git_stash_apply({\"path\": \"/project\"})\n\n\
                 VERIFYING RESOLUTION:\n\
                 1. Check no conflict markers remain:\n\
                    fs_search({\n\
                        \"path\": \"/project\",\n\
                        \"pattern\": \"<<<<<<|>>>>>>|=======\"\n\
                    })\n\
                    // Should find nothing\n\n\
                 2. Verify files compile/work:\n\
                    terminal({\"command\": \"cargo check\"})\n\
                    // Or npm test, python -m pytest, etc.\n\n\
                 3. Check status is clean:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Should show staged changes, no conflicts\n\n\
                 4. Commit resolved changes:\n\
                    git_commit({\n\
                        \"path\": \"/project\",\n\
                        \"message\": \"Apply stashed changes with conflict resolution\"\n\
                    })\n\n\
                 PREVENTING CONFLICTS:\n\
                 - Apply stashes soon after creating them\n\
                 - Keep stashes focused and small\n\
                 - Apply to same branch where created\n\
                 - Update branch before applying old stashes\n\
                 - Use feature branches for experimental work",
            ),
        },
    ]
}

/// Common workflows for stash apply
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are typical workflows for using git_stash_apply?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Master these common stash apply patterns for efficient Git workflows.\n\n\
                 WORKFLOW 1: TRY CHANGES ON DIFFERENT BRANCH\n\
                 Use Case: Test if stashed changes work better on another branch\n\n\
                 Step 1: Save current work\n\
                 git_stash_save({\n\
                     \"path\": \"/project\",\n\
                     \"message\": \"Experimental feature\"\n\
                 })\n\n\
                 Step 2: Switch to different branch\n\
                 git_checkout({\n\
                     \"path\": \"/project\",\n\
                     \"branch\": \"develop\"\n\
                 })\n\n\
                 Step 3: Apply stash to test\n\
                 git_stash_apply({\"path\": \"/project\"})\n\
                 // Test changes on develop\n\n\
                 Step 4: Evaluate results\n\
                 terminal({\"command\": \"cargo test\"})\n\
                 // If works: commit here\n\
                 // If fails: reset and try elsewhere\n\n\
                 Step 5: Stash preserved for more attempts\n\
                 git_stash_list({\"path\": \"/project\"})\n\
                 // Original stash still exists\n\n\
                 WORKFLOW 2: APPLY SAME CHANGES MULTIPLE PLACES\n\
                 Use Case: Need identical changes on multiple branches\n\n\
                 Initial: Save changes once\n\
                 git_stash_save({\n\
                     \"path\": \"/project\",\n\
                     \"message\": \"Bug fix for all branches\"\n\
                 })\n\n\
                 Apply to branch A:\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"release-1.0\"})\n\
                 git_stash_apply({\"path\": \"/project\"})\n\
                 git_commit({\n\
                     \"path\": \"/project\",\n\
                     \"message\": \"Fix bug in release 1.0\"\n\
                 })\n\n\
                 Apply to branch B:\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"release-2.0\"})\n\
                 git_stash_apply({\"path\": \"/project\"})\n\
                 git_commit({\n\
                     \"path\": \"/project\",\n\
                     \"message\": \"Fix bug in release 2.0\"\n\
                 })\n\n\
                 Apply to main:\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\
                 git_stash_apply({\"path\": \"/project\"})\n\
                 git_commit({\n\
                     \"path\": \"/project\",\n\
                     \"message\": \"Fix bug in main\"\n\
                 })\n\n\
                 Cleanup when done:\n\
                 git_stash_drop({\n\
                     \"path\": \"/project\",\n\
                     \"stash\": \"stash@{0}\"\n\
                 })\n\n\
                 WORKFLOW 3: SAFE RESTORE WITH FALLBACK\n\
                 Use Case: Apply changes but keep option to rollback\n\n\
                 Step 1: Apply stash\n\
                 git_stash_apply({\"path\": \"/project\"})\n\n\
                 Step 2: Test thoroughly\n\
                 terminal({\"command\": \"npm test && npm run lint\"})\n\n\
                 Step 3a: If everything works\n\
                 git_add({\"path\": \"/project\", \"files\": [\".\"]}) \n\
                 git_commit({\n\
                     \"path\": \"/project\",\n\
                     \"message\": \"Restore stashed feature\"\n\
                 })\n\
                 // Now safe to drop stash\n\
                 git_stash_drop({\"path\": \"/project\"})\n\n\
                 Step 3b: If something breaks\n\
                 git_checkout({\n\
                     \"path\": \"/project\",\n\
                     \"files\": [\".\"]\n\
                 })\n\
                 // Discard applied changes\n\
                 // Stash still exists, try again later\n\n\
                 WORKFLOW 4: INCREMENTAL TESTING\n\
                 Use Case: Test stashed changes piece by piece\n\n\
                 Apply stash:\n\
                 git_stash_apply({\"path\": \"/project\"})\n\n\
                 Test file by file:\n\
                 git_add({\"path\": \"/project\", \"files\": [\"src/module1.rs\"]})\n\
                 terminal({\"command\": \"cargo test module1\"})\n\n\
                 git_add({\"path\": \"/project\", \"files\": [\"src/module2.rs\"]})\n\
                 terminal({\"command\": \"cargo test module2\"})\n\n\
                 If a file causes issues:\n\
                 git_checkout({\n\
                     \"path\": \"/project\",\n\
                     \"files\": [\"src/module2.rs\"]\n\
                 })\n\
                 // Discard just that file\n\n\
                 Commit what works:\n\
                 git_commit({\n\
                     \"path\": \"/project\",\n\
                     \"message\": \"Restore working parts of stashed changes\"\n\
                 })\n\n\
                 WORKFLOW 5: CHERRY-PICK FROM STASH\n\
                 Use Case: Want only some files from a stash\n\n\
                 Apply full stash:\n\
                 git_stash_apply({\"path\": \"/project\"})\n\n\
                 Keep only desired files:\n\
                 git_add({\n\
                     \"path\": \"/project\",\n\
                     \"files\": [\"src/wanted.rs\"]\n\
                 })\n\n\
                 Discard unwanted files:\n\
                 git_checkout({\n\
                     \"path\": \"/project\",\n\
                     \"files\": [\"src/unwanted.rs\", \"src/other.rs\"]\n\
                 })\n\n\
                 Commit selected files:\n\
                 git_commit({\n\
                     \"path\": \"/project\",\n\
                     \"message\": \"Cherry-pick specific files from stash\"\n\
                 })\n\n\
                 BEST PRACTICES:\n\
                 - Apply before making new changes (clean slate)\n\
                 - Test immediately after applying\n\
                 - Commit or discard before switching contexts\n\
                 - Keep stash list clean (drop after successful apply)\n\
                 - Use descriptive stash messages for tracking\n\
                 - Don't rely on stashes long-term (commit instead)\n\
                 - Document why you're keeping a stash applied but uncommitted",
            ),
        },
    ]
}

/// Comprehensive guide to git_stash_apply
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using git_stash_apply.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Complete guide to applying stashed changes without removing them from the stash. Master all aspects of safe stash restoration.\n\n\
                 =============================================================================\n\
                 TOOL OVERVIEW\n\
                 =============================================================================\n\n\
                 WHAT IT DOES:\n\
                 - Restores stashed changes to working directory\n\
                 - Keeps stash entry for potential reuse\n\
                 - Allows testing changes before committing\n\
                 - Enables applying same changes to multiple branches\n\n\
                 TOOL SIGNATURE:\n\
                 git_stash_apply({\n\
                     \"path\": \"/project\",           // Required: repository path\n\
                     \"stash\": \"stash@{0}\"          // Optional: which stash (default: latest)\n\
                 })\n\n\
                 =============================================================================\n\
                 BASIC OPERATIONS\n\
                 =============================================================================\n\n\
                 APPLY LATEST STASH:\n\
                 git_stash_apply({\"path\": \"/project\"})\n\
                 // Applies stash@{0} - most recent stash\n\
                 // Stash remains in stash list\n\
                 // Changes appear in working directory\n\n\
                 APPLY SPECIFIC STASH:\n\
                 git_stash_apply({\n\
                     \"path\": \"/project\",\n\
                     \"stash\": \"stash@{2}\"\n\
                 })\n\
                 // Applies third-newest stash\n\
                 // Can also use index: \"stash\": \"2\"\n\n\
                 VERIFY APPLICATION:\n\
                 git_status({\"path\": \"/project\"})\n\
                 // Shows restored files as modified\n\
                 // No staged changes (apply doesn't stage)\n\n\
                 CHECK STASH STILL EXISTS:\n\
                 git_stash_list({\"path\": \"/project\"})\n\
                 // Original stash remains in list\n\
                 // Available for reapplication if needed\n\n\
                 =============================================================================\n\
                 RESPONSE FORMAT\n\
                 =============================================================================\n\n\
                 SUCCESS RESPONSE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"stash\": \"stash@{0}\",\n\
                   \"message\": \"WIP on main: Add feature\",\n\
                   \"files_restored\": [\n\
                     \"src/main.rs\",\n\
                     \"src/lib.rs\",\n\
                     \"tests/test.rs\"\n\
                   ],\n\
                   \"additions\": 45,\n\
                   \"deletions\": 12\n\
                 }\n\n\
                 CONFLICT RESPONSE:\n\
                 {\n\
                   \"success\": false,\n\
                   \"stash\": \"stash@{0}\",\n\
                   \"conflicts\": [\"src/main.rs\", \"src/lib.rs\"],\n\
                   \"message\": \"Merge conflicts in 2 files\"\n\
                 }\n\n\
                 =============================================================================\n\
                 APPLY vs POP vs DROP\n\
                 =============================================================================\n\n\
                 APPLY (this tool):\n\
                 - Restores changes to working directory\n\
                 - Keeps stash in stash list\n\
                 - Use when: testing, reusing, uncertain\n\
                 git_stash_apply({\"path\": \"/project\"})\n\n\
                 POP:\n\
                 - Restores changes to working directory\n\
                 - Removes stash from stash list\n\
                 - Use when: permanent restore, done with stash\n\
                 git_stash_pop({\"path\": \"/project\"})\n\n\
                 DROP:\n\
                 - Does NOT restore changes\n\
                 - Only removes stash from list\n\
                 - Use when: stash no longer needed\n\
                 git_stash_drop({\"path\": \"/project\", \"stash\": \"stash@{1}\"})\n\n\
                 DECISION TREE:\n\
                 - Need to test first? → apply\n\
                 - Multiple branches? → apply\n\
                 - Keep backup? → apply\n\
                 - Done with stash? → pop\n\
                 - Stash obsolete? → drop\n\n\
                 =============================================================================\n\
                 STASH REFERENCES\n\
                 =============================================================================\n\n\
                 FULL REFERENCE:\n\
                 \"stash@{0}\" - Most recent stash\n\
                 \"stash@{1}\" - Second most recent\n\
                 \"stash@{N}\" - N+1th stash from top\n\n\
                 SHORTHAND:\n\
                 \"0\" - Same as stash@{0}\n\
                 \"1\" - Same as stash@{1}\n\
                 \"N\" - Same as stash@{N}\n\n\
                 OMITTED:\n\
                 No \"stash\" parameter - Defaults to stash@{0}\n\n\
                 FINDING THE RIGHT STASH:\n\
                 1. List all stashes:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                 2. Read messages and timestamps\n\
                 3. Apply by reference:\n\
                    git_stash_apply({\"path\": \"/project\", \"stash\": \"stash@{3}\"})\n\n\
                 =============================================================================\n\
                 HANDLING CONFLICTS\n\
                 =============================================================================\n\n\
                 WHEN CONFLICTS OCCUR:\n\
                 - Stashed changes overlap with current code\n\
                 - Files modified in both stash and branch\n\
                 - Functions or lines changed in conflicting ways\n\n\
                 CONFLICT RESOLUTION PROCESS:\n\n\
                 1. Apply attempts and reports conflicts:\n\
                    git_stash_apply({\"path\": \"/project\"})\n\
                    // Returns: {\"success\": false, \"conflicts\": [...]}\n\n\
                 2. Check conflicted files:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Shows: both modified: filename\n\n\
                 3. Read file with conflict markers:\n\
                    fs_read_file({\"path\": \"/project/src/main.rs\"})\n\
                    // Contains: <<<<<<<, =======, >>>>>>>\n\n\
                 4. Edit to resolve:\n\
                    fs_edit_block({\n\
                        \"path\": \"/project/src/main.rs\",\n\
                        \"old_string\": \"[conflict markers and code]\",\n\
                        \"new_string\": \"[resolved code]\"\n\
                    })\n\n\
                 5. Stage resolved files:\n\
                    git_add({\"path\": \"/project\", \"files\": [\"src/main.rs\"]})\n\n\
                 6. Continue with commit or further work\n\n\
                 ABORTING CONFLICTED APPLY:\n\
                 git_reset({\"path\": \"/project\", \"mode\": \"hard\"})\n\
                 // Discards all changes, returns to clean state\n\
                 // Stash remains available for retry\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 WORKFLOW 1: Test then commit\n\
                 git_stash_apply({\"path\": \"/project\"})\n\
                 terminal({\"command\": \"cargo test\"})\n\
                 // If tests pass:\n\
                 git_add({\"path\": \"/project\", \"files\": [\".\"]}) \n\
                 git_commit({\"path\": \"/project\", \"message\": \"Restore feature\"})\n\
                 git_stash_drop({\"path\": \"/project\"})  // Cleanup\n\n\
                 WORKFLOW 2: Multi-branch apply\n\
                 git_stash_apply({\"path\": \"/project\"})\n\
                 git_commit({\"path\": \"/project\", \"message\": \"Apply to branch A\"})\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"branch-b\"})\n\
                 git_stash_apply({\"path\": \"/project\"})  // Same stash\n\
                 git_commit({\"path\": \"/project\", \"message\": \"Apply to branch B\"})\n\n\
                 WORKFLOW 3: Safe experimentation\n\
                 git_stash_apply({\"path\": \"/project\"})\n\
                 // Try changes\n\
                 // If doesn't work:\n\
                 git_checkout({\"path\": \"/project\", \"files\": [\".\"]}) \n\
                 // Try different approach or branch\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 DO:\n\
                 - List stashes before applying (know what you're getting)\n\
                 - Apply to clean working directory when possible\n\
                 - Test immediately after applying\n\
                 - Commit or discard promptly (don't leave applied)\n\
                 - Drop stash after successful permanent apply\n\
                 - Use apply for testing, pop for permanent restore\n\n\
                 DON'T:\n\
                 - Don't apply to dirty working directory (risks conflicts)\n\
                 - Don't leave stashes applied indefinitely\n\
                 - Don't use stashes for long-term storage (commit instead)\n\
                 - Don't forget which stashes you've applied where\n\
                 - Don't apply without checking what's in the stash first\n\n\
                 CLEANUP:\n\
                 After successful apply and commit:\n\
                 git_stash_drop({\"path\": \"/project\", \"stash\": \"stash@{0}\"})\n\
                 // Remove the now-unnecessary stash\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Apply latest:\n\
                 git_stash_apply({\"path\": \"/project\"})\n\n\
                 Apply specific:\n\
                 git_stash_apply({\"path\": \"/project\", \"stash\": \"stash@{2}\"})\n\n\
                 List stashes:\n\
                 git_stash_list({\"path\": \"/project\"})\n\n\
                 Check status after:\n\
                 git_status({\"path\": \"/project\"})\n\n\
                 Discard if unwanted:\n\
                 git_checkout({\"path\": \"/project\", \"files\": [\".\"]}) \n\n\
                 Drop stash when done:\n\
                 git_stash_drop({\"path\": \"/project\"})\n\n\
                 Remember: Apply keeps the stash, Pop removes it. Use apply when you want to keep your options open!",
            ),
        },
    ]
}
