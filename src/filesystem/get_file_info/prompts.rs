//! Prompt messages for fs_get_file_info tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::FsGetFileInfoPromptArgs;

/// Prompt provider for fs_get_file_info tool
///
/// This is the ONLY way to provide prompts for fs_get_file_info - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GetFileInfoPrompts;

impl PromptProvider for GetFileInfoPrompts {
    type PromptArgs = FsGetFileInfoPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        // All scenarios route to prompt_basic as single source of truth
        let _ = args;
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![] // No scenario selection needed, single scenario
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO GET FILE METADATA
// ============================================================================

/// Basic file metadata retrieval
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I get file metadata using fs_get_file_info?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The fs_get_file_info tool retrieves detailed metadata about a file or directory without reading its content. Here's how to use it:\n\n\
                 GET FILE METADATA:\n\
                 fs_get_file_info({\"path\": \"/project/src/main.rs\"})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"path\": \"/project/src/main.rs\",\n\
                   \"type\": \"file\",\n\
                   \"size_bytes\": 4523,\n\
                   \"line_count\": 150,\n\
                   \"created_at\": \"2024-01-10T14:30:00Z\",\n\
                   \"modified_at\": \"2024-01-15T09:45:00Z\",\n\
                   \"accessed_at\": \"2024-01-15T10:00:00Z\",\n\
                   \"permissions\": \"rw-r--r--\",\n\
                   \"is_readonly\": false\n\
                 }\n\n\
                 FIELDS EXPLAINED:\n\
                 - type: \"file\" or \"directory\"\n\
                 - size_bytes: File size in bytes\n\
                 - line_count: Number of lines (for text files, null for binary/directories)\n\
                 - created_at: File creation timestamp (ISO 8601 UTC)\n\
                 - modified_at: Last modification timestamp (ISO 8601 UTC)\n\
                 - accessed_at: Last access timestamp (ISO 8601 UTC)\n\
                 - permissions: Unix-style permission string (e.g., \"rw-r--r--\")\n\
                 - is_readonly: Boolean indicating if file is read-only\n\n\
                 DIRECTORY METADATA:\n\
                 fs_get_file_info({\"path\": \"/project/src\"})\n\
                 {\n\
                   \"path\": \"/project/src\",\n\
                   \"type\": \"directory\",\n\
                   \"size_bytes\": 4096,\n\
                   \"line_count\": null,\n\
                   \"created_at\": \"2024-01-10T14:30:00Z\",\n\
                   \"modified_at\": \"2024-01-15T09:45:00Z\",\n\
                   \"accessed_at\": \"2024-01-15T10:00:00Z\",\n\
                   \"permissions\": \"rwxr-xr-x\",\n\
                   \"is_readonly\": false\n\
                 }\n\n\
                 KEY DIFFERENCES:\n\
                 - Directories have type: \"directory\"\n\
                 - line_count is always null for directories\n\
                 - size_bytes for directories represents metadata size, not contents\n\n\
                 WHEN TO USE:\n\
                 - Check file exists before reading\n\
                 - Verify file size before operations\n\
                 - Compare modification times\n\
                 - Check permissions before writing\n\
                 - Get metadata without reading entire file\n\n\
                 COMMON PATTERNS:\n\
                 1. Check if file exists:\n\
                    info = fs_get_file_info({\"path\": \"/config.json\"})\n\
                    // If successful, file exists\n\n\
                 2. Get file size:\n\
                    info = fs_get_file_info({\"path\": \"/data.csv\"})\n\
                    size = info.size_bytes\n\n\
                 3. Check last modified:\n\
                    info = fs_get_file_info({\"path\": \"/cache/results.json\"})\n\
                    modified = info.modified_at\n\n\
                 4. Verify file type:\n\
                    info = fs_get_file_info({\"path\": \"/project/data\"})\n\
                    if info.type == \"directory\":\n\
                        // Use directory operations\n\
                    else:\n\
                        // Use file operations",
            ),
        },
    ]
}
