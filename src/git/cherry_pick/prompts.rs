//! Prompt messages for git_cherry_pick tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitCherryPickPromptArgs;

/// Prompt provider for git_cherry_pick tool
///
/// This is the ONLY way to provide prompts for git_cherry_pick - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct CherryPickPrompts;

impl PromptProvider for CherryPickPrompts {
    type PromptArgs = GitCherryPickPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("single") => prompt_single(),
            Some("multiple") => prompt_multiple(),
            Some("conflicts") => prompt_conflicts(),
            Some("options") => prompt_options(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (single, multiple, conflicts, options)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO CHERRY-PICK COMMITS
// ============================================================================

/// Cherry-picking single commits
fn prompt_single() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I cherry-pick a single commit to apply it to my current branch?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Cherry-pick applies specific commits from one branch to another. This lets you selectively bring changes without merging entire branches.\n\n\
                 CHERRY-PICK SINGLE COMMIT:\n\n\
                 1. Apply one commit:\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"abc1234\"\n\
                    })\n\n\
                 2. Find and apply:\n\
                    // First, find the commit you need\n\
                    git_log({\n\
                        \"path\": \"/project\",\n\
                        \"branch\": \"feature/x\",\n\
                        \"search\": \"fix bug\"\n\
                    })\n\
                    // Then apply it\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"def5678\"\n\
                    })\n\n\
                 3. Backport fix to release:\n\
                    // Switch to release branch\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"release/v1.0\"})\n\
                    // Apply fix from main\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"abc1234\"\n\
                    })\n\n\
                 4. Use full commit hash:\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"abc1234567890abcdef1234567890abcdef12345\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"commit\": \"abc1234\",\n\
                   \"new_commit\": \"xyz7890\",\n\
                   \"success\": true\n\
                 }\n\n\
                 WHAT HAPPENS:\n\
                 - Creates NEW commit with same changes\n\
                 - Different commit hash (xyz7890 vs abc1234)\n\
                 - Applies to current branch HEAD\n\
                 - Preserves original commit message\n\
                 - Records original author by default\n\n\
                 WHEN TO USE CHERRY-PICK:\n\
                 - Backport bug fixes to release branches\n\
                 - Apply specific features without full merge\n\
                 - Copy commits between unrelated branches\n\
                 - Extract commits from abandoned branches\n\
                 - Apply hotfixes to multiple versions\n\n\
                 FINDING COMMITS TO CHERRY-PICK:\n\n\
                 By commit hash:\n\
                 git_log({\"path\": \"/project\", \"branch\": \"feature/x\"})\n\
                 // Find commit: abc1234 \"Fix critical bug\"\n\
                 git_cherry_pick({\"commit\": \"abc1234\"})\n\n\
                 By search:\n\
                 git_log({\"path\": \"/project\", \"search\": \"security fix\"})\n\
                 // Apply the security fix\n\
                 git_cherry_pick({\"commit\": \"def5678\"})\n\n\
                 COMMIT HASH FORMATS:\n\
                 - Short hash: \"abc1234\" (7 characters minimum)\n\
                 - Full hash: \"abc1234567890abcdef1234567890abcdef12345\"\n\
                 - Both work, short is more convenient\n\n\
                 TYPICAL WORKFLOW:\n\
                 1. Identify commit to cherry-pick:\n\
                    git_log({\"path\": \"/project\", \"branch\": \"main\"})\n\n\
                 2. Switch to target branch:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"release/v2.0\"})\n\n\
                 3. Cherry-pick the commit:\n\
                    git_cherry_pick({\"path\": \"/project\", \"commit\": \"abc1234\"})\n\n\
                 4. Verify the result:\n\
                    git_status({\"path\": \"/project\"})\n\
                    git_log({\"path\": \"/project\", \"limit\": 1})\n\n\
                 BEST PRACTICES:\n\
                 - Always verify you're on correct target branch first\n\
                 - Use git_log to find exact commit hash\n\
                 - Check git_status after cherry-pick\n\
                 - Test after cherry-picking to ensure it works\n\
                 - Cherry-pick commits in chronological order\n\
                 - Avoid cherry-picking merge commits (use mainline option)\n\n\
                 UNDERSTANDING NEW COMMIT:\n\
                 Original commit on main:\n\
                 - Hash: abc1234\n\
                 - Message: \"Fix bug\"\n\
                 - Author: Alice\n\n\
                 After cherry-pick to release:\n\
                 - Hash: xyz7890 (NEW, different)\n\
                 - Message: \"Fix bug\" (same)\n\
                 - Author: Alice (preserved)\n\
                 - Committer: You (marked as cherry-picked)\n\n\
                 The new commit is independent - changes to original won't affect it.",
            ),
        },
    ]
}

