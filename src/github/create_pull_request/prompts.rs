//! Prompt messages for github_create_pull_request tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::CreatePullRequestPromptArgs;

/// Prompt provider for create_pull_request tool
///
/// This is the ONLY way to provide prompts for create_pull_request - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct CreatePullRequestPrompts;

impl PromptProvider for CreatePullRequestPrompts {
    type PromptArgs = CreatePullRequestPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("options") => prompt_options(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, options, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO CREATE PULL REQUESTS
// ============================================================================

/// Basic PR creation examples
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create basic pull requests using github_create_pull_request?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_create_pull_request tool creates pull requests on GitHub to merge changes from one branch into another.\n\n\
                 CREATING PULL REQUESTS:\n\n\
                 1. Simple PR:\n\
                    github_create_pull_request({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"title\": \"Add user authentication\",\n\
                        \"head\": \"feature/auth\",\n\
                        \"base\": \"main\"\n\
                    })\n\n\
                 2. PR with description:\n\
                    github_create_pull_request({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"title\": \"Fix login bug\",\n\
                        \"head\": \"fix/login\",\n\
                        \"base\": \"main\",\n\
                        \"body\": \"Fixes #123\\n\\nThis PR fixes the login timeout issue by increasing the session duration.\"\n\
                    })\n\n\
                 3. Multiple commits PR:\n\
                    github_create_pull_request({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"title\": \"Update dependencies\",\n\
                        \"head\": \"chore/deps\",\n\
                        \"base\": \"develop\",\n\
                        \"body\": \"Updates all dependencies to latest versions.\\n\\n- Updated React to 18.2\\n- Updated TypeScript to 5.0\\n- Fixed breaking changes\"\n\
                    })\n\n\
                 RESPONSE FORMAT:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"pr_number\": 456,\n\
                   \"html_url\": \"https://github.com/user/project/pull/456\",\n\
                   \"message\": \"Pull request created successfully\"\n\
                 }\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner (username or organization)\n\
                 - repo: Repository name\n\
                 - title: PR title (clear and descriptive)\n\
                 - head: Source branch containing your changes\n\
                 - base: Target branch to merge into\n\n\
                 UNDERSTANDING BRANCHES:\n\
                 - head: Your feature branch (where changes are)\n\
                 - base: Target branch (usually \"main\" or \"develop\")\n\
                 - Changes flow FROM head TO base\n\
                 - Example: head=\"feature/login\" base=\"main\" means merge feature/login into main\n\n\
                 COMMON BASE BRANCHES:\n\
                 - \"main\" or \"master\": Production branch\n\
                 - \"develop\" or \"dev\": Development branch\n\
                 - \"staging\": Staging environment\n\
                 - Release branches: \"release/v1.0\", \"release/2.0\"\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with:\n\
                 - repo scope (for private repositories)\n\
                 - public_repo scope (for public repositories)\n\n\
                 ERROR HANDLING:\n\
                 1. 404 Not Found:\n\
                    - Repository doesn't exist or no access\n\
                    - Branch doesn't exist\n\
                    - Verify owner, repo, head, and base names\n\n\
                 2. 422 Unprocessable:\n\
                    - No commits between branches\n\
                    - PR already exists for this branch\n\
                    - Ensure head branch has new commits\n\n\
                 3. 403 Forbidden:\n\
                    - Token lacks permissions\n\
                    - No write access to repository\n\
                    - Generate token with repo scope\n\n\
                 BEST PRACTICES:\n\
                 - Clear, descriptive titles (50-72 characters)\n\
                 - Use conventional commit prefixes: feat:, fix:, docs:, chore:\n\
                 - Ensure branch is pushed to remote before creating PR\n\
                 - Check that head branch has commits not in base\n\
                 - Use meaningful branch names that reflect changes",
            ),
        },
    ]
}

