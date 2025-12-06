//! Prompt messages for github_create_repository tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::CreateRepositoryPromptArgs;

/// Prompt provider for create_repository tool
///
/// This is the ONLY way to provide prompts for create_repository - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct CreateRepositoryPrompts;

impl PromptProvider for CreateRepositoryPrompts {
    type PromptArgs = CreateRepositoryPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("options") => prompt_options(),
            Some("organization") => prompt_organization(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, options, organization)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO CREATE GITHUB REPOSITORIES
// ============================================================================

/// Basic repository creation
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create basic GitHub repositories?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_create_repository tool creates new GitHub repositories with customizable settings.\n\n\
                 CREATING REPOSITORIES:\n\n\
                 1. Simple repo:\n\
                    github_create_repository({\n\
                        \"name\": \"my-project\",\n\
                        \"description\": \"My awesome project\"\n\
                    })\n\n\
                 2. Private repo:\n\
                    github_create_repository({\n\
                        \"name\": \"private-project\",\n\
                        \"description\": \"Internal project\",\n\
                        \"private\": true\n\
                    })\n\n\
                 3. Public repo:\n\
                    github_create_repository({\n\
                        \"name\": \"open-source-tool\",\n\
                        \"description\": \"A useful open source utility\",\n\
                        \"private\": false\n\
                    })\n\n\
                 REQUIRED PARAMETERS:\n\
                 - name: Repository name (alphanumeric, hyphens, underscores)\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - description: Short description\n\
                 - private: true for private, false for public (default: false)\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"username\",\n\
                   \"name\": \"my-project\",\n\
                   \"full_name\": \"username/my-project\",\n\
                   \"html_url\": \"https://github.com/username/my-project\",\n\
                   \"clone_url\": \"https://github.com/username/my-project.git\",\n\
                   \"message\": \"Repository created successfully\"\n\
                 }\n\n\
                 COMMON WORKFLOWS:\n\n\
                 New project from scratch:\n\
                 1. Create with initialization:\n\
                    github_create_repository({\n\
                        \"name\": \"new-project\",\n\
                        \"description\": \"Brand new project\",\n\
                        \"auto_init\": true\n\
                    })\n\
                 2. Clone to local machine:\n\
                    git clone https://github.com/username/new-project.git\n\
                 3. Add code, commit, push changes\n\n\
                 Push existing local project:\n\
                 1. Create empty repo (no auto_init):\n\
                    github_create_repository({\n\
                        \"name\": \"existing-project\",\n\
                        \"description\": \"Existing codebase\"\n\
                    })\n\
                 2. Add remote to local repo:\n\
                    git remote add origin https://github.com/username/existing-project.git\n\
                 3. Push local code:\n\
                    git push -u origin main\n\n\
                 NAMING RULES:\n\
                 - Alphanumeric characters, hyphens, underscores only\n\
                 - Cannot start with hyphen or underscore\n\
                 - Must be unique within your account/organization\n\
                 - Use lowercase, descriptive names\n\n\
                 COMMON ERRORS:\n\
                 - 422: Repository name already exists (choose different name)\n\
                 - 401: Authentication failed (check GITHUB_TOKEN)\n\
                 - 400: Invalid repository name (check naming rules)\n\n\
                 BEST PRACTICES:\n\
                 - Always add description for discoverability\n\
                 - Use auto_init: true when starting fresh\n\
                 - Skip auto_init when pushing existing code\n\
                 - Choose appropriate visibility (private vs public)",
            ),
        },
    ]
}

