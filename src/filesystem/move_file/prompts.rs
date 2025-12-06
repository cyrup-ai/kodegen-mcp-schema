//! Prompt messages for fs_move_file tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::FsMoveFilePromptArgs;

/// Prompt provider for fs_move_file tool
///
/// This is the ONLY way to provide prompts for fs_move_file - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct MoveFilePrompts;

impl PromptProvider for MoveFilePrompts {
    type PromptArgs = FsMoveFilePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("rename") => prompt_rename(),
            Some("relocate") => prompt_relocate(),
            _ => prompt_rename(),  // Default to rename scenario
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (rename, relocate)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO MOVE AND RENAME FILES
// ============================================================================

/// Renaming files and directories in the same location
fn prompt_rename() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I rename files and directories using fs_move_file?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The fs_move_file tool renames files and directories by moving them to a new name in the same directory.\n\n\
                 RENAMING FILES:\n\n\
                 1. Rename file in same directory:\n\
                    fs_move_file({\n\
                        \"source\": \"/project/old_name.rs\",\n\
                        \"destination\": \"/project/new_name.rs\"\n\
                    })\n\
                    - Source and destination are in same directory (/project)\n\
                    - Only the filename changes (old_name.rs → new_name.rs)\n\
                    - File contents remain unchanged\n\n\
                 2. Rename directory:\n\
                    fs_move_file({\n\
                        \"source\": \"/project/old_module\",\n\
                        \"destination\": \"/project/new_module\"\n\
                    })\n\
                    - Renames entire directory and all contents\n\
                    - All files inside maintain relative paths\n\
                    - Atomic operation on same filesystem\n\n\
                 3. Change file extension:\n\
                    fs_move_file({\n\
                        \"source\": \"/project/data.txt\",\n\
                        \"destination\": \"/project/data.json\"\n\
                    })\n\
                    - Changes extension from .txt to .json\n\
                    - File contents unchanged (you may need to convert format separately)\n\
                    - Useful for file format conversions\n\n\
                 RENAME DEFINITION:\n\
                 - RENAME = same directory, different name\n\
                 - Both source and destination have identical parent directory\n\
                 - Only the filename/directory name component changes\n\n\
                 COMMON RENAME PATTERNS:\n\n\
                 1. Add prefix to filename:\n\
                    fs_move_file({\n\
                        \"source\": \"/data/report.pdf\",\n\
                        \"destination\": \"/data/final_report.pdf\"\n\
                    })\n\n\
                 2. Add suffix or timestamp:\n\
                    fs_move_file({\n\
                        \"source\": \"/logs/app.log\",\n\
                        \"destination\": \"/logs/app_2024.log\"\n\
                    })\n\n\
                 3. Normalize filename (fix spacing, casing):\n\
                    fs_move_file({\n\
                        \"source\": \"/docs/My Document.txt\",\n\
                        \"destination\": \"/docs/my_document.txt\"\n\
                    })\n\n\
                 RENAME BEHAVIOR:\n\
                 - Operation is atomic on same filesystem\n\
                 - If destination exists, operation fails (won't overwrite)\n\
                 - Parent directory must exist\n\
                 - Works for files, directories, and symlinks\n\
                 - Preserves file permissions and ownership\n\
                 - Preserves timestamps (modified, accessed)\n\n\
                 ERROR CASES:\n\
                 - Source doesn't exist → Error\n\
                 - Destination already exists → Error (won't overwrite)\n\
                 - No permission to modify directory → Error\n\
                 - Destination path invalid → Error\n\n\
                 BEST PRACTICES:\n\
                 1. Check source exists before renaming (use fs_get_file_info)\n\
                 2. Verify destination doesn't exist\n\
                 3. Use descriptive names that indicate content/purpose\n\
                 4. Avoid spaces in filenames (use underscores or hyphens)\n\
                 5. Keep extensions accurate (.rs for Rust, .json for JSON, etc.)",
            ),
        },
    ]
}

