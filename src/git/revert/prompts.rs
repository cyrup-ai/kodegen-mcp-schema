//! Prompt messages for git_revert tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitRevertPromptArgs;

/// Prompt provider for git_revert tool
///
/// This is the ONLY way to provide prompts for git_revert - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct RevertPrompts;

impl PromptProvider for RevertPrompts {
    type PromptArgs = GitRevertPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("single") => prompt_single(),
            Some("multiple") => prompt_multiple(),
            Some("merge") => prompt_merge(),
            Some("conflicts") => prompt_conflicts(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (single, multiple, merge, conflicts)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO REVERT COMMITS
// ============================================================================

/// Reverting single commits
fn prompt_single() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I revert a single commit to undo its changes?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Revert creates a new commit that undoes the changes from a previous commit. This is the safe way to undo changes on shared branches.\n\n\
                 REVERTING SINGLE COMMIT:\n\n\
                 1. Revert a commit:\n\
                    git_revert({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"abc1234\"\n\
                    })\n\
                    // Creates NEW commit undoing abc1234\n\n\
                 2. Find and revert:\n\
                    // First, find the problematic commit\n\
                    git_log({\"path\": \"/project\", \"limit\": 10})\n\
                    // Then revert it\n\
                    git_revert({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"def5678\"\n\
                    })\n\n\
                 3. Revert recent mistake:\n\
                    git_log({\"path\": \"/project\", \"limit\": 1})\n\
                    // Get latest commit hash\n\
                    git_revert({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"latest_hash\"\n\
                    })\n\n\
                 4. Use short or full hash:\n\
                    git_revert({\"path\": \"/project\", \"commit\": \"abc1234\"})\n\
                    // Or full hash\n\
                    git_revert({\"path\": \"/project\", \"commit\": \"abc1234567890abcdef1234567890abcdef12345\"})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"reverted\": \"abc1234\",\n\
                   \"new_commit\": \"xyz7890\",\n\
                   \"message\": \"Revert \\\"Original commit message\\\"\",\n\
                   \"success\": true\n\
                 }\n\n\
                 WHAT HAPPENS:\n\
                 - Creates NEW commit with inverse changes\n\
                 - New commit hash (xyz7890)\n\
                 - Original commit (abc1234) stays in history\n\
                 - Commit message: \"Revert \\\"Original message\\\"\"\n\
                 - History preserved (safe for shared branches)\n\n\
                 REVERT vs RESET:\n\n\
                 Revert (safe for shared branches):\n\
                 - Adds new commit\n\
                 - Undoes changes by applying inverse\n\
                 - Preserves history\n\
                 - Safe for pushed commits\n\
                 - Safe for collaboration\n\
                 - Example: main branch, release branches\n\n\
                 Reset (dangerous for shared branches):\n\
                 - Removes commits from history\n\
                 - Rewrites history\n\
                 - Dangerous if already pushed\n\
                 - Breaks collaborators' repos\n\
                 - Only use on local branches\n\
                 - Example: local work-in-progress\n\n\
                 WHEN TO USE REVERT:\n\
                 - Undo pushed commits\n\
                 - Working on shared branches (main, develop)\n\
                 - Need to preserve history\n\
                 - Collaborating with others\n\
                 - Production hotfixes\n\
                 - Undoing released changes\n\n\
                 FINDING COMMITS TO REVERT:\n\n\
                 By recent history:\n\
                 git_log({\"path\": \"/project\", \"limit\": 20})\n\
                 // Review recent commits\n\
                 git_revert({\"commit\": \"abc1234\"})\n\n\
                 By search:\n\
                 git_log({\"path\": \"/project\", \"search\": \"breaking change\"})\n\
                 // Find the problematic commit\n\
                 git_revert({\"commit\": \"def5678\"})\n\n\
                 By branch comparison:\n\
                 git_log({\"path\": \"/project\", \"branch\": \"main\", \"limit\": 10})\n\
                 // Identify commit to undo\n\
                 git_revert({\"commit\": \"ghi9012\"})\n\n\
                 COMMIT HASH FORMATS:\n\
                 - Short: \"abc1234\" (7+ characters)\n\
                 - Full: \"abc1234567890abcdef1234567890abcdef12345\"\n\
                 - Both work, short is more common\n\n\
                 TYPICAL WORKFLOW:\n\n\
                 1. Identify problematic commit:\n\
                    git_log({\"path\": \"/project\", \"branch\": \"main\"})\n\
                    // Find: abc1234 \"Bad feature\"\n\n\
                 2. Revert the commit:\n\
                    git_revert({\"path\": \"/project\", \"commit\": \"abc1234\"})\n\
                    // Creates: xyz7890 \"Revert \\\"Bad feature\\\"\"\n\n\
                 3. Verify the revert:\n\
                    git_status({\"path\": \"/project\"})\n\
                    git_log({\"path\": \"/project\", \"limit\": 1})\n\n\
                 4. Push to remote:\n\
                    git_push({\"path\": \"/project\"})\n\n\
                 UNDERSTANDING THE NEW COMMIT:\n\n\
                 Original commit (abc1234):\n\
                 + Added line A\n\
                 + Added line B\n\
                 - Removed line C\n\n\
                 Revert commit (xyz7890):\n\
                 - Removed line A (undoing the add)\n\
                 - Removed line B (undoing the add)\n\
                 + Added line C (undoing the remove)\n\n\
                 Result:\n\
                 Code returns to state before abc1234\n\
                 Both commits remain in history\n\n\
                 BEST PRACTICES:\n\
                 - Always verify you're reverting correct commit\n\
                 - Use git_log to find exact commit hash\n\
                 - Check git_status after reverting\n\
                 - Test after reverting to ensure it works\n\
                 - Push revert commits immediately\n\
                 - Add clear commit messages explaining why\n\
                 - Use revert for shared branches, not reset\n\n\
                 REVERT MESSAGE FORMAT:\n\
                 Automatic message:\n\
                 Revert \"Original commit message\"\n\n\
                 This reverts commit abc1234567890abcdef.\n\n\
                 You can customize:\n\
                 git_revert({\n\
                     \"commit\": \"abc1234\",\n\
                     \"message\": \"Revert bad feature\\n\\nThis feature caused production issues.\"\n\
                 })\n\n\
                 PRACTICAL EXAMPLES:\n\n\
                 Undo last commit:\n\
                 git_log({\"limit\": 1})\n\
                 git_revert({\"commit\": \"latest_hash\"})\n\n\
                 Undo specific feature:\n\
                 git_log({\"search\": \"feature X\"})\n\
                 git_revert({\"commit\": \"feature_hash\"})\n\n\
                 Undo production bug:\n\
                 git_checkout({\"branch\": \"main\"})\n\
                 git_log({\"limit\": 5})\n\
                 git_revert({\"commit\": \"bug_hash\"})\n\
                 git_push()\n\n\
                 HISTORY AFTER REVERT:\n\
                 Before:\n\
                 A - B - C - D (HEAD)\n\n\
                 Revert C:\n\
                 A - B - C - D - C' (HEAD)\n\
                 Where C' undoes changes from C\n\n\
                 All commits preserved!\n\n\
                 COMPARING WITH OTHER OPERATIONS:\n\n\
                 To undo last commit:\n\
                 - git_revert: Adds new commit (safe)\n\
                 - git_reset: Removes commit (dangerous)\n\n\
                 To undo old commit:\n\
                 - git_revert: Adds new commit undoing old one\n\
                 - git_rebase: Rewrites history (complex)\n\n\
                 To undo merge:\n\
                 - git_revert with mainline: Undoes merge safely\n\
                 - git_reset: Removes merge (dangerous)",
            ),
        },
    ]
}

