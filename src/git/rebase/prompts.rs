//! Prompt messages for git_rebase tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitRebasePromptArgs;

/// Prompt provider for git_rebase tool
///
/// This is the ONLY way to provide prompts for git_rebase - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct RebasePrompts;

impl PromptProvider for RebasePrompts {
    type PromptArgs = GitRebasePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("onto") => prompt_onto(),
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
                description: Some("Scenario to show (basic, onto, conflicts, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO REBASE BRANCHES
// ============================================================================

/// Basic rebasing operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I rebase branches using the git_rebase tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_rebase tool reapplies commits on top of another base. Here's how to use it for basic rebasing:\n\n\
                 REBASING BRANCHES:\n\n\
                 1. Rebase onto main:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature/x\"})\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"upstream\": \"main\"\n\
                    })\n\
                    // Your commits now on top of main\n\n\
                 2. Rebase onto origin/main:\n\
                    git_fetch({\"path\": \"/project\"})\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"upstream\": \"origin/main\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"upstream\": \"main\",\n\
                   \"commits_rebased\": 5,\n\
                   \"success\": true\n\
                 }\n\n\
                 WHAT REBASE DOES:\n\
                 1. Finds common ancestor between branches\n\
                 2. Temporarily removes your commits\n\
                 3. Applies upstream commits to current branch\n\
                 4. Replays your commits on top, one by one\n\
                 5. Each commit gets a new hash\n\n\
                 RESULT:\n\
                 - Linear history without merge commits\n\
                 - Your commits have new SHA hashes\n\
                 - Branch appears to start from new base\n\
                 - Clean, straight-line history\n\n\
                 REBASE DIRECTION:\n\
                 - You must be ON the branch to rebase\n\
                 - Upstream = the base you want to rebase ONTO\n\
                 - Current branch's commits are replayed\n\n\
                 EXAMPLE WORKFLOW:\n\
                 Step 1: Check current branch\n\
                 git_status({\"path\": \"/project\"})\n\
                 // Ensure you're on feature branch\n\n\
                 Step 2: Fetch latest changes\n\
                 git_fetch({\"path\": \"/project\"})\n\
                 // Get latest from remote\n\n\
                 Step 3: Rebase onto main\n\
                 git_rebase({\"path\": \"/project\", \"upstream\": \"origin/main\"})\n\
                 // Replay your commits on top of main\n\n\
                 Step 4: Verify result\n\
                 git_log({\"path\": \"/project\", \"max_count\": 10})\n\
                 // See linear history\n\n\
                 PARAMETERS:\n\
                 - path (required): Repository path\n\
                 - upstream (required): Branch/commit to rebase onto\n\
                 - onto (optional): Different commit to place commits onto\n\
                 - interactive (optional): Interactive rebase mode\n\
                 - continue (optional): Continue after resolving conflicts\n\
                 - skip (optional): Skip current commit during rebase\n\
                 - abort (optional): Abort rebase and return to original state\n\n\
                 COMMON PATTERNS:\n\
                 1. Update feature with latest main:\n\
                    git_checkout({\"path\": \"/repo\", \"branch\": \"feature/api\"})\n\
                    git_fetch({\"path\": \"/repo\"})\n\
                    git_rebase({\"path\": \"/repo\", \"upstream\": \"origin/main\"})\n\n\
                 2. Rebase before PR:\n\
                    git_fetch({\"path\": \"/repo\"})\n\
                    git_rebase({\"path\": \"/repo\", \"upstream\": \"origin/main\"})\n\
                    // Clean up history before merging\n\n\
                 3. Keep feature current:\n\
                    // Regular maintenance\n\
                    git_fetch({\"path\": \"/repo\"})\n\
                    git_rebase({\"path\": \"/repo\", \"upstream\": \"origin/main\"})\n\
                    // Stay up to date with team\n\n\
                 REBASE vs MERGE:\n\
                 Rebase:\n\
                 - Creates linear history\n\
                 - Rewrites commit hashes\n\
                 - Cleaner git log\n\
                 - Use for local feature branches\n\n\
                 Merge:\n\
                 - Preserves branch history\n\
                 - Keeps original commit hashes\n\
                 - Shows true development flow\n\
                 - Use for shared branches\n\n\
                 GOLDEN RULE:\n\
                 Never rebase commits that have been pushed\n\
                 and shared with others!\n\
                 \n\
                 Why? Because rebase changes commit hashes,\n\
                 causing conflicts for anyone who based work\n\
                 on the original commits.\n\n\
                 SAFE REBASE SCENARIOS:\n\
                 - Local feature branch not yet pushed\n\
                 - Feature branch only you work on\n\
                 - Before creating pull request\n\
                 - With explicit team agreement\n\n\
                 BEFORE REBASING:\n\
                 - Ensure working directory is clean\n\
                 - Commit or stash uncommitted changes\n\
                 - Fetch latest changes from remote\n\
                 - Be on correct branch\n\
                 - Verify branch hasn't been shared\n\n\
                 AFTER REBASING:\n\
                 - Check rebase result (success/conflicts)\n\
                 - Test rebased code\n\
                 - Force push to remote: git_push with force_with_lease\n\
                 - Never use regular push after rebase",
            ),
        },
    ]
}

