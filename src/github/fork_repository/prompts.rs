//! Prompt messages for github_fork_repository tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitHubForkRepositoryPromptArgs;

/// Prompt provider for fork_repository tool
///
/// This is the ONLY way to provide prompts for fork_repository - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ForkRepositoryPrompts;

impl PromptProvider for ForkRepositoryPrompts {
    type PromptArgs = GitHubForkRepositoryPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("organization") => prompt_organization(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, organization, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO FORK REPOSITORIES
// ============================================================================

/// Basic forking operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I fork a repository to my personal account?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_fork_repository tool creates a personal copy of any repository, preserving full history and enabling independent development.\n\n\
                 FORKING REPOSITORIES:\n\n\
                 1. Fork to your account:\n\
                    github_fork_repository({\n\
                        \"owner\": \"original-owner\",\n\
                        \"repo\": \"cool-project\"\n\
                    })\n\n\
                 2. Fork with custom name:\n\
                    github_fork_repository({\n\
                        \"owner\": \"original-owner\",\n\
                        \"repo\": \"project\",\n\
                        \"name\": \"my-custom-project\"\n\
                    })\n\n\
                 3. Fork only default branch (faster):\n\
                    github_fork_repository({\n\
                        \"owner\": \"original-owner\",\n\
                        \"repo\": \"large-project\",\n\
                        \"default_branch_only\": true\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"full_name\": \"your-user/cool-project\",\n\
                   \"html_url\": \"https://github.com/your-user/cool-project\",\n\
                   \"clone_url\": \"https://github.com/your-user/cool-project.git\",\n\
                   \"parent\": {\n\
                     \"full_name\": \"original-owner/cool-project\",\n\
                     \"html_url\": \"https://github.com/original-owner/cool-project\"\n\
                   },\n\
                   \"default_branch\": \"main\",\n\
                   \"created_at\": \"2025-12-05T10:30:00Z\"\n\
                 }\n\n\
                 WHAT FORK CREATES:\n\
                 - Complete copy in your account\n\
                 - All branches and history (unless default_branch_only: true)\n\
                 - Link to original repository (parent)\n\
                 - Independent commit history going forward\n\
                 - Ready for contributions via pull requests\n\
                 - Maintains fork network relationship\n\n\
                 PARAMETERS:\n\
                 - owner (required): Original repository owner\n\
                 - repo (required): Original repository name\n\
                 - name (optional): Custom name for your fork (default: same as original)\n\
                 - default_branch_only (optional): Fork only default branch (default: false)\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo: For forking private repositories\n\
                 - public_repo: For forking public repositories\n\n\
                 COMMON USE CASES:\n\
                 1. Contributing to open source:\n\
                    Fork repository to propose changes via pull request\n\
                 2. Experimentation:\n\
                    Fork to safely test modifications without affecting original\n\
                 3. Personal customization:\n\
                    Fork to maintain your own version with custom features\n\
                 4. Learning:\n\
                    Fork to study code and make experimental changes\n\n\
                 FORK BEHAVIOR:\n\
                 - Cannot fork your own repositories (use branches instead)\n\
                 - Cannot create duplicate forks (only one fork per account)\n\
                 - Forks inherit original repository's license\n\
                 - Fork network tracks relationship to original\n\
                 - Forks can be kept in sync with upstream\n\n\
                 AFTER FORKING:\n\
                 1. Clone your fork:\n\
                    git_clone({\n\
                        \"url\": \"https://github.com/your-user/cool-project.git\",\n\
                        \"path\": \"/local/path\"\n\
                    })\n\
                 2. Add upstream remote:\n\
                    git remote add upstream https://github.com/original-owner/cool-project.git\n\
                 3. Create feature branch:\n\
                    git checkout -b feature/my-contribution\n\
                 4. Make changes, commit, and push to your fork\n\
                 5. Create pull request from your fork to original\n\n\
                 ERROR HANDLING:\n\
                 - 404 Not Found: Repository doesn't exist or not accessible\n\
                 - 403 Forbidden: Cannot fork own repository\n\
                 - 422 Unprocessable: Fork already exists in your account\n\
                 - Rate limiting: Check X-RateLimit-Remaining header\n\n\
                 BEST PRACTICES:\n\
                 - Fork only repositories you intend to contribute to\n\
                 - Use meaningful custom names if forking multiple versions\n\
                 - Configure upstream remote after cloning\n\
                 - Keep fork synced with upstream regularly\n\
                 - Delete fork when no longer needed\n\
                 - Use default_branch_only for large repositories",
            ),
        },
    ]
}

