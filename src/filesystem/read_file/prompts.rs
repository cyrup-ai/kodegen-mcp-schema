//! Prompt messages for fs_read_file tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::FsReadFilePromptArgs;

/// Prompt provider for fs_read_file tool
///
/// This is the ONLY way to provide prompts for fs_read_file - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ReadFilePrompts;

impl PromptProvider for ReadFilePrompts {
    type PromptArgs = FsReadFilePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("large_files") => prompt_large_files(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, large_files)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO READ FILES
// ============================================================================

/// Basic file reading operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I read files using the fs_read_file tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The fs_read_file tool reads file contents from the filesystem or URLs.\n\n\
                 BASIC USAGE:\n\n\
                 Read entire file:\n\
                 fs_read_file({\"path\": \"/project/config.json\"})\n\n\
                 Read with offset/length:\n\
                 fs_read_file({\"path\": \"/project/large.log\", \"offset\": 0, \"length\": 100})\n\n\
                 Read from URL:\n\
                 fs_read_file({\"path\": \"https://raw.githubusercontent.com/user/repo/main/README.md\"})\n\n\
                 PARAMETERS:\n\
                 - path (required): Absolute path to file or URL\n\
                 - offset (optional): Line to start from (0-based, negative = from end)\n\
                 - length (optional): Number of lines to read\n\
                 - is_url (optional): Explicitly treat path as URL\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"path\": \"/project/config.json\",\n\
                   \"content\": \"1\\t{...file content...}\\n2\\t{...line 2...}\",\n\
                   \"total_lines\": 50,\n\
                   \"lines_read\": 50,\n\
                   \"is_partial\": false\n\
                 }\n\n\
                 Output format: Line numbers prepended (like cat -n)\n\
                 Lines > 2000 chars are truncated\n\n\
                 WHEN TO USE:\n\
                 - ALWAYS read before editing (understand current content)\n\
                 - Understanding code structure\n\
                 - Checking configuration values\n\
                 - Debugging issues\n\
                 - Reviewing documentation\n\n\
                 COMMON PATTERNS:\n\n\
                 Read before edit:\n\
                 fs_read_file({\"path\": \"/project/src/handler.rs\"})\n\
                 // Review content, identify exact strings\n\
                 fs_edit_block({\"path\": \"/project/src/handler.rs\", \"old_string\": \"...\", \"new_string\": \"...\"})\n\n\
                 Read config file:\n\
                 fs_read_file({\"path\": \"/project/config.json\"})\n\
                 // Understand current settings before changes\n\n\
                 Read remote URL:\n\
                 fs_read_file({\"path\": \"https://example.com/data.txt\"})\n\
                 // Auto-detects URLs starting with http:// or https://\n\n\
                 QUICK REFERENCE:\n\
                 Read entire file:    {\"path\": \"/path/to/file\"}\n\
                 Read first 100:      {\"path\": \"/path/to/file\", \"offset\": 0, \"length\": 100}\n\
                 Read last 50:        {\"path\": \"/path/to/file\", \"offset\": -50}\n\
                 Read from URL:       {\"path\": \"https://example.com/file.txt\"}",
            ),
        },
    ]
}

/// Handling large files with offset and length
fn prompt_large_files() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I read large files efficiently using offset and length?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "For large files (> 2000 lines), use offset and length parameters to read specific sections efficiently.\n\n\
                 WHY USE OFFSET/LENGTH:\n\
                 - Reduces memory usage\n\
                 - Faster response times\n\
                 - Targeted reading of relevant sections\n\
                 - Essential for files with thousands of lines\n\n\
                 EXAMPLES:\n\n\
                 Read first 100 lines (head pattern):\n\
                 fs_read_file({\"path\": \"/project/large_log.txt\", \"offset\": 0, \"length\": 100})\n\n\
                 Read last 50 lines (tail pattern):\n\
                 fs_read_file({\"path\": \"/project/large_log.txt\", \"offset\": -50})\n\n\
                 Read lines 500-600:\n\
                 fs_read_file({\"path\": \"/project/large_log.txt\", \"offset\": 500, \"length\": 100})\n\n\
                 Skip to specific section:\n\
                 fs_read_file({\"path\": \"/var/log/application.log\", \"offset\": 1000, \"length\": 200})\n\n\
                 OFFSET BEHAVIOR:\n\
                 - Positive: Start from beginning (0 = first line, 100 = line 101)\n\
                 - Negative: Start from end (-50 = last 50 lines, like tail -n 50)\n\
                 - Length ignored when offset is negative\n\n\
                 PAGINATION PATTERN:\n\
                 Read file in chunks:\n\
                 fs_read_file({\"path\": \"/data/big.log\", \"offset\": 0, \"length\": 100})     // Page 1\n\
                 fs_read_file({\"path\": \"/data/big.log\", \"offset\": 100, \"length\": 100})   // Page 2\n\
                 fs_read_file({\"path\": \"/data/big.log\", \"offset\": 200, \"length\": 100})   // Page 3\n\n\
                 MEMORY EFFICIENCY:\n\
                 Without offset/length: Tool loads entire 10,000 line file into memory\n\
                 With offset/length: Tool loads only requested 100 lines\n\
                 Result: 100x reduction in memory usage\n\n\
                 WHEN TO PAGINATE VS READ ALL:\n\
                 Read all (< 2000 lines):\n\
                 - Small files\n\
                 - Need complete context\n\
                 - Performance not a concern\n\n\
                 Paginate (> 2000 lines):\n\
                 - Large log files\n\
                 - Database dumps\n\
                 - Generated code\n\
                 - Data files (CSV, JSON arrays)\n\n\
                 QUICK EXAMPLES:\n\n\
                 Read first portion to understand structure:\n\
                 fs_read_file({\"path\": \"/data/huge.csv\", \"offset\": 0, \"length\": 50})\n\n\
                 Use fs_search to find specific content:\n\
                 fs_search({\"path\": \"/data\", \"pattern\": \"ERROR\"})\n\n\
                 Read targeted sections based on search:\n\
                 fs_read_file({\"path\": \"/data/huge.csv\", \"offset\": 5000, \"length\": 100})\n\n\
                 Tail recent log entries:\n\
                 fs_read_file({\"path\": \"/var/log/error.log\", \"offset\": -100})\n\n\
                 RESPONSE INDICATORS:\n\
                 - is_partial: true means file has more lines than returned\n\
                 - total_lines: Total lines in file\n\
                 - lines_read: Lines included in response\n\
                 - Use these to determine if more pages exist",
            ),
        },
    ]
}

