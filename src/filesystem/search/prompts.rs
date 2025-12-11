//! Prompt messages for fs_search tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptArgument, PromptMessage, PromptMessageContent, PromptMessageRole};
use super::prompt_args::FsSearchPromptArgs;

/// Prompt provider for fs_search tool
///
/// This is the ONLY way to provide prompts for fs_search - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct SearchPrompts;

impl PromptProvider for SearchPrompts {
    type PromptArgs = FsSearchPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("patterns") => prompt_patterns(),
            Some("pattern_modes") => prompt_pattern_modes(),
            Some("options") => prompt_options(),
            Some("background") => prompt_background(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some(
                "Scenario to show (basic, patterns, pattern_modes, options, background, workflows)".to_string(),
            ),
            required: Some(false),
        }]
    }
}

/// Basic content and filename search
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I use fs_search for basic searches?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "SEARCHING FILES:\n\n\
                 1. Search file contents:\n\
                    fs_search({\n\
                        \"pattern\": \"TODO\",\n\
                        \"path\": \"./src\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"matches\": [\n\
                     {\n\
                       \"path\": \"src/main.rs\",\n\
                       \"line_number\": 45,\n\
                       \"content\": \"    // TODO: implement error handling\"\n\
                     }\n\
                   ],\n\
                   \"total_matches\": 1\n\
                 }\n\n\
                 2. Search filenames:\n\
                    fs_search({\n\
                        \"pattern\": \"test\",\n\
                        \"path\": \"./\",\n\
                        \"search_in\": \"filenames\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"pattern_type\": \"substring\",\n\
                   \"matches\": [\n\
                     { \"path\": \"tests/unit_test.rs\" },\n\
                     { \"path\": \"tests/integration_test.rs\" }\n\
                   ]\n\
                 }\n\n\
                 3. Return modes:\n\
                    // Just file paths\n\
                    fs_search({\n\
                        \"pattern\": \"error\",\n\
                        \"path\": \"./src\",\n\
                        \"return_only\": \"paths\"\n\
                    })\n\n\
                    // Match counts per file\n\
                    fs_search({\n\
                        \"pattern\": \"error\",\n\
                        \"path\": \"./src\",\n\
                        \"return_only\": \"counts\"\n\
                    })\n\n\
                 SEARCH_IN OPTIONS:\n\
                 - \"content\": Search inside files (default)\n\
                 - \"filenames\": Search file paths",
            ),
        },
    ]
}

/// Regex patterns and filtering
fn prompt_patterns() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I use regex patterns and filtering?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "REGEX PATTERNS:\n\n\
                 1. Simple pattern:\n\
                    fs_search({\n\
                        \"pattern\": \"fn main\",\n\
                        \"path\": \"./src\"\n\
                    })\n\n\
                 2. Regex pattern:\n\
                    fs_search({\n\
                        \"pattern\": \"fn\\\\s+\\\\w+\\\\(\",\n\
                        \"path\": \"./src\"\n\
                    })\n\
                    // Matches function definitions\n\n\
                 3. Word boundary:\n\
                    fs_search({\n\
                        \"pattern\": \"error\",\n\
                        \"path\": \"./src\",\n\
                        \"boundary_mode\": \"word\"\n\
                    })\n\
                    // Matches \"error\" not \"errors\" or \"errorHandler\"\n\n\
                 4. Case insensitive:\n\
                    fs_search({\n\
                        \"pattern\": \"Error\",\n\
                        \"path\": \"./src\",\n\
                        \"case_mode\": \"insensitive\"\n\
                    })\n\n\
                 5. Smart case:\n\
                    fs_search({\n\
                        \"pattern\": \"error\",\n\
                        \"case_mode\": \"smart\"\n\
                    })\n\
                    // lowercase = insensitive, has uppercase = sensitive\n\n\
                 6. Literal search:\n\
                    fs_search({\n\
                        \"pattern\": \"Result<T, E>\",\n\
                        \"path\": \"./src\",\n\
                        \"literal_search\": true\n\
                    })\n\
                    // No regex interpretation\n\n\
                 FILE FILTERING:\n\
                 - file_pattern: \"*.rs\"\n\
                 - type: [\"rust\"]\n\
                 - type_not: [\"test\"]",
            ),
        },
    ]
}