/// Fork to organization with custom naming
fn prompt_organization() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I fork a repository to an organization instead of my personal account?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "You can fork repositories to organizations you're a member of, enabling team collaboration on forked projects.\n\n\
                 FORK TO ORGANIZATION:\n\n\
                 1. Fork to org (same name):\n\
                    github_fork_repository({\n\
                        \"owner\": \"original-owner\",\n\
                        \"repo\": \"project\",\n\
                        \"organization\": \"my-org\"\n\
                    })\n\n\
                 2. Fork to org with custom name:\n\
                    github_fork_repository({\n\
                        \"owner\": \"original-owner\",\n\
                        \"repo\": \"project\",\n\
                        \"organization\": \"my-org\",\n\
                        \"name\": \"custom-name\"\n\
                    })\n\n\
                 3. Fork only default branch to org:\n\
                    github_fork_repository({\n\
                        \"owner\": \"original-owner\",\n\
                        \"repo\": \"large-project\",\n\
                        \"organization\": \"my-company\",\n\
                        \"default_branch_only\": true\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"full_name\": \"my-org/project\",\n\
                   \"html_url\": \"https://github.com/my-org/project\",\n\
                   \"clone_url\": \"https://github.com/my-org/project.git\",\n\
                   \"owner\": {\n\
                     \"login\": \"my-org\",\n\
                     \"type\": \"Organization\"\n\
                   },\n\
                   \"parent\": {\n\
                     \"full_name\": \"original-owner/project\"\n\
                   },\n\
                   \"permissions\": {\n\
                     \"admin\": true,\n\
                     \"push\": true,\n\
                     \"pull\": true\n\
                   }\n\
                 }\n\n\
                 ORGANIZATION FORK REQUIREMENTS:\n\
                 - You must be a member of the organization\n\
                 - Your organization membership must include fork creation permissions\n\
                 - Organization must have available repository slots (for paid plans)\n\
                 - Organization settings must allow forking\n\n\
                 ORG FORK USE CASES:\n\n\
                 1. Company needs fork:\n\
                    Fork open source project to organization for internal customization\n\
                    github_fork_repository({\n\
                        \"owner\": \"nodejs\",\n\
                        \"repo\": \"node\",\n\
                        \"organization\": \"acme-corp\",\n\
                        \"name\": \"acme-node\"\n\
                    })\n\n\
                 2. Team maintenance:\n\
                    Fork to organization for team collaboration on contributions\n\
                    github_fork_repository({\n\
                        \"owner\": \"kubernetes\",\n\
                        \"repo\": \"kubernetes\",\n\
                        \"organization\": \"devops-team\"\n\
                    })\n\n\
                 3. Organization contributions:\n\
                    Fork to organization to make contributions under org name\n\
                    github_fork_repository({\n\
                        \"owner\": \"apache\",\n\
                        \"repo\": \"kafka\",\n\
                        \"organization\": \"data-team\",\n\
                        \"name\": \"kafka-fork\"\n\
                    })\n\n\
                 4. Template for multiple projects:\n\
                    Fork template to organization for reuse across projects\n\
                    github_fork_repository({\n\
                        \"owner\": \"template-org\",\n\
                        \"repo\": \"api-template\",\n\
                        \"organization\": \"my-company\",\n\
                        \"name\": \"standard-api-template\"\n\
                    })\n\n\
                 ORGANIZATION VS PERSONAL FORK:\n\n\
                 Personal Fork:\n\
                 - Only you have direct access\n\
                 - Good for individual contributions\n\
                 - Simpler permission management\n\
                 - Counts against your repository limit\n\n\
                 Organization Fork:\n\
                 - Multiple team members can collaborate\n\
                 - Good for team contributions\n\
                 - Complex permission management via teams\n\
                 - Counts against organization's repository limit\n\
                 - Better for long-term maintenance\n\n\
                 PERMISSIONS:\n\
                 When forking to organization:\n\
                 - You inherit organization's default permissions\n\
                 - Organization owners have full access\n\
                 - Team permissions apply as configured\n\
                 - Can configure branch protection rules\n\
                 - Can manage collaborators via organization teams\n\n\
                 TEAM COLLABORATION WORKFLOW:\n\
                 1. Fork to organization:\n\
                    github_fork_repository({\"owner\": \"upstream\", \"repo\": \"project\", \"organization\": \"team\"})\n\
                 2. Team members clone org fork:\n\
                    git clone https://github.com/team/project.git\n\
                 3. Configure upstream remote:\n\
                    git remote add upstream https://github.com/upstream/project.git\n\
                 4. Team members create feature branches\n\
                 5. Push to org fork, create PR to upstream\n\
                 6. Multiple team members can review and iterate\n\n\
                 NAMING CONVENTIONS:\n\
                 When forking to organizations:\n\
                 - Use descriptive names for purpose: \"acme-customized-react\"\n\
                 - Include org identifier: \"companyname-project\"\n\
                 - Indicate fork status: \"project-fork\" or \"project-internal\"\n\
                 - Match internal naming standards\n\
                 - Consider version suffixes: \"project-v2\"\n\n\
                 ERROR SCENARIOS:\n\
                 - 403 Forbidden: Not a member of organization or insufficient permissions\n\
                 - 422 Unprocessable: Organization already has a fork of this repository\n\
                 - 422 Unprocessable: Organization name doesn't exist\n\
                 - Repository limit exceeded: Organization hit repository cap\n\n\
                 BEST PRACTICES:\n\
                 - Use organizations for team collaboration\n\
                 - Configure appropriate team permissions after forking\n\
                 - Set up branch protection on org forks\n\
                 - Document fork purpose in README\n\
                 - Establish sync strategy with upstream\n\
                 - Use GitHub teams for access control\n\
                 - Consider GitHub Apps for automation\n\
                 - Archive org forks when no longer needed",
            ),
        },
    ]
}