/// Rebasing onto specific commits
fn prompt_onto() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use the onto option for advanced rebasing?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The onto option allows you to rebase commits onto a different base than the upstream. Here's the complete guide:\n\n\
                 REBASE ONTO:\n\n\
                 1. Rebase onto specific commit:\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"upstream\": \"abc1234\",\n\
                        \"onto\": \"def5678\"\n\
                    })\n\
                    // Commits after abc1234 placed onto def5678\n\n\
                 2. Move commits between branches:\n\
                    // Move feature from old-base to new-base\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"upstream\": \"old-base\",\n\
                        \"onto\": \"new-base\"\n\
                    })\n\
                    // Changes base of current branch\n\n\
                 3. Transplant feature branch:\n\
                    // Originally branched from develop\n\
                    // Want to base on main instead\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature/x\"})\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"upstream\": \"develop\",\n\
                        \"onto\": \"main\"\n\
                    })\n\
                    // Now feature/x branches from main\n\n\
                 UNDERSTANDING ONTO:\n\
                 \n\
                 Without onto:\n\
                 git_rebase({\"upstream\": \"main\"})\n\
                 - Finds common ancestor with main\n\
                 - Replays commits after ancestor\n\
                 - Places them on top of main\n\
                 \n\
                 With onto:\n\
                 git_rebase({\"upstream\": \"old-base\", \"onto\": \"new-base\"})\n\
                 - Finds commits after old-base\n\
                 - Replays those commits\n\
                 - Places them on top of new-base instead\n\n\
                 ONTO USE CASES:\n\n\
                 1. Change base branch:\n\
                    // Feature was based on develop\n\
                    // Now needs to be based on main\n\
                    git_checkout({\"path\": \"/repo\", \"branch\": \"feature\"})\n\
                    git_rebase({\n\
                        \"path\": \"/repo\",\n\
                        \"upstream\": \"develop\",\n\
                        \"onto\": \"main\"\n\
                    })\n\n\
                 2. Move range of commits:\n\
                    // Extract commits from feature-a\n\
                    // Place onto feature-b instead\n\
                    git_rebase({\n\
                        \"path\": \"/repo\",\n\
                        \"upstream\": \"feature-a\",\n\
                        \"onto\": \"feature-b\"\n\
                    })\n\n\
                 3. Reorganize branch structure:\n\
                    // Refactor where features branch from\n\
                    git_rebase({\n\
                        \"path\": \"/repo\",\n\
                        \"upstream\": \"old-parent\",\n\
                        \"onto\": \"new-parent\"\n\
                    })\n\n\
                 4. Skip commits in middle:\n\
                    // Want commits after X but not commits between Y and X\n\
                    git_rebase({\n\
                        \"path\": \"/repo\",\n\
                        \"upstream\": \"commit-X\",\n\
                        \"onto\": \"commit-before-Y\"\n\
                    })\n\n\
                 VISUAL EXAMPLE:\n\
                 \n\
                 Before:\n\
                 main:    A---B---C\n\
                              \\\n\
                 develop:      D---E\n\
                                   \\\n\
                 feature:           F---G---H\n\
                 \n\
                 Command:\n\
                 git_checkout feature\n\
                 git_rebase({\"upstream\": \"develop\", \"onto\": \"main\"})\n\
                 \n\
                 After:\n\
                 main:    A---B---C\n\
                              |    \\\n\
                 develop:     |     D---E\n\
                              |\n\
                              F'--G'--H' (feature)\n\
                 \n\
                 Result: F-G-H are now based on main instead of develop\n\n\
                 COMMIT HASH SYNTAX:\n\
                 You can use:\n\
                 - Branch names: \"main\", \"develop\"\n\
                 - Commit hashes: \"abc1234\", \"def5678\"\n\
                 - Relative refs: \"HEAD~3\", \"main~2\"\n\
                 - Tags: \"v1.0.0\"\n\n\
                 PRACTICAL SCENARIOS:\n\n\
                 Scenario 1: Wrong base branch\n\
                 Problem: Created feature from wrong branch\n\
                 Solution:\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"feature\"})\n\
                 git_rebase({\n\
                     \"path\": \"/repo\",\n\
                     \"upstream\": \"wrong-branch\",\n\
                     \"onto\": \"correct-branch\"\n\
                 })\n\n\
                 Scenario 2: Extract commits\n\
                 Problem: Some commits belong on different branch\n\
                 Solution:\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"source-branch\"})\n\
                 git_rebase({\n\
                     \"path\": \"/repo\",\n\
                     \"upstream\": \"source-branch~5\",\n\
                     \"onto\": \"target-branch\"\n\
                 })\n\
                 // Last 5 commits move to target-branch\n\n\
                 Scenario 3: Rebase after squash\n\
                 Problem: Upstream was squashed, can't find common ancestor\n\
                 Solution:\n\
                 git_rebase({\n\
                     \"path\": \"/repo\",\n\
                     \"upstream\": \"old-tip\",\n\
                     \"onto\": \"new-squashed-commit\"\n\
                 })\n\n\
                 COMPLETE WORKFLOW:\n\
                 \n\
                 Step 1: Identify current base\n\
                 git_log({\"path\": \"/repo\", \"max_count\": 20})\n\
                 // Find where feature branches from\n\
                 \n\
                 Step 2: Identify target base\n\
                 // Decide where you want it to branch from\n\
                 \n\
                 Step 3: Checkout feature branch\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"feature\"})\n\
                 \n\
                 Step 4: Rebase onto new base\n\
                 git_rebase({\n\
                     \"path\": \"/repo\",\n\
                     \"upstream\": \"current-base\",\n\
                     \"onto\": \"new-base\"\n\
                 })\n\
                 \n\
                 Step 5: Verify result\n\
                 git_log({\"path\": \"/repo\", \"max_count\": 20})\n\
                 // Confirm new base\n\n\
                 ADVANCED USAGE:\n\
                 \n\
                 Cherry-pick range onto different base:\n\
                 git_rebase({\n\
                     \"path\": \"/repo\",\n\
                     \"upstream\": \"start-commit\",\n\
                     \"onto\": \"target-branch\"\n\
                 })\n\
                 // All commits after start-commit move to target-branch\n\
                 \n\
                 Rebase subset of commits:\n\
                 git_rebase({\n\
                     \"path\": \"/repo\",\n\
                     \"upstream\": \"feature~5\",\n\
                     \"onto\": \"main\"\n\
                 })\n\
                 // Last 5 commits from feature go onto main\n\n\
                 TIPS:\n\
                 - Use git_log to visualize branch structure first\n\
                 - Test with backup branch before rebasing\n\
                 - Verify common ancestor before rebasing\n\
                 - Use onto when changing branch parentage\n\
                 - Remember: onto is where commits GO, upstream is where they COME FROM",
            ),
        },
    ]
}

