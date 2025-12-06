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
            Some("basic") => prompt_basic(),
            Some("strategies") => prompt_strategies(),
            Some("messages") => prompt_messages(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, strategies, messages, workflows)".to_string()),
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
                 ERROR HANDLING:\n\
                 If merge fails, check:\n\
                 - PR has conflicts (resolve first)\n\
                 - Branch protection rules (may require reviews)\n\
                 - CI checks failing (must pass first)\n\
                 - Insufficient permissions (need write access)\n\
                 - PR already merged or closed\n\n\
                 AUTHENTICATION:\n\
                 Requires GitHub token with:\n\
                 - repo scope (for private repos)\n\
                 - public_repo scope (for public repos)\n\n\
                 BEST PRACTICES:\n\
                 - Check PR status before merging\n\
                 - Verify CI checks passed\n\
                 - Ensure required reviews approved\n\
                 - Delete branch after merge\n\
                 - Use appropriate merge strategy\n\
                 - Review changes before merging",
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
                 TEAM CONVENTIONS:\n\
                 Establish team-wide merge strategy:\n\
                 - Feature PRs → SQUASH\n\
                 - Release PRs → MERGE\n\
                 - Hotfix PRs → REBASE\n\
                 - Documentation → SQUASH\n\n\
                 BRANCH PROTECTION:\n\
                 Some strategies may be restricted by:\n\
                 - Branch protection rules\n\
                 - Organization policies\n\
                 - Repository settings\n\
                 Always verify allowed strategies first.\n\n\
                 DEFAULT BEHAVIOR:\n\
                 If merge_method not specified:\n\
                 - Defaults to merge commit\n\
                 - Repository settings determine behavior\n\
                 - Check repo default merge method",
            ),
        },
    ]
}

