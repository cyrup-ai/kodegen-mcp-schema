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

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE MEMORY_LIST_LIBRARIES
// ============================================================================

/// Basic library listing and understanding namespaces
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list all memory libraries and understand what they are?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The memory_list_libraries tool shows all available memory libraries that have been created. Libraries are separate namespaces for organizing memories.\n\n\
                 LISTING MEMORY LIBRARIES:\n\n\
                 1. Get all libraries:\n\
                    memory_list_libraries({})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"libraries\": [\n\
                     \"project-notes\",\n\
                     \"code-patterns\",\n\
                     \"decisions\"\n\
                   ]\n\
                 }\n\n\
                 WHAT ARE LIBRARIES?\n\
                 - Libraries are separate namespaces for memories\n\
                 - Each library has a unique name\n\
                 - Memories within one library don't affect other libraries\n\
                 - Use different libraries for different contexts\n\n\
                 LIBRARY NAMING:\n\
                 - Library names are strings (e.g., \"project-notes\", \"code_patterns\")\n\
                 - Names should be descriptive and meaningful\n\
                 - Common naming patterns:\n\
                   - Kebab-case: \"project-notes\", \"api-patterns\"\n\
                   - Snake_case: \"project_notes\", \"api_patterns\"\n\
                   - CamelCase: \"ProjectNotes\", \"ApiPatterns\"\n\n\
                 WHEN TO LIST LIBRARIES:\n\
                 1. Before storing a new memory:\n\
                    // Check what libraries exist\n\
                    memory_list_libraries({})\n\
                    // Then choose or create one\n\
                    memory_memorize({\"library\": \"project-notes\", \"content\": \"...\"})\n\n\
                 2. Before recalling memories:\n\
                    // See available libraries\n\
                    memory_list_libraries({})\n\
                    // Then recall from specific library\n\
                    memory_recall({\"library\": \"project-notes\", \"context\": \"...\"})\n\n\
                 3. To understand your memory organization:\n\
                    // What libraries do I have?\n\
                    memory_list_libraries({})\n\
                    // Returns list of all library names\n\n\
                 4. To check if a library exists:\n\
                    const result = memory_list_libraries({});\n\
                    const hasProjectNotes = result.libraries.includes(\"project-notes\");\n\n\
                 EMPTY RESPONSE:\n\
                 If no libraries exist yet:\n\
                 {\n\
                   \"libraries\": []\n\
                 }\n\
                 This means no memories have been stored yet.\n\n\
                 LIBRARY LIFECYCLE:\n\
                 - Libraries are created automatically when you first memorize to them\n\
                 - No need to explicitly create a library\n\
                 - Libraries appear in the list after first use\n\
                 - Example:\n\
                   memory_list_libraries({})  // Returns: {\"libraries\": []}\n\
                   memory_memorize({\"library\": \"new-lib\", \"content\": \"...\"})  // Creates library\n\
                   memory_list_libraries({})  // Returns: {\"libraries\": [\"new-lib\"]}\n\n\
                 USE CASES:\n\n\
                 1. DISCOVERY - See what knowledge is available:\n\
                    memory_list_libraries({})\n\
                    // Shows all libraries you can recall from\n\n\
                 2. VERIFICATION - Check if library exists before recalling:\n\
                    const libs = memory_list_libraries({});\n\
                    if (libs.libraries.includes(\"api-docs\")) {\n\
                      memory_recall({\"library\": \"api-docs\", \"context\": \"authentication\"});\n\
                    }\n\n\
                 3. ORGANIZATION - Understand your memory structure:\n\
                    memory_list_libraries({})\n\
                    // Returns: [\"project-A\", \"project-B\", \"general-knowledge\"]\n\
                    // Shows you have 3 separate knowledge domains\n\n\
                 4. CHOOSING LIBRARY - Decide where to store new memory:\n\
                    const libs = memory_list_libraries({});\n\
                    // If \"code-reviews\" exists, use it. Otherwise create it.\n\
                    memory_memorize({\"library\": \"code-reviews\", \"content\": \"...\"});\n\n\
                 PARAMETERS:\n\
                 - No parameters required (empty object: {})\n\
                 - Always returns list of library names\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"libraries\": [\"name1\", \"name2\", \"name3\"]\n\
                 }\n\
                 - libraries: Array of library name strings\n\
                 - Alphabetically sorted (implementation may vary)\n\
                 - Empty array if no libraries exist\n\n\
                 COMMON PATTERNS:\n\n\
                 1. List before storing:\n\
                    memory_list_libraries({})  // See what exists\n\
                    memory_memorize({\"library\": \"chosen-lib\", \"content\": \"...\"})  // Store\n\n\
                 2. List before recalling:\n\
                    memory_list_libraries({})  // See what's available\n\
                    memory_recall({\"library\": \"chosen-lib\", \"context\": \"...\"})  // Recall\n\n\
                 3. Check existence:\n\
                    const {libraries} = memory_list_libraries({});\n\
                    const exists = libraries.includes(\"target-lib\");\n\n\
                 4. Iterate over all libraries:\n\
                    const {libraries} = memory_list_libraries({});\n\
                    for (const lib of libraries) {\n\
                      console.log(`Library: ${lib}`);\n\
                      // Could recall from each library if needed\n\
                    }\n\n\
                 BEST PRACTICES:\n\
                 - List libraries when you're unsure what exists\n\
                 - Use descriptive library names for clarity\n\
                 - Organize memories by project, topic, or purpose\n\
                 - Check library existence before assuming it's there\n\
                 - Use consistent naming conventions across libraries",
            ),
        },
    ]
}

