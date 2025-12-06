//! Prompt messages for delete_prompt tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::DeletePromptPromptArgs;

/// Prompt provider for delete_prompt tool
///
/// This is the ONLY way to provide prompts for delete_prompt - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct PromptDeletePrompts;

impl PromptProvider for PromptDeletePrompts {
    type PromptArgs = DeletePromptPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("safety") => prompt_safety(),
            _ => prompt_safety(),
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
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO DELETE PROMPTS
// ============================================================================

/// Basic deletion operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I delete prompts using the delete_prompt tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "DELETING PROMPTS:\n\n\
                 1. Delete by name:\n\
                    delete_prompt({\n\
                        \"name\": \"old-prompt\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"name\": \"old-prompt\",\n\
                   \"deleted\": true\n\
                 }\n\n\
                 2. Delete returns info:\n\
                    delete_prompt({\n\
                        \"name\": \"draft-v1\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"name\": \"draft-v1\",\n\
                   \"deleted\": true,\n\
                   \"content\": \"The deleted template content...\"\n\
                 }\n\n\
                 DELETION BEHAVIOR:\n\
                 - Permanent removal\n\
                 - Returns deleted content for reference\n\
                 - No automatic backup",
            ),
        },
    ]
}

/// Safety practices
fn prompt_safety() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are safe deletion practices for prompts?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "SAFE DELETION PRACTICES:\n\n\
                 1. Verify before delete:\n\
                    // Check it's the right prompt\n\
                    get_prompt({ \"name\": \"to-delete\" })\n\n\
                    // Confirm and delete\n\
                    delete_prompt({ \"name\": \"to-delete\" })\n\n\
                 2. Save content first:\n\
                    // Get content for backup\n\
                    result = get_prompt({ \"name\": \"important-prompt\" })\n\
                    // result.content now saved\n\n\
                    // Then delete\n\
                    delete_prompt({ \"name\": \"important-prompt\" })\n\n\
                 3. Check dependencies:\n\
                    // List prompts using this name pattern\n\
                    // Ensure nothing depends on it\n\
                    // Then delete\n\n\
                 4. Delete with caution:\n\
                    // Don't delete if unsure\n\
                    // Better to rename: create new, then delete old\n\n\
                 SAFETY CHECKLIST:\n\
                 ✓ Verify correct prompt name\n\
                 ✓ Check if in use anywhere\n\
                 ✓ Save content if might need later\n\
                 ✓ Delete only when certain\n\n\
                 WARNINGS:\n\
                 - Deletion is permanent\n\
                 - No undo functionality\n\
                 - Check twice, delete once",
            ),
        },
    ]
}