/// Handling rebase conflicts
fn prompt_conflicts() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I handle conflicts during a rebase?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Rebase conflicts occur when replaying commits creates conflicts. Here's how to handle them:\n\n\
                 HANDLING REBASE CONFLICTS:\n\n\
                 1. Conflict occurs:\n\
                    git_rebase({\"path\": \"/project\", \"upstream\": \"main\"})\n\
                    // Response: conflict during rebase at commit abc1234\n\n\
                    WHAT HAPPENED:\n\
                    - Rebase paused at conflicting commit\n\
                    - Conflict markers added to files\n\
                    - Repository in \"rebasing\" state\n\
                    - Must resolve before continuing\n\n\
                 2. View rebase status:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Shows current rebase state and conflicts\n\n\
                    OUTPUT:\n\
                    {\n\
                      \"branch\": \"feature/x\",\n\
                      \"rebasing\": true,\n\
                      \"rebasing_onto\": \"main\",\n\
                      \"conflicts\": [\n\
                        {\n\
                          \"path\": \"src/main.rs\",\n\
                          \"type\": \"both_modified\"\n\
                        }\n\
                      ]\n\
                    }\n\n\
                 3. Resolve conflicts:\n\
                    // Read conflicted file\n\
                    fs_read_file({\"path\": \"/project/src/main.rs\"})\n\
                    // Fix conflicts (remove markers, choose code)\n\
                    fs_edit_block({\n\
                        \"path\": \"/project/src/main.rs\",\n\
                        \"old_string\": \"<<<<<<< HEAD\\nold code\\n=======\\nnew code\\n>>>>>>>\",\n\
                        \"new_string\": \"resolved code\"\n\
                    })\n\n\
                 4. Stage and continue:\n\
                    git_add({\"path\": \"/project\", \"files\": [\"src/main.rs\"]})\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"continue\": true\n\
                    })\n\
                    // Rebase continues with next commit\n\n\
                    WHAT HAPPENS:\n\
                    - Git applies the resolved commit\n\
                    - Moves to next commit in sequence\n\
                    - May stop again if another conflict\n\
                    - Repeats until all commits replayed\n\n\
                 5. Skip problematic commit:\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"skip\": true\n\
                    })\n\
                    // Skips current commit entirely\n\n\
                    WHEN TO SKIP:\n\
                    - Commit no longer needed\n\
                    - Changes already applied in upstream\n\
                    - Empty commit after resolution\n\
                    - Duplicate of existing commit\n\n\
                 6. Abort rebase:\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"abort\": true\n\
                    })\n\
                    // Returns to state before rebase started\n\n\
                    WHEN TO ABORT:\n\
                    - Too many conflicts\n\
                    - Wrong upstream chosen\n\
                    - Need to prepare better\n\
                    - Want to try different approach\n\n\
                 CONFLICT WORKFLOW:\n\
                 \n\
                 1. Rebase starts, hits conflict\n\
                 2. Git stops at conflicting commit\n\
                 3. You resolve conflicts in files\n\
                 4. Stage resolved files with git_add\n\
                 5. Continue rebase with continue: true\n\
                 6. If another conflict, repeat steps 3-5\n\
                 7. If no more conflicts, rebase completes\n\n\
                 CONFLICT MARKERS:\n\
                 <<<<<<< HEAD (current base)\n\
                 code from upstream branch\n\
                 =======\n\
                 code from your commit being applied\n\
                 >>>>>>> commit-message\n\
                 \n\
                 During rebase:\n\
                 - HEAD = the new base you're rebasing onto\n\
                 - Below ======= = your commit's changes\n\n\
                 COMPLETE EXAMPLE:\n\
                 \n\
                 Step 1: Start rebase\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"feature\"})\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"main\"})\n\
                 // Conflict in src/api.rs at commit #2 of 5\n\
                 \n\
                 Step 2: Check what's happening\n\
                 git_status({\"path\": \"/repo\"})\n\
                 // Shows rebasing state, which commit, conflicts\n\
                 \n\
                 Step 3: View conflict\n\
                 fs_read_file({\"path\": \"/repo/src/api.rs\"})\n\
                 // See conflict markers\n\
                 \n\
                 Step 4: Resolve conflict\n\
                 fs_edit_block({\n\
                     \"path\": \"/repo/src/api.rs\",\n\
                     \"old_string\": \"<<<<<<< HEAD\\nupstream code\\n=======\\nyour code\\n>>>>>>>\",\n\
                     \"new_string\": \"merged code\"\n\
                 })\n\
                 \n\
                 Step 5: Stage resolution\n\
                 git_add({\"path\": \"/repo\", \"files\": [\"src/api.rs\"]})\n\
                 \n\
                 Step 6: Continue rebase\n\
                 git_rebase({\"path\": \"/repo\", \"continue\": true})\n\
                 // Applies commit #2, moves to commit #3\n\
                 \n\
                 Step 7: Another conflict at commit #4\n\
                 // Repeat steps 2-6 for each conflict\n\
                 \n\
                 Step 8: All commits applied\n\
                 git_status({\"path\": \"/repo\"})\n\
                 // Shows clean state, rebase complete\n\n\
                 MULTIPLE CONFLICTS IN ONE COMMIT:\n\
                 \n\
                 If one commit conflicts in multiple files:\n\
                 1. git_status shows all conflicted files\n\
                 2. Resolve each file\n\
                 3. Stage ALL resolved files together\n\
                 4. Then continue rebase\n\
                 \n\
                 Example:\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"main\"})\n\
                 // Conflict in file1.rs, file2.rs, file3.rs\n\
                 \n\
                 fs_edit_block({\"path\": \"/repo/file1.rs\", ...})\n\
                 fs_edit_block({\"path\": \"/repo/file2.rs\", ...})\n\
                 fs_edit_block({\"path\": \"/repo/file3.rs\", ...})\n\
                 \n\
                 git_add({\"path\": \"/repo\", \"files\": [\"file1.rs\", \"file2.rs\", \"file3.rs\"]})\n\
                 git_rebase({\"path\": \"/repo\", \"continue\": true})\n\n\
                 HANDLING MULTIPLE COMMITS:\n\
                 \n\
                 Rebase replays commits one at a time.\n\
                 Each commit can conflict independently.\n\
                 \n\
                 Timeline:\n\
                 Start: 5 commits to rebase\n\
                 Commit 1: Applied successfully\n\
                 Commit 2: Conflict! Stop and resolve\n\
                 [You resolve, stage, continue]\n\
                 Commit 3: Applied successfully\n\
                 Commit 4: Conflict! Stop and resolve\n\
                 [You resolve, stage, continue]\n\
                 Commit 5: Applied successfully\n\
                 Done: All commits rebased\n\n\
                 SKIP vs ABORT vs CONTINUE:\n\
                 \n\
                 continue: Keep going after fixing conflicts\n\
                 - Use when: Fixed conflicts, ready for next commit\n\
                 - Effect: Applies current commit, moves forward\n\
                 \n\
                 skip: Skip current commit entirely\n\
                 - Use when: Commit not needed or already applied\n\
                 - Effect: Discards current commit, moves to next\n\
                 \n\
                 abort: Cancel entire rebase\n\
                 - Use when: Want to start over or give up\n\
                 - Effect: Returns to state before rebase started\n\n\
                 PREVENTING CONFLICTS:\n\
                 - Rebase small changes frequently\n\
                 - Keep feature branches short-lived\n\
                 - Sync with main regularly\n\
                 - Understand what changed in upstream\n\
                 - Communicate with team about large changes\n\n\
                 TIPS:\n\
                 - Read conflict carefully - understand both sides\n\
                 - Test after each resolution\n\
                 - Can abort and try different approach\n\
                 - Use git_log to see commit being applied\n\
                 - Check git_status frequently during rebase\n\
                 - Document complex resolutions in commit message",
            ),
        },
    ]
}

