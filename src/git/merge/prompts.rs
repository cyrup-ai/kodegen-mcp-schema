//! Prompt messages for git_merge tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitMergePromptArgs;

/// Prompt provider for git_merge tool
///
/// This is the ONLY way to provide prompts for git_merge - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct MergePrompts;

impl PromptProvider for MergePrompts {
    type PromptArgs = GitMergePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("strategies") => prompt_strategies(),
            Some("conflicts") => prompt_conflicts(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (strategies, conflicts, basic)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO MERGE BRANCHES
// ============================================================================

/// Simple branch merging
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I merge branches using the git_merge tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_merge tool merges one branch into another. Here's how to use it:\n\n\
                 MERGE DIRECTION:\n\
                 - You must be ON the target branch\n\
                 - Branch being merged INTO = current branch\n\
                 - Branch being merged FROM = specified branch\n\n\
                 Example:\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\
                 // Now on main (target branch)\n\
                 git_merge({\"path\": \"/project\", \"branch\": \"feature/user-auth\"})\n\
                 // Merges feature/user-auth INTO main\n\n\
                 BASIC EXAMPLES:\n\n\
                 Simple merge:\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature/add-tests\"})\n\n\
                 With custom message:\n\
                 git_merge({\n\
                     \"path\": \"/repo\",\n\
                     \"branch\": \"feature/api\",\n\
                     \"message\": \"Add REST API endpoints\"\n\
                 })\n\n\
                 Merge remote branch:\n\
                 git_fetch({\"path\": \"/repo\"})\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"origin/feature/x\"})\n\n\
                 RESPONSE (success):\n\
                 {\"merged\": \"feature/user-auth\", \"into\": \"main\", \"type\": \"fast-forward\", \"success\": true}\n\n\
                 RESPONSE (conflict):\n\
                 {\"merged\": \"feature/x\", \"into\": \"main\", \"conflicts\": [\"src/main.rs\"], \"success\": false}\n\n\
                 COMPLETE WORKFLOW:\n\n\
                 Step 1: Check current branch\n\
                 git_status({\"path\": \"/project\"})\n\n\
                 Step 2: Switch to target branch\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\n\
                 Step 3: Merge feature branch\n\
                 git_merge({\"path\": \"/project\", \"branch\": \"feature/login\"})\n\n\
                 Step 4: Verify result\n\
                 git_log({\"path\": \"/project\", \"max_count\": 5})\n\n\
                 MERGE TYPES:\n\n\
                 Fast-forward:\n\
                 - Target branch hasn't diverged\n\
                 - Just moves branch pointer forward\n\
                 - No merge commit created\n\
                 - Linear history\n\n\
                 Three-way merge:\n\
                 - Both branches have diverged\n\
                 - Creates a merge commit\n\
                 - Preserves both branch histories\n\n\
                 PARAMETERS:\n\
                 - path (required): Repository path\n\
                 - branch (required): Branch to merge FROM\n\
                 - message (optional): Custom merge commit message\n\
                 - no_ff (optional): Force merge commit\n\
                 - squash (optional): Combine all commits into one\n\
                 - strategy (optional): ours, theirs, recursive\n\
                 - abort (optional): Abort ongoing merge\n\
                 - continue (optional): Continue after resolving conflicts\n\n\
                 COMMON PATTERNS:\n\n\
                 1. Simple merge:\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\"})\n\n\
                 2. Merge with message:\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\", \"message\": \"Add feature X\"})\n\n\
                 3. Ensure clean working directory:\n\
                 git_status({\"path\": \"/repo\"})\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\"})\n\n\
                 4. Merge remote branch:\n\
                 git_fetch({\"path\": \"/repo\"})\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"origin/feature\"})\n\n\
                 BEFORE MERGING:\n\
                 - Ensure working directory is clean\n\
                 - Be on the correct target branch\n\
                 - Pull latest changes from remote\n\
                 - Consider creating a backup branch\n\n\
                 AFTER MERGING:\n\
                 - Check merge result (success/conflicts)\n\
                 - Resolve conflicts if needed\n\
                 - Test merged code\n\
                 - Push to remote",
            ),
        },
    ]
}