/// Complete contribution workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Show me the complete workflow for contributing to an open source project using forks.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Fork-based contribution is the standard workflow for open source. Here's the complete process from fork to merged PR.\n\n\
                 CONTRIBUTION WORKFLOWS:\n\n\
                 =============================================================================\n\
                 WORKFLOW 1: COMPLETE FORK CONTRIBUTION\n\
                 =============================================================================\n\n\
                 Step 1: Fork the repository\n\
                 github_fork_repository({\n\
                     \"owner\": \"open-source\",\n\
                     \"repo\": \"project\"\n\
                 })\n\n\
                 Step 2: Clone your fork\n\
                 git_clone({\n\
                     \"url\": \"https://github.com/your-username/project.git\",\n\
                     \"path\": \"/workspace/project\"\n\
                 })\n\n\
                 Step 3: Add upstream remote\n\
                 terminal({\n\
                     \"command\": \"cd /workspace/project && git remote add upstream https://github.com/open-source/project.git\"\n\
                 })\n\n\
                 Step 4: Create feature branch\n\
                 terminal({\n\
                     \"command\": \"cd /workspace/project && git checkout -b feature/my-contribution\"\n\
                 })\n\n\
                 Step 5: Make changes and commit\n\
                 terminal({\n\
                     \"command\": \"cd /workspace/project && git add . && git commit -m 'Add feature: description'\"\n\
                 })\n\n\
                 Step 6: Push to your fork\n\
                 terminal({\n\
                     \"command\": \"cd /workspace/project && git push origin feature/my-contribution\"\n\
                 })\n\n\
                 Step 7: Create pull request to original\n\
                 github_create_pull_request({\n\
                     \"owner\": \"open-source\",\n\
                     \"repo\": \"project\",\n\
                     \"title\": \"Add feature: description\",\n\
                     \"body\": \"This PR adds...\",\n\
                     \"head\": \"your-username:feature/my-contribution\",\n\
                     \"base\": \"main\"\n\
                 })\n\n\
                 =============================================================================\n\
                 WORKFLOW 2: KEEP FORK UPDATED WITH UPSTREAM\n\
                 =============================================================================\n\n\
                 Step 1: Fetch upstream changes\n\
                 terminal({\n\
                     \"command\": \"cd /workspace/project && git fetch upstream\"\n\
                 })\n\n\
                 Step 2: Checkout main branch\n\
                 terminal({\n\
                     \"command\": \"cd /workspace/project && git checkout main\"\n\
                 })\n\n\
                 Step 3: Merge upstream changes\n\
                 terminal({\n\
                     \"command\": \"cd /workspace/project && git merge upstream/main\"\n\
                 })\n\n\
                 Step 4: Push updates to your fork\n\
                 terminal({\n\
                     \"command\": \"cd /workspace/project && git push origin main\"\n\
                 })\n\n\
                 Step 5: Rebase feature branch (if needed)\n\
                 terminal({\n\
                     \"command\": \"cd /workspace/project && git checkout feature/my-contribution && git rebase main\"\n\
                 })\n\n\
                 =============================================================================\n\
                 WORKFLOW 3: MULTIPLE CONTRIBUTIONS FROM SAME FORK\n\
                 =============================================================================\n\n\
                 You only fork once, then create multiple feature branches:\n\n\
                 Fork once (if not already forked):\n\
                 github_fork_repository({\"owner\": \"upstream\", \"repo\": \"project\"})\n\n\
                 For each contribution:\n\
                 1. Sync with upstream:\n\
                    terminal({\"command\": \"cd /project && git checkout main && git fetch upstream && git merge upstream/main\"})\n\n\
                 2. Create new feature branch:\n\
                    terminal({\"command\": \"cd /project && git checkout -b feature/new-feature\"})\n\n\
                 3. Make changes, commit:\n\
                    terminal({\"command\": \"cd /project && git add . && git commit -m 'New feature'\"})\n\n\
                 4. Push to fork:\n\
                    terminal({\"command\": \"cd /project && git push origin feature/new-feature\"})\n\n\
                 5. Create PR:\n\
                    github_create_pull_request({\n\
                        \"owner\": \"upstream\",\n\
                        \"repo\": \"project\",\n\
                        \"head\": \"your-username:feature/new-feature\",\n\
                        \"base\": \"main\"\n\
                    })\n\n\
                 =============================================================================\n\
                 WORKFLOW 4: ORGANIZATION TEAM CONTRIBUTION\n\
                 =============================================================================\n\n\
                 Step 1: Fork to organization\n\
                 github_fork_repository({\n\
                     \"owner\": \"upstream\",\n\
                     \"repo\": \"project\",\n\
                     \"organization\": \"my-team\"\n\
                 })\n\n\
                 Step 2: Team members clone org fork\n\
                 git_clone({\n\
                     \"url\": \"https://github.com/my-team/project.git\",\n\
                     \"path\": \"/workspace/project\"\n\
                 })\n\n\
                 Step 3: Add upstream remote\n\
                 terminal({\"command\": \"cd /workspace/project && git remote add upstream https://github.com/upstream/project.git\"})\n\n\
                 Step 4: Each team member creates branches\n\
                 terminal({\"command\": \"cd /workspace/project && git checkout -b feature/team-member-feature\"})\n\n\
                 Step 5: Push to org fork\n\
                 terminal({\"command\": \"cd /workspace/project && git push origin feature/team-member-feature\"})\n\n\
                 Step 6: Create PR from org fork to upstream\n\
                 github_create_pull_request({\n\
                     \"owner\": \"upstream\",\n\
                     \"repo\": \"project\",\n\
                     \"head\": \"my-team:feature/team-member-feature\",\n\
                     \"base\": \"main\"\n\
                 })\n\n\
                 =============================================================================\n\
                 WORKFLOW 5: RESPONDING TO PR FEEDBACK\n\
                 =============================================================================\n\n\
                 Step 1: Checkout your feature branch\n\
                 terminal({\"command\": \"cd /workspace/project && git checkout feature/my-contribution\"})\n\n\
                 Step 2: Make requested changes\n\
                 (Edit files based on review feedback)\n\n\
                 Step 3: Commit changes\n\
                 terminal({\"command\": \"cd /workspace/project && git add . && git commit -m 'Address review feedback'\"})\n\n\
                 Step 4: Push to same branch\n\
                 terminal({\"command\": \"cd /workspace/project && git push origin feature/my-contribution\"})\n\n\
                 PR automatically updates with new commits!\n\n\
                 =============================================================================\n\
                 COMMON FORK PATTERNS\n\
                 =============================================================================\n\n\
                 Pattern: First-time contributor\n\
                 1. Fork repository\n\
                 2. Make small, focused change\n\
                 3. Follow CONTRIBUTING.md guidelines\n\
                 4. Create clear PR with description\n\
                 5. Respond promptly to feedback\n\n\
                 Pattern: Regular contributor\n\
                 1. Maintain one fork per project\n\
                 2. Keep fork synced regularly\n\
                 3. Use separate branches per feature\n\
                 4. Clean up merged branches\n\
                 5. Update fork before each new contribution\n\n\
                 Pattern: Long-term maintainer\n\
                 1. Fork to organization\n\
                 2. Set up CI/CD on fork\n\
                 3. Maintain internal customizations\n\
                 4. Regularly sync with upstream\n\
                 5. Contribute improvements back\n\n\
                 =============================================================================\n\
                 FORK MAINTENANCE BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. SYNC REGULARLY:\n\
                    Sync fork with upstream before starting new work\n\
                    terminal({\"command\": \"git fetch upstream && git merge upstream/main\"})\n\n\
                 2. CLEAN BRANCHES:\n\
                    Delete merged feature branches\n\
                    terminal({\"command\": \"git branch -d feature/merged-feature\"})\n\n\
                 3. REBASE STRATEGY:\n\
                    Keep feature branches clean with rebase\n\
                    terminal({\"command\": \"git rebase upstream/main\"})\n\n\
                 4. FORCE PUSH CAREFULLY:\n\
                    Only force push to your own branches\n\
                    terminal({\"command\": \"git push origin feature/my-branch --force-with-lease\"})\n\n\
                 5. COMMUNICATION:\n\
                    Comment on PRs\n\
                    Respond to reviews promptly\n\
                    Update PR description as changes evolve\n\n\
                 =============================================================================\n\
                 TROUBLESHOOTING FORK ISSUES\n\
                 =============================================================================\n\n\
                 Issue: Fork out of sync with upstream\n\
                 Solution:\n\
                 terminal({\"command\": \"git fetch upstream && git checkout main && git reset --hard upstream/main && git push origin main --force\"})\n\n\
                 Issue: Accidentally committed to main branch\n\
                 Solution:\n\
                 1. Create branch from current state:\n\
                    terminal({\"command\": \"git checkout -b feature/my-changes\"})\n\
                 2. Reset main to upstream:\n\
                    terminal({\"command\": \"git checkout main && git reset --hard upstream/main\"})\n\
                 3. Push both:\n\
                    terminal({\"command\": \"git push origin main --force && git push origin feature/my-changes\"})\n\n\
                 Issue: PR has merge conflicts\n\
                 Solution:\n\
                 1. Sync with upstream:\n\
                    terminal({\"command\": \"git fetch upstream\"})\n\
                 2. Rebase on upstream:\n\
                    terminal({\"command\": \"git rebase upstream/main\"})\n\
                 3. Resolve conflicts in files\n\
                 4. Continue rebase:\n\
                    terminal({\"command\": \"git rebase --continue\"})\n\
                 5. Force push:\n\
                    terminal({\"command\": \"git push origin feature/my-branch --force-with-lease\"})\n\n\
                 Issue: Need to update PR with upstream changes\n\
                 Solution:\n\
                 terminal({\"command\": \"git fetch upstream && git rebase upstream/main && git push origin feature/branch --force-with-lease\"})\n\n\
                 =============================================================================\n\
                 FORK ETIQUETTE\n\
                 =============================================================================\n\n\
                 DO:\n\
                 - Read CONTRIBUTING.md before contributing\n\
                 - Make focused, single-purpose PRs\n\
                 - Write clear commit messages\n\
                 - Test your changes thoroughly\n\
                 - Respond to feedback constructively\n\
                 - Keep your fork updated\n\
                 - Follow project's code style\n\n\
                 DON'T:\n\
                 - Don't create PRs with unrelated changes\n\
                 - Don't ignore maintainer feedback\n\
                 - Don't force push to others' branches\n\
                 - Don't commit directly to main branch\n\
                 - Don't create duplicate PRs\n\
                 - Don't forget to sync before new work\n\
                 - Don't mix code changes with whitespace fixes",
            ),
        },
    ]
}