/// Commit message customization
fn prompt_messages() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I customize commit messages when merging pull requests?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "You can customize the merge commit title and message using commit_title and commit_message parameters.\n\n\
                 COMMIT MESSAGE CUSTOMIZATION:\n\n\
                 1. Custom title only:\n\
                    github_merge_pr({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"pull_number\": 456,\n\
                        \"commit_title\": \"feat: Add dark mode (#456)\"\n\
                    })\n\
                    Uses custom title, default PR description as message\n\n\
                 2. Custom title and message:\n\
                    github_merge_pr({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"pull_number\": 456,\n\
                        \"commit_title\": \"feat: Add dark mode\",\n\
                        \"commit_message\": \"Implements dark mode theme option.\\n\\nCloses #123\\nTested on Chrome, Firefox, Safari\"\n\
                    })\n\
                    Full control over title and message\n\n\
                 3. Squash with custom message:\n\
                    github_merge_pr({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"pull_number\": 456,\n\
                        \"merge_method\": \"squash\",\n\
                        \"commit_title\": \"fix: Resolve authentication bug\",\n\
                        \"commit_message\": \"Fixes token refresh logic.\\n\\n- Updated OAuth flow\\n- Added retry mechanism\\n- Improved error handling\\n\\nFixes #789\"\n\
                    })\n\
                    Squash all commits with descriptive message\n\n\
                 COMMIT MESSAGE PATTERNS:\n\n\
                 CONVENTIONAL COMMITS:\n\
                 Format: <type>: <description>\n\
                 Types: feat, fix, docs, style, refactor, test, chore\n\
                 Examples:\n\
                 - \"feat: Add user authentication\"\n\
                 - \"fix: Resolve memory leak in cache\"\n\
                 - \"docs: Update API documentation\"\n\
                 - \"refactor: Simplify error handling\"\n\n\
                 WITH PR NUMBER:\n\
                 Include PR number for traceability:\n\
                 - \"feat: Add dark mode (#456)\"\n\
                 - \"fix: Resolve login issue (#789)\"\n\
                 - \"docs: Update README (#123)\"\n\n\
                 WITH ISSUE REFERENCES:\n\
                 Link to related issues:\n\
                 - \"Closes #123\"\n\
                 - \"Fixes #456\"\n\
                 - \"Resolves #789\"\n\
                 - \"Related to #101\"\n\n\
                 MULTI-LINE MESSAGES:\n\
                 Use \\n for line breaks:\n\
                 github_merge_pr({\n\
                     \"owner\": \"org\",\n\
                     \"repo\": \"app\",\n\
                     \"pull_number\": 42,\n\
                     \"commit_title\": \"feat: Add payment gateway\",\n\
                     \"commit_message\": \"Implements Stripe integration.\\n\\nFeatures:\\n- Credit card processing\\n- Subscription management\\n- Webhook handling\\n\\nCloses #100, #101, #102\"\n\
                 })\n\n\
                 MESSAGE TEMPLATES:\n\n\
                 Feature addition:\n\
                 {\n\
                     \"commit_title\": \"feat: <short description> (#<pr_number>)\",\n\
                     \"commit_message\": \"<detailed description>\\n\\n<implementation details>\\n\\nCloses #<issue>\"\n\
                 }\n\n\
                 Bug fix:\n\
                 {\n\
                     \"commit_title\": \"fix: <issue description> (#<pr_number>)\",\n\
                     \"commit_message\": \"<problem description>\\n\\n<solution description>\\n\\nFixes #<issue>\"\n\
                 }\n\n\
                 Refactoring:\n\
                 {\n\
                     \"commit_title\": \"refactor: <what was refactored>\",\n\
                     \"commit_message\": \"<why refactoring was needed>\\n\\n<what changed>\\n\\nNo functional changes\"\n\
                 }\n\n\
                 EXAMPLES BY USE CASE:\n\n\
                 1. Feature merge with details:\n\
                    github_merge_pr({\n\
                        \"owner\": \"company\",\n\
                        \"repo\": \"product\",\n\
                        \"pull_number\": 256,\n\
                        \"merge_method\": \"squash\",\n\
                        \"commit_title\": \"feat: Add OAuth2 authentication (#256)\",\n\
                        \"commit_message\": \"Implements OAuth2 authentication flow.\\n\\nSupports providers:\\n- Google\\n- GitHub\\n- Microsoft\\n\\nIncludes:\\n- Login/logout endpoints\\n- Token refresh logic\\n- User profile sync\\n\\nCloses #200\\nTested with all providers\"\n\
                    })\n\n\
                 2. Bug fix with impact:\n\
                    github_merge_pr({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"service\",\n\
                        \"pull_number\": 512,\n\
                        \"merge_method\": \"squash\",\n\
                        \"commit_title\": \"fix: Prevent race condition in cache (#512)\",\n\
                        \"commit_message\": \"Resolves cache corruption under high load.\\n\\nProblem:\\n- Concurrent writes caused data loss\\n- Cache inconsistency on cache miss\\n\\nSolution:\\n- Added mutex locking\\n- Implemented write-ahead log\\n- Added retry mechanism\\n\\nFixes #489\\nPerformance impact: <1ms overhead\"\n\
                    })\n\n\
                 3. Documentation update:\n\
                    github_merge_pr({\n\
                        \"owner\": \"oss\",\n\
                        \"repo\": \"library\",\n\
                        \"pull_number\": 128,\n\
                        \"commit_title\": \"docs: Update installation instructions (#128)\",\n\
                        \"commit_message\": \"Updates README with new installation steps.\\n\\n- Added prerequisites\\n- Updated CLI commands\\n- Added troubleshooting section\\n\\nImproves #115\"\n\
                    })\n\n\
                 WHEN TO CUSTOMIZE:\n\
                 Always customize for:\n\
                 - Squash merges (default message lists all commits)\n\
                 - Release merges (include version info)\n\
                 - Breaking changes (highlight impact)\n\
                 - Security fixes (mark as security)\n\n\
                 DEFAULT BEHAVIOR:\n\
                 If not specified:\n\
                 - commit_title: PR title\n\
                 - commit_message: PR description\n\
                 - For squash: List of all commit messages\n\n\
                 BEST PRACTICES:\n\
                 - Use conventional commit format\n\
                 - Include PR number in title\n\
                 - Reference related issues\n\
                 - Describe why, not just what\n\
                 - Keep title under 72 characters\n\
                 - Use imperative mood (\"Add\" not \"Added\")\n\
                 - Be specific and descriptive\n\
                 - Include breaking change warnings\n\
                 - Add testing notes if relevant\n\
                 - Use proper grammar and spelling",
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
                 3. RELEASE MERGE WORKFLOW:\n\
                    // Step 1: Verify release PR\n\
                    github_get_pr({\n\
                        \"owner\": \"org\",\n\
                        \"repo\": \"app\",\n\
                        \"pull_number\": 1000\n\
                    })\n\
                    // Check: Merging to main, no conflicts\n\
                    \n\
                    // Step 2: Ensure all checks pass\n\
                    github_get_pull_request_status({\n\
                        \"owner\": \"org\",\n\
                        \"repo\": \"app\",\n\
                        \"pull_number\": 1000\n\
                    })\n\
                    \n\
                    // Step 3: Merge with version info\n\
                    github_merge_pr({\n\
                        \"owner\": \"org\",\n\
                        \"repo\": \"app\",\n\
                        \"pull_number\": 1000,\n\
                        \"merge_method\": \"merge\",\n\
                        \"commit_title\": \"Release v2.0.0 (#1000)\",\n\
                        \"commit_message\": \"Release version 2.0.0\\n\\nChangelog:\\n- New payment system\\n- Dark mode\\n- Performance improvements\\n\\nBreaking changes documented in CHANGELOG.md\"\n\
                    })\n\
                    \n\
                    // Step 4: Create release tag\n\
                    github_create_release({\n\
                        \"owner\": \"org\",\n\
                        \"repo\": \"app\",\n\
                        \"tag_name\": \"v2.0.0\",\n\
                        \"name\": \"Version 2.0.0\",\n\
                        \"body\": \"Major release with new features...\"\n\
                    })\n\n\
                 4. HOTFIX MERGE WORKFLOW:\n\
                    // Step 1: Quick verification\n\
                    github_get_pr({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"service\",\n\
                        \"pull_number\": 555\n\
                    })\n\
                    \n\
                    // Step 2: Emergency merge\n\
                    github_merge_pr({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"service\",\n\
                        \"pull_number\": 555,\n\
                        \"merge_method\": \"rebase\",\n\
                        \"commit_title\": \"hotfix: Critical security patch (#555)\",\n\
                        \"commit_message\": \"Fixes authentication bypass vulnerability.\\n\\nSecurity: CVE-2024-XXXX\\nSeverity: Critical\\n\\nImmediate deployment required.\"\n\
                    })\n\
                    \n\
                    // Step 3: Delete branch\n\
                    github_delete_branch({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"service\",\n\
                        \"branch\": \"hotfix/auth-bypass\"\n\
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

/// Comprehensive guide to PR merging
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to merging pull requests with github_merge_pr.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_merge_pr tool merges pull requests with support for multiple strategies and custom commit messages.\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 SIMPLE MERGE:\n\
                 github_merge_pr({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"repository\",\n\
                     \"pull_number\": 123\n\
                 })\n\n\
                 WITH STRATEGY:\n\
                 github_merge_pr({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"repository\",\n\
                     \"pull_number\": 123,\n\
                     \"merge_method\": \"squash\"\n\
                 })\n\n\
                 WITH CUSTOM MESSAGE:\n\
                 github_merge_pr({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"repository\",\n\
                     \"pull_number\": 123,\n\
                     \"merge_method\": \"squash\",\n\
                     \"commit_title\": \"feat: Add new feature (#123)\",\n\
                     \"commit_message\": \"Detailed description of changes.\\n\\nCloses #100\"\n\
                 })\n\n\
                 =============================================================================\n\
                 PARAMETERS\n\
                 =============================================================================\n\n\
                 REQUIRED:\n\
                 - owner: Repository owner (user or organization)\n\
                 - repo: Repository name\n\
                 - pull_number: PR number to merge\n\n\
                 OPTIONAL:\n\
                 - merge_method: \"merge\" (default), \"squash\", or \"rebase\"\n\
                 - commit_title: Custom merge commit title\n\
                 - commit_message: Additional commit message details\n\
                 - sha: Head SHA for verification (prevents stale merges)\n\n\
                 =============================================================================\n\
                 MERGE STRATEGIES\n\
                 =============================================================================\n\n\
                 MERGE COMMIT (\"merge\"):\n\
                 - Creates merge commit\n\
                 - Preserves all individual commits\n\
                 - Maintains full history\n\
                 - Best for: Feature branches, releases\n\n\
                 SQUASH MERGE (\"squash\"):\n\
                 - Combines all commits into one\n\
                 - Creates single commit on base\n\
                 - Simplifies history\n\
                 - Best for: Bug fixes, small features\n\n\
                 REBASE MERGE (\"rebase\"):\n\
                 - Rebases commits onto base\n\
                 - No merge commit\n\
                 - Linear history\n\
                 - Best for: Hotfixes, clean history\n\n\
                 =============================================================================\n\
                 COMMIT MESSAGE CUSTOMIZATION\n\
                 =============================================================================\n\n\
                 CONVENTIONAL COMMITS:\n\
                 Format: <type>: <description>\n\
                 \n\
                 Types:\n\
                 - feat: New feature\n\
                 - fix: Bug fix\n\
                 - docs: Documentation\n\
                 - style: Formatting\n\
                 - refactor: Code restructuring\n\
                 - test: Adding tests\n\
                 - chore: Maintenance\n\n\
                 EXAMPLES:\n\
                 \"feat: Add user authentication (#123)\"\n\
                 \"fix: Resolve memory leak in cache (#456)\"\n\
                 \"docs: Update API documentation (#789)\"\n\n\
                 WITH ISSUE REFERENCES:\n\
                 Include in commit_message:\n\
                 - \"Closes #123\"\n\
                 - \"Fixes #456\"\n\
                 - \"Resolves #789\"\n\
                 - \"Related to #101\"\n\n\
                 =============================================================================\n\
                 COMPLETE WORKFLOWS\n\
                 =============================================================================\n\n\
                 STANDARD MERGE:\n\
                 1. Check PR status:\n\
                    github_get_pr({\"owner\": \"user\", \"repo\": \"repo\", \"pull_number\": 123})\n\
                 2. Verify mergeable=true, CI passed\n\
                 3. Merge:\n\
                    github_merge_pr({\"owner\": \"user\", \"repo\": \"repo\", \"pull_number\": 123})\n\
                 4. Delete branch:\n\
                    github_delete_branch({\"owner\": \"user\", \"repo\": \"repo\", \"branch\": \"feature-x\"})\n\n\
                 PRODUCTION MERGE:\n\
                 1. Get PR: github_get_pr(...)\n\
                 2. Check reviews: github_get_pull_request_reviews(...)\n\
                 3. Check CI: github_get_pull_request_status(...)\n\
                 4. Merge with custom message: github_merge_pr(...)\n\
                 5. Clean up: github_delete_branch(...)\n\n\
                 RELEASE MERGE:\n\
                 1. Verify PR ready\n\
                 2. Merge with version info\n\
                 3. Create release tag\n\
                 4. Keep branch for reference\n\n\
                 =============================================================================\n\
                 RESPONSE FORMAT\n\
                 =============================================================================\n\n\
                 Success response:\n\
                 {\n\
                     \"sha\": \"abc123...\",\n\
                     \"merged\": true,\n\
                     \"message\": \"Pull Request successfully merged\"\n\
                 }\n\n\
                 Fields:\n\
                 - sha: Merge commit SHA\n\
                 - merged: Boolean success indicator\n\
                 - message: Status message\n\n\
                 =============================================================================\n\
                 ERROR HANDLING\n\
                 =============================================================================\n\n\
                 COMMON ERRORS:\n\n\
                 Not mergeable:\n\
                 - Cause: Conflicts, failed CI, no approvals\n\
                 - Fix: Resolve conflicts, fix CI, get reviews\n\n\
                 SHA mismatch:\n\
                 - Cause: PR updated after SHA retrieved\n\
                 - Fix: Get fresh SHA, retry merge\n\n\
                 Permission denied:\n\
                 - Cause: Insufficient token permissions\n\
                 - Fix: Use token with repo scope\n\n\
                 Method not allowed:\n\
                 - Cause: Repository settings disable strategy\n\
                 - Fix: Use allowed merge method\n\n\
                 =============================================================================\n\
                 AUTHENTICATION\n\
                 =============================================================================\n\n\
                 REQUIRED TOKEN SCOPES:\n\
                 - repo: Full repository access (private repos)\n\
                 - public_repo: Public repository access only\n\n\
                 Set via:\n\
                 - GITHUB_TOKEN environment variable\n\
                 - GitHub App installation\n\
                 - OAuth token\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 BEFORE MERGING:\n\
                 - Check PR is open\n\
                 - Verify no conflicts\n\
                 - Confirm CI passed\n\
                 - Ensure reviews approved\n\
                 - Validate branch up to date\n\n\
                 MERGE STRATEGY:\n\
                 - Feature → SQUASH (clean history)\n\
                 - Release → MERGE (preserve commits)\n\
                 - Hotfix → REBASE (linear history)\n\n\
                 COMMIT MESSAGES:\n\
                 - Use conventional commits\n\
                 - Include PR number\n\
                 - Reference issues\n\
                 - Be descriptive\n\
                 - Keep title under 72 chars\n\n\
                 AFTER MERGING:\n\
                 - Delete merged branch\n\
                 - Update related issues\n\
                 - Tag releases\n\
                 - Deploy if automated\n\
                 - Notify team\n\n\
                 SAFETY:\n\
                 - Use sha parameter for verification\n\
                 - Check status before merge\n\
                 - Verify permissions\n\
                 - Test thoroughly first\n\
                 - Follow branch protection\n\n\
                 =============================================================================\n\
                 REPOSITORY SETTINGS\n\
                 =============================================================================\n\n\
                 ENABLE MERGE METHODS:\n\
                 Settings → Pull Requests:\n\
                 - Allow merge commits\n\
                 - Allow squash merging\n\
                 - Allow rebase merging\n\n\
                 AUTO-DELETE BRANCHES:\n\
                 Settings → Pull Requests:\n\
                 - Automatically delete head branches\n\n\
                 BRANCH PROTECTION:\n\
                 Settings → Branches → Protection rules:\n\
                 - Require pull request reviews\n\
                 - Require status checks\n\
                 - Require conversation resolution\n\
                 - Restrict who can merge\n\n\
                 =============================================================================\n\
                 EXAMPLES\n\
                 =============================================================================\n\n\
                 Quick merge:\n\
                 github_merge_pr({\"owner\": \"user\", \"repo\": \"app\", \"pull_number\": 42})\n\n\
                 Squash with message:\n\
                 github_merge_pr({\n\
                     \"owner\": \"org\",\n\
                     \"repo\": \"service\",\n\
                     \"pull_number\": 100,\n\
                     \"merge_method\": \"squash\",\n\
                     \"commit_title\": \"feat: Add OAuth (#100)\",\n\
                     \"commit_message\": \"Complete OAuth implementation.\\n\\nCloses #95\"\n\
                 })\n\n\
                 Rebase hotfix:\n\
                 github_merge_pr({\n\
                     \"owner\": \"company\",\n\
                     \"repo\": \"api\",\n\
                     \"pull_number\": 999,\n\
                     \"merge_method\": \"rebase\",\n\
                     \"commit_title\": \"hotfix: Security patch (#999)\"\n\
                 })\n\n\
                 Remember: Always verify PR status, use appropriate merge strategy, customize messages, and clean up branches!",
            ),
        },
    ]
}
