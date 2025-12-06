//! Prompt messages for memory_check_memorize_status tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::CheckMemorizeStatusPromptArgs;

/// Prompt provider for memory_check_memorize_status tool
///
/// This is the ONLY way to provide prompts for memory_check_memorize_status - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct CheckMemorizeStatusPrompts;

impl PromptProvider for CheckMemorizeStatusPrompts {
    type PromptArgs = CheckMemorizeStatusPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]  // No customization arguments for this tool
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE MEMORY_CHECK_MEMORIZE_STATUS
// ============================================================================

/// Basic usage guide for checking memorization status
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I check if a memorization operation has completed?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The memory_check_memorize_status tool checks the progress of asynchronous memorization operations.\n\n\
                 DESCRIPTION\n\n\
                 This tool verifies whether a memorization operation has completed. Memorization involves\n\
                 embedding generation which can take time for large content. The tool requires a session_id\n\
                 (returned from memory_memorize) and returns one of three status values:\n\n\
                 - COMPLETED: Memorization finished successfully, content is ready for recall\n\
                 - IN_PROGRESS: Still processing, check again after a short delay\n\
                 - FAILED: Operation failed, check error field for details\n\n\
                 Use this tool after calling memory_memorize and before memory_recall when you need to\n\
                 ensure the content has been fully stored and indexed.\n\n\
                 BASIC USAGE\n\n\
                 memory_check_memorize_status({\"session_id\": \"mem_abc123\"})\n\n\
                 Response when completed:\n\
                 {\"session_id\": \"mem_abc123\", \"status\": \"COMPLETED\", \"memory_id\": \"mem_xyz789\",\n\
                  \"library\": \"project-notes\", \"progress\": {\"stage\": \"embedding_complete\",\n\
                  \"files_loaded\": 1, \"total_size_bytes\": 2048}, \"runtime_ms\": 1250}\n\n\
                 Response when in progress:\n\
                 {\"session_id\": \"mem_abc123\", \"status\": \"IN_PROGRESS\", \"library\": \"project-notes\",\n\
                  \"progress\": {\"stage\": \"generating_embedding\", \"percent_complete\": 45},\n\
                  \"runtime_ms\": 500}\n\n\
                 RESPONSE STRUCTURE\n\n\
                 - session_id: The session being checked\n\
                 - status: COMPLETED, IN_PROGRESS, or FAILED\n\
                 - memory_id: Stored memory ID (only present when COMPLETED)\n\
                 - library: Target library name\n\
                 - progress: Current progress details\n\
                   - stage: validating, generating_embedding, storing, indexing, or embedding_complete\n\
                   - percent_complete: Progress percentage (when available)\n\
                   - files_loaded: Number of files processed\n\
                   - total_size_bytes: Size of stored content\n\
                 - error: Error message (only present when FAILED)\n\
                 - runtime_ms: Time elapsed since memorization started\n\n\
                 WHEN TO USE\n\n\
                 - Verify completion before recalling: Check status after storing to ensure content is ready.\n\
                   This prevents recall from returning empty results due to incomplete indexing.\n\
                 - Monitor large content operations: Track progress for large documents or multiple files\n\
                   where embedding generation takes longer.\n\
                 - Debug failed operations: Get error details when memorization fails (service unavailability,\n\
                   content size limits, etc.).\n\
                 - Track batch operations: Verify batch completion when storing multiple memories before\n\
                   proceeding with dependent operations.\n\n\
                 COMMON PATTERN\n\n\
                 Verify before recall:\n\n\
                 // Store content and capture session_id\n\
                 result = memory_memorize({\"library\": \"critical-data\", \"content\": \"...\"})\n\n\
                 // Check if memorization completed\n\
                 status = memory_check_memorize_status({\"session_id\": result.session_id})\n\n\
                 // Only proceed if completed successfully\n\
                 if status.status == \"COMPLETED\":\n\
                     memory_recall({\"library\": \"critical-data\", \"context\": \"configuration\"})\n\
                 else if status.status == \"FAILED\":\n\
                     handle_error(status.error)  // May need to retry\n\
                 else:\n\
                     sleep(500ms)  // Still in progress, check again\n\n\
                 QUICK REFERENCE\n\n\
                 Command: memory_check_memorize_status({\"session_id\": \"...\"})\n\
                 Status values: COMPLETED, IN_PROGRESS, FAILED\n\
                 Progress stages: validating → generating_embedding → storing → indexing → embedding_complete\n\n\
                 Note: Most small content memorizations complete in under 100ms and don't require status\n\
                 checks. Only verify status for large content or when immediate recall is critical.\n\
                 The session_id parameter is required and comes from the memory_memorize response.",
            ),
        },
    ]
}