/// Search options and modes
fn prompt_options() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("What search options are available?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "SEARCH OPTIONS:\n\n\
                 1. Filter by file type:\n\
                    fs_search({\n\
                        \"pattern\": \"import\",\n\
                        \"path\": \"./\",\n\
                        \"type\": [\"typescript\", \"javascript\"]\n\
                    })\n\n\
                 2. Filter by glob:\n\
                    fs_search({\n\
                        \"pattern\": \"test\",\n\
                        \"path\": \"./src\",\n\
                        \"file_pattern\": \"*.rs\"\n\
                    })\n\n\
                 3. With context lines:\n\
                    fs_search({\n\
                        \"pattern\": \"panic!\",\n\
                        \"path\": \"./src\",\n\
                        \"context\": 3\n\
                    })\n\
                    // 3 lines before and after\n\n\
                 4. Before/after context:\n\
                    fs_search({\n\
                        \"pattern\": \"error\",\n\
                        \"path\": \"./src\",\n\
                        \"before_context\": 2,\n\
                        \"after_context\": 5\n\
                    })\n\n\
                 5. Limit results:\n\
                    fs_search({\n\
                        \"pattern\": \"TODO\",\n\
                        \"path\": \"./\",\n\
                        \"max_results\": 20\n\
                    })\n\n\
                 6. Depth limit:\n\
                    fs_search({\n\
                        \"pattern\": \"config\",\n\
                        \"path\": \"./\",\n\
                        \"max_depth\": 2\n\
                    })\n\n\
                 7. Include hidden:\n\
                    fs_search({\n\
                        \"pattern\": \"secret\",\n\
                        \"path\": \"./\",\n\
                        \"include_hidden\": true\n\
                    })\n\n\
                 8. Multiline:\n\
                    fs_search({\n\
                        \"pattern\": \"struct.*\\\\{[\\\\s\\\\S]*?\\\\}\",\n\
                        \"path\": \"./src\",\n\
                        \"multiline\": true\n\
                    })",
            ),
        },
    ]
}

/// Background search management
fn prompt_background() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I manage background searches?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "BACKGROUND SEARCH:\n\n\
                 1. Start background search:\n\
                    fs_search({\n\
                        \"action\": \"SEARCH\",\n\
                        \"pattern\": \"TODO|FIXME|HACK\",\n\
                        \"path\": \"./\",\n\
                        \"await_completion_ms\": 0\n\
                    })\n\
                    // Returns immediately with search ID\n\n\
                 2. Check progress:\n\
                    fs_search({\n\
                        \"action\": \"READ\",\n\
                        \"search\": 0\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"search\": 0,\n\
                   \"status\": \"running\",\n\
                   \"matches_so_far\": 150,\n\
                   \"files_searched\": 45\n\
                 }\n\n\
                 3. List all searches:\n\
                    fs_search({\n\
                        \"action\": \"LIST\"\n\
                    })\n\n\
                 4. Kill search:\n\
                    fs_search({\n\
                        \"action\": \"KILL\",\n\
                        \"search\": 0\n\
                    })\n\n\
                 TIMEOUT BEHAVIOR:\n\
                 - await_completion_ms: 0 = fire-and-forget\n\
                 - await_completion_ms: N = wait up to N ms\n\
                 - Default: 60000 (1 minute)\n\
                 - On timeout: returns current results\n\n\
                 ACTIONS:\n\
                 - SEARCH: Execute search (default)\n\
                 - READ: Get current state\n\
                 - LIST: List active searches\n\
                 - KILL: Cancel search",
            ),
        },
    ]
}