/// Complete rebase workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are the common rebase workflows I should use?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Here are complete workflows for common rebasing scenarios:\n\n\
                 REBASE WORKFLOWS:\n\n\
                 1. Update feature branch with latest main:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature/x\"})\n\
                    git_fetch({\"path\": \"/project\"})\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"upstream\": \"origin/main\"\n\
                    })\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"force_with_lease\": true\n\
                    })\n\n\
                    WHY THIS WORKFLOW:\n\
                    - Keeps feature current with main\n\
                    - Creates linear history\n\
                    - Reduces merge conflicts later\n\
                    - Makes PR review easier\n\n\
                    WHEN TO USE:\n\
                    - Before creating pull request\n\
                    - Regular feature branch maintenance\n\
                    - When main has important updates\n\
                    - To ensure feature works with latest code\n\n\
                    CRITICAL: Use force_with_lease, never regular push!\n\n\
                 2. Clean up before pull request:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature/x\"})\n\
                    git_fetch({\"path\": \"/project\"})\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"upstream\": \"origin/main\"\n\
                    })\n\
                    // Now PR will merge cleanly into main\n\n\
                    WHY THIS WORKFLOW:\n\
                    - Ensures clean merge into main\n\
                    - Shows how feature integrates\n\
                    - Prevents merge conflicts in PR\n\
                    - Makes CI tests accurate\n\n\
                    WHEN TO USE:\n\
                    - Right before opening PR\n\
                    - When PR has merge conflicts\n\
                    - After main has moved forward\n\
                    - To pass CI checks\n\n\
                 3. Keep long-running feature current:\n\
                    // Weekly or as needed\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature/big\"})\n\
                    git_fetch({\"path\": \"/project\"})\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"upstream\": \"origin/main\"\n\
                    })\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"force_with_lease\": true\n\
                    })\n\
                    // Feature stays in sync with team\n\n\
                    WHY THIS WORKFLOW:\n\
                    - Prevents massive conflicts later\n\
                    - Keeps feature compatible with main\n\
                    - Easier to merge when done\n\
                    - Catches integration issues early\n\n\
                    WHEN TO USE:\n\
                    - Long-running feature branches\n\
                    - Active main branch with many changes\n\
                    - Regular maintenance schedule\n\
                    - Before major feature work\n\n\
                 4. Rebase after upstream changes:\n\
                    // Main was force-pushed or rebased\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature/x\"})\n\
                    git_fetch({\"path\": \"/project\"})\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"upstream\": \"origin/main\"\n\
                    })\n\
                    // Realigns with new main history\n\n\
                    WHY THIS WORKFLOW:\n\
                    - Adapts to upstream rewrite\n\
                    - Fixes diverged history\n\
                    - Re-establishes common ancestor\n\
                    - Resolves \"diverged branches\" error\n\n\
                 5. Interactive rebase for cleanup:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature/x\"})\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"upstream\": \"main\",\n\
                        \"interactive\": true\n\
                    })\n\
                    // Clean up commit history\n\
                    // Squash, reorder, edit commits\n\n\
                    WHY THIS WORKFLOW:\n\
                    - Clean up messy commits\n\
                    - Squash \"WIP\" commits\n\
                    - Reorder logical changes\n\
                    - Perfect history before PR\n\n\
                    WHEN TO USE:\n\
                    - Before opening PR\n\
                    - Many small commits to consolidate\n\
                    - Commits out of logical order\n\
                    - Want clean git history\n\n\
                 6. Conflict resolution workflow:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature/x\"})\n\
                    git_fetch({\"path\": \"/project\"})\n\
                    git_rebase({\"path\": \"/project\", \"upstream\": \"origin/main\"})\n\
                    // Conflict detected\n\
                    \n\
                    git_status({\"path\": \"/project\"})\n\
                    // View conflicted files\n\
                    \n\
                    fs_read_file({\"path\": \"/project/src/file.rs\"})\n\
                    fs_edit_block({...})  // Fix conflicts\n\
                    git_add({\"path\": \"/project\", \"files\": [\"src/file.rs\"]})\n\
                    \n\
                    git_rebase({\"path\": \"/project\", \"continue\": true})\n\
                    // Repeat until done\n\
                    \n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"force_with_lease\": true\n\
                    })\n\n\
                 7. Safe rebase with backup:\n\
                    // Create backup first\n\
                    git_branch_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/x-backup\"\n\
                    })\n\
                    \n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature/x\"})\n\
                    git_rebase({\"path\": \"/project\", \"upstream\": \"main\"})\n\
                    \n\
                    // If successful, delete backup\n\
                    git_branch_delete({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"feature/x-backup\"\n\
                    })\n\
                    \n\
                    // If failed, restore from backup\n\
                    git_rebase({\"path\": \"/project\", \"abort\": true})\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature/x-backup\"})\n\n\
                    WHY THIS WORKFLOW:\n\
                    - Safety net for complex rebases\n\
                    - Can easily undo if problems\n\
                    - Learn rebasing without risk\n\
                    - Good for important branches\n\n\
                 8. Rebase onto different branch:\n\
                    // Move feature from develop to main\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"feature/x\"})\n\
                    git_rebase({\n\
                        \"path\": \"/project\",\n\
                        \"upstream\": \"develop\",\n\
                        \"onto\": \"main\"\n\
                    })\n\
                    // Feature now branches from main\n\n\
                    WHY THIS WORKFLOW:\n\
                    - Change base branch\n\
                    - Reorganize branch structure\n\
                    - Adapt to workflow changes\n\
                    - Target different release\n\n\
                 REBASE vs MERGE DECISION:\n\
                 \n\
                 Use REBASE when:\n\
                 - Working on local feature branch\n\
                 - Want linear history\n\
                 - Preparing for PR\n\
                 - Syncing with upstream\n\
                 - Branch not shared with others\n\
                 \n\
                 Use MERGE when:\n\
                 - Integrating completed features\n\
                 - Working on shared branch\n\
                 - Want to preserve history\n\
                 - Following Git Flow\n\
                 - Branch already pushed and public\n\n\
                 GOLDEN RULES:\n\
                 \n\
                 1. Never rebase public/shared branches\n\
                    - Causes problems for collaborators\n\
                    - Rewrites shared history\n\
                    - Creates divergent branches\n\
                 \n\
                 2. Always use force_with_lease after rebase\n\
                    - Safer than force push\n\
                    - Checks for upstream changes\n\
                    - Prevents accidental overwrites\n\
                 \n\
                 3. Fetch before rebasing\n\
                    - Get latest upstream changes\n\
                    - Prevents unnecessary conflicts\n\
                    - Ensures accurate rebase\n\
                 \n\
                 4. Test after rebasing\n\
                    - Run tests to verify\n\
                    - Check functionality\n\
                    - Ensure nothing broke\n\n\
                 BEST PRACTICES:\n\
                 - Rebase frequently on long features\n\
                 - Use interactive rebase to clean history\n\
                 - Create backup branch for complex rebases\n\
                 - Communicate with team about rebases\n\
                 - Force push with --force-with-lease only\n\
                 - Test thoroughly after rebase\n\
                 - Resolve conflicts carefully\n\
                 - Keep commits atomic and logical\n\n\
                 COMMON MISTAKES TO AVOID:\n\
                 - Rebasing main or master branch\n\
                 - Regular push after rebase (use force_with_lease)\n\
                 - Rebasing shared team branches\n\
                 - Not fetching before rebase\n\
                 - Skipping tests after rebase\n\
                 - Losing work by not backing up",
            ),
        },
    ]
}

