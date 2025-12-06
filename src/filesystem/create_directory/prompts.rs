//! Prompt messages for fs_create_directory tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::FsCreateDirectoryPromptArgs;

/// Prompt provider for fs_create_directory tool
///
/// This is the ONLY way to provide prompts for fs_create_directory - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct CreateDirectoryPrompts;

impl PromptProvider for CreateDirectoryPrompts {
    type PromptArgs = FsCreateDirectoryPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("nested") => prompt_nested(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show examples for (basic, nested)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic directory creation patterns
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create directories using the fs_create_directory tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The fs_create_directory tool creates new directories with automatic parent directory creation. Here's how to use it for basic directory operations:\n\n\
                 BASIC DIRECTORY CREATION:\n\n\
                 1. Create single directory:\n\
                    fs_create_directory({\"path\": \"/project/data\"})\n\n\
                 2. Create in current working directory:\n\
                    fs_create_directory({\"path\": \"./output\"})\n\n\
                 3. Absolute vs relative paths:\n\
                    fs_create_directory({\"path\": \"/absolute/path/dir\"})\n\
                    fs_create_directory({\"path\": \"relative/path/dir\"})\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"path\": \"/project/data\",\n\
                   \"created\": true\n\
                 }\n\n\
                 RESPONSE FIELDS:\n\
                 - success: Operation completed successfully\n\
                 - path: Full resolved path of the directory\n\
                 - created: true if new directory was created, false if already existed\n\n\
                 WHEN TO USE:\n\
                 - Before writing files to a new location\n\
                 - Setting up project structure\n\
                 - Creating output directories\n\
                 - Organizing data into folders\n\
                 - Preparing directories for downloads or exports\n\n\
                 PATH TYPES:\n\
                 - Absolute paths: Start with / (Unix) or C:\\ (Windows)\n\
                   Example: /home/user/projects/new_dir\n\
                 - Relative paths: Relative to current working directory\n\
                   Example: ./data or output/results\n\n\
                 COMMON PATTERNS:\n\
                 1. Create output directory before processing:\n\
                    fs_create_directory({\"path\": \"./output\"})\n\
                    // Then write files to ./output/\n\n\
                 2. Create data directory for current date:\n\
                    fs_create_directory({\"path\": \"./data/2024-01-15\"})\n\n\
                 3. Create user-specific directory:\n\
                    fs_create_directory({\"path\": \"/home/user/documents/my_project\"})\n\n\
                 BEST PRACTICES:\n\
                 - Always create directories before writing files to them\n\
                 - Use absolute paths for clarity in production code\n\
                 - Use relative paths for portable scripts\n\
                 - Check the 'created' field to know if directory was new or existed\n\
                 - No need to check if directory exists first - operation is idempotent\n\n\
                 IDEMPOTENCY:\n\
                 - It is safe to call multiple times with the same path (idempotent operation)\n\
                 - If directory already exists, response returns created: false, existing contents preserved\n\
                 - No need to check if directory exists before calling - just create it",
            ),
        },
    ]
}

/// Creating nested directory structures
fn prompt_nested() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create nested directories (like mkdir -p)?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The fs_create_directory tool automatically creates all parent directories in one call, just like mkdir -p in Unix systems.\n\n\
                 CREATING NESTED DIRECTORIES:\n\n\
                 Auto-creates parent directories (like mkdir -p):\n\n\
                 1. Deep nested path:\n\
                    fs_create_directory({\"path\": \"/project/src/components/ui/buttons\"})\n\
                    // Creates: /project, /project/src, /project/src/components, etc.\n\n\
                 2. Multiple levels:\n\
                    fs_create_directory({\"path\": \"./data/2024/01/15/exports\"})\n\
                    // Creates entire hierarchy if any part is missing\n\n\
                 3. Complex project structure:\n\
                    fs_create_directory({\"path\": \"/home/user/workspace/client/app/assets/images\"})\n\n\
                 NO NEED TO CREATE PARENTS FIRST:\n\n\
                 ❌ Manual approach (unnecessary):\n\
                    fs_create_directory({\"path\": \"./data\"})\n\
                    fs_create_directory({\"path\": \"./data/2024\"})\n\
                    fs_create_directory({\"path\": \"./data/2024/01\"})\n\
                    fs_create_directory({\"path\": \"./data/2024/01/15\"})\n\n\
                 ✅ Single call creates all:\n\
                    fs_create_directory({\"path\": \"./data/2024/01/15\"})\n\n\
                 HOW IT WORKS:\n\
                 - Tool analyzes the path and identifies all directory levels\n\
                 - Creates parent directories from top to bottom\n\
                 - Skips directories that already exist\n\
                 - Returns success when final directory is created or verified\n\n\
                 PRACTICAL EXAMPLES:\n\n\
                 1. Year/Month/Day structure for logs:\n\
                    fs_create_directory({\"path\": \"./logs/2024/january/week1\"})\n\n\
                 2. Feature-based code organization:\n\
                    fs_create_directory({\"path\": \"./src/features/authentication/components\"})\n\n\
                 3. Build output with platform/architecture:\n\
                    fs_create_directory({\"path\": \"./build/linux/x86_64/release\"})\n\n\
                 BENEFITS:\n\
                 - Simpler code - one call instead of many\n\
                 - Atomic operation - all or nothing\n\
                 - No race conditions when creating hierarchies\n\
                 - Works regardless of which parents exist\n\
                 - Consistent behavior across platforms\n\n\
                 ERROR HANDLING:\n\
                 - If any part of the path cannot be created, operation fails\n\
                 - Permissions are checked for each level\n\
                 - Clear error messages indicate which directory failed",
            ),
        },
    ]
}