/// Reverting multiple commits
fn prompt_multiple() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I revert multiple commits at once?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Revert multiple commits to undo a series of changes. Each commit gets its own revert, or you can combine them into one.\n\n\
                 REVERTING MULTIPLE COMMITS:\n\n\
                 1. Revert several commits:\n\
                    git_revert({\n\
                        \"path\": \"/project\",\n\
                        \"commits\": [\"abc1234\", \"def5678\", \"ghi9012\"]\n\
                    })\n\
                    // Creates revert commit for each\n\n\
                 2. Revert commit range:\n\
                    git_revert({\n\
                        \"path\": \"/project\",\n\
                        \"range\": \"abc1234..ghi9012\"\n\
                    })\n\
                    // Reverts all commits between abc1234 and ghi9012\n\n\
                 3. Combined revert (single commit):\n\
                    git_revert({\n\
                        \"path\": \"/project\",\n\
                        \"commits\": [\"abc1234\", \"def5678\"],\n\
                        \"no_commit\": true\n\
                    })\n\
                    // Stage all changes without committing\n\
                    git_commit({\n\
                        \"path\": \"/project\",\n\
                        \"message\": \"Revert feature X (combined)\"\n\
                    })\n\n\
                 4. Revert recent commits:\n\
                    git_log({\"path\": \"/project\", \"limit\": 5})\n\
                    // Identify last 3 commits to undo\n\
                    git_revert({\n\
                        \"path\": \"/project\",\n\
                        \"commits\": [\"commit3\", \"commit2\", \"commit1\"]\n\
                    })\n\n\
                 ORDER MATTERS:\n\
                 - Revert in REVERSE chronological order (newest first)\n\
                 - Example: To revert A-B-C, revert C, then B, then A\n\
                 - This ensures dependencies are handled correctly\n\
                 - Each revert applies to current state\n\n\
                 Why reverse order:\n\
                 Commits: A - B - C (C depends on B, B depends on A)\n\
                 Wrong order: Revert A first breaks B and C\n\
                 Right order: Revert C, then B, then A (no dependencies break)\n\n\
                 RANGE SYNTAX:\n\n\
                 abc1234..ghi9012\n\
                 - Reverts commits from abc1234 (exclusive) to ghi9012 (inclusive)\n\
                 - Does not include abc1234 itself\n\
                 - Includes ghi9012\n\
                 - All commits in between\n\n\
                 abc1234^..ghi9012\n\
                 - Includes abc1234 (using ^)\n\
                 - Includes ghi9012\n\
                 - All commits in between\n\n\
                 PRACTICAL EXAMPLES:\n\n\
                 Undo last 3 commits:\n\
                 git_log({\"path\": \"/project\", \"limit\": 3})\n\
                 // Get: C (newest), B, A (oldest)\n\
                 git_revert({\n\
                     \"path\": \"/project\",\n\
                     \"commits\": [\"C\", \"B\", \"A\"]\n\
                 })\n\
                 // Creates: C', B', A' (three revert commits)\n\n\
                 Undo feature branch merged commits:\n\
                 git_log({\"path\": \"/project\", \"search\": \"feature X\"})\n\
                 git_revert({\n\
                     \"path\": \"/project\",\n\
                     \"commits\": [\"feat3\", \"feat2\", \"feat1\"]\n\
                 })\n\n\
                 Undo range of commits:\n\
                 // Commits: A - B - C - D - E\n\
                 // Want to undo C, D, E\n\
                 git_revert({\n\
                     \"path\": \"/project\",\n\
                     \"range\": \"B..E\"\n\
                 })\n\
                 // Reverts E, D, C (automatic reverse order)\n\n\
                 Combined revert for clean history:\n\
                 git_revert({\n\
                     \"path\": \"/project\",\n\
                     \"commits\": [\"commit3\", \"commit2\", \"commit1\"],\n\
                     \"no_commit\": true\n\
                 })\n\
                 // All changes staged\n\
                 git_commit({\"message\": \"Revert entire feature X\"})\n\
                 // Single revert commit instead of three\n\n\
                 RESPONSE FORMAT:\n\n\
                 Multiple separate commits:\n\
                 {\n\
                   \"reverts\": [\n\
                     {\"original\": \"abc1234\", \"revert_commit\": \"rev1234\"},\n\
                     {\"original\": \"def5678\", \"revert_commit\": \"rev5678\"},\n\
                     {\"original\": \"ghi9012\", \"revert_commit\": \"rev9012\"}\n\
                   ],\n\
                   \"success\": true\n\
                 }\n\n\
                 With no_commit:\n\
                 {\n\
                   \"reverted\": [\"abc1234\", \"def5678\", \"ghi9012\"],\n\
                   \"staged\": true,\n\
                   \"committed\": false,\n\
                   \"success\": true\n\
                 }\n\n\
                 HANDLING PARTIAL SUCCESS:\n\
                 If second commit fails:\n\
                 {\n\
                   \"reverts\": [\n\
                     {\"original\": \"abc1234\", \"revert_commit\": \"rev1234\", \"success\": true},\n\
                     {\"original\": \"def5678\", \"error\": \"conflict\", \"success\": false}\n\
                   ],\n\
                   \"success\": false,\n\
                   \"stopped_at\": \"def5678\"\n\
                 }\n\
                 First revert succeeded, second failed (see conflict handling)\n\n\
                 NO_COMMIT OPTION:\n\n\
                 Use when:\n\
                 - Want single revert commit for multiple changes\n\
                 - Need to modify revert before committing\n\
                 - Cleaner history (one commit vs many)\n\
                 - Testing reverts before committing\n\n\
                 Workflow:\n\
                 1. Revert with no_commit:\n\
                    git_revert({\"commits\": [...], \"no_commit\": true})\n\n\
                 2. Check staged changes:\n\
                    git_status({\"path\": \"/project\"})\n\n\
                 3. Optionally modify:\n\
                    fs_edit_block({...additional changes...})\n\
                    git_add({\"files\": [...]})\n\n\
                 4. Commit manually:\n\
                    git_commit({\"message\": \"Combined revert\"})\n\n\
                 FINDING COMMITS TO REVERT:\n\n\
                 Recent commits:\n\
                 git_log({\"path\": \"/project\", \"limit\": 10})\n\
                 // Review and select\n\n\
                 By author:\n\
                 git_log({\"path\": \"/project\", \"author\": \"alice\"})\n\
                 // Find commits by specific author\n\n\
                 By date:\n\
                 git_log({\"path\": \"/project\", \"since\": \"2024-01-01\"})\n\
                 // Find commits in date range\n\n\
                 By search:\n\
                 git_log({\"path\": \"/project\", \"search\": \"feature\"})\n\
                 // Find feature-related commits\n\n\
                 BEST PRACTICES:\n\
                 - Always revert in REVERSE chronological order\n\
                 - Use specific commits list for precise control\n\
                 - Use ranges for consecutive commits only\n\
                 - Test after reverting multiple commits\n\
                 - Consider no_commit for cleaner history\n\
                 - Verify each commit hash before reverting\n\
                 - Document why multiple commits are being reverted\n\n\
                 WHEN TO USE EACH METHOD:\n\n\
                 Individual revert commits:\n\
                 - Default behavior\n\
                 - Each revert is independent\n\
                 - Can track which specific commit was reverted\n\
                 - Easier to re-revert if needed\n\
                 - More history detail\n\n\
                 Combined revert (no_commit):\n\
                 - Single commit for all reverts\n\
                 - Cleaner history\n\
                 - Logical grouping (one feature)\n\
                 - Less commit noise\n\
                 - Better for related changes\n\n\
                 Range revert:\n\
                 - Consecutive commits\n\
                 - Cleaner syntax\n\
                 - Automatic reverse order\n\
                 - Linear history segment\n\n\
                 AVOIDING MISTAKES:\n\
                 - DON'T revert in chronological order (wrong!)\n\
                 - DO revert in reverse order (newest first)\n\
                 - DON'T skip dependency commits\n\
                 - DO verify all commits before reverting\n\
                 - DON'T mix unrelated commits\n\
                 - DO test after reverting\n\n\
                 HISTORY VISUALIZATION:\n\n\
                 Before:\n\
                 A - B - C - D - E (HEAD)\n\n\
                 Revert D, E:\n\
                 A - B - C - D - E - E' - D' (HEAD)\n\
                 Where E' undoes E, D' undoes D\n\n\
                 With no_commit:\n\
                 A - B - C - D - E - (E+D)' (HEAD)\n\
                 Single commit undoes both E and D",
            ),
        },
    ]
}

