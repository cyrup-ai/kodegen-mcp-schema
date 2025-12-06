//! Prompt messages for fs_read_multiple_files tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::FsReadMultipleFilesPromptArgs;

/// Prompt provider for fs_read_multiple_files tool
///
/// This is the ONLY way to provide prompts for fs_read_multiple_files - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ReadMultipleFilesPrompts;

impl PromptProvider for ReadMultipleFilesPrompts {
    type PromptArgs = FsReadMultipleFilesPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Basic usage guide (only scenario available)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO READ MULTIPLE FILES
// ============================================================================

/// Basic multiple file reading
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I read multiple files at once?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The fs_read_multiple_files tool reads multiple files in parallel for better performance. Here's how to use it:\n\n\
                 READING MULTIPLE FILES:\n\n\
                 Basic usage:\n\
                 fs_read_multiple_files({\n\
                     \"paths\": [\n\
                         \"/project/src/main.rs\",\n\
                         \"/project/src/lib.rs\",\n\
                         \"/project/Cargo.toml\"\n\
                     ]\n\
                 })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"files_requested\": 3,\n\
                   \"files_read\": 3,\n\
                   \"files_failed\": 0,\n\
                   \"results\": [\n\
                     {\n\
                       \"path\": \"/project/src/main.rs\",\n\
                       \"success\": true,\n\
                       \"content\": \"...file content...\",\n\
                       \"mime_type\": \"text/x-rust\",\n\
                       \"total_lines\": 150,\n\
                       \"lines_read\": 150,\n\
                       \"is_partial\": false\n\
                     },\n\
                     {\n\
                       \"path\": \"/project/src/lib.rs\",\n\
                       \"success\": true,\n\
                       \"content\": \"...file content...\",\n\
                       \"mime_type\": \"text/x-rust\",\n\
                       \"total_lines\": 200,\n\
                       \"lines_read\": 200,\n\
                       \"is_partial\": false\n\
                     },\n\
                     {\n\
                       \"path\": \"/project/Cargo.toml\",\n\
                       \"success\": true,\n\
                       \"content\": \"...file content...\",\n\
                       \"mime_type\": \"text/x-toml\",\n\
                       \"total_lines\": 25,\n\
                       \"lines_read\": 25,\n\
                       \"is_partial\": false\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 WHY USE MULTIPLE FILE READING:\n\
                 - More efficient than sequential reads (parallel execution)\n\
                 - Single tool call vs N separate calls\n\
                 - Files are read concurrently\n\
                 - Perfect for reading related files together\n\
                 - Reduces round-trip overhead\n\n\
                 WHEN TO USE:\n\
                 - Reading 2 or more related files\n\
                 - Loading all files in a module\n\
                 - Comparing multiple files\n\
                 - Batch processing file contents\n\
                 - Any time you need multiple files at once\n\n\
                 WHEN TO USE fs_read_file INSTEAD:\n\
                 - Reading a single file\n\
                 - Need to process files sequentially\n\
                 - Files are unrelated\n\n\
                 PARAMETERS:\n\
                 - paths (required): Array of file paths to read\n\
                 - offset (optional): Line number to start reading from (0-based)\n\
                   - Positive: Start from line N\n\
                   - Negative: Read last N lines from end (tail behavior)\n\
                 - length (optional): Maximum number of lines to read per file\n\
                   - Applied to ALL files\n\
                   - Ignored when offset is negative\n\n\
                 RESPONSE FIELDS:\n\
                 Top-level summary:\n\
                 - success: Overall operation success (true even if some files fail)\n\
                 - files_requested: Total number of files requested\n\
                 - files_read: Number of files successfully read\n\
                 - files_failed: Number of files that failed\n\
                 - results: Array of per-file results\n\n\
                 Per-file result:\n\
                 - path: File path\n\
                 - success: Whether this file was read successfully\n\
                 - content: File content (if successful)\n\
                 - error: Error message (if failed)\n\
                 - mime_type: Detected MIME type\n\
                 - total_lines: Total lines in file\n\
                 - lines_read: Number of lines read\n\
                 - is_partial: Whether this is a partial read\n\n\
                 EXAMPLES:\n\
                 1. Read all source files:\n\
                    fs_read_multiple_files({\n\
                        \"paths\": [\n\
                            \"/project/src/main.rs\",\n\
                            \"/project/src/lib.rs\",\n\
                            \"/project/src/utils.rs\"\n\
                        ]\n\
                    })\n\n\
                 2. Read with offset (skip first 10 lines of each file):\n\
                    fs_read_multiple_files({\n\
                        \"paths\": [\"/file1.txt\", \"/file2.txt\"],\n\
                        \"offset\": 10\n\
                    })\n\n\
                 3. Read last 50 lines of each file:\n\
                    fs_read_multiple_files({\n\
                        \"paths\": [\"/log1.txt\", \"/log2.txt\"],\n\
                        \"offset\": -50\n\
                    })\n\n\
                 4. Read first 100 lines of each file:\n\
                    fs_read_multiple_files({\n\
                        \"paths\": [\"/file1.txt\", \"/file2.txt\"],\n\
                        \"offset\": 0,\n\
                        \"length\": 100\n\
                    })",
            ),
        },
    ]
}