/// Moving files to different locations
fn prompt_relocate() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I move files and directories to different locations?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The fs_move_file tool relocates files and directories to different locations in the filesystem.\n\n\
                 MOVING FILES TO DIFFERENT DIRECTORIES:\n\n\
                 1. Move file to different directory (keep same name):\n\
                    fs_move_file({\n\
                        \"source\": \"/project/temp/output.json\",\n\
                        \"destination\": \"/project/data/output.json\"\n\
                    })\n\
                    - Moves from /project/temp/ to /project/data/\n\
                    - Filename stays the same (output.json)\n\
                    - Destination directory MUST exist first\n\n\
                 2. Move directory with all contents:\n\
                    fs_move_file({\n\
                        \"source\": \"/project/src/old_module\",\n\
                        \"destination\": \"/project/archive/old_module\"\n\
                    })\n\
                    - Moves entire directory tree\n\
                    - All files and subdirectories move together\n\
                    - Preserves directory structure\n\n\
                 3. Move and rename in single operation:\n\
                    fs_move_file({\n\
                        \"source\": \"/downloads/report.pdf\",\n\
                        \"destination\": \"/project/docs/q4_report.pdf\"\n\
                    })\n\
                    - Changes both location (/downloads → /project/docs)\n\
                    - Changes name (report.pdf → q4_report.pdf)\n\
                    - Efficient single operation\n\n\
                 DESTINATION DIRECTORY REQUIREMENTS:\n\
                 - Destination parent directory MUST exist\n\
                 - Cannot create intermediate directories automatically\n\
                 - Use fs_create_directory first if needed\n\n\
                 Example workflow:\n\
                 // Create destination directory first\n\
                 fs_create_directory({\"path\": \"/project/archive\"})\n\
                 // Then move file\n\
                 fs_move_file({\n\
                     \"source\": \"/project/old_data.json\",\n\
                     \"destination\": \"/project/archive/old_data.json\"\n\
                 })\n\n\
                 OVERWRITE BEHAVIOR:\n\
                 - Cannot overwrite existing files by default\n\
                 - If destination exists, operation fails\n\
                 - Must delete or move existing file first\n\n\
                 Example:\n\
                 // This will fail if /backup/data.json exists\n\
                 fs_move_file({\n\
                     \"source\": \"/tmp/data.json\",\n\
                     \"destination\": \"/backup/data.json\"\n\
                 })\n\
                 // Solution: delete or rename existing file first\n\
                 fs_delete_file({\"path\": \"/backup/data.json\"})\n\
                 fs_move_file({\n\
                     \"source\": \"/tmp/data.json\",\n\
                     \"destination\": \"/backup/data.json\"\n\
                 })\n\n\
                 CROSS-FILESYSTEM MOVES:\n\
                 When moving across different filesystems/drives:\n\
                 fs_move_file({\n\
                     \"source\": \"/mnt/drive1/data.db\",\n\
                     \"destination\": \"/mnt/drive2/data.db\"\n\
                 })\n\
                 - Not atomic (copy + delete operation)\n\
                 - Slower than same-filesystem moves\n\
                 - May fail mid-operation (if space runs out, etc.)\n\
                 - Original file preserved if copy fails\n\n\
                 SAME-FILESYSTEM MOVES:\n\
                 fs_move_file({\n\
                     \"source\": \"/home/user/file.txt\",\n\
                     \"destination\": \"/home/archive/file.txt\"\n\
                 })\n\
                 - Atomic operation\n\
                 - Nearly instantaneous (just updates metadata)\n\
                 - Safe from interruption\n\
                 - Original file never at risk\n\n\
                 COMMON RELOCATION PATTERNS:\n\n\
                 1. Move to archive:\n\
                    fs_move_file({\n\
                        \"source\": \"/project/2023_data.csv\",\n\
                        \"destination\": \"/archive/2023_data.csv\"\n\
                    })\n\n\
                 2. Move from downloads:\n\
                    fs_move_file({\n\
                        \"source\": \"/downloads/library.pdf\",\n\
                        \"destination\": \"/documents/books/library.pdf\"\n\
                    })\n\n\
                 3. Move to backup location:\n\
                    fs_move_file({\n\
                        \"source\": \"/data/important.db\",\n\
                        \"destination\": \"/backups/2024/important.db\"\n\
                    })\n\n\
                 4. Move temporary to permanent:\n\
                    fs_move_file({\n\
                        \"source\": \"/tmp/processing/result.json\",\n\
                        \"destination\": \"/project/results/result.json\"\n\
                    })\n\n\
                 ERROR CASES:\n\
                 - Source doesn't exist → Error\n\
                 - Destination parent directory doesn't exist → Error\n\
                 - Destination file already exists → Error\n\
                 - No permission for source or destination → Error\n\n\
                 BEST PRACTICES:\n\
                 1. Create destination directories first (fs_create_directory)\n\
                 2. Check source exists (fs_get_file_info)\n\
                 3. Verify destination doesn't exist\n\
                 4. Use absolute paths for clarity\n\
                 5. For cross-filesystem moves, ensure enough space\n\
                 6. Consider atomic operations (same filesystem preferred)\n\
                 7. Keep backups before moving critical files",
            ),
        },
    ]
}