/// Merge strategies and options
fn prompt_strategies() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are the different merge strategies and when should I use them?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Git provides several merge strategies for different scenarios:\n\n\
                 STRATEGY REFERENCE:\n\n\
                 1. Fast-forward (default):\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\"})\n\
                 When it happens: Target branch has no new commits, feature is ahead, no divergence.\n\
                 Result: No merge commit, linear history, pointer moves forward.\n\n\
                 2. No-FF (force merge commit):\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\", \"no_ff\": true})\n\
                 When to use: Preserve feature branch history, clear feature boundaries, Git Flow workflows.\n\
                 Result: Always creates merge commit, preserves branch structure, easy to revert entire feature.\n\n\
                 3. Squash:\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\", \"squash\": true})\n\
                 git_commit({\"path\": \"/repo\", \"message\": \"Add feature X\"})\n\
                 When to use: Messy commit history, want clean main branch, all commits are one logical unit.\n\
                 Result: Combines all commits into one, requires manual commit, cleaner history, loses commit details.\n\n\
                 4. Ours (keep our version):\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\", \"strategy\": \"ours\"})\n\
                 When to use: Record merge but ignore their changes, merging obsolete branch, already applied changes manually.\n\
                 Result: Merge commit created, all conflicts favor current branch, their changes discarded.\n\n\
                 5. Theirs (take their version):\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\", \"strategy\": \"theirs\"})\n\
                 When to use: Other branch has correct version, syncing with upstream, reverting bad changes.\n\
                 Result: Merge commit created, all conflicts favor other branch, our conflicting changes discarded.\n\n\
                 6. Recursive (default for diverged branches):\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\", \"strategy\": \"recursive\"})\n\
                 Automatically chosen by Git for diverged branches, three-way merge, may require manual resolution.\n\n\
                 DECISION TREE:\n\n\
                 Need to preserve feature branch history?\n\
                 → Use no_ff: true\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\", \"no_ff\": true})\n\n\
                 Feature has messy commit history?\n\
                 → Use squash: true\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\", \"squash\": true})\n\
                 git_commit({\"path\": \"/repo\", \"message\": \"Clean feature summary\"})\n\n\
                 Know which version to keep on conflicts?\n\
                 → Use strategy: \"ours\" or \"theirs\"\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\", \"strategy\": \"ours\"})\n\n\
                 Simple update with no expected conflicts?\n\
                 → Use default (fast-forward if possible)\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\"})\n\n\
                 Complex merge with uncertainty?\n\
                 → Test in separate branch first\n\
                 git_branch_create({\"path\": \"/repo\", \"name\": \"test-merge\"})\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"test-merge\"})\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature\"})\n\n\
                 WORKFLOW EXAMPLES:\n\n\
                 Example 1: Feature completion with no-ff\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"main\"})\n\
                 git_pull({\"path\": \"/repo\"})\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature/login\", \"no_ff\": true})\n\
                 git_push({\"path\": \"/repo\"})\n\n\
                 Example 2: Cleanup before merge with squash\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"main\"})\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature/messy\", \"squash\": true})\n\
                 git_commit({\"path\": \"/repo\", \"message\": \"Add feature with clean history\"})\n\n\
                 BEST PRACTICES:\n\
                 - Use no-ff for feature branches to preserve context\n\
                 - Use squash for cleanup before merging to main\n\
                 - Let Git choose strategy (recursive) for most merges",
            ),
        },
    ]
}