/// Pattern modes for filename searches (regex, glob, substring)
fn prompt_pattern_modes() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I use pattern_mode for filename searches?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "PATTERN MODES (filename search only):\n\n\
                 The pattern_mode parameter controls how filename patterns are interpreted.\n\
                 It only works when search_in: \"filenames\".\n\n\
                 AUTO-DETECTION PRIORITY:\n\
                 1. Regex (if regex markers detected)\n\
                 2. Glob (if glob markers detected)\n\
                 3. Substring (fallback)\n\n\
                 1. REGEX MODE:\n\
                    fs_search({\n\
                        \"pattern\": \"^test_.*\\\\.rs$\",\n\
                        \"path\": \"./src\",\n\
                        \"search_in\": \"filenames\",\n\
                        \"pattern_mode\": \"regex\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"pattern_type\": \"regex\",\n\
                   \"matches\": [\n\
                     { \"path\": \"src/test_utils.rs\" },\n\
                     { \"path\": \"src/test_helpers.rs\" }\n\
                   ]\n\
                 }\n\n\
                 REGEX MARKERS (auto-detects as regex):\n\
                 - Anchors: ^, $\n\
                 - Escape sequences: \\., \\d, \\w, \\s\n\
                 - Quantifiers: .*, .+, \\w+\n\
                 - Alternation: |\n\
                 - Character classes: [a-z]+, [0-9]{2,4}\n\n\
                 2. GLOB MODE:\n\
                    fs_search({\n\
                        \"pattern\": \"**/*.test.{js,ts}\",\n\
                        \"path\": \"./src\",\n\
                        \"search_in\": \"filenames\",\n\
                        \"pattern_mode\": \"glob\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"pattern_type\": \"glob\",\n\
                   \"matches\": [\n\
                     { \"path\": \"src/utils.test.ts\" },\n\
                     { \"path\": \"src/helpers.test.js\" }\n\
                   ]\n\
                 }\n\n\
                 GLOB MARKERS (auto-detects as glob):\n\
                 - Wildcards: *, ?, **\n\
                 - Brace expansion: {a,b,c}\n\
                 - Character sets: [abc], [a-z]\n\n\
                 3. SUBSTRING MODE:\n\
                    fs_search({\n\
                        \"pattern\": \"config\",\n\
                        \"path\": \"./src\",\n\
                        \"search_in\": \"filenames\",\n\
                        \"pattern_mode\": \"substring\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"pattern_type\": \"substring\",\n\
                   \"matches\": [\n\
                     { \"path\": \"src/config.rs\" },\n\
                     { \"path\": \"src/app_config.toml\" }\n\
                   ]\n\
                 }\n\n\
                 DEFAULT: Substring (if no special markers)\n\n\
                 AUTO-DETECTION EXAMPLES:\n\
                 - \"test\" → substring (no markers)\n\
                 - \"*.rs\" → glob (has *)\n\
                 - \"^main\" → regex (has ^)\n\
                 - \"foo|bar\" → regex (has |)\n\
                 - \"test?.rs\" → glob (has ?)\n\
                 - \"config.json\" → substring (no markers)\n\n\
                 CONTENT SEARCH NOTE:\n\
                 pattern_mode ONLY works with search_in: \"filenames\".\n\
                 Content searches always use regex (via ripgrep).\n\
                 When searching content, pattern_type returns None.",
            ),
        },
    ]
}

/// Search workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("What are common search workflows?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "SEARCH WORKFLOWS:\n\n\
                 1. Find and analyze:\n\
                    // Find usages\n\
                    fs_search({\n\
                        \"pattern\": \"deprecated_function\",\n\
                        \"path\": \"./src\",\n\
                        \"return_only\": \"paths\"\n\
                    })\n\n\
                    // Read each file\n\
                    fs_read_file({ \"path\": \"found_file.rs\" })\n\n\
                 2. Codebase exploration:\n\
                    // Find entry points\n\
                    fs_search({\n\
                        \"pattern\": \"fn main\",\n\
                        \"path\": \"./src\",\n\
                        \"type\": [\"rust\"]\n\
                    })\n\n\
                    // Find public APIs\n\
                    fs_search({\n\
                        \"pattern\": \"pub fn|pub struct|pub enum\",\n\
                        \"path\": \"./src\"\n\
                    })\n\n\
                 3. Debug investigation:\n\
                    // Find error handling\n\
                    fs_search({\n\
                        \"pattern\": \"unwrap\\\\(\\\\)|expect\\\\(\",\n\
                        \"path\": \"./src\",\n\
                        \"context\": 2\n\
                    })\n\n\
                 4. Refactoring prep:\n\
                    // Count usages\n\
                    fs_search({\n\
                        \"pattern\": \"old_name\",\n\
                        \"path\": \"./src\",\n\
                        \"return_only\": \"counts\"\n\
                    })\n\n\
                 5. Security audit:\n\
                    fs_search({\n\
                        \"pattern\": \"unsafe|raw_pointer|transmute\",\n\
                        \"path\": \"./src\",\n\
                        \"context\": 5\n\
                    })\n\n\
                 WORKFLOW PATTERNS:\n\
                 - Search → Read → Understand\n\
                 - Search → Count → Plan\n\
                 - Search → Context → Debug",
            ),
        },
    ]
}