/// Memory organization workflows and patterns
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are the best practices for organizing memories with libraries?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Memory libraries enable powerful organizational patterns. Use memory_list_libraries to discover, verify, and manage your knowledge organization.\n\n\
                 =============================================================================\n\
                 MEMORY ORGANIZATION WORKFLOWS\n\
                 =============================================================================\n\n\
                 WORKFLOW 1: DISCOVERY-FIRST PATTERN\n\
                 Start by seeing what knowledge already exists:\n\n\
                 1. Check existing libraries:\n\
                    memory_list_libraries({})\n\
                    // Returns: {\"libraries\": [\"project-notes\", \"code-patterns\"]}\n\n\
                 2. Decide where to store based on what exists:\n\
                    // If \"project-notes\" exists, add to it\n\
                    memory_memorize({\n\
                        \"library\": \"project-notes\",\n\
                        \"content\": \"New project insight...\"\n\
                    })\n\
                    // Or create new library for different context\n\
                    memory_memorize({\n\
                        \"library\": \"meeting-notes\",\n\
                        \"content\": \"Meeting summary...\"\n\
                    })\n\n\
                 3. Verify new library was created:\n\
                    memory_list_libraries({})\n\
                    // Returns: {\"libraries\": [\"project-notes\", \"code-patterns\", \"meeting-notes\"]}\n\n\
                 WORKFLOW 2: VERIFICATION-BEFORE-RECALL\n\
                 Always check library exists before recalling:\n\n\
                 1. List available libraries:\n\
                    const {libraries} = memory_list_libraries({});\n\n\
                 2. Check if target library exists:\n\
                    if (libraries.includes(\"api-documentation\")) {\n\
                      // Library exists, safe to recall\n\
                      memory_recall({\n\
                        \"library\": \"api-documentation\",\n\
                        \"context\": \"authentication endpoints\"\n\
                      });\n\
                    } else {\n\
                      // Library doesn't exist, handle appropriately\n\
                      console.log(\"No API documentation stored yet\");\n\
                    }\n\n\
                 3. Fallback to different library or create new one:\n\
                    if (!libraries.includes(\"api-documentation\")) {\n\
                      // Store initial documentation\n\
                      memory_memorize({\n\
                        \"library\": \"api-documentation\",\n\
                        \"content\": \"Initial API docs...\"\n\
                      });\n\
                    }\n\n\
                 WORKFLOW 3: ORGANIZE BY PROJECT\n\
                 Use separate libraries for each project:\n\n\
                 Projects:\n\
                 - \"project-alpha\": All knowledge about Project Alpha\n\
                 - \"project-beta\": All knowledge about Project Beta\n\
                 - \"project-gamma\": All knowledge about Project Gamma\n\n\
                 Usage:\n\
                 1. List projects:\n\
                    memory_list_libraries({})\n\
                    // Shows all project libraries\n\n\
                 2. Store project-specific knowledge:\n\
                    memory_memorize({\n\
                      \"library\": \"project-alpha\",\n\
                      \"content\": \"Project Alpha uses React and TypeScript...\"\n\
                    })\n\n\
                 3. Recall from specific project:\n\
                    memory_recall({\n\
                      \"library\": \"project-alpha\",\n\
                      \"context\": \"frontend architecture\"\n\
                    })\n\n\
                 WORKFLOW 4: ORGANIZE BY TOPIC\n\
                 Use libraries for knowledge domains:\n\n\
                 Topics:\n\
                 - \"code-patterns\": Reusable code patterns and best practices\n\
                 - \"architecture-decisions\": Architecture and design decisions\n\
                 - \"debugging-notes\": Common bugs and solutions\n\
                 - \"api-integrations\": Third-party API integration details\n\
                 - \"deployment-procedures\": Deployment and operations knowledge\n\n\
                 Usage:\n\
                 1. See all topics:\n\
                    memory_list_libraries({})\n\n\
                 2. Store topic-specific knowledge:\n\
                    memory_memorize({\n\
                      \"library\": \"code-patterns\",\n\
                      \"content\": \"Factory pattern implementation in Rust...\"\n\
                    })\n\n\
                 3. Recall from topic:\n\
                    memory_recall({\n\
                      \"library\": \"code-patterns\",\n\
                      \"context\": \"factory pattern\"\n\
                    })\n\n\
                 WORKFLOW 5: ORGANIZE BY TIME PERIOD\n\
                 Use libraries for temporal organization:\n\n\
                 Time-based libraries:\n\
                 - \"q1-2024\": First quarter 2024 knowledge\n\
                 - \"q2-2024\": Second quarter 2024 knowledge\n\
                 - \"sprint-23\": Sprint 23 specific information\n\
                 - \"release-v2\": Release 2.0 related knowledge\n\n\
                 Usage:\n\
                 1. Check current period libraries:\n\
                    memory_list_libraries({})\n\n\
                 2. Store time-bound knowledge:\n\
                    memory_memorize({\n\
                      \"library\": \"q1-2024\",\n\
                      \"content\": \"Q1 goals and achievements...\"\n\
                    })\n\n\
                 WORKFLOW 6: HYBRID ORGANIZATION\n\
                 Combine multiple organizational schemes:\n\n\
                 Example libraries:\n\
                 - \"alpha-backend-patterns\": Project + Component + Type\n\
                 - \"beta-frontend-bugs\": Project + Component + Type\n\
                 - \"2024-architecture-decisions\": Time + Type\n\
                 - \"client-acme-requirements\": Client + Type\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. CONSISTENT NAMING:\n\
                    - Use consistent naming convention (kebab-case recommended)\n\
                    - Good: \"project-notes\", \"api-patterns\", \"code-reviews\"\n\
                    - Avoid: \"ProjectNotes\", \"api_patterns\", \"code-reviews\" (mixed styles)\n\n\
                 2. DESCRIPTIVE NAMES:\n\
                    - Library names should clearly indicate contents\n\
                    - Good: \"authentication-patterns\", \"database-migrations\"\n\
                    - Avoid: \"misc\", \"temp\", \"stuff\"\n\n\
                 3. APPROPRIATE GRANULARITY:\n\
                    - Not too broad: \"everything\" (hard to organize)\n\
                    - Not too narrow: \"login-button-color\" (too specific)\n\
                    - Just right: \"ui-components\", \"authentication\"\n\n\
                 4. REGULAR DISCOVERY:\n\
                    - Periodically list libraries to understand your knowledge base\n\
                    - memory_list_libraries({}) shows what you have\n\
                    - Helps avoid duplicates and maintain organization\n\n\
                 5. VERIFICATION:\n\
                    - Always check library existence before assuming it's there\n\
                    - Use memory_list_libraries({}) before recall operations\n\
                    - Handle missing libraries gracefully\n\n\
                 6. PURPOSEFUL CREATION:\n\
                    - Don't create libraries arbitrarily\n\
                    - Plan your organizational structure\n\
                    - Each library should have clear purpose\n\n\
                 =============================================================================\n\
                 ANTI-PATTERNS TO AVOID\n\
                 =============================================================================\n\n\
                 1. TOO MANY LIBRARIES:\n\
                    Bad: Creating a new library for every single memory\n\
                    - \"login-feature-bug-1\"\n\
                    - \"login-feature-bug-2\"\n\
                    - \"signup-feature-bug-1\"\n\
                    Better: Group related memories\n\
                    - \"authentication-bugs\"\n\n\
                 2. TOO FEW LIBRARIES:\n\
                    Bad: Putting everything in one library\n\
                    - \"all-my-memories\" (contains everything)\n\
                    Better: Separate by context\n\
                    - \"project-notes\"\n\
                    - \"code-patterns\"\n\
                    - \"decisions\"\n\n\
                 3. INCONSISTENT NAMING:\n\
                    Bad: Mixed naming conventions\n\
                    - \"project-notes\" (kebab-case)\n\
                    - \"code_patterns\" (snake_case)\n\
                    - \"ApiDocs\" (PascalCase)\n\
                    Better: Pick one and stick with it\n\
                    - \"project-notes\"\n\
                    - \"code-patterns\"\n\
                    - \"api-docs\"\n\n\
                 4. FORGETTING TO LIST:\n\
                    Bad: Assuming libraries exist without checking\n\
                    memory_recall({\"library\": \"might-not-exist\", ...})  // May fail\n\
                    Better: Verify first\n\
                    const {libraries} = memory_list_libraries({});\n\
                    if (libraries.includes(\"target-lib\")) { ... }\n\n\
                 =============================================================================\n\
                 COMPLETE WORKFLOW EXAMPLE\n\
                 =============================================================================\n\n\
                 // Step 1: Discover what exists\n\
                 const {libraries} = memory_list_libraries({});\n\
                 console.log(`Existing libraries: ${libraries.join(\", \")}`);\n\
                 // Output: \"project-alpha, code-patterns\"\n\n\
                 // Step 2: Store new knowledge\n\
                 memory_memorize({\n\
                   \"library\": \"code-patterns\",  // Reuse existing library\n\
                   \"content\": \"Observer pattern: Use for event-driven architecture...\"\n\
                 });\n\n\
                 memory_memorize({\n\
                   \"library\": \"architecture-decisions\",  // Create new library\n\
                   \"content\": \"Decision: Use microservices architecture for scalability...\"\n\
                 });\n\n\
                 // Step 3: Verify new structure\n\
                 const updated = memory_list_libraries({});\n\
                 console.log(`Updated libraries: ${updated.libraries.join(\", \")}`);\n\
                 // Output: \"project-alpha, code-patterns, architecture-decisions\"\n\n\
                 // Step 4: Recall from specific library\n\
                 if (updated.libraries.includes(\"code-patterns\")) {\n\
                   const patterns = memory_recall({\n\
                     \"library\": \"code-patterns\",\n\
                     \"context\": \"design patterns\"\n\
                   });\n\
                   // Use the recalled patterns\n\
                 }\n\n\
                 =============================================================================\n\
                 SUMMARY\n\
                 =============================================================================\n\n\
                 memory_list_libraries is your window into memory organization:\n\
                 - Discovers what libraries exist\n\
                 - Verifies library presence before operations\n\
                 - Helps maintain organized knowledge structure\n\
                 - Enables informed decisions about where to store/recall\n\
                 - Simple to use: memory_list_libraries({})\n\
                 - Returns: {\"libraries\": [\"name1\", \"name2\", ...]}\n\n\
                 Use it frequently to understand and manage your memory organization!",
            ),
        },
    ]
}