/// Handling merge conflicts
fn prompt_conflicts() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I handle merge conflicts when they occur?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Merge conflicts occur when Git cannot automatically resolve differences. Here's how to handle them:\n\n\
                 5-STEP CONFLICT RESOLUTION:\n\n\
                 1. Conflict occurs:\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature/x\"})\n\
                 // Response: conflicts in [\"src/main.rs\"]\n\
                 What happened: Merge paused, files marked with conflict markers, repository in \"merging\" state.\n\n\
                 2. View conflicts:\n\
                 git_status({\"path\": \"/repo\"})\n\
                 // Shows all conflicted files\n\
                 Output: {\"branch\": \"main\", \"merging\": true, \"conflicts\": [{\"path\": \"src/main.rs\", \"type\": \"both_modified\"}]}\n\n\
                 3. Resolve manually:\n\
                 fs_read_file({\"path\": \"/repo/src/main.rs\"})\n\
                 // View conflict markers\n\
                 fs_edit_block({\n\
                     \"path\": \"/repo/src/main.rs\",\n\
                     \"old_string\": \"<<<<<<< HEAD\\nfn old_code() {}\\n=======\\nfn new_code() {}\\n>>>>>>> feature/x\",\n\
                     \"new_string\": \"fn merged_code() {}\"\n\
                 })\n\n\
                 4. Mark resolved:\n\
                 git_add({\"path\": \"/repo\", \"files\": [\"src/main.rs\"]})\n\n\
                 5. Continue or abort:\n\
                 git_merge({\"path\": \"/repo\", \"continue\": true})\n\
                 // OR\n\
                 git_merge({\"path\": \"/repo\", \"abort\": true})\n\n\
                 CONFLICT MARKERS:\n\n\
                 Visual representation:\n\
                 <<<<<<< HEAD\n\
                 your code here\n\
                 =======\n\
                 their code here\n\
                 >>>>>>> feature/x\n\n\
                 Marker breakdown:\n\
                 <<<<<<< HEAD - Start of your changes (current branch)\n\
                 ======= - Separator between versions\n\
                 >>>>>>> feature/x - End of their changes (branch being merged)\n\n\
                 How to identify: Everything between HEAD and ======= is yours, everything between ======= and >>>>>>> is theirs.\n\n\
                 RESOLUTION STRATEGIES:\n\n\
                 Keep yours:\n\
                 fs_edit_block({\"old_string\": \"<<<<<<< HEAD\\nYOUR_CODE\\n=======\\nTHEIR_CODE\\n>>>>>>> branch\", \"new_string\": \"YOUR_CODE\"})\n\n\
                 Keep theirs:\n\
                 fs_edit_block({\"old_string\": \"<<<<<<< HEAD\\nYOUR_CODE\\n=======\\nTHEIR_CODE\\n>>>>>>> branch\", \"new_string\": \"THEIR_CODE\"})\n\n\
                 Keep both:\n\
                 fs_edit_block({\"old_string\": \"<<<<<<< HEAD\\nYOUR_CODE\\n=======\\nTHEIR_CODE\\n>>>>>>> branch\", \"new_string\": \"YOUR_CODE\\nTHEIR_CODE\"})\n\n\
                 Merge manually:\n\
                 fs_edit_block({\"old_string\": \"<<<<<<< HEAD\\nYOUR_CODE\\n=======\\nTHEIR_CODE\\n>>>>>>> branch\", \"new_string\": \"MERGED_CODE\"})\n\n\
                 COMPLETE WORKFLOW:\n\n\
                 Step 1: Attempt merge\n\
                 git_merge({\"path\": \"/repo\", \"branch\": \"feature/api\"})\n\n\
                 Step 2: Check all conflicts\n\
                 git_status({\"path\": \"/repo\"})\n\n\
                 Step 3: For EACH file - read, resolve, stage\n\
                 fs_read_file({\"path\": \"/repo/src/api.rs\"})\n\
                 fs_edit_block({\"path\": \"/repo/src/api.rs\", \"old_string\": \"...\", \"new_string\": \"...\"})\n\
                 git_add({\"path\": \"/repo\", \"files\": [\"src/api.rs\"]})\n\n\
                 Step 4: After ALL files resolved\n\
                 git_merge({\"path\": \"/repo\", \"continue\": true})\n\n\
                 Step 5: Verify final state\n\
                 git_status({\"path\": \"/repo\"})\n\n\
                 MULTIPLE CONFLICTS:\n\n\
                 Resolve each file independently:\n\
                 fs_edit_block({\"path\": \"/repo/file1.rs\", ...})\n\
                 git_add({\"path\": \"/repo\", \"files\": [\"file1.rs\"]})\n\
                 fs_edit_block({\"path\": \"/repo/file2.rs\", ...})\n\
                 git_add({\"path\": \"/repo\", \"files\": [\"file2.rs\"]})\n\n\
                 Stage each resolved file, only continue after ALL files resolved.\n\n\
                 PREVENTING CONFLICTS:\n\
                 - Merge main into feature frequently\n\
                 - Keep branches short-lived\n\
                 - Pull before starting work",
            ),
        },
    ]
}