/// PR options and advanced features
fn prompt_options() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What options and features are available when creating pull requests?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Pull requests support various options for different workflows and requirements.\n\n\
                 PR OPTIONS:\n\n\
                 1. Draft PR (work in progress):\n\
                    github_create_pull_request({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"title\": \"WIP: Implementing new feature\",\n\
                        \"head\": \"feature/wip\",\n\
                        \"base\": \"main\",\n\
                        \"draft\": true\n\
                    })\n\n\
                 2. Cross-repository PR (from fork):\n\
                    github_create_pull_request({\n\
                        \"owner\": \"upstream-org\",\n\
                        \"repo\": \"project\",\n\
                        \"title\": \"Add feature X\",\n\
                        \"head\": \"myusername:feature/x\",\n\
                        \"base\": \"main\",\n\
                        \"body\": \"Contributing this feature from my fork.\",\n\
                        \"maintainer_can_modify\": true\n\
                    })\n\n\
                 3. PR with maintainer modifications disabled:\n\
                    github_create_pull_request({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"title\": \"Security patch\",\n\
                        \"head\": \"security/fix\",\n\
                        \"base\": \"main\",\n\
                        \"maintainer_can_modify\": false\n\
                    })\n\n\
                 DRAFT PULL REQUESTS:\n\
                 Benefits of draft PRs:\n\
                 - Show work in progress\n\
                 - Get early feedback\n\
                 - Prevent accidental merge\n\
                 - Run CI tests while developing\n\n\
                 When to use draft:\n\
                 - Code not yet ready for review\n\
                 - Seeking design feedback\n\
                 - Running CI to test changes\n\
                 - Want to show progress\n\n\
                 Converting to ready:\n\
                 - Use github_update_pull_request to change draft: false\n\
                 - Or use GitHub UI \"Ready for review\" button\n\n\
                 CROSS-REPOSITORY PRs (FORKS):\n\
                 Head branch format: \"username:branch\"\n\
                 - username: Fork owner\n\
                 - branch: Branch name in fork\n\n\
                 Fork workflow:\n\
                 1. Fork repository to your account\n\
                 2. Clone your fork\n\
                 3. Create feature branch\n\
                 4. Push to your fork\n\
                 5. Create PR with head=\"youruser:feature\"\n\n\
                 Note: Owner/repo should be upstream repository, not your fork.\n\n\
                 MAINTAINER MODIFICATIONS:\n\
                 maintainer_can_modify parameter:\n\
                 - true (default): Maintainers can push to PR branch\n\
                 - false: Only PR author can modify\n\n\
                 When to disable:\n\
                 - Security sensitive changes\n\
                 - Automated bot PRs\n\
                 - Corporate policy requires it\n\n\
                 When to enable (default):\n\
                 - Open source contributions\n\
                 - Want maintainer help\n\
                 - Collaborative development\n\n\
                 PARAMETER REFERENCE:\n\
                 - owner (required): Repository owner\n\
                 - repo (required): Repository name\n\
                 - title (required): PR title\n\
                 - head (required): Source branch (or username:branch for forks)\n\
                 - base (required): Target branch\n\
                 - body (optional): PR description (Markdown)\n\
                 - draft (optional): Create as draft (default: false)\n\
                 - maintainer_can_modify (optional): Allow maintainer edits (default: true)\n\n\
                 BEST PRACTICES:\n\
                 - Use draft for WIP\n\
                 - Enable maintainer_can_modify for open source\n\
                 - Add detailed description\n\
                 - Link related issues\n\
                 - Request reviews after creation",
            ),
        },
    ]
}

/// Complete PR workflows and patterns
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows for creating pull requests?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Pull requests are part of larger development workflows. Here are complete patterns.\n\n\
                 FEATURE BRANCH WORKFLOW:\n\
                 1. Push changes to remote:\n\
                    git_push({\n\
                        \"path\": \"/project\",\n\
                        \"branch\": \"feature/new-login\",\n\
                        \"set_upstream\": true\n\
                    })\n\
                 2. Create PR:\n\
                    github_create_pull_request({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"title\": \"feat: Add OAuth login support\",\n\
                        \"head\": \"feature/new-login\",\n\
                        \"base\": \"main\",\n\
                        \"body\": \"## Summary\\nAdds OAuth authentication.\\n\\n## Changes\\n- Implemented OAuth flow\\n- Added tests\"\n\
                    })\n\
                 3. Request reviewers and add labels\n\
                 4. Address feedback\n\
                 5. Merge when approved\n\n\
                 FORK CONTRIBUTION WORKFLOW:\n\
                 1. Fork repository (via GitHub UI)\n\
                 2. Clone your fork\n\
                 3. Create feature branch\n\
                 4. Make changes and push to your fork\n\
                 5. Create PR to upstream:\n\
                    github_create_pull_request({\n\
                        \"owner\": \"original-org\",\n\
                        \"repo\": \"project\",\n\
                        \"title\": \"fix: Correct typo in documentation\",\n\
                        \"head\": \"myusername:fix/typo\",\n\
                        \"base\": \"main\",\n\
                        \"body\": \"Fixed typo in README.md\",\n\
                        \"maintainer_can_modify\": true\n\
                    })\n\n\
                 PRE-CREATION CHECKLIST:\n\
                 Before creating PR:\n\
                 - [ ] Branch is pushed to remote\n\
                 - [ ] All tests pass locally\n\
                 - [ ] No merge conflicts with base\n\
                 - [ ] Commits are clean and logical\n\n\
                 POST-CREATION STEPS:\n\
                 After creating PR:\n\
                 1. Request reviewers\n\
                 2. Add labels\n\
                 3. Monitor CI checks\n\
                 4. Address review comments\n\
                 5. Merge when approved\n\n\
                 WORKFLOW PATTERNS:\n\n\
                 GitFlow:\n\
                 - main: Production\n\
                 - develop: Integration\n\
                 - feature/* → develop\n\
                 - release/* → main\n\
                 - hotfix/* → main + develop\n\n\
                 GitHub Flow:\n\
                 - main: Always deployable\n\
                 - feature/* → main\n\
                 - Deploy after merge\n\n\
                 Trunk-Based:\n\
                 - main: Single branch\n\
                 - Short-lived feature branches\n\
                 - Frequent integration\n\n\
                 BEST PRACTICES:\n\
                 - Keep PRs small and focused\n\
                 - One feature/fix per PR\n\
                 - Link all related issues\n\
                 - Respond to reviews promptly\n\
                 - Keep branch up to date",
            ),
        },
    ]
}