/// Reverting merge commits
fn prompt_merge() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I revert merge commits?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Reverting merge commits requires specifying which parent to keep. This is the safe way to undo a merged pull request or feature branch.\n\n\
                 REVERTING MERGE COMMITS:\n\n\
                 1. Revert a merge:\n\
                    git_revert({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"abc1234\",\n\
                        \"mainline\": 1\n\
                    })\n\
                    // mainline: 1 = keep first parent (usually main branch)\n\n\
                 2. Understanding mainline:\n\
                    // Merge commit has TWO parents\n\
                    // Parent 1: Main branch (before merge)\n\
                    // Parent 2: Feature branch (what was merged)\n\
                    //\n\
                    // mainline: 1 = Go back to parent 1 (main)\n\
                    // mainline: 2 = Go back to parent 2 (feature)\n\
                    //\n\
                    // Usually want mainline: 1\n\n\
                 3. Typical PR revert workflow:\n\
                    // Feature was merged to main via PR\n\
                    // Now need to revert the merge\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\
                    git_log({\"path\": \"/project\", \"limit\": 5})\n\
                    // Find merge commit hash\n\
                    git_revert({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"merge_commit_hash\",\n\
                        \"mainline\": 1\n\
                    })\n\
                    // Undoes entire feature merge\n\
                    git_push({\"path\": \"/project\"})\n\n\
                 4. Identify merge commits:\n\
                    git_log({\"path\": \"/project\", \"limit\": 20})\n\
                    // Look for: \"Merge pull request\" or \"Merge branch\"\n\
                    // Or commits with multiple parents\n\n\
                 MAINLINE EXPLAINED:\n\n\
                 Merge commit structure:\n\
                 main: A - B - C ----------- M (merge commit)\n\
                                            /\n\
                 feature:           D - E - F\n\n\
                 Merge commit M has two parents:\n\
                 - Parent 1: C (from main branch)\n\
                 - Parent 2: F (from feature branch)\n\n\
                 To revert the merge:\n\
                 git_revert({\"commit\": \"M\", \"mainline\": 1})\n\
                 // Says: \"Go back to parent 1 (C)\"\n\
                 // Effect: Undoes changes D, E, F\n\n\
                 mainline: 1 (most common):\n\
                 - Keep main branch state\n\
                 - Undo feature branch changes\n\
                 - Typical PR revert\n\n\
                 mainline: 2 (rare):\n\
                 - Keep feature branch state\n\
                 - Undo main branch changes\n\
                 - Unusual scenario\n\n\
                 ERROR WITHOUT MAINLINE:\n\
                 git_revert({\"commit\": \"merge_commit\"})\n\
                 → Error: Commit is a merge but no -m option given\n\n\
                 ALWAYS specify mainline for merge commits:\n\
                 git_revert({\"commit\": \"merge_commit\", \"mainline\": 1})\n\n\
                 TYPICAL USE CASES:\n\n\
                 Revert merged PR:\n\
                 // PR #123 was merged, but has bugs\n\
                 git_checkout({\"branch\": \"main\"})\n\
                 git_log({\"search\": \"Merge pull request #123\"})\n\
                 // Find merge commit\n\
                 git_revert({\n\
                     \"commit\": \"merge_hash\",\n\
                     \"mainline\": 1,\n\
                     \"message\": \"Revert PR #123 due to production bugs\"\n\
                 })\n\
                 git_push()\n\n\
                 Revert feature branch merge:\n\
                 git_log({\"search\": \"Merge branch 'feature/x'\"})\n\
                 git_revert({\n\
                     \"commit\": \"merge_hash\",\n\
                     \"mainline\": 1\n\
                 })\n\n\
                 Revert hotfix merge:\n\
                 git_log({\"branch\": \"release\", \"limit\": 10})\n\
                 git_revert({\n\
                     \"commit\": \"hotfix_merge\",\n\
                     \"mainline\": 1\n\
                 })\n\n\
                 FINDING MERGE COMMITS:\n\n\
                 By message pattern:\n\
                 git_log({\"path\": \"/project\", \"search\": \"Merge pull request\"})\n\
                 git_log({\"path\": \"/project\", \"search\": \"Merge branch\"})\n\n\
                 By recent history:\n\
                 git_log({\"path\": \"/project\", \"limit\": 20})\n\
                 // Look for merge commit messages\n\n\
                 By PR number:\n\
                 git_log({\"path\": \"/project\", \"search\": \"#123\"})\n\n\
                 RESPONSE FORMAT:\n\
                 {\n\
                   \"reverted\": \"abc1234\",\n\
                   \"new_commit\": \"rev7890\",\n\
                   \"message\": \"Revert \\\"Merge pull request #123\\\"\",\n\
                   \"mainline_used\": 1,\n\
                   \"success\": true\n\
                 }\n\n\
                 WHAT GETS REVERTED:\n\n\
                 Original merge:\n\
                 main: A - B - C ----------- M (HEAD)\n\
                                            /\n\
                 feature:           D - E - F\n\
                 M includes changes from D, E, F\n\n\
                 After revert with mainline: 1:\n\
                 main: A - B - C - M - M' (HEAD)\n\
                 M' undoes all changes from D, E, F\n\
                 Back to state at C\n\n\
                 COMPLETE WORKFLOW:\n\n\
                 1. Ensure on correct branch:\n\
                    git_checkout({\"branch\": \"main\"})\n\
                    git_pull({\"path\": \"/project\"})\n\n\
                 2. Find merge commit:\n\
                    git_log({\"limit\": 20})\n\
                    // Identify: \"Merge pull request #123\"\n\
                    // Hash: abc1234\n\n\
                 3. Revert the merge:\n\
                    git_revert({\n\
                        \"commit\": \"abc1234\",\n\
                        \"mainline\": 1,\n\
                        \"message\": \"Revert PR #123\\n\\nCauses production errors.\"\n\
                    })\n\n\
                 4. Verify the revert:\n\
                    git_status()\n\
                    git_log({\"limit\": 1})\n\n\
                 5. Push immediately:\n\
                    git_push()\n\n\
                 BEST PRACTICES:\n\
                 - Always use mainline: 1 for PR reverts\n\
                 - Add clear message explaining why\n\
                 - Reference PR number in commit message\n\
                 - Push immediately after reverting\n\
                 - Notify team about the revert\n\
                 - Create new PR if re-applying later\n\n\
                 WHICH MAINLINE TO USE:\n\n\
                 mainline: 1 (99% of cases):\n\
                 - Reverting PR merged to main\n\
                 - Reverting feature branch merge\n\
                 - Reverting hotfix merge\n\
                 - Undoing feature addition\n\
                 - Standard merge revert\n\n\
                 mainline: 2 (rare):\n\
                 - Very unusual\n\
                 - Check with team first\n\
                 - Usually not what you want\n\n\
                 When in doubt, use mainline: 1\n\n\
                 RE-APPLYING REVERTED MERGE:\n\n\
                 If you later want to re-apply:\n\
                 1. Fix the issues in feature branch\n\
                 2. Create NEW merge commit (don't revert the revert)\n\
                 3. Or cherry-pick specific commits\n\
                 4. Or revert the revert commit (advanced)\n\n\
                 COMMON MISTAKES:\n\
                 - Forgetting mainline parameter (error)\n\
                 - Using wrong mainline number\n\
                 - Not pushing after reverting\n\
                 - Not documenting why\n\
                 - Trying to reset instead (dangerous)\n\n\
                 REVERT vs RESET FOR MERGES:\n\n\
                 Revert (SAFE):\n\
                 git_revert({\"commit\": \"merge\", \"mainline\": 1})\n\
                 - Adds revert commit\n\
                 - Preserves history\n\
                 - Safe for pushed commits\n\
                 - Safe for shared branches\n\n\
                 Reset (DANGEROUS):\n\
                 git_reset({\"commit\": \"before_merge\"})\n\
                 - Removes merge from history\n\
                 - Rewrites history\n\
                 - Breaks others' repos\n\
                 - Never use on pushed commits\n\n\
                 ALWAYS use revert for merge commits on shared branches!",
            ),
        },
    ]
}