/// Cherry-picking multiple commits
fn prompt_multiple() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I cherry-pick multiple commits at once?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Cherry-pick multiple commits to bring several changes from another branch. Apply them individually or as a range.\n\n\
                 CHERRY-PICK MULTIPLE COMMITS:\n\n\
                 1. Several specific commits:\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"commits\": [\"abc1234\", \"def5678\", \"ghi9012\"]\n\
                    })\n\n\
                 2. Range of commits:\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"range\": \"abc1234..def5678\"\n\
                    })\n\
                    // Applies all commits between abc1234 and def5678\n\n\
                 3. Inclusive range (include start):\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"range\": \"abc1234^..def5678\"\n\
                    })\n\
                    // Includes abc1234 itself\n\n\
                 4. From another branch:\n\
                    // First, find commits on feature branch\n\
                    git_log({\n\
                        \"path\": \"/project\",\n\
                        \"branch\": \"feature/auth\",\n\
                        \"limit\": 5\n\
                    })\n\
                    // Cherry-pick the ones you want\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"commits\": [\"abc1234\", \"def5678\", \"ghi9012\"]\n\
                    })\n\n\
                 ORDER MATTERS:\n\
                 - Commits applied in order given\n\
                 - Oldest first is recommended\n\
                 - Each creates a new commit\n\
                 - If one fails, process stops\n\n\
                 RANGE SYNTAX:\n\
                 abc1234..def5678\n\
                 - Excludes abc1234 (start)\n\
                 - Includes def5678 (end)\n\
                 - All commits in between\n\n\
                 abc1234^..def5678\n\
                 - Includes abc1234 (start)\n\
                 - Includes def5678 (end)\n\
                 - All commits in between\n\n\
                 PRACTICAL EXAMPLES:\n\n\
                 Backport feature to release:\n\
                 // Find feature commits\n\
                 git_log({\"path\": \"/project\", \"branch\": \"main\", \"search\": \"feature X\"})\n\
                 // Switch to release\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"release/v1.0\"})\n\
                 // Apply all feature commits\n\
                 git_cherry_pick({\n\
                     \"path\": \"/project\",\n\
                     \"commits\": [\"commit1\", \"commit2\", \"commit3\"]\n\
                 })\n\n\
                 Apply fixes from main:\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"hotfix\"})\n\
                 git_cherry_pick({\n\
                     \"path\": \"/project\",\n\
                     \"commits\": [\n\
                         \"fix1_hash\",\n\
                         \"fix2_hash\",\n\
                         \"fix3_hash\"\n\
                     ]\n\
                 })\n\n\
                 Range cherry-pick:\n\
                 // Apply last 10 commits from feature branch\n\
                 git_cherry_pick({\n\
                     \"path\": \"/project\",\n\
                     \"range\": \"start_commit..end_commit\"\n\
                 })\n\n\
                 FINDING COMMITS FOR RANGE:\n\
                 1. List commits:\n\
                    git_log({\"path\": \"/project\", \"branch\": \"feature/x\"})\n\n\
                 2. Identify range:\n\
                    First commit: abc1234\n\
                    Last commit: def5678\n\n\
                 3. Apply range:\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"range\": \"abc1234^..def5678\"\n\
                    })\n\n\
                 RESPONSE FORMAT:\n\
                 {\n\
                   \"commits\": [\n\
                     {\"original\": \"abc1234\", \"new\": \"new1234\"},\n\
                     {\"original\": \"def5678\", \"new\": \"new5678\"},\n\
                     {\"original\": \"ghi9012\", \"new\": \"new9012\"}\n\
                   ],\n\
                   \"success\": true\n\
                 }\n\n\
                 HANDLING PARTIAL SUCCESS:\n\
                 If third commit fails:\n\
                 {\n\
                   \"commits\": [\n\
                     {\"original\": \"abc1234\", \"new\": \"new1234\", \"success\": true},\n\
                     {\"original\": \"def5678\", \"new\": \"new5678\", \"success\": true},\n\
                     {\"original\": \"ghi9012\", \"error\": \"conflict\", \"success\": false}\n\
                   ],\n\
                   \"success\": false\n\
                 }\n\
                 First two commits applied successfully, third failed.\n\n\
                 BEST PRACTICES:\n\
                 - Apply commits in chronological order (oldest first)\n\
                 - Verify each commit applies cleanly\n\
                 - Test after cherry-picking multiple commits\n\
                 - Use specific commits list for precise control\n\
                 - Use ranges for consecutive commits only\n\
                 - Check git_status between operations\n\n\
                 WHEN TO USE EACH METHOD:\n\n\
                 Use specific commits list:\n\
                 - Non-consecutive commits\n\
                 - Selective cherry-picking\n\
                 - Different source branches\n\
                 - Maximum control\n\n\
                 Use range:\n\
                 - Consecutive commits\n\
                 - All commits from a feature\n\
                 - Cleaner syntax for many commits\n\
                 - Linear history segment\n\n\
                 AVOIDING MISTAKES:\n\
                 - Don't cherry-pick in wrong order (breaks dependencies)\n\
                 - Don't skip dependency commits\n\
                 - Don't mix commits from different features\n\
                 - Always verify target branch first\n\
                 - Test after applying multiple commits",
            ),
        },
    ]
}