/// Repository options and configuration
fn prompt_options() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What options can I configure when creating a GitHub repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "GitHub repositories support configuration options for initialization and features.\n\n\
                 INITIALIZATION OPTIONS:\n\n\
                 With README:\n\
                 github_create_repository({\n\
                     \"name\": \"my-project\",\n\
                     \"auto_init\": true\n\
                 })\n\n\
                 With gitignore:\n\
                 github_create_repository({\n\
                     \"name\": \"rust-project\",\n\
                     \"auto_init\": true,\n\
                     \"gitignore_template\": \"Rust\"\n\
                 })\n\n\
                 With license:\n\
                 github_create_repository({\n\
                     \"name\": \"open-source\",\n\
                     \"auto_init\": true,\n\
                     \"license_template\": \"mit\"\n\
                 })\n\n\
                 Full configuration:\n\
                 github_create_repository({\n\
                     \"name\": \"full-project\",\n\
                     \"description\": \"Complete setup\",\n\
                     \"auto_init\": true,\n\
                     \"gitignore_template\": \"Python\",\n\
                     \"license_template\": \"apache-2.0\",\n\
                     \"has_issues\": true,\n\
                     \"has_wiki\": false,\n\
                     \"has_projects\": true\n\
                 })\n\n\
                 GITIGNORE TEMPLATES:\n\
                 Common: Rust, Python, Node, Go, Java, C++, Ruby\n\n\
                 LICENSE TEMPLATES:\n\
                 - mit: MIT License (permissive)\n\
                 - apache-2.0: Apache 2.0 (permissive)\n\
                 - gpl-3.0: GPL v3 (copyleft)\n\
                 - bsd-3-clause: BSD 3-Clause\n\n\
                 FEATURE OPTIONS:\n\
                 - has_issues: Enable issue tracking (default: true)\n\
                 - has_wiki: Enable wiki (default: true)\n\
                 - has_projects: Enable project boards (default: true)\n\n\
                 When to use:\n\
                 - has_issues: For bug tracking and feature requests\n\
                 - has_wiki: For documentation (disable if using external docs)\n\
                 - has_projects: For kanban-style project management\n\n\
                 MERGE STRATEGY OPTIONS:\n\
                 - allow_squash_merge: Squash commits into one (default: true)\n\
                 - allow_merge_commit: Create merge commit (default: true)\n\
                 - allow_rebase_merge: Rebase and merge (default: true)\n\
                 - delete_branch_on_merge: Auto-delete after PR merge (default: false)\n\n\
                 Example (enforce squash-only):\n\
                 github_create_repository({\n\
                     \"name\": \"squash-only\",\n\
                     \"allow_squash_merge\": true,\n\
                     \"allow_merge_commit\": false,\n\
                     \"allow_rebase_merge\": false,\n\
                     \"delete_branch_on_merge\": true\n\
                 })\n\n\
                 BEST PRACTICES:\n\
                 - Use auto_init when starting fresh (creates initial commit)\n\
                 - Choose gitignore for your language\n\
                 - Add license for open source projects\n\
                 - Disable unused features to reduce clutter\n\
                 - Match team's merge strategy preferences",
            ),
        },
    ]
}

/// Organization repository creation
fn prompt_organization() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create repositories in GitHub organizations?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Create repositories in organizations with appropriate permissions.\n\n\
                 ORGANIZATION REPOSITORIES:\n\n\
                 Basic org repo:\n\
                 github_create_repository({\n\
                     \"name\": \"org-project\",\n\
                     \"organization\": \"my-org\",\n\
                     \"description\": \"Organization project\"\n\
                 })\n\n\
                 Private org repo with team:\n\
                 github_create_repository({\n\
                     \"name\": \"team-service\",\n\
                     \"organization\": \"company\",\n\
                     \"description\": \"Microservice for team\",\n\
                     \"private\": true,\n\
                     \"team_id\": 12345\n\
                 })\n\n\
                 Full org setup:\n\
                 github_create_repository({\n\
                     \"name\": \"enterprise-api\",\n\
                     \"organization\": \"acme-corp\",\n\
                     \"description\": \"Enterprise REST API\",\n\
                     \"private\": true,\n\
                     \"auto_init\": true,\n\
                     \"gitignore_template\": \"Go\",\n\
                     \"license_template\": \"apache-2.0\"\n\
                 })\n\n\
                 ORGANIZATION PARAMETERS:\n\
                 - organization: Target org name (creates in org namespace)\n\
                 - team_id: Numeric team identifier (optional)\n\n\
                 PERMISSIONS REQUIRED:\n\
                 - Organization membership\n\
                 - Repo creation permissions (usually admin/owner)\n\
                 - Team admin access for team assignment\n\n\
                 TEAM ASSIGNMENT:\n\
                 - team_id: Numeric identifier from GitHub\n\
                 - Grants team access to repository\n\
                 - Can assign multiple teams after creation\n\n\
                 PERSONAL vs ORGANIZATION:\n\n\
                 Personal repos (no organization field):\n\
                 - Under your account namespace\n\
                 - You control and own\n\
                 - Limited to your private repo quota\n\n\
                 Organization repos (with organization field):\n\
                 - Under org namespace\n\
                 - Organization controls and owns\n\
                 - Uses org's private repo quota\n\
                 - Team-based permissions\n\n\
                 TYPICAL WORKFLOW:\n\
                 1. Create org repo with initialization\n\
                 2. Clone to local:\n\
                    git clone https://github.com/company/team-service.git\n\
                 3. Set up branch protection rules\n\
                 4. Configure required reviews and status checks\n\
                 5. Add teams with appropriate access levels\n\n\
                 COMMON ERRORS:\n\
                 - 404: Organization not found or no access\n\
                 - 403: No permission to create repos in org\n\
                 - 422: Repository name conflicts with existing org repo",
            ),
        },
    ]
}
