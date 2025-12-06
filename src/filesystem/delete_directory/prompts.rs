//! Prompt messages for fs_delete_directory tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::FsDeleteDirectoryPromptArgs;

/// Prompt provider for fs_delete_directory tool
///
/// This is the ONLY way to provide prompts for fs_delete_directory - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct DeleteDirectoryPrompts;

impl PromptProvider for DeleteDirectoryPrompts {
    type PromptArgs = FsDeleteDirectoryPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("safety") => prompt_safety(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, safety)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO SAFELY DELETE DIRECTORIES
// ============================================================================

/// Basic directory deletion with safety confirmation
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I delete a directory and all its contents?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The fs_delete_directory tool permanently deletes directories and all their contents. This operation is IRREVERSIBLE.\n\n\
                 ⚠️ DELETION IS PERMANENT AND IRREVERSIBLE ⚠️\n\n\
                 BASIC DELETION:\n\
                 1. Delete directory with confirmation:\n\
                    fs_delete_directory({\n\
                        \"path\": \"/project/temp\",\n\
                        \"recursive\": true\n\
                    })\n\n\
                 2. recursive=true is REQUIRED:\n\
                    // Without recursive, deletion fails:\n\
                    fs_delete_directory({\"path\": \"/project/temp\"})\n\
                    // ERROR: Must specify recursive=true\n\n\
                    // With recursive=false, deletion fails:\n\
                    fs_delete_directory({\"path\": \"/project/temp\", \"recursive\": false})\n\
                    // ERROR: Must specify recursive=true\n\n\
                 WHY REQUIRE recursive=true:\n\
                 - Forces explicit confirmation\n\
                 - Prevents accidental deletion\n\
                 - Acknowledges \"I understand this deletes everything inside\"\n\
                 - Similar to typing 'rm -rf' - intentional friction for safety\n\n\
                 WHAT GETS DELETED:\n\
                 - The directory itself\n\
                 - ALL files inside\n\
                 - ALL subdirectories (recursively)\n\
                 - ALL contents of subdirectories\n\
                 - Everything is gone permanently\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"path\": \"/project/temp\",\n\
                   \"deleted\": true\n\
                 }\n\n\
                 CRITICAL WARNINGS:\n\
                 - Deletion is PERMANENT - no undo\n\
                 - No trash/recycle bin\n\
                 - No recovery option\n\
                 - Always verify path before deletion\n\
                 - Consider alternatives (move/archive) instead\n\n\
                 COMMON USE CASES:\n\
                 - Deleting temporary build directories\n\
                 - Removing cache directories\n\
                 - Cleaning up test output directories\n\
                 - Removing confirmed disposable content\n\n\
                 WHEN TO BE EXTRA CAREFUL:\n\
                 - Source code directories\n\
                 - Configuration directories\n\
                 - User data directories\n\
                 - Anything without backup\n\
                 - Production directories",
            ),
        },
    ]
}

/// Safety confirmation and understanding recursive requirement
fn prompt_safety() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Why does fs_delete_directory require recursive=true? What are the safety implications?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The recursive=true requirement is intentional friction to prevent catastrophic accidental deletions.\n\n\
                 SAFETY CONFIRMATION:\n\n\
                 recursive=true is intentional friction:\n\n\
                 MUST explicitly confirm recursive deletion:\n\
                 fs_delete_directory({\n\
                     \"path\": \"/path/to/dir\",\n\
                     \"recursive\": true  // Required confirmation\n\
                 })\n\n\
                 WITHOUT recursive=true:\n\
                 - Tool will error\n\
                 - Prevents accidental rm -rf style disasters\n\
                 - Forces you to acknowledge danger\n\
                 - Makes you think twice before deleting\n\n\
                 WHAT GETS DELETED:\n\
                 When you set recursive=true, you confirm understanding that:\n\
                 - The directory itself will be deleted\n\
                 - ALL files inside (every single one)\n\
                 - ALL subdirectories (recursively, no matter how deep)\n\
                 - ALL contents of subdirectories (files, links, everything)\n\
                 - Hidden files and directories (starting with .)\n\
                 - System files if you have permission\n\n\
                 THERE IS NO UNDO:\n\
                 - No trash/recycle bin\n\
                 - No recovery option\n\
                 - No confirmation dialog\n\
                 - No way to get files back\n\
                 - Gone permanently from filesystem\n\n\
                 EXAMPLES OF WHAT CAN GO WRONG:\n\n\
                 1. Typo in path:\n\
                    // Intended: /project/temp\n\
                    fs_delete_directory({\"path\": \"/project\", \"recursive\": true})\n\
                    // DISASTER: Deleted entire project!\n\n\
                 2. Wrong nesting level:\n\
                    // Intended: /home/user/project/build\n\
                    fs_delete_directory({\"path\": \"/home/user/project\", \"recursive\": true})\n\
                    // DISASTER: Deleted entire project instead of build dir!\n\n\
                 3. Autocomplete mistake:\n\
                    // Intended: /data/temp\n\
                    fs_delete_directory({\"path\": \"/data\", \"recursive\": true})\n\
                    // DISASTER: Deleted all data!\n\n\
                 HOW recursive=true HELPS:\n\
                 - Makes you stop and think\n\
                 - Forces explicit confirmation\n\
                 - You can't accidentally delete by omission\n\
                 - Provides moment to double-check path\n\
                 - Similar to sudo password - friction for safety\n\n\
                 BEST PRACTICES:\n\
                 1. ALWAYS verify path first with fs_list_directory\n\
                 2. Read the full path carefully before confirming\n\
                 3. Check for typos in directory names\n\
                 4. Verify you're at the right nesting level\n\
                 5. Consider using fs_move_file to archive instead\n\
                 6. Make sure you have backups if data is important\n\
                 7. Test on a copy first if unsure\n\n\
                 COMPARISON TO SHELL:\n\
                 fs_delete_directory with recursive=true is equivalent to:\n\
                 - rm -rf /path/to/dir (Unix/Linux/Mac)\n\
                 - rmdir /s /q /path/to/dir (Windows)\n\n\
                 Both are destructive and permanent. The recursive=true requirement\n\
                 forces the same level of intentionality as typing 'rm -rf'.",
            ),
        },
    ]
}






