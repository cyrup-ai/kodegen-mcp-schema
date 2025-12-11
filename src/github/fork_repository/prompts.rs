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
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, organization)".to_string()),
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