/// Handling cherry-pick conflicts
fn prompt_conflicts() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I handle conflicts when cherry-picking?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Cherry-pick conflicts occur when changes can't be applied cleanly. You must resolve conflicts manually, then continue or abort.\n\n\
                 HANDLING CONFLICTS:\n\n\
                 1. Conflict occurs:\n\
                    git_cherry_pick({\"path\": \"/project\", \"commit\": \"abc1234\"})\n\
                    // Response indicates conflict:\n\
                    {\n\
                      \"success\": false,\n\
                      \"conflicts\": [\"src/main.rs\", \"src/lib.rs\"],\n\
                      \"state\": \"cherry_picking\"\n\
                    }\n\n\
                 2. Check status:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Shows conflicted files\n\n\
                 3. Resolve conflicts:\n\
                    // Read conflicted file\n\
                    fs_read_file({\"path\": \"/project/src/main.rs\"})\n\
                    // Edit to resolve conflicts\n\
                    fs_edit_block({\n\
                        \"path\": \"/project/src/main.rs\",\n\
                        \"old_string\": \"<<<<<<< HEAD\\n...conflict...\\n=======\\n...\\n>>>>>>>\",\n\
                        \"new_string\": \"resolved code\"\n\
                    })\n\n\
                 4. Stage resolved files:\n\
                    git_add({\n\
                        \"path\": \"/project\",\n\
                        \"files\": [\"src/main.rs\", \"src/lib.rs\"]\n\
                    })\n\n\
                 5. Continue cherry-pick:\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"continue\": true\n\
                    })\n\n\
                 ABORT CHERRY-PICK:\n\n\
                 If you want to cancel:\n\
                 git_cherry_pick({\n\
                     \"path\": \"/project\",\n\
                     \"abort\": true\n\
                 })\n\
                 // Returns to state before cherry-pick\n\
                 // All changes discarded\n\
                 // Safe to start over\n\n\
                 SKIP PROBLEMATIC COMMIT:\n\n\
                 When cherry-picking multiple commits:\n\
                 git_cherry_pick({\n\
                     \"path\": \"/project\",\n\
                     \"skip\": true\n\
                 })\n\
                 // Skips current commit\n\
                 // Continues with next commit in sequence\n\
                 // Use when conflict is too complex or commit not needed\n\n\
                 CONFLICT WORKFLOW:\n\n\
                 Step 1: Cherry-pick reports conflict\n\
                 git_cherry_pick({\"commit\": \"abc1234\"})\n\
                 → {\"success\": false, \"conflicts\": [\"file.rs\"]}\n\n\
                 Step 2: See conflicted files\n\
                 git_status({\"path\": \"/project\"})\n\
                 → Shows: both modified: file.rs\n\n\
                 Step 3: Read and understand conflict\n\
                 fs_read_file({\"path\": \"/project/file.rs\"})\n\
                 → See conflict markers:\n\
                 <<<<<<< HEAD\n\
                 current branch code\n\
                 =======\n\
                 cherry-picked code\n\
                 >>>>>>>\n\n\
                 Step 4: Edit to resolve\n\
                 fs_edit_block({...remove markers, keep correct code...})\n\n\
                 Step 5: Stage resolution\n\
                 git_add({\"files\": [\"file.rs\"]})\n\n\
                 Step 6: Complete cherry-pick\n\
                 git_cherry_pick({\"continue\": true})\n\n\
                 CONFLICT MARKERS:\n\n\
                 <<<<<<< HEAD\n\
                 This is your current branch's code\n\
                 =======\n\
                 This is the cherry-picked commit's code\n\
                 >>>>>>> abc1234 (commit message)\n\n\
                 You must:\n\
                 - Remove all markers (<<<, ===, >>>)\n\
                 - Keep correct code (yours, theirs, or combination)\n\
                 - Ensure code compiles and works\n\n\
                 RESOLUTION STRATEGIES:\n\n\
                 Keep your version:\n\
                 - Remove cherry-picked code\n\
                 - Remove markers\n\
                 - Keep HEAD code\n\n\
                 Keep cherry-picked version:\n\
                 - Remove HEAD code\n\
                 - Remove markers\n\
                 - Keep cherry-picked code\n\n\
                 Combine both:\n\
                 - Merge both changes\n\
                 - Remove markers\n\
                 - Create working combination\n\n\
                 MULTIPLE CONFLICTS:\n\n\
                 If multiple files conflict:\n\
                 1. git_status to see all conflicts\n\
                 2. Resolve each file individually\n\
                 3. Stage each resolved file with git_add\n\
                 4. Once all resolved, git_cherry_pick --continue\n\n\
                 Example:\n\
                 git_status()\n\
                 → both modified: src/a.rs\n\
                 → both modified: src/b.rs\n\
                 → both modified: src/c.rs\n\n\
                 // Resolve a.rs\n\
                 fs_edit_block({\"path\": \"src/a.rs\", ...})\n\
                 git_add({\"files\": [\"src/a.rs\"]})\n\n\
                 // Resolve b.rs\n\
                 fs_edit_block({\"path\": \"src/b.rs\", ...})\n\
                 git_add({\"files\": [\"src/b.rs\"]})\n\n\
                 // Resolve c.rs\n\
                 fs_edit_block({\"path\": \"src/c.rs\", ...})\n\
                 git_add({\"files\": [\"src/c.rs\"]})\n\n\
                 // All resolved, continue\n\
                 git_cherry_pick({\"continue\": true})\n\n\
                 CHECKING RESOLUTION:\n\n\
                 Before continuing:\n\
                 1. Ensure all conflicts resolved\n\
                 2. Run tests if available\n\
                 3. Verify code compiles\n\
                 4. Check git_status shows all files staged\n\n\
                 COMMON MISTAKES:\n\
                 - Forgetting to remove conflict markers\n\
                 - Not staging resolved files\n\
                 - Resolving incorrectly (breaking functionality)\n\
                 - Continuing with unresolved conflicts\n\n\
                 BEST PRACTICES:\n\
                 - Read both versions carefully\n\
                 - Understand why conflict occurred\n\
                 - Test resolution before continuing\n\
                 - Keep commit message meaningful\n\
                 - Document complex resolutions\n\
                 - Consider aborting if too complex\n\n\
                 WHEN TO ABORT:\n\
                 - Too many conflicts\n\
                 - Can't determine correct resolution\n\
                 - Breaking changes in current branch\n\
                 - Better to merge entire branch instead\n\
                 - Need to consult with team\n\n\
                 OPERATION FLAGS:\n\
                 - continue: true → Resume after resolving conflicts\n\
                 - abort: true → Cancel and return to pre-cherry-pick state\n\
                 - skip: true → Skip current commit, continue with next",
            ),
        },
    ]
}

