//! Prompt messages for git_init tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitInitPromptArgs;

/// Prompt provider for git_init tool
///
/// This is the ONLY way to provide prompts for git_init - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct InitPrompts;

impl PromptProvider for InitPrompts {
    type PromptArgs = GitInitPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("bare") => prompt_bare(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, bare)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT INIT
// ============================================================================

/// Basic repository creation
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create a new Git repository using git_init?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_init tool creates a new Git repository by initializing the .git directory structure. Here's how to use it:\n\n\
                 CREATING A NEW REPOSITORY:\n\n\
                 1. Initialize in directory:\n\
                    git_init({\n\
                        \"path\": \"/projects/new-project\"\n\
                    })\n\n\
                 2. Initialize current directory:\n\
                    git_init({\n\
                        \"path\": \".\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"path\": \"/projects/new-project\",\n\
                   \"branch\": \"main\",\n\
                   \"success\": true\n\
                 }\n\n\
                 WHAT GIT INIT CREATES:\n\
                 - .git/ directory (repository data)\n\
                 - Initial branch (main or master)\n\
                 - Ready for first commit\n\n\
                 The .git/ directory contains:\n\
                 - objects/ - Git object database\n\
                 - refs/ - Branch and tag references\n\
                 - HEAD - Current branch pointer\n\
                 - config - Repository configuration\n\
                 - hooks/ - Git hook scripts\n\n\
                 VERIFY:\n\
                 git_status({\"path\": \"/projects/new-project\"})\n\
                 // Shows \"On branch main, No commits yet\"\n\n\
                 IMPORTANT:\n\
                 - Creates empty repository with no commits\n\
                 - No files are tracked initially\n\
                 - Must add files and commit to create history\n\
                 - Safe to run in existing directory (won't overwrite files)\n\n\
                 PATH REQUIREMENTS:\n\
                 - Can be absolute: /projects/new-project\n\
                 - Can be relative: ./new-project\n\
                 - Directory is created if it doesn't exist\n\
                 - Fails if .git already exists in directory\n\n\
                 AFTER INITIALIZATION:\n\
                 - Repository has no commits yet\n\
                 - No branches exist until first commit\n\
                 - Use git_add to stage files\n\
                 - Use git_commit to create first commit",
            ),
        },
    ]
}

/// Bare repository creation
fn prompt_bare() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What is a bare repository and when should I use it?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "A bare repository is a Git repository without a working directory. It stores only the version control data.\n\n\
                 BARE REPOSITORIES:\n\n\
                 1. Create bare repo:\n\
                    git_init({\n\
                        \"path\": \"/repos/project.git\",\n\
                        \"bare\": true\n\
                    })\n\n\
                 BARE vs NORMAL:\n\
                 - Normal: Working tree + .git/\n\
                   Example: /project/ contains files + .git/ subdirectory\n\
                 - Bare: Only git data, no working files\n\
                   Example: /project.git/ contains refs/, objects/, HEAD, etc.\n\n\
                 BARE REPO CHARACTERISTICS:\n\
                 - No working directory (cannot checkout files)\n\
                 - No .git subdirectory (entire directory is the repo)\n\
                 - Cannot make commits directly\n\
                 - Only receives pushes from other repositories\n\
                 - More compact (no duplicate files)\n\n\
                 BARE REPO USES:\n\
                 - Server-side repos (GitHub-style central repository)\n\
                 - Central repository for team collaboration\n\
                 - Can push to, can't work in directly\n\
                 - Acts as remote for local development repositories\n\
                 - Backup repositories\n\n\
                 NAMING CONVENTION:\n\
                 - /repos/project.git (bare repository)\n\
                 - /workspace/project (normal repository)\n\
                 - .git suffix indicates bare repository\n\
                 - Not required but strongly recommended\n\n\
                 SETUP BARE REPOSITORY WORKFLOW:\n\
                 Step 1 - Create bare repo on server:\n\
                 git_init({\"path\": \"/repos/project.git\", \"bare\": true})\n\n\
                 Step 2 - On local machine, add as remote:\n\
                 git_remote_add({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"origin\",\n\
                     \"url\": \"/repos/project.git\"\n\
                 })\n\n\
                 Step 3 - Push to bare repository:\n\
                 git_push({\"path\": \"/project\"})\n\n\
                 WHY USE BARE REPOSITORIES:\n\
                 1. Server deployments:\n\
                    - No need for working directory on server\n\
                    - Saves disk space (no duplicate files)\n\
                    - More secure (no files to accidentally modify)\n\n\
                 2. Team collaboration:\n\
                    - Central point for all team members\n\
                    - Everyone pushes/pulls from same location\n\
                    - Prevents confusion about \"current\" state\n\n\
                 3. Backup purposes:\n\
                    - Store complete repository history\n\
                    - No working files to backup separately\n\
                    - Easy to clone from for recovery\n\n\
                 WHEN NOT TO USE BARE:\n\
                 - Local development (need working files)\n\
                 - Want to edit files directly\n\
                 - Need to make commits in that location\n\
                 - Running builds or tests from repository\n\n\
                 CONVERTING BETWEEN BARE AND NORMAL:\n\
                 Normal to bare:\n\
                 - Copy .git directory to new location\n\
                 - Set core.bare = true in config\n\n\
                 Bare to normal:\n\
                 - Clone the bare repository\n\
                 - Results in normal repository with working tree\n\n\
                 EXAMPLE TEAM SETUP:\n\
                 Server (bare repository):\n\
                 git_init({\"path\": \"/var/repos/app.git\", \"bare\": true})\n\n\
                 Developer A:\n\
                 git_clone({\"url\": \"/var/repos/app.git\", \"path\": \"/home/alice/app\"})\n\
                 // Work, commit, push\n\n\
                 Developer B:\n\
                 git_clone({\"url\": \"/var/repos/app.git\", \"path\": \"/home/bob/app\"})\n\
                 // Work, commit, push\n\n\
                 Both developers share central bare repository!",
            ),
        },
    ]
}