/// Handling revert conflicts
fn prompt_conflicts() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I handle conflicts when reverting commits?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Revert conflicts occur when code has changed since the commit you're reverting. Manual resolution is required.\n\n\
                 HANDLING REVERT CONFLICTS:\n\n\
                 1. Conflict occurs:\n\
                    git_revert({\"path\": \"/project\", \"commit\": \"abc1234\"})\n\
                    // Response indicates conflict:\n\
                    {\n\
                      \"success\": false,\n\
                      \"conflicts\": [\"src/main.rs\", \"src/lib.rs\"],\n\
                      \"state\": \"reverting\"\n\
                    }\n\n\
                 2. Check conflicted files:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Shows:\n\
                    // both modified: src/main.rs\n\
                    // both modified: src/lib.rs\n\n\
                 3. Read and understand conflicts:\n\
                    fs_read_file({\"path\": \"/project/src/main.rs\"})\n\
                    // See conflict markers:\n\
                    // <<<<<<< HEAD\n\
                    // current code\n\
                    // =======\n\
                    // reverted code\n\
                    // >>>>>>>\n\n\
                 4. Resolve conflicts:\n\
                    fs_edit_block({\n\
                        \"path\": \"/project/src/main.rs\",\n\
                        \"old_string\": \"<<<<<<< HEAD\\ncurrent\\n=======\\nreverted\\n>>>>>>>\",\n\
                        \"new_string\": \"resolved code\"\n\
                    })\n\
                    // Remove markers, keep correct code\n\n\
                 5. Stage resolved files:\n\
                    git_add({\n\
                        \"path\": \"/project\",\n\
                        \"files\": [\"src/main.rs\", \"src/lib.rs\"]\n\
                    })\n\n\
                 6. Continue revert:\n\
                    git_revert({\n\
                        \"path\": \"/project\",\n\
                        \"continue\": true\n\
                    })\n\n\
                 ABORT REVERT:\n\n\
                 If you want to cancel:\n\
                 git_revert({\n\
                     \"path\": \"/project\",\n\
                     \"abort\": true\n\
                 })\n\
                 // Returns to state before revert\n\
                 // All changes discarded\n\
                 // Safe to try different approach\n\n\
                 SKIP COMMIT:\n\n\
                 When reverting multiple commits:\n\
                 git_revert({\n\
                     \"path\": \"/project\",\n\
                     \"skip\": true\n\
                 })\n\
                 // Skips current problematic commit\n\
                 // Continues with next in sequence\n\
                 // Use when conflict too complex\n\n\
                 WHY CONFLICTS OCCUR:\n\n\
                 1. Code changed after the commit:\n\
                    Original commit: Added function foo()\n\
                    Later changes: Modified function foo()\n\
                    Revert attempt: Try to remove foo()\n\
                    Conflict: Can't remove modified foo()\n\n\
                 2. Dependencies on reverted code:\n\
                    Original commit: Added class A\n\
                    Later commits: Class B uses class A\n\
                    Revert attempt: Remove class A\n\
                    Conflict: Class B still needs A\n\n\
                 3. Overlapping changes:\n\
                    Original commit: Modified line 10\n\
                    Later changes: Also modified line 10\n\
                    Revert attempt: Undo modification\n\
                    Conflict: Which change to keep?\n\n\
                 CONFLICT WORKFLOW:\n\n\
                 Step 1: Revert reports conflict\n\
                 git_revert({\"commit\": \"abc1234\"})\n\
                 → {\"success\": false, \"conflicts\": [\"file.rs\"]}\n\n\
                 Step 2: See conflicted files\n\
                 git_status({\"path\": \"/project\"})\n\
                 → both modified: src/file.rs\n\n\
                 Step 3: Read conflict\n\
                 fs_read_file({\"path\": \"/project/src/file.rs\"})\n\
                 → See markers:\n\
                 <<<<<<< HEAD\n\
                 Current version of code\n\
                 =======\n\
                 What revert wants to apply\n\
                 >>>>>>> parent of abc1234\n\n\
                 Step 4: Understand context\n\
                 - What did original commit do?\n\
                 - What changed since then?\n\
                 - What should final state be?\n\n\
                 Step 5: Resolve\n\
                 fs_edit_block({\n\
                     \"path\": \"src/file.rs\",\n\
                     \"old_string\": \"[full conflict block]\",\n\
                     \"new_string\": \"[resolved code]\"\n\
                 })\n\n\
                 Step 6: Stage\n\
                 git_add({\"files\": [\"src/file.rs\"]})\n\n\
                 Step 7: Continue\n\
                 git_revert({\"continue\": true})\n\n\
                 CONFLICT MARKERS:\n\n\
                 <<<<<<< HEAD\n\
                 This is the CURRENT code in your branch\n\
                 What exists NOW\n\
                 =======\n\
                 This is what the REVERT wants to apply\n\
                 The inverse of the original commit\n\
                 >>>>>>> parent of abc1234 (commit message)\n\n\
                 You must:\n\
                 1. Remove ALL markers (<<<, ===, >>>)\n\
                 2. Decide what code to keep\n\
                 3. Ensure code compiles and works\n\
                 4. Preserve intent of revert where possible\n\n\
                 RESOLUTION STRATEGIES:\n\n\
                 Keep current version (HEAD):\n\
                 - When changes since commit are important\n\
                 - When revert would break new code\n\
                 - Remove revert changes\n\
                 - Keep HEAD code\n\
                 - May need manual adjustments\n\n\
                 Apply revert version:\n\
                 - When revert is critical\n\
                 - When current changes can be redone\n\
                 - Remove HEAD code\n\
                 - Keep revert changes\n\
                 - May break dependent code\n\n\
                 Combine both:\n\
                 - Partial revert\n\
                 - Keep some current changes\n\
                 - Apply some revert changes\n\
                 - Most complex but often necessary\n\n\
                 MULTIPLE CONFLICTS:\n\n\
                 If multiple files conflict:\n\
                 1. List all conflicts:\n\
                    git_status()\n\n\
                 2. Resolve each file:\n\
                    fs_read_file({\"path\": \"file1.rs\"})\n\
                    fs_edit_block({\"path\": \"file1.rs\", ...})\n\
                    git_add({\"files\": [\"file1.rs\"]})\n\n\
                    fs_read_file({\"path\": \"file2.rs\"})\n\
                    fs_edit_block({\"path\": \"file2.rs\", ...})\n\
                    git_add({\"files\": [\"file2.rs\"]})\n\n\
                 3. Verify all resolved:\n\
                    git_status()\n\
                    // Should show all files staged\n\n\
                 4. Continue revert:\n\
                    git_revert({\"continue\": true})\n\n\
                 CHECKING BEFORE CONTINUING:\n\n\
                 1. All conflicts resolved:\n\
                    git_status()\n\
                    // No \"both modified\" entries\n\n\
                 2. Code compiles:\n\
                    // Run build if possible\n\
                    // Ensure no syntax errors\n\n\
                 3. Tests pass:\n\
                    // Run tests if available\n\
                    // Verify functionality\n\n\
                 4. All files staged:\n\
                    git_status()\n\
                    // All resolved files should be staged\n\n\
                 WHEN TO ABORT:\n\n\
                 Abort if:\n\
                 - Too many conflicts (dozens of files)\n\
                 - Can't determine correct resolution\n\
                 - Would break critical functionality\n\
                 - Dependencies too complex\n\
                 - Need team discussion first\n\
                 - Better approach exists (rewrite)\n\n\
                 How to abort:\n\
                 git_revert({\"abort\": true})\n\
                 // Returns to pre-revert state\n\
                 // Try different approach\n\n\
                 ALTERNATIVES TO CONFLICTED REVERT:\n\n\
                 1. Revert in steps:\n\
                    // Instead of reverting old commit with conflicts\n\
                    // Revert recent dependent commits first\n\
                    git_revert({\"commits\": [\"recent1\", \"recent2\", \"old\"]})\n\n\
                 2. Manual rewrite:\n\
                    // Instead of reverting with conflicts\n\
                    // Manually remove unwanted code\n\
                    fs_edit_block({...remove bad code...})\n\
                    git_commit({\"message\": \"Remove bad feature manually\"})\n\n\
                 3. Feature flag:\n\
                    // Instead of reverting\n\
                    // Disable feature with flag\n\
                    fs_edit_block({...add if (feature_disabled) return...})\n\
                    git_commit({\"message\": \"Disable problematic feature\"})\n\n\
                 BEST PRACTICES:\n\
                 - Read both versions carefully\n\
                 - Understand original commit\n\
                 - Understand subsequent changes\n\
                 - Test resolution thoroughly\n\
                 - Document complex resolutions\n\
                 - Ask for help if unsure\n\
                 - Consider abort if too complex\n\n\
                 COMMON MISTAKES:\n\
                 - Forgetting to remove conflict markers\n\
                 - Not staging all resolved files\n\
                 - Choosing wrong resolution\n\
                 - Not testing after resolution\n\
                 - Continuing with unresolved conflicts\n\n\
                 OPERATION FLAGS:\n\
                 continue: true\n\
                 - Resume after resolving conflicts\n\
                 - All conflicts must be resolved first\n\
                 - All resolved files must be staged\n\n\
                 abort: true\n\
                 - Cancel revert operation\n\
                 - Return to pre-revert state\n\
                 - Discards all changes\n\
                 - Safe to start over\n\n\
                 skip: true\n\
                 - Skip current commit\n\
                 - Continue with next in sequence\n\
                 - Use for complex conflicts\n\
                 - Only during multiple reverts",
            ),
        },
    ]
}