/// Comprehensive guide covering all rebase operations
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using the git_rebase tool effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_rebase tool reapplies commits on top of another base. Here's your complete guide:\n\n\
                 BASIC USAGE:\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"main\"})\n\n\
                 PARAMETERS:\n\
                 - path (required): Repository path\n\
                 - upstream (required): Branch/commit to rebase onto\n\
                 - onto (optional): Different commit to place commits onto\n\
                 - interactive (optional): Interactive rebase mode\n\
                 - continue (optional): Continue after resolving conflicts\n\
                 - skip (optional): Skip current commit during rebase\n\
                 - abort (optional): Abort rebase and return to original state\n\n\
                 =============================================================================\n\
                 WHAT IS REBASING?\n\
                 =============================================================================\n\n\
                 Rebasing rewrites history by replaying commits:\n\
                 1. Find common ancestor between branches\n\
                 2. Temporarily save your commits\n\
                 3. Update current branch to upstream\n\
                 4. Replay your commits one by one\n\
                 5. Each commit gets new hash\n\n\
                 Result: Linear history without merge commits\n\n\
                 Before rebase:\n\
                 main:    A---B---C\n\
                           \\\n\
                 feature:   D---E---F\n\
                 \n\
                 After rebase onto main:\n\
                 main:    A---B---C\n\
                                   \\\n\
                 feature:           D'--E'--F'\n\
                 \n\
                 Note: D, E, F have new hashes (D', E', F')\n\n\
                 =============================================================================\n\
                 BASIC REBASING\n\
                 =============================================================================\n\n\
                 Simple rebase:\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"feature\"})\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"main\"})\n\
                 // Replays feature commits on top of main\n\n\
                 Rebase onto remote:\n\
                 git_fetch({\"path\": \"/repo\"})\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"origin/main\"})\n\
                 // Updates with remote main\n\n\
                 =============================================================================\n\
                 ADVANCED: ONTO OPTION\n\
                 =============================================================================\n\n\
                 Change base branch:\n\
                 git_rebase({\n\
                     \"path\": \"/repo\",\n\
                     \"upstream\": \"old-base\",\n\
                     \"onto\": \"new-base\"\n\
                 })\n\
                 // Moves commits from old-base to new-base\n\n\
                 Use cases:\n\
                 - Transplant feature to different branch\n\
                 - Change branch parentage\n\
                 - Extract range of commits\n\
                 - Reorganize branch structure\n\n\
                 Example: Move feature from develop to main\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"feature\"})\n\
                 git_rebase({\n\
                     \"path\": \"/repo\",\n\
                     \"upstream\": \"develop\",\n\
                     \"onto\": \"main\"\n\
                 })\n\n\
                 =============================================================================\n\
                 CONFLICT RESOLUTION\n\
                 =============================================================================\n\n\
                 When conflicts occur during rebase:\n\
                 \n\
                 Step 1: Rebase stops at conflict\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"main\"})\n\
                 // Conflict in file.rs\n\
                 \n\
                 Step 2: Check status\n\
                 git_status({\"path\": \"/repo\"})\n\
                 // Shows rebasing state and conflicts\n\
                 \n\
                 Step 3: Resolve conflicts\n\
                 fs_read_file({\"path\": \"/repo/file.rs\"})\n\
                 // View conflict markers\n\
                 fs_edit_block({...})  // Fix conflicts\n\
                 \n\
                 Step 4: Stage resolved files\n\
                 git_add({\"path\": \"/repo\", \"files\": [\"file.rs\"]})\n\
                 \n\
                 Step 5: Continue rebase\n\
                 git_rebase({\"path\": \"/repo\", \"continue\": true})\n\
                 // Applies commit and moves to next\n\
                 \n\
                 Alternative: Skip commit\n\
                 git_rebase({\"path\": \"/repo\", \"skip\": true})\n\
                 // Discards current commit\n\
                 \n\
                 Alternative: Abort rebase\n\
                 git_rebase({\"path\": \"/repo\", \"abort\": true})\n\
                 // Returns to state before rebase\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 Update feature before PR:\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"feature\"})\n\
                 git_fetch({\"path\": \"/repo\"})\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"origin/main\"})\n\
                 git_push({\"path\": \"/repo\", \"force_with_lease\": true})\n\n\
                 Keep feature current:\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"feature\"})\n\
                 git_fetch({\"path\": \"/repo\"})\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"origin/main\"})\n\n\
                 Safe rebase with backup:\n\
                 git_branch_create({\"path\": \"/repo\", \"name\": \"feature-backup\"})\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"feature\"})\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"main\"})\n\
                 // Delete backup if successful\n\n\
                 =============================================================================\n\
                 REBASE vs MERGE\n\
                 =============================================================================\n\n\
                 REBASE:\n\
                 Pros:\n\
                 - Linear history\n\
                 - Cleaner git log\n\
                 - No merge commits\n\
                 - Easier to understand\n\
                 \n\
                 Cons:\n\
                 - Rewrites history\n\
                 - Changes commit hashes\n\
                 - Can't rebase public branches\n\
                 - More conflict resolution\n\
                 \n\
                 Use for:\n\
                 - Local feature branches\n\
                 - Before opening PR\n\
                 - Syncing with upstream\n\
                 - Cleaning up history\n\n\
                 MERGE:\n\
                 Pros:\n\
                 - Preserves history\n\
                 - Safe for public branches\n\
                 - Shows true development flow\n\
                 - One-time conflict resolution\n\
                 \n\
                 Cons:\n\
                 - Creates merge commits\n\
                 - More complex history\n\
                 - Harder to read git log\n\
                 \n\
                 Use for:\n\
                 - Integrating features\n\
                 - Shared/public branches\n\
                 - Preserving context\n\
                 - Following Git Flow\n\n\
                 =============================================================================\n\
                 GOLDEN RULES\n\
                 =============================================================================\n\n\
                 1. NEVER REBASE PUBLIC/SHARED BRANCHES\n\
                 - Don't rebase main, master, develop\n\
                 - Don't rebase pushed shared branches\n\
                 - Only rebase local feature branches\n\
                 - Get team agreement before rebasing shared work\n\
                 \n\
                 Why? Rebase changes commit hashes, breaking\n\
                 anyone who based work on original commits.\n\n\
                 2. ALWAYS USE FORCE_WITH_LEASE AFTER REBASE\n\
                 git_push({\"path\": \"/repo\", \"force_with_lease\": true})\n\
                 \n\
                 Never regular push after rebase:\n\
                 - Will fail (diverged branches)\n\
                 - History has been rewritten\n\
                 - Hashes changed\n\
                 \n\
                 force_with_lease:\n\
                 - Safer than force\n\
                 - Checks remote hasn't changed\n\
                 - Prevents accidental overwrites\n\n\
                 3. FETCH BEFORE REBASING\n\
                 git_fetch({\"path\": \"/repo\"})\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"origin/main\"})\n\
                 \n\
                 Why?\n\
                 - Get latest upstream changes\n\
                 - Rebase onto correct target\n\
                 - Avoid unnecessary conflicts\n\n\
                 4. TEST AFTER REBASING\n\
                 - Run tests\n\
                 - Check functionality\n\
                 - Verify nothing broke\n\
                 - Confirm integration works\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 Before rebasing:\n\
                 - Commit or stash uncommitted changes\n\
                 - Fetch latest from remote\n\
                 - Verify branch is local/not shared\n\
                 - Consider creating backup branch\n\n\
                 During rebase:\n\
                 - Resolve conflicts carefully\n\
                 - Test each resolution\n\
                 - Use skip for obsolete commits\n\
                 - Abort if too complex\n\n\
                 After rebasing:\n\
                 - Run full test suite\n\
                 - Verify functionality\n\
                 - Force push with --force-with-lease\n\
                 - Communicate rebase to team\n\n\
                 General:\n\
                 - Rebase frequently to avoid large conflicts\n\
                 - Keep feature branches short-lived\n\
                 - Use interactive rebase to clean history\n\
                 - Document complex rebases\n\
                 - Coordinate with team on shared branches\n\n\
                 =============================================================================\n\
                 TROUBLESHOOTING\n\
                 =============================================================================\n\n\
                 \"Diverged branches\" after rebase:\n\
                 - Expected! History was rewritten\n\
                 - Use force_with_lease to push\n\
                 - Never use regular push\n\n\
                 Too many conflicts:\n\
                 - Abort rebase\n\
                 - Sync more frequently next time\n\
                 - Consider using merge instead\n\
                 - Break into smaller rebases\n\n\
                 Lost commits after rebase:\n\
                 - Check git reflog\n\
                 - Commits not lost, just detached\n\
                 - Can recover from reflog\n\n\
                 Rebase of shared branch:\n\
                 - Communicate immediately\n\
                 - Team needs to reset their branches\n\
                 - Consider reverting and using merge\n\
                 - Learn: never again!\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Basic rebase:\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"main\"})\n\n\
                 With onto:\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"old\", \"onto\": \"new\"})\n\n\
                 Continue after conflict:\n\
                 git_rebase({\"path\": \"/repo\", \"continue\": true})\n\n\
                 Skip commit:\n\
                 git_rebase({\"path\": \"/repo\", \"skip\": true})\n\n\
                 Abort:\n\
                 git_rebase({\"path\": \"/repo\", \"abort\": true})\n\n\
                 Complete workflow:\n\
                 git_fetch({\"path\": \"/repo\"})\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"feature\"})\n\
                 git_rebase({\"path\": \"/repo\", \"upstream\": \"origin/main\"})\n\
                 git_push({\"path\": \"/repo\", \"force_with_lease\": true})\n\n\
                 Remember: Rebase rewrites history. Use on local branches only. Always force_with_lease after rebasing. Test thoroughly!",
            ),
        },
    ]
}
