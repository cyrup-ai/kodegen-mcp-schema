//! Prompt messages for github_merge_pr tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::MergePullRequestPromptArgs;

/// Prompt provider for merge_pull_request tool
///
/// This is the ONLY way to provide prompts for merge_pull_request - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct MergePullRequestPrompts;

impl PromptProvider for MergePullRequestPrompts {
    type PromptArgs = MergePullRequestPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("strategies") => prompt_strategies(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, strategies, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO MERGE PULL REQUESTS
// ============================================================================

/// Basic PR merging operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I merge pull requests using github_merge_pr?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_merge_pr tool merges pull requests on GitHub. Here's how to use it for basic merging:\n\n\
                 BASIC PR MERGING:\n\
                 1. Simple merge:\n\
                    github_merge_pr({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"pull_number\": 456\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"sha\": \"abc1234567890def...\",\n\
                   \"merged\": true,\n\
                   \"message\": \"Pull Request successfully merged\"\n\
                 }\n\n\
                 2. Check before merge:\n\
                    // First verify PR is ready\n\
                    github_get_pr({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"pull_number\": 456\n\
                    })\n\
                    // Check response:\n\
                    // - mergeable: true (no conflicts)\n\
                    // - state: \"open\"\n\
                    // Then merge\n\
                    github_merge_pr({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"pull_number\": 456\n\
                    })\n\n\
                 3. Merge with verification:\n\
                    // Get PR details first\n\
                    github_get_pr({\"owner\": \"org\", \"repo\": \"repo\", \"pull_number\": 123})\n\
                    // Verify from response:\n\
                    // - mergeable: true\n\
                    // - mergeable_state: \"clean\" or \"unstable\"\n\
                    // - All CI checks passed\n\
                    // Then merge\n\
                    github_merge_pr({\"owner\": \"org\", \"repo\": \"repo\", \"pull_number\": 123})\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner (username or organization)\n\
                 - repo: Repository name\n\
                 - pull_number: PR number to merge\n\n\
                 RESPONSE FIELDS:\n\
                 - sha: Merge commit SHA\n\
                 - merged: Boolean indicating success\n\
                 - message: Status message\n\n\
                 COMMON PATTERNS:\n\
                 1. Quick merge:\n\
                    github_merge_pr({\"owner\": \"myorg\", \"repo\": \"api\", \"pull_number\": 42})\n\
                 2. Verify then merge:\n\
                    github_get_pr({...})  // Check status\n\
                    github_merge_pr({...})  // Merge if ready\n\
                 3. Merge multiple PRs:\n\
                    github_merge_pr({\"owner\": \"org\", \"repo\": \"repo\", \"pull_number\": 10})\n\
                    github_merge_pr({\"owner\": \"org\", \"repo\": \"repo\", \"pull_number\": 11})\n\
                    github_merge_pr({\"owner\": \"org\", \"repo\": \"repo\", \"pull_number\": 12})\n\n\
                 BEFORE MERGING:\n\
                 Always verify:\n\
                 - PR is in \"open\" state\n\
                 - No merge conflicts (mergeable: true)\n\
                 - CI checks passed\n\
                 - Required reviews approved\n\
                 - Branch is up to date with base\n\n\
                 Remember: Always verify status, check CI, confirm approvals, and clean up branches!",
            ),
        },
    ]
}