/// Advanced cherry-pick options
fn prompt_options() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What advanced options are available for cherry-picking?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Advanced cherry-pick options give you fine control over how commits are applied. Use these for special situations.\n\n\
                 ADVANCED OPTIONS:\n\n\
                 1. No commit (stage only):\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"abc1234\",\n\
                        \"no_commit\": true\n\
                    })\n\
                    // Changes staged but not committed\n\
                    // Allows modifying before committing\n\
                    // Useful for squashing multiple cherry-picks\n\n\
                 2. Edit commit message:\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"abc1234\",\n\
                        \"edit\": true\n\
                    })\n\
                    // Pauses to let you edit commit message\n\
                    // Useful for adding context\n\n\
                 3. Custom commit message:\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"abc1234\",\n\
                        \"message\": \"Custom message for cherry-picked commit\"\n\
                    })\n\
                    // Overrides original commit message\n\n\
                 4. Mainline for merge commits:\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"abc1234\",\n\
                        \"mainline\": 1\n\
                    })\n\
                    // Required when cherry-picking merge commits\n\
                    // Mainline 1 = first parent, 2 = second parent\n\n\
                 5. Preserve original author:\n\
                    // This is DEFAULT behavior\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"abc1234\"\n\
                    })\n\
                    // Author field preserved from original\n\
                    // Committer field set to you\n\n\
                 NO COMMIT OPTION:\n\n\
                 Use case: Squash multiple cherry-picks\n\
                 git_cherry_pick({\"commit\": \"abc1234\", \"no_commit\": true})\n\
                 git_cherry_pick({\"commit\": \"def5678\", \"no_commit\": true})\n\
                 git_cherry_pick({\"commit\": \"ghi9012\", \"no_commit\": true})\n\
                 // All changes staged, not committed\n\
                 git_commit({\"message\": \"Combined changes from three commits\"})\n\n\
                 Use case: Modify before committing\n\
                 git_cherry_pick({\"commit\": \"abc1234\", \"no_commit\": true})\n\
                 // Make additional changes\n\
                 fs_edit_block({...modify files...})\n\
                 git_add({\"files\": [...]})\n\
                 git_commit({\"message\": \"Cherry-picked with modifications\"})\n\n\
                 EDIT MESSAGE OPTION:\n\n\
                 Add context to cherry-picked commit:\n\
                 git_cherry_pick({\n\
                     \"commit\": \"abc1234\",\n\
                     \"message\": \"Fix bug (cherry-picked from main)\\n\\nOriginal commit: abc1234\"\n\
                 })\n\n\
                 Document backport:\n\
                 git_cherry_pick({\n\
                     \"commit\": \"abc1234\",\n\
                     \"message\": \"Security fix\\n\\nBackported to v1.0 from main branch\\nOriginal: abc1234\"\n\
                 })\n\n\
                 MAINLINE OPTION:\n\n\
                 Required for merge commits:\n\
                 // Merge commit has two parents:\n\
                 // Parent 1: Main branch\n\
                 // Parent 2: Feature branch\n\n\
                 To cherry-pick merge's changes:\n\
                 git_cherry_pick({\n\
                     \"commit\": \"merge_commit_hash\",\n\
                     \"mainline\": 1\n\
                 })\n\
                 // mainline: 1 = Use first parent as base\n\
                 // mainline: 2 = Use second parent as base\n\n\
                 Without mainline, merge commits fail:\n\
                 git_cherry_pick({\"commit\": \"merge_commit\"})\n\
                 → Error: Commit is a merge, specify mainline\n\n\
                 AUTHOR PRESERVATION:\n\n\
                 Original commit:\n\
                 Author: Alice <alice@example.com>\n\
                 Committer: Alice <alice@example.com>\n\n\
                 After cherry-pick:\n\
                 Author: Alice <alice@example.com> (preserved)\n\
                 Committer: Bob <bob@example.com> (you)\n\n\
                 Git log shows both:\n\
                 - Author: Who wrote the original change\n\
                 - Committer: Who cherry-picked it\n\n\
                 COMBINING OPTIONS:\n\n\
                 Multiple cherry-picks with custom message:\n\
                 git_cherry_pick({\n\
                     \"commits\": [\"abc1234\", \"def5678\"],\n\
                     \"no_commit\": true\n\
                 })\n\
                 git_commit({\"message\": \"Combined fixes from main\"})\n\n\
                 Merge commit with custom message:\n\
                 git_cherry_pick({\n\
                     \"commit\": \"merge_hash\",\n\
                     \"mainline\": 1,\n\
                     \"message\": \"Backport feature merge to release\"\n\
                 })\n\n\
                 PRACTICAL EXAMPLES:\n\n\
                 Squash multiple fixes:\n\
                 git_cherry_pick({\"commit\": \"fix1\", \"no_commit\": true})\n\
                 git_cherry_pick({\"commit\": \"fix2\", \"no_commit\": true})\n\
                 git_cherry_pick({\"commit\": \"fix3\", \"no_commit\": true})\n\
                 git_commit({\"message\": \"Backport security fixes to v1.0\"})\n\n\
                 Cherry-pick with modifications:\n\
                 git_cherry_pick({\"commit\": \"abc1234\", \"no_commit\": true})\n\
                 // Modify for compatibility\n\
                 fs_edit_block({...update API calls...})\n\
                 git_add({\"files\": [\"src/api.rs\"]})\n\
                 git_commit({\"message\": \"Backport feature (adapted for v1.0 API)\"})\n\n\
                 Cherry-pick merge commit:\n\
                 git_cherry_pick({\n\
                     \"commit\": \"merge_abc1234\",\n\
                     \"mainline\": 1,\n\
                     \"message\": \"Merge feature X (cherry-picked to release)\"\n\
                 })\n\n\
                 USE CASES:\n\n\
                 no_commit:\n\
                 - Squashing multiple cherry-picks into one commit\n\
                 - Modifying cherry-picked code before committing\n\
                 - Building up complex backports\n\
                 - Testing changes before committing\n\n\
                 message:\n\
                 - Adding backport documentation\n\
                 - Providing release-specific context\n\
                 - Recording original commit hash\n\
                 - Explaining modifications made\n\n\
                 mainline:\n\
                 - Cherry-picking merge commits\n\
                 - Applying entire feature merges\n\
                 - Backporting merged branches\n\n\
                 BEST PRACTICES:\n\
                 - Use no_commit for related cherry-picks\n\
                 - Document backports in commit messages\n\
                 - Specify mainline for merge commits\n\
                 - Test thoroughly when using no_commit\n\
                 - Preserve author information (default)\n\
                 - Add context in custom messages\n\n\
                 PARAMETERS SUMMARY:\n\
                 - commit: Commit hash to cherry-pick\n\
                 - commits: Array of commit hashes\n\
                 - range: Commit range (start..end)\n\
                 - no_commit: Stage but don't commit (default: false)\n\
                 - edit: Edit commit message (default: false)\n\
                 - message: Custom commit message\n\
                 - mainline: Parent number for merge commits (1 or 2)\n\
                 - continue: Continue after resolving conflicts\n\
                 - abort: Cancel cherry-pick operation\n\
                 - skip: Skip current commit",
            ),
        },
    ]
}

