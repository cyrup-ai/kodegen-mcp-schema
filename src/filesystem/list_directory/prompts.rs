//! Prompt messages for fs_list_directory tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::FsListDirectoryPromptArgs;

/// Prompt provider for fs_list_directory tool
///
/// This is the ONLY way to provide prompts for fs_list_directory - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ListDirectoryPrompts;

impl PromptProvider for ListDirectoryPrompts {
    type PromptArgs = FsListDirectoryPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("hidden") => prompt_hidden(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, hidden)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST DIRECTORIES
// ============================================================================

/// Basic directory listing
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list the contents of a directory?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The fs_list_directory tool lists all files and directories in a specified path. Here's how to use it for basic directory listing:\n\n\
                 BASIC USAGE:\n\
                 fs_list_directory({\"path\": \"/project\"})\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"path\": \"/project\",\n\
                   \"total_entries\": 4,\n\
                   \"directories\": 1,\n\
                   \"files\": 3,\n\
                   \"entries\": [\n\
                     {\"name\": \"src\", \"is_directory\": true},\n\
                     {\"name\": \"Cargo.toml\", \"is_directory\": false},\n\
                     {\"name\": \"README.md\", \"is_directory\": false},\n\
                     {\"name\": \"tests\", \"is_directory\": true}\n\
                   ]\n\
                 }\n\n\
                 INTERPRETING RESULTS:\n\
                 - is_directory: true → subdirectory (can navigate into it)\n\
                 - is_directory: false → file (can read/edit it)\n\
                 - total_entries: Total count of items\n\
                 - directories: Count of subdirectories\n\
                 - files: Count of files\n\n\
                 ENTRY TYPES:\n\
                 Each entry in the results has:\n\
                 - name: File or directory name (without path)\n\
                 - is_directory: Boolean indicating type\n\n\
                 COMMON PATTERNS:\n\
                 1. List current directory:\n\
                    fs_list_directory({\"path\": \".\"})\n\n\
                 2. List absolute path:\n\
                    fs_list_directory({\"path\": \"/Users/name/projects\"})\n\n\
                 3. List subdirectory:\n\
                    fs_list_directory({\"path\": \"/project/src\"})\n\n\
                 4. List user home:\n\
                    fs_list_directory({\"path\": \"~\"})\n\n\
                 WHEN TO USE:\n\
                 - Exploring unknown directory structure\n\
                 - Finding specific files in a directory\n\
                 - Understanding project organization\n\
                 - Checking what files exist before operations\n\
                 - Verifying directory contents after changes\n\n\
                 DEFAULT BEHAVIOR:\n\
                 - Hidden files (starting with .) are NOT shown by default\n\
                 - Use include_hidden: true to see hidden files\n\
                 - Entries are returned in filesystem order (not sorted)\n\
                 - Both files and directories are included\n\n\
                 ERROR CASES:\n\
                 - Path does not exist: Error returned\n\
                 - Path is a file (not directory): Error returned\n\
                 - No permission to read: Error returned\n\
                 - Always check success field in response",
            ),
        },
    ]
}