/// Merge strategies (merge, squash, rebase)
fn prompt_strategies() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What merge strategies are available and when should I use each one?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "GitHub supports three merge strategies: merge commit, squash merge, and rebase merge. Each has different use cases.\n\n\
                 MERGE STRATEGIES:\n\n\
                 1. MERGE COMMIT (default):\n\
                    github_merge_pr({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"pull_number\": 456,\n\
                        \"merge_method\": \"merge\"\n\
                    })\n\
                    Creates a merge commit preserving all individual commits.\n\
                    Result: Full commit history maintained\n\n\
                 2. SQUASH MERGE:\n\
                    github_merge_pr({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"pull_number\": 456,\n\
                        \"merge_method\": \"squash\"\n\
                    })\n\
                    Combines all PR commits into a single commit.\n\
                    Result: Clean, single commit on base branch\n\n\
                 3. REBASE MERGE:\n\
                    github_merge_pr({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"pull_number\": 456,\n\
                        \"merge_method\": \"rebase\"\n\
                    })\n\
                    Rebases PR commits onto base branch.\n\
                    Result: Linear history without merge commit\n\n\
                 CHOOSING THE RIGHT STRATEGY:\n\n\
                 Use MERGE when:\n\
                 - Preserving full commit history is important\n\
                 - Multiple authors contributed to PR\n\
                 - Individual commits are meaningful\n\
                 - Working on feature branches\n\
                 - Need to maintain commit signatures\n\
                 Example: Large features with well-structured commits\n\n\
                 Use SQUASH when:\n\
                 - Want clean, simple history\n\
                 - PR has many small/WIP commits\n\
                 - Individual commits aren't meaningful\n\
                 - One logical change per PR\n\
                 - Simplifying changelog generation\n\
                 Example: Bug fixes, small features, cleanup work\n\n\
                 Use REBASE when:\n\
                 - Want linear history\n\
                 - Each commit stands alone\n\
                 - No merge commits desired\n\
                 - Working with well-structured commits\n\
                 - Maintaining clean git log\n\
                 Example: Well-organized feature work, hotfixes\n\n\
                 STRATEGY COMPARISON:\n\n\
                 Feature Branch (3 commits):\n\
                 - commit A: \"Add feature\"\n\
                 - commit B: \"Fix typo\"\n\
                 - commit C: \"Update tests\"\n\n\
                 After MERGE:\n\
                 - All 3 commits preserved\n\
                 - Plus 1 merge commit\n\
                 - Total: 4 commits in history\n\n\
                 After SQUASH:\n\
                 - 1 combined commit with all changes\n\
                 - Total: 1 commit in history\n\n\
                 After REBASE:\n\
                 - All 3 commits preserved\n\
                 - No merge commit\n\
                 - Total: 3 commits in history\n\n\
                 REPOSITORY SETTINGS:\n\
                 Check repository allows your chosen strategy:\n\
                 - Repository settings → Pull Requests\n\
                 - Enable: \"Allow merge commits\"\n\
                 - Enable: \"Allow squash merging\"\n\
                 - Enable: \"Allow rebase merging\"\n\n\
                 EXAMPLES BY WORKFLOW:\n\n\
                 1. Feature development (SQUASH):\n\
                    github_merge_pr({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"app\",\n\
                        \"pull_number\": 123,\n\
                        \"merge_method\": \"squash\"\n\
                    })\n\
                    Clean single commit for feature\n\n\
                 2. Release branch (MERGE):\n\
                    github_merge_pr({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"app\",\n\
                        \"pull_number\": 456,\n\
                        \"merge_method\": \"merge\"\n\
                    })\n\
                    Preserve all commits for audit trail\n\n\
                 3. Hotfix (REBASE):\n\
                    github_merge_pr({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"app\",\n\
                        \"pull_number\": 789,\n\
                        \"merge_method\": \"rebase\"\n\
                    })\n\
                    Clean linear history for quick fix\n\n\
",
            ),
        },
    ]
}