/// Comprehensive cherry-pick guide
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using git_cherry_pick effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Master git_cherry_pick to selectively apply commits across branches. This guide covers all cherry-pick operations and best practices.\n\n\
                 =============================================================================\n\
                 WHAT IS CHERRY-PICKING?\n\
                 =============================================================================\n\n\
                 Cherry-picking applies specific commits from one branch to another:\n\
                 - Takes changes from a commit\n\
                 - Creates NEW commit on current branch\n\
                 - Different hash, same changes\n\
                 - Preserves original author\n\n\
                 Unlike merge:\n\
                 - Selective: Choose specific commits\n\
                 - Creates new commits (not merge commit)\n\
                 - No branch relationship created\n\
                 - Each commit applied individually\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 Single commit:\n\
                 git_cherry_pick({\n\
                     \"path\": \"/project\",\n\
                     \"commit\": \"abc1234\"\n\
                 })\n\n\
                 Multiple commits:\n\
                 git_cherry_pick({\n\
                     \"path\": \"/project\",\n\
                     \"commits\": [\"abc1234\", \"def5678\", \"ghi9012\"]\n\
                 })\n\n\
                 Commit range:\n\
                 git_cherry_pick({\n\
                     \"path\": \"/project\",\n\
                     \"range\": \"abc1234..def5678\"\n\
                 })\n\n\
                 =============================================================================\n\
                 PARAMETERS\n\
                 =============================================================================\n\n\
                 path (required):\n\
                 - Repository root directory\n\
                 - Example: \"/home/user/project\"\n\n\
                 commit (optional):\n\
                 - Single commit hash to cherry-pick\n\
                 - Short hash: \"abc1234\"\n\
                 - Full hash: \"abc1234567890abcdef...\"\n\n\
                 commits (optional):\n\
                 - Array of commit hashes\n\
                 - Applied in order given\n\
                 - Example: [\"commit1\", \"commit2\", \"commit3\"]\n\n\
                 range (optional):\n\
                 - Range of commits\n\
                 - \"start..end\" (excludes start, includes end)\n\
                 - \"start^..end\" (includes both)\n\n\
                 no_commit (optional, default: false):\n\
                 - Stage changes but don't commit\n\
                 - Allows modification before committing\n\
                 - Example: \"no_commit\": true\n\n\
                 message (optional):\n\
                 - Custom commit message\n\
                 - Overrides original message\n\
                 - Example: \"message\": \"Backported fix\"\n\n\
                 mainline (optional):\n\
                 - Required for merge commits\n\
                 - 1 = first parent, 2 = second parent\n\
                 - Example: \"mainline\": 1\n\n\
                 continue (optional):\n\
                 - Resume after resolving conflicts\n\
                 - Example: \"continue\": true\n\n\
                 abort (optional):\n\
                 - Cancel cherry-pick operation\n\
                 - Returns to pre-cherry-pick state\n\
                 - Example: \"abort\": true\n\n\
                 skip (optional):\n\
                 - Skip current problematic commit\n\
                 - Continue with next in sequence\n\
                 - Example: \"skip\": true\n\n\
                 =============================================================================\n\
                 TYPICAL WORKFLOW\n\
                 =============================================================================\n\n\
                 1. Find commit to cherry-pick:\n\
                    git_log({\n\
                        \"path\": \"/project\",\n\
                        \"branch\": \"main\",\n\
                        \"search\": \"fix bug\"\n\
                    })\n\n\
                 2. Switch to target branch:\n\
                    git_checkout({\n\
                        \"path\": \"/project\",\n\
                        \"branch\": \"release/v1.0\"\n\
                    })\n\n\
                 3. Cherry-pick the commit:\n\
                    git_cherry_pick({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"abc1234\"\n\
                    })\n\n\
                 4. Verify result:\n\
                    git_status({\"path\": \"/project\"})\n\
                    git_log({\"path\": \"/project\", \"limit\": 1})\n\n\
                 =============================================================================\n\
                 HANDLING CONFLICTS\n\
                 =============================================================================\n\n\
                 When conflict occurs:\n\
                 git_cherry_pick({\"commit\": \"abc1234\"})\n\
                 → {\"success\": false, \"conflicts\": [\"file.rs\"]}\n\n\
                 Resolution steps:\n\
                 1. Check status:\n\
                    git_status({\"path\": \"/project\"})\n\n\
                 2. Read conflicted files:\n\
                    fs_read_file({\"path\": \"/project/file.rs\"})\n\n\
                 3. Resolve conflicts:\n\
                    fs_edit_block({...remove markers, fix code...})\n\n\
                 4. Stage resolved files:\n\
                    git_add({\"files\": [\"file.rs\"]})\n\n\
                 5. Continue cherry-pick:\n\
                    git_cherry_pick({\"continue\": true})\n\n\
                 Or abort if needed:\n\
                 git_cherry_pick({\"abort\": true})\n\n\
                 =============================================================================\n\
                 COMMON USE CASES\n\
                 =============================================================================\n\n\
                 BACKPORT BUG FIX:\n\
                 // Fix merged to main\n\
                 git_checkout({\"branch\": \"release/v2.0\"})\n\
                 git_cherry_pick({\"commit\": \"fix_hash\"})\n\
                 git_push()\n\n\
                 SELECTIVE FEATURE PORTING:\n\
                 // Apply specific feature commits\n\
                 git_checkout({\"branch\": \"release\"})\n\
                 git_cherry_pick({\n\
                     \"commits\": [\"feature_commit1\", \"feature_commit2\"]\n\
                 })\n\n\
                 HOTFIX TO MULTIPLE VERSIONS:\n\
                 // Apply to v1.0\n\
                 git_checkout({\"branch\": \"release/v1.0\"})\n\
                 git_cherry_pick({\"commit\": \"hotfix\"})\n\
                 git_push()\n\
                 // Apply to v2.0\n\
                 git_checkout({\"branch\": \"release/v2.0\"})\n\
                 git_cherry_pick({\"commit\": \"hotfix\"})\n\
                 git_push()\n\n\
                 EXTRACT COMMITS FROM ABANDONED BRANCH:\n\
                 git_log({\"branch\": \"old-feature\"})\n\
                 git_checkout({\"branch\": \"new-feature\"})\n\
                 git_cherry_pick({\n\
                     \"commits\": [\"useful1\", \"useful2\", \"useful3\"]\n\
                 })\n\n\
                 SQUASH MULTIPLE CHERRY-PICKS:\n\
                 git_cherry_pick({\"commit\": \"fix1\", \"no_commit\": true})\n\
                 git_cherry_pick({\"commit\": \"fix2\", \"no_commit\": true})\n\
                 git_cherry_pick({\"commit\": \"fix3\", \"no_commit\": true})\n\
                 git_commit({\"message\": \"Combined fixes\"})\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 DO:\n\
                 ✓ Verify target branch before cherry-picking\n\
                 ✓ Use git_log to find exact commit hashes\n\
                 ✓ Cherry-pick in chronological order\n\
                 ✓ Test after cherry-picking\n\
                 ✓ Document backports in commit messages\n\
                 ✓ Resolve conflicts carefully\n\
                 ✓ Check git_status after operations\n\n\
                 DON'T:\n\
                 ✗ Cherry-pick commits out of order\n\
                 ✗ Cherry-pick merge commits without mainline\n\
                 ✗ Skip dependency commits\n\
                 ✗ Cherry-pick without testing\n\
                 ✗ Forget to push after cherry-picking\n\
                 ✗ Cherry-pick when merge is better\n\n\
                 =============================================================================\n\
                 CHERRY-PICK VS MERGE\n\
                 =============================================================================\n\n\
                 Use cherry-pick when:\n\
                 - Need specific commits, not entire branch\n\
                 - Backporting fixes to releases\n\
                 - Applying hotfixes to multiple versions\n\
                 - Extracting commits from abandoned work\n\
                 - Commits are independent\n\n\
                 Use merge when:\n\
                 - Bringing entire branch history\n\
                 - Feature is complete\n\
                 - Want to preserve branch relationships\n\
                 - Many interconnected commits\n\
                 - Standard development workflow\n\n\
                 =============================================================================\n\
                 UNDERSTANDING COMMIT HASHES\n\
                 =============================================================================\n\n\
                 Original commit on main:\n\
                 Hash: abc1234567890abcdef\n\
                 Author: Alice\n\
                 Message: \"Fix bug\"\n\n\
                 After cherry-pick to release:\n\
                 Hash: xyz7890abcdef123456 (NEW, different)\n\
                 Author: Alice (preserved)\n\
                 Committer: Bob (you)\n\
                 Message: \"Fix bug\" (same, or custom)\n\n\
                 Key points:\n\
                 - New commit has different hash\n\
                 - Original author preserved\n\
                 - Committer is person who cherry-picked\n\
                 - Changes are identical\n\
                 - Commits are independent\n\n\
                 =============================================================================\n\
                 ADVANCED TECHNIQUES\n\
                 =============================================================================\n\n\
                 Range cherry-pick:\n\
                 git_cherry_pick({\"range\": \"commit1^..commit5\"})\n\
                 // Applies commits 1 through 5\n\n\
                 Mainline for merges:\n\
                 git_cherry_pick({\"commit\": \"merge_hash\", \"mainline\": 1})\n\
                 // Required for merge commits\n\n\
                 No-commit mode:\n\
                 git_cherry_pick({\"commit\": \"abc\", \"no_commit\": true})\n\
                 // Stage changes, manual commit later\n\n\
                 Custom message:\n\
                 git_cherry_pick({\n\
                     \"commit\": \"abc1234\",\n\
                     \"message\": \"Backported fix from main (abc1234)\"\n\
                 })\n\n\
                 =============================================================================\n\
                 TROUBLESHOOTING\n\
                 =============================================================================\n\n\
                 Empty commit:\n\
                 - Changes already present in target branch\n\
                 - Cherry-pick will skip or warn\n\n\
                 Conflict:\n\
                 - Resolve manually\n\
                 - Stage resolved files\n\
                 - Continue with --continue\n\n\
                 Wrong branch:\n\
                 - Abort cherry-pick\n\
                 - Checkout correct branch\n\
                 - Start over\n\n\
                 Merge commit without mainline:\n\
                 - Add mainline: 1 or mainline: 2\n\n\
                 =============================================================================\n\
                 RESPONSE FORMAT\n\
                 =============================================================================\n\n\
                 Success:\n\
                 {\n\
                   \"success\": true,\n\
                   \"commit\": \"abc1234\",\n\
                   \"new_commit\": \"xyz7890\"\n\
                 }\n\n\
                 Conflict:\n\
                 {\n\
                   \"success\": false,\n\
                   \"conflicts\": [\"file1.rs\", \"file2.rs\"],\n\
                   \"state\": \"cherry_picking\"\n\
                 }\n\n\
                 Multiple commits:\n\
                 {\n\
                   \"success\": true,\n\
                   \"commits\": [\n\
                     {\"original\": \"abc1234\", \"new\": \"new1234\"},\n\
                     {\"original\": \"def5678\", \"new\": \"new5678\"}\n\
                   ]\n\
                 }\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Single commit:\n\
                 git_cherry_pick({\"path\": \"/project\", \"commit\": \"abc1234\"})\n\n\
                 Multiple commits:\n\
                 git_cherry_pick({\"path\": \"/project\", \"commits\": [\"abc\", \"def\", \"ghi\"]})\n\n\
                 Commit range:\n\
                 git_cherry_pick({\"path\": \"/project\", \"range\": \"start^..end\"})\n\n\
                 No commit:\n\
                 git_cherry_pick({\"commit\": \"abc\", \"no_commit\": true})\n\n\
                 Continue after conflict:\n\
                 git_cherry_pick({\"path\": \"/project\", \"continue\": true})\n\n\
                 Abort:\n\
                 git_cherry_pick({\"path\": \"/project\", \"abort\": true})\n\n\
                 Skip:\n\
                 git_cherry_pick({\"path\": \"/project\", \"skip\": true})\n\n\
                 Merge commit:\n\
                 git_cherry_pick({\"commit\": \"merge\", \"mainline\": 1})\n\n\
                 Remember: Cherry-pick creates new commits with same changes. Test thoroughly and document backports!",
            ),
        },
    ]
}