/// Comprehensive guide covering all patterns
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to forking repositories and contributing to open source projects.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_fork_repository tool creates independent copies of repositories, enabling safe experimentation and collaborative contribution to open source projects.\n\n\
                 =============================================================================\n\
                 WHAT IS A FORK?\n\
                 =============================================================================\n\n\
                 A fork is a complete copy of a repository that:\n\
                 - Exists in your account or organization\n\
                 - Preserves full commit history\n\
                 - Links to the original (parent) repository\n\
                 - Allows independent development\n\
                 - Enables contributing back via pull requests\n\
                 - Maintains connection in GitHub's fork network\n\n\
                 Forks vs Branches:\n\
                 - Fork: Separate repository copy (cross-account collaboration)\n\
                 - Branch: Within same repository (same-team collaboration)\n\n\
                 Use forks when:\n\
                 - Contributing to repositories you don't have write access to\n\
                 - Experimenting with major changes\n\
                 - Creating personal versions of projects\n\
                 - Learning from existing codebases\n\n\
                 Use branches when:\n\
                 - Working within your own repositories\n\
                 - Collaborating within same organization\n\
                 - You have write access to the repository\n\n\
                 =============================================================================\n\
                 BASIC FORKING\n\
                 =============================================================================\n\n\
                 SIMPLE FORK:\n\
                 github_fork_repository({\n\
                     \"owner\": \"original-owner\",\n\
                     \"repo\": \"project\"\n\
                 })\n\n\
                 Creates: https://github.com/your-username/project\n\n\
                 WITH CUSTOM NAME:\n\
                 github_fork_repository({\n\
                     \"owner\": \"original-owner\",\n\
                     \"repo\": \"project\",\n\
                     \"name\": \"my-custom-name\"\n\
                 })\n\n\
                 Creates: https://github.com/your-username/my-custom-name\n\n\
                 TO ORGANIZATION:\n\
                 github_fork_repository({\n\
                     \"owner\": \"original-owner\",\n\
                     \"repo\": \"project\",\n\
                     \"organization\": \"my-org\"\n\
                 })\n\n\
                 Creates: https://github.com/my-org/project\n\n\
                 FAST FORK (default branch only):\n\
                 github_fork_repository({\n\
                     \"owner\": \"original-owner\",\n\
                     \"repo\": \"large-project\",\n\
                     \"default_branch_only\": true\n\
                 })\n\n\
                 Forks only default branch (faster for large repositories)\n\n\
                 =============================================================================\n\
                 FORK PARAMETERS\n\
                 =============================================================================\n\n\
                 REQUIRED:\n\
                 - owner: Original repository owner (user or organization)\n\
                 - repo: Original repository name\n\n\
                 OPTIONAL:\n\
                 - organization: Fork to organization instead of personal account\n\
                   - You must be member of organization\n\
                   - Organization must allow fork creation\n\
                 - name: Custom name for forked repository\n\
                   - Must be unique within your account/organization\n\
                   - Should follow naming conventions\n\
                 - default_branch_only: Fork only default branch (default: false)\n\
                   - Faster for large repositories\n\
                   - Reduces storage usage\n\
                   - Still maintains fork relationship\n\n\
                 =============================================================================\n\
                 FORK RESPONSE\n\
                 =============================================================================\n\n\
                 Successful fork returns:\n\
                 {\n\
                   \"id\": 123456789,\n\
                   \"full_name\": \"your-username/project\",\n\
                   \"name\": \"project\",\n\
                   \"owner\": {\n\
                     \"login\": \"your-username\",\n\
                     \"type\": \"User\"\n\
                   },\n\
                   \"html_url\": \"https://github.com/your-username/project\",\n\
                   \"clone_url\": \"https://github.com/your-username/project.git\",\n\
                   \"parent\": {\n\
                     \"id\": 987654321,\n\
                     \"full_name\": \"original-owner/project\",\n\
                     \"html_url\": \"https://github.com/original-owner/project\"\n\
                   },\n\
                   \"source\": {\n\
                     \"full_name\": \"root-owner/project\"\n\
                   },\n\
                   \"fork\": true,\n\
                   \"forks_count\": 1,\n\
                   \"default_branch\": \"main\",\n\
                   \"created_at\": \"2025-12-05T10:30:00Z\",\n\
                   \"private\": false\n\
                 }\n\n\
                 Key fields:\n\
                 - full_name: Your fork's identifier\n\
                 - clone_url: URL for cloning\n\
                 - parent: Original repository\n\
                 - source: Root of fork network\n\
                 - fork: Always true for forks\n\n\
                 =============================================================================\n\
                 COMPLETE CONTRIBUTION WORKFLOW\n\
                 =============================================================================\n\n\
                 STEP 1: Fork repository\n\
                 github_fork_repository({\"owner\": \"upstream\", \"repo\": \"project\"})\n\n\
                 STEP 2: Clone fork locally\n\
                 git_clone({\n\
                     \"url\": \"https://github.com/your-username/project.git\",\n\
                     \"path\": \"/workspace/project\"\n\
                 })\n\n\
                 STEP 3: Configure upstream remote\n\
                 terminal({\n\
                     \"command\": \"cd /workspace/project && git remote add upstream https://github.com/upstream/project.git\"\n\
                 })\n\n\
                 STEP 4: Verify remotes\n\
                 terminal({\"command\": \"cd /workspace/project && git remote -v\"})\n\
                 Shows:\n\
                 origin    https://github.com/your-username/project.git (fetch)\n\
                 origin    https://github.com/your-username/project.git (push)\n\
                 upstream  https://github.com/upstream/project.git (fetch)\n\
                 upstream  https://github.com/upstream/project.git (push)\n\n\
                 STEP 5: Create feature branch\n\
                 terminal({\"command\": \"cd /workspace/project && git checkout -b feature/my-contribution\"})\n\n\
                 STEP 6: Make changes\n\
                 (Edit files, add features, fix bugs)\n\n\
                 STEP 7: Commit changes\n\
                 terminal({\"command\": \"cd /workspace/project && git add . && git commit -m 'Add feature: description'\"})\n\n\
                 STEP 8: Push to fork\n\
                 terminal({\"command\": \"cd /workspace/project && git push origin feature/my-contribution\"})\n\n\
                 STEP 9: Create pull request\n\
                 github_create_pull_request({\n\
                     \"owner\": \"upstream\",\n\
                     \"repo\": \"project\",\n\
                     \"title\": \"Add feature: description\",\n\
                     \"body\": \"This PR adds...\\n\\nFixes #123\",\n\
                     \"head\": \"your-username:feature/my-contribution\",\n\
                     \"base\": \"main\"\n\
                 })\n\n\
                 STEP 10: Address review feedback\n\
                 - Make requested changes\n\
                 - Commit to same branch\n\
                 - Push to same branch\n\
                 - PR updates automatically\n\n\
                 STEP 11: After merge\n\
                 - Sync fork with upstream\n\
                 - Delete feature branch\n\
                 - Ready for next contribution\n\n\
                 =============================================================================\n\
                 KEEPING FORK SYNCHRONIZED\n\
                 =============================================================================\n\n\
                 Sync main branch with upstream:\n\n\
                 1. Fetch upstream changes:\n\
                    terminal({\"command\": \"cd /workspace/project && git fetch upstream\"})\n\n\
                 2. Checkout main:\n\
                    terminal({\"command\": \"cd /workspace/project && git checkout main\"})\n\n\
                 3. Merge upstream/main:\n\
                    terminal({\"command\": \"cd /workspace/project && git merge upstream/main\"})\n\n\
                 4. Push to fork:\n\
                    terminal({\"command\": \"cd /workspace/project && git push origin main\"})\n\n\
                 One-liner sync:\n\
                 terminal({\"command\": \"cd /workspace/project && git checkout main && git fetch upstream && git merge upstream/main && git push origin main\"})\n\n\
                 Sync before each new contribution!\n\n\
                 =============================================================================\n\
                 ORGANIZATION FORKS\n\
                 =============================================================================\n\n\
                 Fork to organization for team collaboration:\n\n\
                 github_fork_repository({\n\
                     \"owner\": \"upstream\",\n\
                     \"repo\": \"project\",\n\
                     \"organization\": \"my-team\"\n\
                 })\n\n\
                 Benefits:\n\
                 - Multiple team members can collaborate\n\
                 - Centralized fork management\n\
                 - Organization-wide permissions\n\
                 - Better for long-term maintenance\n\n\
                 Requirements:\n\
                 - Organization membership\n\
                 - Fork creation permissions\n\
                 - Available repository slots\n\n\
                 Team workflow:\n\
                 1. One person forks to organization\n\
                 2. Team members clone org fork\n\
                 3. Each member works on feature branches\n\
                 4. Push to org fork\n\
                 5. Create PRs from org fork to upstream\n\n\
                 =============================================================================\n\
                 ERROR HANDLING\n\
                 =============================================================================\n\n\
                 404 NOT FOUND:\n\
                 - Repository doesn't exist\n\
                 - Repository is private and you lack access\n\
                 - Owner/repo name is incorrect\n\
                 Fix: Verify repository exists and is accessible\n\n\
                 403 FORBIDDEN:\n\
                 - Cannot fork your own repository\n\
                 - Insufficient permissions\n\
                 - Organization doesn't allow forks\n\
                 Fix: Use branches for own repos, verify org permissions\n\n\
                 422 UNPROCESSABLE ENTITY:\n\
                 - Fork already exists in your account\n\
                 - Cannot create duplicate fork\n\
                 Fix: Use existing fork, or delete old fork first\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Forking has additional rate limits\n\
                 Fix: Wait for rate limit reset, check X-RateLimit-Reset header\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 FORKING:\n\
                 - Fork only repositories you intend to use/contribute to\n\
                 - Use organizations for team forks\n\
                 - Use descriptive custom names when needed\n\
                 - Use default_branch_only for large repositories\n\
                 - Delete forks when no longer needed\n\n\
                 CONTRIBUTING:\n\
                 - Read CONTRIBUTING.md first\n\
                 - Sync fork before starting work\n\
                 - Use feature branches, never commit to main\n\
                 - Write clear, focused commits\n\
                 - Test thoroughly before creating PR\n\
                 - Respond promptly to review feedback\n\
                 - Keep PRs focused and single-purpose\n\n\
                 MAINTENANCE:\n\
                 - Sync fork regularly with upstream\n\
                 - Clean up merged branches\n\
                 - Keep fork updated even when not actively contributing\n\
                 - Configure upstream remote immediately after cloning\n\
                 - Use git remote -v to verify remote configuration\n\n\
                 COLLABORATION:\n\
                 - Use organization forks for team work\n\
                 - Communicate in PR comments\n\
                 - Follow project's code style\n\
                 - Document changes clearly\n\
                 - Be respectful and professional\n\n\
                 =============================================================================\n\
                 AUTHENTICATION\n\
                 =============================================================================\n\n\
                 Requires GITHUB_TOKEN environment variable:\n\n\
                 For public repositories:\n\
                 - Scope: public_repo\n\n\
                 For private repositories:\n\
                 - Scope: repo\n\n\
                 For organization forks:\n\
                 - Same scopes as above\n\
                 - Plus: Organization membership\n\
                 - Plus: Fork creation permissions in organization\n\n\
                 Generate token at: https://github.com/settings/tokens\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Fork to personal account:\n\
                 github_fork_repository({\"owner\": \"upstream\", \"repo\": \"project\"})\n\n\
                 Fork to organization:\n\
                 github_fork_repository({\"owner\": \"upstream\", \"repo\": \"project\", \"organization\": \"my-org\"})\n\n\
                 Fork with custom name:\n\
                 github_fork_repository({\"owner\": \"upstream\", \"repo\": \"project\", \"name\": \"custom\"})\n\n\
                 Fast fork (default branch only):\n\
                 github_fork_repository({\"owner\": \"upstream\", \"repo\": \"project\", \"default_branch_only\": true})\n\n\
                 Sync fork:\n\
                 terminal({\"command\": \"git fetch upstream && git checkout main && git merge upstream/main && git push origin main\"})\n\n\
                 Create PR from fork:\n\
                 github_create_pull_request({\"owner\": \"upstream\", \"repo\": \"project\", \"head\": \"your-username:feature-branch\", \"base\": \"main\"})\n\n\
                 Remember: Forks are for external collaboration, branches are for internal collaboration!",
            ),
        },
    ]
}