/// Complete merge workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are complete workflows for merging pull requests safely?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Complete merge workflows involve checking PR status, verifying requirements, merging, and cleanup.\n\n\
                 COMPLETE MERGE WORKFLOWS:\n\n\
                 1. BASIC SAFE MERGE:\n\
                    // Step 1: Check PR status\n\
                    github_get_pr({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"pull_number\": 456\n\
                    })\n\
                    // Verify from response:\n\
                    // - state: \"open\"\n\
                    // - mergeable: true\n\
                    // - mergeable_state: \"clean\"\n\
                    \n\
                    // Step 2: Merge PR\n\
                    github_merge_pr({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"pull_number\": 456,\n\
                        \"merge_method\": \"squash\"\n\
                    })\n\
                    \n\
                    // Step 3: Delete branch (optional)\n\
                    github_delete_branch({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"branch\": \"feature/dark-mode\"\n\
                    })\n\n\
                 2. PRODUCTION MERGE WITH VERIFICATION:\n\
                    // Step 1: Get PR details\n\
                    github_get_pr({\n\
                        \"owner\": \"company\",\n\
                        \"repo\": \"api\",\n\
                        \"pull_number\": 789\n\
                    })\n\
                    // Check: state=open, mergeable=true\n\
                    \n\
                    // Step 2: Check reviews\n\
                    github_get_pull_request_reviews({\n\
                        \"owner\": \"company\",\n\
                        \"repo\": \"api\",\n\
                        \"pull_number\": 789\n\
                    })\n\
                    // Verify: At least 2 approvals\n\
                    \n\
                    // Step 3: Check CI status\n\
                    github_get_pull_request_status({\n\
                        \"owner\": \"company\",\n\
                        \"repo\": \"api\",\n\
                        \"pull_number\": 789\n\
                    })\n\
                    // Verify: All checks passed\n\
                    \n\
                    // Step 4: Merge with custom message\n\
                    github_merge_pr({\n\
                        \"owner\": \"company\",\n\
                        \"repo\": \"api\",\n\
                        \"pull_number\": 789,\n\
                        \"merge_method\": \"squash\",\n\
                        \"commit_title\": \"feat: Add payment API (#789)\",\n\
                        \"commit_message\": \"Production-ready payment integration.\\n\\nTested and approved.\\n\\nCloses #750\"\n\
                    })\n\
                    \n\
                    // Step 5: Clean up branch\n\
                    github_delete_branch({\n\
                        \"owner\": \"company\",\n\
                        \"repo\": \"api\",\n\
                        \"branch\": \"feature/payment-api\"\n\
                    })\n\n\
WORKFLOW DECISION TREE:\n\n\
                 Feature PR:\n\
                 1. Check mergeable\n\
                 2. Verify CI passed\n\
                 3. Confirm reviews approved\n\
                 4. Squash merge\n\
                 5. Delete branch\n\n\
                 Release PR:\n\
                 1. Verify all features included\n\
                 2. Check all tests pass\n\
                 3. Confirm changelog updated\n\
                 4. Merge commit (preserve history)\n\
                 5. Create release tag\n\
                 6. Keep branch for reference\n\n\
                 Hotfix PR:\n\
                 1. Quick status check\n\
                 2. Verify fix works\n\
                 3. Rebase merge (clean history)\n\
                 4. Delete branch\n\
                 5. Deploy immediately\n\n\
                 BRANCH CLEANUP:\n\
                 Always delete merged branches:\n\
                 github_delete_branch({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"branch\": \"feature/branch-name\"\n\
                 })\n\
                 Exception: Keep release branches\n\n\
                 AUTO-DELETE CONFIGURATION:\n\
                 Enable in repository settings:\n\
                 - Settings → Pull Requests\n\
                 - Check \"Automatically delete head branches\"\n\
                 - Branches auto-deleted after merge\n\n\
                 ERROR HANDLING IN WORKFLOWS:\n\n\
                 If github_get_pr shows not mergeable:\n\
                 1. Check for conflicts\n\
                 2. Update branch with base\n\
                 3. Re-run CI checks\n\
                 4. Try merge again\n\n\
                 If CI checks failing:\n\
                 1. Review check results\n\
                 2. Fix issues in new commit\n\
                 3. Push to PR branch\n\
                 4. Wait for checks\n\
                 5. Merge when green\n\n\
                 If merge fails:\n\
                 1. Check error message\n\
                 2. Verify permissions\n\
                 3. Check branch protection\n\
                 4. Resolve blocking issues\n\
                 5. Retry merge\n\n\
                 BEST PRACTICES:\n\
                 - Always check status before merging\n\
                 - Verify CI passes\n\
                 - Confirm required reviews\n\
                 - Use appropriate merge strategy\n\
                 - Customize commit messages\n\
                 - Delete merged branches\n\
                 - Document merge in commit\n\
                 - Follow team conventions\n\
                 - Test before merging to main\n\
                 - Keep merge commits clean\n\
                 - Use automation where possible\n\
                 - Monitor for conflicts\n\
                 - Communicate with team\n\
                 - Update related issues\n\
                 - Tag releases appropriately",
            ),
        },
    ]
}