/// Comprehensive revert guide
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using git_revert effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Master git_revert to safely undo commits on shared branches. This guide covers all revert operations and best practices.\n\n\
                 =============================================================================\n\
                 WHAT IS REVERT?\n\
                 =============================================================================\n\n\
                 Revert creates NEW commits that undo previous commits:\n\
                 - Adds commits (doesn't remove)\n\
                 - Safe for shared branches\n\
                 - Preserves history\n\
                 - Applies inverse changes\n\
                 - No history rewriting\n\n\
                 Original: + Added line A\n\
                 Revert: - Removed line A (undo)\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 Single commit:\n\
                 git_revert({\n\
                     \"path\": \"/project\",\n\
                     \"commit\": \"abc1234\"\n\
                 })\n\n\
                 Multiple commits:\n\
                 git_revert({\n\
                     \"path\": \"/project\",\n\
                     \"commits\": [\"abc1234\", \"def5678\", \"ghi9012\"]\n\
                 })\n\n\
                 Merge commit:\n\
                 git_revert({\n\
                     \"path\": \"/project\",\n\
                     \"commit\": \"merge_hash\",\n\
                     \"mainline\": 1\n\
                 })\n\n\
                 =============================================================================\n\
                 PARAMETERS\n\
                 =============================================================================\n\n\
                 path (required):\n\
                 - Repository root directory\n\
                 - Example: \"/home/user/project\"\n\n\
                 commit (optional):\n\
                 - Single commit hash to revert\n\
                 - Short: \"abc1234\"\n\
                 - Full: \"abc1234567890abcdef...\"\n\n\
                 commits (optional):\n\
                 - Array of commit hashes\n\
                 - Reverted in order given\n\
                 - Example: [\"newest\", \"older\", \"oldest\"]\n\n\
                 range (optional):\n\
                 - Range of commits\n\
                 - \"start..end\" (excludes start, includes end)\n\
                 - \"start^..end\" (includes both)\n\n\
                 mainline (optional):\n\
                 - Required for merge commits\n\
                 - 1 = first parent (usually main)\n\
                 - 2 = second parent (usually feature)\n\
                 - Example: \"mainline\": 1\n\n\
                 no_commit (optional, default: false):\n\
                 - Stage changes but don't commit\n\
                 - Allows manual commit\n\
                 - Example: \"no_commit\": true\n\n\
                 message (optional):\n\
                 - Custom commit message\n\
                 - Overrides default \"Revert\"\n\
                 - Example: \"message\": \"Undo broken feature\"\n\n\
                 continue (optional):\n\
                 - Resume after resolving conflicts\n\
                 - Example: \"continue\": true\n\n\
                 abort (optional):\n\
                 - Cancel revert operation\n\
                 - Returns to pre-revert state\n\
                 - Example: \"abort\": true\n\n\
                 skip (optional):\n\
                 - Skip current problematic commit\n\
                 - Continue with next\n\
                 - Example: \"skip\": true\n\n\
                 =============================================================================\n\
                 TYPICAL WORKFLOW\n\
                 =============================================================================\n\n\
                 1. Find commit to revert:\n\
                    git_log({\n\
                        \"path\": \"/project\",\n\
                        \"limit\": 20\n\
                    })\n\n\
                 2. Revert the commit:\n\
                    git_revert({\n\
                        \"path\": \"/project\",\n\
                        \"commit\": \"abc1234\"\n\
                    })\n\n\
                 3. Verify the revert:\n\
                    git_status({\"path\": \"/project\"})\n\
                    git_log({\"path\": \"/project\", \"limit\": 1})\n\n\
                 4. Push to remote:\n\
                    git_push({\"path\": \"/project\"})\n\n\
                 =============================================================================\n\
                 HANDLING CONFLICTS\n\
                 =============================================================================\n\n\
                 When conflict occurs:\n\
                 git_revert({\"commit\": \"abc1234\"})\n\
                 → {\"success\": false, \"conflicts\": [\"file.rs\"]}\n\n\
                 Resolution steps:\n\
                 1. Check status:\n\
                    git_status({\"path\": \"/project\"})\n\n\
                 2. Read conflicts:\n\
                    fs_read_file({\"path\": \"/project/file.rs\"})\n\n\
                 3. Resolve conflicts:\n\
                    fs_edit_block({...remove markers...})\n\n\
                 4. Stage resolved files:\n\
                    git_add({\"files\": [\"file.rs\"]})\n\n\
                 5. Continue revert:\n\
                    git_revert({\"continue\": true})\n\n\
                 Or abort:\n\
                 git_revert({\"abort\": true})\n\n\
                 =============================================================================\n\
                 COMMON USE CASES\n\
                 =============================================================================\n\n\
                 UNDO LAST COMMIT:\n\
                 git_log({\"limit\": 1})\n\
                 git_revert({\"commit\": \"latest_hash\"})\n\
                 git_push()\n\n\
                 UNDO MERGED PR:\n\
                 git_log({\"search\": \"Merge pull request #123\"})\n\
                 git_revert({\"commit\": \"merge_hash\", \"mainline\": 1})\n\
                 git_push()\n\n\
                 UNDO MULTIPLE COMMITS:\n\
                 git_log({\"limit\": 10})\n\
                 git_revert({\n\
                     \"commits\": [\"newest\", \"middle\", \"oldest\"]\n\
                 })\n\
                 git_push()\n\n\
                 UNDO PRODUCTION BUG:\n\
                 git_checkout({\"branch\": \"main\"})\n\
                 git_log({\"search\": \"bad feature\"})\n\
                 git_revert({\"commit\": \"bad_hash\"})\n\
                 git_push()\n\n\
                 COMBINED REVERT:\n\
                 git_revert({\n\
                     \"commits\": [\"c1\", \"c2\", \"c3\"],\n\
                     \"no_commit\": true\n\
                 })\n\
                 git_commit({\"message\": \"Revert entire feature X\"})\n\
                 git_push()\n\n\
                 =============================================================================\n\
                 REVERT vs RESET\n\
                 =============================================================================\n\n\
                 REVERT (Safe for shared branches):\n\
                 - Adds new commit\n\
                 - Undoes changes by applying inverse\n\
                 - Preserves all history\n\
                 - Safe for pushed commits\n\
                 - Safe for collaboration\n\
                 - No force push needed\n\n\
                 Use revert when:\n\
                 ✓ Working on main/shared branches\n\
                 ✓ Commit already pushed\n\
                 ✓ Others might have pulled\n\
                 ✓ Need to preserve history\n\
                 ✓ Production hotfix needed\n\n\
                 RESET (Dangerous for shared branches):\n\
                 - Removes commits\n\
                 - Rewrites history\n\
                 - Dangerous if already pushed\n\
                 - Breaks collaborators\n\
                 - Requires force push\n\n\
                 Use reset when:\n\
                 ✓ Only on local branches\n\
                 ✓ Commits not pushed\n\
                 ✓ Working alone\n\
                 ✓ Private feature branch\n\
                 ✓ Before sharing changes\n\n\
                 =============================================================================\n\
                 REVERTING MERGE COMMITS\n\
                 =============================================================================\n\n\
                 Merge commits require mainline:\n\
                 git_revert({\n\
                     \"commit\": \"merge_hash\",\n\
                     \"mainline\": 1\n\
                 })\n\n\
                 Mainline explanation:\n\
                 - Merge has TWO parents\n\
                 - Parent 1: Main branch (before merge)\n\
                 - Parent 2: Feature branch (what was merged)\n\
                 - mainline: 1 = Keep parent 1 (undo feature)\n\
                 - mainline: 2 = Keep parent 2 (rare)\n\n\
                 Always use mainline: 1 for PR reverts:\n\
                 git_log({\"search\": \"Merge pull request\"})\n\
                 git_revert({\"commit\": \"merge\", \"mainline\": 1})\n\n\
                 =============================================================================\n\
                 REVERTING MULTIPLE COMMITS\n\
                 =============================================================================\n\n\
                 Order matters - revert in REVERSE order:\n\
                 Commits: A - B - C (C newest)\n\
                 Revert order: C, B, A\n\
                 git_revert({\"commits\": [\"C\", \"B\", \"A\"]})\n\n\
                 Why reverse order:\n\
                 - Dependencies handled correctly\n\
                 - Each revert applies to current state\n\
                 - Avoids breaking intermediate state\n\n\
                 Individual commits (default):\n\
                 git_revert({\"commits\": [\"c1\", \"c2\", \"c3\"]})\n\
                 → Creates 3 revert commits\n\n\
                 Combined (single commit):\n\
                 git_revert({\"commits\": [\"c1\", \"c2\", \"c3\"], \"no_commit\": true})\n\
                 git_commit({\"message\": \"Revert feature X\"})\n\
                 → Creates 1 revert commit\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 DO:\n\
                 ✓ Use revert for shared branches\n\
                 ✓ Verify commit hash before reverting\n\
                 ✓ Test after reverting\n\
                 ✓ Push immediately after revert\n\
                 ✓ Document why in commit message\n\
                 ✓ Specify mainline for merge commits\n\
                 ✓ Revert multiple commits in reverse order\n\n\
                 DON'T:\n\
                 ✗ Use reset on shared branches\n\
                 ✗ Revert without verifying commit\n\
                 ✗ Forget mainline for merges\n\
                 ✗ Revert in wrong order\n\
                 ✗ Leave revert unpushed\n\
                 ✗ Skip testing after revert\n\n\
                 =============================================================================\n\
                 HISTORY VISUALIZATION\n\
                 =============================================================================\n\n\
                 Before revert:\n\
                 A - B - C - D (HEAD)\n\n\
                 After reverting C:\n\
                 A - B - C - D - C' (HEAD)\n\
                 Where C' undoes C\n\n\
                 All commits preserved!\n\n\
                 Before revert (merge):\n\
                 main: A - B ------- M (HEAD)\n\
                              \\     /\n\
                 feature:      C - D\n\n\
                 After reverting M:\n\
                 main: A - B - M - M' (HEAD)\n\
                 Where M' undoes merge (C, D changes)\n\n\
                 =============================================================================\n\
                 RESPONSE FORMAT\n\
                 =============================================================================\n\n\
                 Success:\n\
                 {\n\
                   \"success\": true,\n\
                   \"reverted\": \"abc1234\",\n\
                   \"new_commit\": \"rev7890\",\n\
                   \"message\": \"Revert \\\"Original message\\\"\"\n\
                 }\n\n\
                 Conflict:\n\
                 {\n\
                   \"success\": false,\n\
                   \"conflicts\": [\"file1.rs\", \"file2.rs\"],\n\
                   \"state\": \"reverting\"\n\
                 }\n\n\
                 Multiple reverts:\n\
                 {\n\
                   \"success\": true,\n\
                   \"reverts\": [\n\
                     {\"original\": \"abc1234\", \"revert_commit\": \"rev1\"},\n\
                     {\"original\": \"def5678\", \"revert_commit\": \"rev2\"}\n\
                   ]\n\
                 }\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Single commit:\n\
                 git_revert({\"path\": \"/project\", \"commit\": \"abc1234\"})\n\n\
                 Multiple commits (reverse order):\n\
                 git_revert({\"commits\": [\"newest\", \"older\", \"oldest\"]})\n\n\
                 Merge commit:\n\
                 git_revert({\"commit\": \"merge_hash\", \"mainline\": 1})\n\n\
                 Combined revert:\n\
                 git_revert({\"commits\": [\"c1\", \"c2\"], \"no_commit\": true})\n\
                 git_commit({\"message\": \"Revert feature\"})\n\n\
                 Continue after conflict:\n\
                 git_revert({\"continue\": true})\n\n\
                 Abort revert:\n\
                 git_revert({\"abort\": true})\n\n\
                 =============================================================================\n\
                 WHEN TO USE REVERT\n\
                 =============================================================================\n\n\
                 Use revert when:\n\
                 - Commit already pushed to shared branch\n\
                 - Working on main, develop, release branches\n\
                 - Others may have pulled the commit\n\
                 - Need to preserve history\n\
                 - Production hotfix needed\n\
                 - Reverting merged PR\n\
                 - Safe collaboration required\n\n\
                 Don't use revert when:\n\
                 - Local unpushed commits (use reset)\n\
                 - Private feature branch (use reset)\n\
                 - No one else has pulled (use reset)\n\
                 - Want clean history (use rebase/reset locally)\n\n\
                 =============================================================================\n\
                 REMEMBER\n\
                 =============================================================================\n\n\
                 - Revert = Safe undo for shared branches\n\
                 - Creates new commits (no history rewriting)\n\
                 - Preserves all history\n\
                 - Safe for collaboration\n\
                 - Always use mainline for merge commits\n\
                 - Revert multiple commits in reverse order\n\
                 - Test and push immediately after revert\n\n\
                 Revert is your friend for shared branches!",
            ),
        },
    ]
}
