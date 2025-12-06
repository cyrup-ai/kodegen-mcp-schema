//! Prompt messages for fs_delete_file tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::FsDeleteFilePromptArgs;

/// Prompt provider for fs_delete_file tool
///
/// This is the ONLY way to provide prompts for fs_delete_file - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct DeleteFilePrompts;

impl PromptProvider for DeleteFilePrompts {
    type PromptArgs = FsDeleteFilePromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

/// Basic file deletion usage
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I delete a file using fs_delete_file?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The fs_delete_file tool permanently deletes a file from the filesystem. Here's how to use it:\n\n\
                 BASIC USAGE:\n\
                 fs_delete_file({\"path\": \"/project/temp.log\"})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"path\": \"/project/temp.log\",\n\
                   \"deleted\": true\n\
                 }\n\n\
                 CRITICAL WARNINGS:\n\
                 - Deletion is PERMANENT and IRREVERSIBLE\n\
                 - There is NO undo operation\n\
                 - The file is immediately removed from the filesystem\n\
                 - Cannot recover deleted files through this tool\n\n\
                 FILE VS DIRECTORY:\n\
                 - fs_delete_file: Only deletes FILES\n\
                 - fs_delete_directory: Use for directories\n\
                 - Attempting to delete a directory with this tool will ERROR\n\n\
                 EXAMPLE - Deleting a temporary file:\n\
                 fs_delete_file({\"path\": \"/tmp/output.log\"})\n\
                 Result: File is permanently removed\n\n\
                 EXAMPLE - Deleting a build artifact:\n\
                 fs_delete_file({\"path\": \"./target/debug/app.exe\"})\n\
                 Result: Binary is deleted\n\n\
                 EXAMPLE - Wrong usage (directory):\n\
                 fs_delete_file({\"path\": \"/tmp/cache/\"})\n\
                 Result: ERROR - Cannot delete directory\n\
                 Solution: Use fs_delete_directory({\"path\": \"/tmp/cache/\", \"recursive\": true})\n\n\
                 PARAMETERS:\n\
                 - path (required): Absolute or relative path to the file to delete\n\n\
                 RESPONSE FIELDS:\n\
                 - success: true if deletion succeeded, false otherwise\n\
                 - path: The path that was deleted\n\
                 - deleted: true if file was removed\n\n\
                 ERROR CASES:\n\
                 1. File doesn't exist:\n\
                    Error: No such file or directory\n\
                 2. Path is a directory:\n\
                    Error: Is a directory (use fs_delete_directory)\n\
                 3. Permission denied:\n\
                    Error: Permission denied\n\
                 4. File is in use:\n\
                    Error: Resource busy or locked\n\n\
                 COMMON USE CASES:\n\
                 - Deleting temporary files after processing\n\
                 - Removing old log files\n\
                 - Cleaning up build artifacts\n\
                 - Removing cache files\n\
                 - Deleting generated output files\n\n\
                 REMEMBER:\n\
                 - Always verify the path before deletion\n\
                 - Deletion is permanent - no recovery\n\
                 - Use fs_get_file_info to check file exists first\n\
                 - Consider alternatives like moving or renaming instead",
            ),
        },
    ]
}


