//! Prompt messages for fs_write_file tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::FsWriteFilePromptArgs;

/// Prompt provider for fs_write_file tool
///
/// This is the ONLY way to provide prompts for fs_write_file - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct WriteFilePrompts;

impl PromptProvider for WriteFilePrompts {
    type PromptArgs = FsWriteFilePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("append") => prompt_append(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, append)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO WRITE FILES
// ============================================================================

/// Basic file writing operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I write files using fs_write_file?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The fs_write_file tool writes or appends content to files. Here's how to use it for basic file writing:\n\n\
                 WRITING FILES:\n\n\
                 1. Write new file:\n\
                    fs_write_file({\n\
                        \"path\": \"/project/output.txt\",\n\
                        \"content\": \"Hello, World!\"\n\
                    })\n\n\
                 2. Overwrite existing:\n\
                    fs_write_file({\n\
                        \"path\": \"/project/config.json\",\n\
                        \"content\": \"{\\n  \\\"setting\\\": \\\"value\\\"\\n}\",\n\
                        \"mode\": \"rewrite\"\n\
                    })\n\n\
                 3. Write with explicit mode:\n\
                    fs_write_file({\n\
                        \"path\": \"/project/data.csv\",\n\
                        \"content\": \"name,value\\nfoo,123\",\n\
                        \"mode\": \"rewrite\"\n\
                    })\n\n\
                 WRITE MODES:\n\
                 - \"rewrite\" (default): Replace entire file\n\
                 - \"append\": Add to end of file\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"path\": \"/project/output.txt\",\n\
                   \"bytes_written\": 13,\n\
                   \"success\": true\n\
                 }\n\n\
                 WHEN TO USE:\n\
                 - Creating new files\n\
                 - Replacing entire file contents\n\
                 - Generating output files\n\n\
                 PARAMETERS:\n\
                 - path (required): Absolute path to file\n\
                 - content (required): String content to write\n\
                 - mode (optional): \"rewrite\" or \"append\" (default: \"rewrite\")\n\n\
                 PATH HANDLING:\n\
                 - Automatically creates parent directories if needed\n\
                 - Validates paths to prevent security issues\n\
                 - Use absolute paths for clarity\n\n\
                 CONTENT HANDLING:\n\
                 - Include newlines explicitly with \\n\n\
                 - Escape quotes properly in JSON strings\n\
                 - UTF-8 encoding by default\n\
                 - Binary content not supported (use for text files)\n\n\
                 COMMON PATTERNS:\n\
                 1. Create simple text file:\n\
                    fs_write_file({\"path\": \"/tmp/notes.txt\", \"content\": \"My notes\\n\"})\n\
                 2. Write JSON:\n\
                    fs_write_file({\"path\": \"/config/app.json\", \"content\": \"{...}\"})\n\
                 3. Generate output:\n\
                    fs_write_file({\"path\": \"/results/output.txt\", \"content\": results})\n\n\
                 ERROR HANDLING:\n\
                 - Tool returns error if path invalid\n\
                 - Parent directories created automatically\n\
                 - File permissions respected (may fail if no write access)\n\
                 - Success field indicates if write completed",
            ),
        },
    ]
}

/// Appending to files
fn prompt_append() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I append content to existing files without overwriting them?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use mode: \"append\" to add content to the end of files without reading or modifying existing content.\n\n\
                 APPENDING TO FILES:\n\n\
                 1. Add log entry:\n\
                    fs_write_file({\n\
                        \"path\": \"/project/app.log\",\n\
                        \"content\": \"\\n[2024-01-15] New event occurred\",\n\
                        \"mode\": \"append\"\n\
                    })\n\n\
                 2. Add to data file:\n\
                    fs_write_file({\n\
                        \"path\": \"/project/data.csv\",\n\
                        \"content\": \"\\nnew_item,456\",\n\
                        \"mode\": \"append\"\n\
                    })\n\n\
                 3. Append multiple lines:\n\
                    fs_write_file({\n\
                        \"path\": \"/project/notes.txt\",\n\
                        \"content\": \"\\n\\n## New Section\\n\\nAdditional notes here.\",\n\
                        \"mode\": \"append\"\n\
                    })\n\n\
                 4. Continuous logging:\n\
                    fs_write_file({\n\
                        \"path\": \"/logs/debug.log\",\n\
                        \"content\": \"\\n[DEBUG] Function called with args: {x: 1, y: 2}\",\n\
                        \"mode\": \"append\"\n\
                    })\n\n\
                 APPEND PATTERNS:\n\
                 - Start with \\n for new line\n\
                 - Good for logs, data accumulation\n\
                 - Doesn't read existing content first\n\
                 - Faster than read-modify-write\n\
                 - Atomic operation at filesystem level\n\n\
                 WHEN TO APPEND:\n\
                 - Log files (continuous logging)\n\
                 - Accumulating data (CSV, TSV)\n\
                 - Adding to list files\n\
                 - Progressive output\n\
                 - Event streams\n\n\
                 WHEN NOT TO APPEND:\n\
                 - Need to modify existing lines\n\
                 - Must maintain specific format\n\
                 - File has size limits\n\
                 - Need to deduplicate\n\n\
                 NEWLINE HANDLING:\n\
                 - Append does NOT automatically add newline\n\
                 - Always include \\n explicitly if needed\n\
                 - Example: \"\\n[LOG] message\" ensures new line\n\
                 - Without \\n, content appends to last line\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"path\": \"/project/app.log\",\n\
                   \"bytes_written\": 34,\n\
                   \"success\": true\n\
                 }\n\n\
                 COMPARISON: APPEND vs REWRITE:\n\
                 APPEND:\n\
                 - Preserves existing content\n\
                 - Adds to end of file\n\
                 - Fast (no read required)\n\
                 - Good for logs and accumulation\n\n\
                 REWRITE:\n\
                 - Replaces entire file\n\
                 - Old content lost\n\
                 - Need to read first if preserving content\n\
                 - Good for complete file replacement\n\n\
                 BEST PRACTICES:\n\
                 1. Use append for logs and continuous data\n\
                 2. Always include leading \\n for new line\n\
                 3. Consider file size growth over time\n\
                 4. Use for write-only workflows (no read needed)\n\
                 5. Good for concurrent writes (multiple appenders)",
            ),
        },
    ]
}


