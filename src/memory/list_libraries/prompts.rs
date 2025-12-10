//! Prompt messages for memory_list_libraries tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::MemoryListLibrariesPromptArgs;

/// Prompt provider for memory_list_libraries tool
///
/// This is the ONLY way to provide prompts for memory_list_libraries - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct MemoryListLibrariesPrompts;

impl PromptProvider for MemoryListLibrariesPrompts {
    type PromptArgs = MemoryListLibrariesPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

/// List all memory library names
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list memory libraries?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "# memory_list_libraries\n\n\
                 Lists all memory library names that have been created.\n\n\
                 ## Usage\n\n\
                 memory_list_libraries({})\n\n\
                 No parameters required. Returns array of library names.\n\n\
                 ## Response Structure\n\n\
                 {\n\
                   \"libraries\": [\"project-notes\", \"code-patterns\", \"decisions\"]\n\
                 }\n\n\
                 - libraries: Array of library name strings\n\
                 - Empty array [] if no libraries exist yet\n\
                 - Libraries are separate namespaces for organizing memories\n\n\
                 ## When to Use\n\n\
                 - **Before memorizing**: Check if library exists, or see what's available\n\
                 - **Before recalling**: Verify the library you want to recall from exists\n\
                 - **Discovery**: See what knowledge domains you have stored\n\
                 - **Debugging**: Understand your current memory organization\n\n\
                 ## Common Pattern\n\n\
                 // Check what libraries exist\n\
                 const {libraries} = memory_list_libraries({});\n\
                 console.log(libraries);  // [\"project-alpha\", \"code-patterns\"]\n\n\
                 // Check if specific library exists\n\
                 if (libraries.includes(\"api-docs\")) {\n\
                   // Library exists, safe to recall\n\
                   memory_recall({\"library\": \"api-docs\", \"context\": \"authentication\"});\n\
                 } else {\n\
                   // Library doesn't exist, create it by memorizing\n\
                   memory_memorize({\"library\": \"api-docs\", \"content\": \"API documentation...\"});\n\
                 }\n\n\
                 // Verify new library was created\n\
                 const updated = memory_list_libraries({});\n\
                 console.log(updated.libraries);  // [\"project-alpha\", \"code-patterns\", \"api-docs\"]\n\n\
                 ## Quick Reference\n\n\
                 Command: memory_list_libraries({})\n\
                 Returns: {\"libraries\": [\"name1\", \"name2\"]}\n\
                 Parameters: None\n\
                 Related: memory_memorize, memory_recall\n\n\
                 Libraries are created automatically when you first memorize to them.",
            ),
        },
    ]
}