/// Hidden files handling
fn prompt_hidden() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I see hidden files when listing a directory?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Hidden files start with a dot (.) and are excluded by default. Use include_hidden parameter to show them.\n\n\
                 HIDDEN FILES EXPLAINED:\n\
                 On Unix/Linux/macOS, files starting with . are hidden:\n\
                 - .git - Git repository data\n\
                 - .gitignore - Git ignore rules\n\
                 - .env - Environment variables\n\
                 - .config - Configuration files\n\
                 - .cache - Cache directories\n\
                 - .DS_Store - macOS metadata\n\
                 - .bashrc - Shell configuration\n\n\
                 DEFAULT (HIDDEN FILES EXCLUDED):\n\
                 fs_list_directory({\"path\": \"/project\"})\n\
                 Response:\n\
                 {\n\
                   \"entries\": [\n\
                     {\"name\": \"src\", \"is_directory\": true},\n\
                     {\"name\": \"Cargo.toml\", \"is_directory\": false},\n\
                     {\"name\": \"README.md\", \"is_directory\": false}\n\
                   ]\n\
                 }\n\
                 // .git, .gitignore, .env NOT shown\n\n\
                 INCLUDING HIDDEN FILES:\n\
                 fs_list_directory({\"path\": \"/project\", \"include_hidden\": true})\n\
                 Response:\n\
                 {\n\
                   \"entries\": [\n\
                     {\"name\": \".git\", \"is_directory\": true},\n\
                     {\"name\": \".gitignore\", \"is_directory\": false},\n\
                     {\"name\": \".env\", \"is_directory\": false},\n\
                     {\"name\": \"src\", \"is_directory\": true},\n\
                     {\"name\": \"Cargo.toml\", \"is_directory\": false},\n\
                     {\"name\": \"README.md\", \"is_directory\": false}\n\
                   ]\n\
                 }\n\
                 // Now .git, .gitignore, .env are visible\n\n\
                 COMMON HIDDEN FILES BY PURPOSE:\n\
                 1. Version Control:\n\
                    - .git (directory) - Git repository\n\
                    - .gitignore - Files to ignore\n\
                    - .gitattributes - Git attributes\n\
                    - .svn (directory) - Subversion repository\n\n\
                 2. Configuration:\n\
                    - .env - Environment variables\n\
                    - .config (directory) - App configurations\n\
                    - .eslintrc - ESLint config\n\
                    - .prettierrc - Prettier config\n\
                    - .editorconfig - Editor settings\n\n\
                 3. Shell & System:\n\
                    - .bashrc - Bash configuration\n\
                    - .zshrc - Zsh configuration\n\
                    - .profile - Shell profile\n\
                    - .bash_history - Command history\n\
                    - .DS_Store - macOS metadata\n\n\
                 4. Cache & Temporary:\n\
                    - .cache (directory) - Cache files\n\
                    - .tmp (directory) - Temporary files\n\
                    - .npm (directory) - npm cache\n\
                    - .cargo (directory) - Cargo cache\n\n\
                 5. IDE & Editors:\n\
                    - .vscode (directory) - VS Code settings\n\
                    - .idea (directory) - IntelliJ IDEA settings\n\
                    - .vim (directory) - Vim configuration\n\n\
                 WHEN TO INCLUDE HIDDEN FILES:\n\
                 1. Looking for config files:\n\
                    fs_list_directory({\"path\": \"/project\", \"include_hidden\": true})\n\
                    // Find .env, .config, etc.\n\n\
                 2. Checking git status:\n\
                    fs_list_directory({\"path\": \"/project\", \"include_hidden\": true})\n\
                    // Verify .git directory exists\n\n\
                 3. Finding dotfiles:\n\
                    fs_list_directory({\"path\": \"~\", \"include_hidden\": true})\n\
                    // List all dotfiles in home directory\n\n\
                 4. Troubleshooting issues:\n\
                    fs_list_directory({\"path\": \"/project\", \"include_hidden\": true})\n\
                    // Check for hidden files causing problems\n\n\
                 5. Complete directory audit:\n\
                    fs_list_directory({\"path\": \"/project\", \"include_hidden\": true})\n\
                    // See EVERYTHING in the directory\n\n\
                 WHEN NOT TO INCLUDE HIDDEN:\n\
                 - General file browsing\n\
                 - Looking for source code only\n\
                 - Checking project structure\n\
                 - Listing user content\n\
                 Default (include_hidden: false) is usually what you want\n\n\
                 WORKFLOW EXAMPLE:\n\
                 1. First, list without hidden files:\n\
                    fs_list_directory({\"path\": \"/project\"})\n\
                    // See main project structure\n\n\
                 2. If you need to check configs:\n\
                    fs_list_directory({\"path\": \"/project\", \"include_hidden\": true})\n\
                    // Now see .env, .gitignore, etc.\n\n\
                 IMPORTANT NOTES:\n\
                 - Hidden files are only hidden by convention (. prefix)\n\
                 - They are NOT secured or protected\n\
                 - System still processes them normally\n\
                 - Many critical configs are hidden files\n\
                 - include_hidden: false is safer default (less clutter)\n\
                 - include_hidden: true shows complete truth",
            ),
        },
    ]
}

