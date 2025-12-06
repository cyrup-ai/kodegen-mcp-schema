//! Prompt messages for introspection_list_tools tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::IntrospectionListToolsPromptArgs;

/// Prompt provider for introspection_list_tools tool
///
/// This is the ONLY way to provide prompts for introspection_list_tools - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct IntrospectionListToolsPrompts;

impl PromptProvider for IntrospectionListToolsPrompts {
    type PromptArgs = IntrospectionListToolsPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("filtering") => prompt_filtering(),
            Some("discovery") => prompt_discovery(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, filtering, discovery)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE INTROSPECTION_LIST_TOOLS
// ============================================================================

/// Basic tool listing and understanding tool structure
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list all available tools and see what they do?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The introspection_list_tools tool shows all available tools and their capabilities. Here's how to use it:\n\n\
                 LISTING ALL TOOLS:\n\
                 1. Get complete tool list:\n\
                    introspection_list_tools({})\n\
                    Returns all available tools with descriptions\n\n\
                 2. See what tools you can use:\n\
                    introspection_list_tools({})\n\
                    Discover capabilities at your disposal\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"tools\": [\n\
                     {\n\
                       \"name\": \"fs_read_file\",\n\
                       \"category\": \"filesystem\",\n\
                       \"description\": \"Read file contents from the filesystem\",\n\
                       \"read_only\": true,\n\
                       \"destructive\": false,\n\
                       \"idempotent\": true\n\
                     },\n\
                     {\n\
                       \"name\": \"fs_write_file\",\n\
                       \"category\": \"filesystem\",\n\
                       \"description\": \"Write or append to file contents\",\n\
                       \"read_only\": false,\n\
                       \"destructive\": false,\n\
                       \"idempotent\": false\n\
                     },\n\
                     {\n\
                       \"name\": \"terminal\",\n\
                       \"category\": \"execution\",\n\
                       \"description\": \"Execute shell commands in persistent terminal sessions\",\n\
                       \"read_only\": false,\n\
                       \"destructive\": false,\n\
                       \"idempotent\": false\n\
                     }\n\
                   ],\n\
                   \"total_count\": 3,\n\
                   \"categories\": [\"filesystem\", \"execution\"]\n\
                 }\n\n\
                 TOOL FIELDS EXPLAINED:\n\
                 - name: Tool identifier used when calling it (e.g., \"fs_read_file\")\n\
                 - category: Logical grouping (filesystem, git, terminal, etc.)\n\
                 - description: What the tool does and its purpose\n\
                 - read_only: true = tool only reads data, false = can modify state\n\
                 - destructive: true = can delete/destroy data, false = safe operations\n\
                 - idempotent: true = same input always gives same result\n\n\
                 UNDERSTANDING TOOL PROPERTIES:\n\
                 1. read_only tools:\n\
                    - Safe to call repeatedly\n\
                    - Don't modify system state\n\
                    - Examples: fs_read_file, fs_search, git_diff\n\n\
                 2. destructive tools:\n\
                    - Can delete or permanently modify data\n\
                    - Use with caution\n\
                    - Examples: fs_delete_file, fs_delete_directory\n\n\
                 3. idempotent tools:\n\
                    - Same input produces same output\n\
                    - Safe to retry on failure\n\
                    - Examples: fs_read_file, fs_create_directory\n\n\
                 COMMON USE CASES:\n\
                 1. Discover available capabilities:\n\
                    introspection_list_tools({})\n\
                    See what operations you can perform\n\n\
                 2. Find tools for a task:\n\
                    introspection_list_tools({})\n\
                    Review descriptions to find the right tool\n\n\
                 3. Understand tool safety:\n\
                    Check read_only and destructive flags\n\
                    Know which tools are safe to experiment with\n\n\
                 4. Explore tool categories:\n\
                    Review categories field\n\
                    Understand how tools are organized\n\n\
                 RESPONSE INTERPRETATION:\n\
                 - total_count: Total number of available tools\n\
                 - categories: All tool categories available\n\
                 - tools array: Complete list with metadata\n\
                 - Empty tools array: No tools available (unusual)\n\n\
                 BASIC WORKFLOW:\n\
                 // Step 1: List all tools\n\
                 introspection_list_tools({})\n\
                 // Step 2: Review the tools array\n\
                 // Step 3: Note the tool names you need\n\
                 // Step 4: Use those tools by name\n\
                 // Example: fs_read_file({\"path\": \"/path/to/file\"})\n\n\
                 TOOL NAMING CONVENTION:\n\
                 - Format: {category}_{action}\n\
                 - Examples:\n\
                   - fs_read_file: filesystem category, read action\n\
                   - git_clone: git category, clone action\n\
                   - terminal: execution category (single word)\n\
                   - fs_search: filesystem category, search action\n\n\
                 UNDERSTANDING CATEGORIES:\n\
                 Common categories include:\n\
                 - filesystem: File and directory operations\n\
                 - git: Version control operations\n\
                 - github: GitHub API interactions\n\
                 - terminal: Shell command execution\n\
                 - process: Process management\n\
                 - browser: Web page rendering and interaction\n\
                 - database: Database queries and schema exploration\n\
                 - introspection: Tool and execution monitoring\n\
                 - config: Configuration management\n\
                 - memory: Persistent memory storage\n\n\
                 TOOL DISCOVERY TIPS:\n\
                 - Read descriptions carefully to find exact matches\n\
                 - Tools in same category have related functionality\n\
                 - Check read_only flag for safe exploration tools\n\
                 - Avoid destructive tools until you understand them\n\
                 - Idempotent tools are safe to retry\n\n\
                 NO TOOLS SCENARIO:\n\
                 If tools array is empty:\n\
                 - Server may not be properly configured\n\
                 - Or tool registration failed\n\
                 - This is unusual - all servers should have tools",
            ),
        },
    ]
}

/// Filtering tools by category and characteristics
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I filter the tool list to find specific types of tools?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The introspection_list_tools tool supports filtering to help you find exactly what you need. Here's how:\n\n\
                 FILTERING BY CATEGORY:\n\
                 1. Filesystem tools only:\n\
                    introspection_list_tools({\"category\": \"filesystem\"})\n\
                    Shows: fs_read_file, fs_write_file, fs_search, etc.\n\n\
                 2. Git tools only:\n\
                    introspection_list_tools({\"category\": \"git\"})\n\
                    Shows: git_clone, git_commit, git_branch, etc.\n\n\
                 3. Terminal/execution tools:\n\
                    introspection_list_tools({\"category\": \"terminal\"})\n\
                    Shows: terminal tool and related execution tools\n\n\
                 4. GitHub tools:\n\
                    introspection_list_tools({\"category\": \"github\"})\n\
                    Shows: github_search_code, github_search_issues, etc.\n\n\
                 5. Introspection tools:\n\
                    introspection_list_tools({\"category\": \"introspection\"})\n\
                    Shows: introspection_list_tools, introspection_get_events, etc.\n\n\
                 FILTERING BY TOOL PROPERTIES:\n\
                 1. Only read-only tools (safe to explore):\n\
                    introspection_list_tools({\"read_only\": true})\n\
                    Find tools that don't modify state\n\n\
                 2. Only non-destructive tools:\n\
                    introspection_list_tools({\"destructive\": false})\n\
                    Find safe tools that won't delete data\n\n\
                 3. Only destructive tools (use with caution):\n\
                    introspection_list_tools({\"destructive\": true})\n\
                    Find tools that can delete/destroy data\n\n\
                 4. Only idempotent tools:\n\
                    introspection_list_tools({\"idempotent\": true})\n\
                    Find tools safe to retry on failure\n\n\
                 SEARCHING BY NAME PATTERN:\n\
                 1. Search for tools with specific name:\n\
                    introspection_list_tools({\"search\": \"read\"})\n\
                    Finds: fs_read_file, fs_read_multiple_files, etc.\n\n\
                 2. Search for write operations:\n\
                    introspection_list_tools({\"search\": \"write\"})\n\
                    Finds: fs_write_file and other write tools\n\n\
                 3. Search for search tools:\n\
                    introspection_list_tools({\"search\": \"search\"})\n\
                    Finds: fs_search, github_search_code, etc.\n\n\
                 4. Search for delete operations:\n\
                    introspection_list_tools({\"search\": \"delete\"})\n\
                    Finds: fs_delete_file, fs_delete_directory, etc.\n\n\
                 COMBINING FILTERS:\n\
                 1. Safe filesystem tools:\n\
                    introspection_list_tools({\n\
                        \"category\": \"filesystem\",\n\
                        \"read_only\": true\n\
                    })\n\
                    Filesystem tools that only read data\n\n\
                 2. Non-destructive git tools:\n\
                    introspection_list_tools({\n\
                        \"category\": \"git\",\n\
                        \"destructive\": false\n\
                    })\n\
                    Git tools that won't delete branches/data\n\n\
                 3. Idempotent filesystem operations:\n\
                    introspection_list_tools({\n\
                        \"category\": \"filesystem\",\n\
                        \"idempotent\": true\n\
                    })\n\
                    Filesystem tools safe to retry\n\n\
                 4. Destructive tools in filesystem category:\n\
                    introspection_list_tools({\n\
                        \"category\": \"filesystem\",\n\
                        \"destructive\": true\n\
                    })\n\
                    Find file/directory deletion tools\n\n\
                 PRACTICAL FILTERING PATTERNS:\n\
                 1. \"Show me safe tools to explore\":\n\
                    introspection_list_tools({\"read_only\": true})\n\
                    Only tools that read, never modify\n\n\
                 2. \"What file operations can I do?\":\n\
                    introspection_list_tools({\"category\": \"filesystem\"})\n\
                    All filesystem capabilities\n\n\
                 3. \"How do I work with git?\":\n\
                    introspection_list_tools({\"category\": \"git\"})\n\
                    All version control operations\n\n\
                 4. \"What tools can delete things?\":\n\
                    introspection_list_tools({\"destructive\": true})\n\
                    All tools that can destroy data\n\n\
                 5. \"Which tools are safe to retry?\":\n\
                    introspection_list_tools({\"idempotent\": true})\n\
                    Tools that produce same result every time\n\n\
                 CATEGORY-SPECIFIC DISCOVERY:\n\
                 1. File operations:\n\
                    introspection_list_tools({\"category\": \"filesystem\"})\n\
                    Read, write, search, edit, delete files\n\n\
                 2. Version control:\n\
                    introspection_list_tools({\"category\": \"git\"})\n\
                    Clone, commit, branch, merge, diff\n\n\
                 3. Code execution:\n\
                    introspection_list_tools({\"category\": \"terminal\"})\n\
                    Run shell commands, manage processes\n\n\
                 4. Web operations:\n\
                    introspection_list_tools({\"category\": \"browser\"})\n\
                    Scrape, crawl, render web pages\n\n\
                 5. Data storage:\n\
                    introspection_list_tools({\"category\": \"database\"})\n\
                    Query databases, explore schemas\n\n\
                 FILTER PARAMETER SUMMARY:\n\
                 - category: Filter by tool category (exact match)\n\
                 - read_only: Filter by read-only status (true/false)\n\
                 - destructive: Filter by destructive nature (true/false)\n\
                 - idempotent: Filter by idempotence (true/false)\n\
                 - search: Search tool names/descriptions (substring match)\n\
                 - limit: Maximum tools to return (optional)\n\
                 - offset: Skip this many tools (pagination)\n\n\
                 BEST PRACTICES:\n\
                 1. Use category filter when you know the domain\n\
                 2. Use read_only: true for safe exploration\n\
                 3. Use destructive: true to find dangerous operations\n\
                 4. Use search for name-based discovery\n\
                 5. Combine filters for precise results\n\
                 6. Start broad, then narrow with filters\n\n\
                 SAFETY FILTERING:\n\
                 Before experimenting, find safe tools:\n\
                 introspection_list_tools({\n\
                     \"read_only\": true,\n\
                     \"destructive\": false\n\
                 })\n\
                 These tools are safe to try without consequences\n\n\
                 DISCOVERING NEW CAPABILITIES:\n\
                 1. List all categories:\n\
                    introspection_list_tools({})\n\
                    Review categories field\n\n\
                 2. Explore each category:\n\
                    introspection_list_tools({\"category\": \"<name>\"})\n\
                    See what's available in that domain\n\n\
                 3. Search for specific operations:\n\
                    introspection_list_tools({\"search\": \"<keyword>\"})\n\
                    Find tools matching your need",
            ),
        },
    ]
}

/// Tool discovery patterns and finding the right tool for the job
fn prompt_discovery() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I discover which tool to use for my task?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use introspection_list_tools to discover the right tool for any task. Here are effective discovery patterns:\n\n\
                 TASK-BASED DISCOVERY:\n\
                 1. \"I need to read a file\":\n\
                    introspection_list_tools({\"search\": \"read\"})\n\
                    Finds: fs_read_file, fs_read_multiple_files\n\
                    Pick: fs_read_file for single files\n\n\
                 2. \"I need to search for code\":\n\
                    introspection_list_tools({\"search\": \"search\"})\n\
                    Finds: fs_search, github_search_code\n\
                    Pick: fs_search for local, github_search_code for remote\n\n\
                 3. \"I need to run commands\":\n\
                    introspection_list_tools({\"category\": \"terminal\"})\n\
                    Finds: terminal tool\n\
                    Pick: terminal for shell command execution\n\n\
                 4. \"I need to work with git\":\n\
                    introspection_list_tools({\"category\": \"git\"})\n\
                    Finds: git_clone, git_commit, git_branch, etc.\n\
                    Pick based on specific git operation needed\n\n\
                 DISCOVERY BY OPERATION TYPE:\n\
                 1. Reading data:\n\
                    introspection_list_tools({\"read_only\": true})\n\
                    Find all tools that retrieve information\n\
                    Examples: fs_read_file, git_diff, introspection_get_events\n\n\
                 2. Writing data:\n\
                    introspection_list_tools({\n\
                        \"read_only\": false,\n\
                        \"destructive\": false\n\
                    })\n\
                    Find tools that create/modify but don't delete\n\
                    Examples: fs_write_file, git_commit, fs_edit_block\n\n\
                 3. Deleting data:\n\
                    introspection_list_tools({\"destructive\": true})\n\
                    Find tools that can permanently remove data\n\
                    Examples: fs_delete_file, fs_delete_directory\n\n\
                 4. Searching/querying:\n\
                    introspection_list_tools({\"search\": \"search\"})\n\
                    Find all search-related tools\n\
                    Examples: fs_search, github_search_code, github_search_issues\n\n\
                 DISCOVERY BY DOMAIN:\n\
                 1. File system operations:\n\
                    introspection_list_tools({\"category\": \"filesystem\"})\n\
                    Capabilities: read, write, search, edit, delete, list, move\n\
                    Use for: Working with local files and directories\n\n\
                 2. Version control:\n\
                    introspection_list_tools({\"category\": \"git\"})\n\
                    Capabilities: clone, commit, branch, merge, diff, worktree\n\
                    Use for: Git repository operations\n\n\
                 3. Remote code hosting:\n\
                    introspection_list_tools({\"category\": \"github\"})\n\
                    Capabilities: search code, search issues, get file contents\n\
                    Use for: GitHub API operations\n\n\
                 4. Command execution:\n\
                    introspection_list_tools({\"category\": \"terminal\"})\n\
                    Capabilities: execute commands, manage sessions, monitor output\n\
                    Use for: Running build tools, scripts, system commands\n\n\
                 5. Process management:\n\
                    introspection_list_tools({\"category\": \"process\"})\n\
                    Capabilities: list processes, kill processes, monitor\n\
                    Use for: Managing running applications\n\n\
                 6. Web interactions:\n\
                    introspection_list_tools({\"category\": \"browser\"})\n\
                    Capabilities: render pages, scrape content, crawl sites\n\
                    Use for: Web automation and data extraction\n\n\
                 WORKFLOW-BASED DISCOVERY:\n\
                 1. Development workflow:\n\
                    // Files: filesystem category\n\
                    // Git: git category\n\
                    // Build: terminal category\n\
                    introspection_list_tools({\"category\": \"filesystem\"})\n\
                    introspection_list_tools({\"category\": \"git\"})\n\
                    introspection_list_tools({\"category\": \"terminal\"})\n\n\
                 2. Research workflow:\n\
                    // Search: fs_search, github_search_code\n\
                    // Read: fs_read_file\n\
                    // Analyze: introspection tools\n\
                    introspection_list_tools({\"search\": \"search\"})\n\
                    introspection_list_tools({\"category\": \"introspection\"})\n\n\
                 3. Data analysis workflow:\n\
                    // Query: database category\n\
                    // Process: terminal category\n\
                    // Store: filesystem category\n\
                    introspection_list_tools({\"category\": \"database\"})\n\
                    introspection_list_tools({\"category\": \"filesystem\"})\n\n\
                 KEYWORD-BASED DISCOVERY:\n\
                 Common keywords to search:\n\
                 - \"read\": Reading operations\n\
                 - \"write\": Writing operations\n\
                 - \"search\": Search functionality\n\
                 - \"list\": Listing/enumeration\n\
                 - \"create\": Creation operations\n\
                 - \"delete\": Deletion operations\n\
                 - \"edit\": Modification operations\n\
                 - \"move\": Moving/renaming\n\
                 - \"clone\": Cloning operations\n\
                 - \"commit\": Committing changes\n\
                 - \"execute\": Running commands\n\n\
                 DISCOVERY USE CASES:\n\
                 1. \"How do I find files containing text?\":\n\
                    introspection_list_tools({\"search\": \"search\"})\n\
                    → fs_search tool\n\n\
                 2. \"How do I modify a file without replacing it?\":\n\
                    introspection_list_tools({\"search\": \"edit\"})\n\
                    → fs_edit_block tool\n\n\
                 3. \"How do I clone a repository?\":\n\
                    introspection_list_tools({\"category\": \"git\"})\n\
                    → git_clone tool\n\n\
                 4. \"How do I search GitHub for code examples?\":\n\
                    introspection_list_tools({\"category\": \"github\"})\n\
                    → github_search_code tool\n\n\
                 5. \"How do I run a build script?\":\n\
                    introspection_list_tools({\"category\": \"terminal\"})\n\
                    → terminal tool\n\n\
                 EXPLORATION STRATEGY:\n\
                 1. Start with broad category:\n\
                    introspection_list_tools({\"category\": \"filesystem\"})\n\n\
                 2. Read all descriptions:\n\
                    Review each tool's description field\n\n\
                 3. Note relevant tools:\n\
                    Write down names that match your need\n\n\
                 4. Check tool properties:\n\
                    Verify read_only, destructive, idempotent flags\n\n\
                 5. Try the tool:\n\
                    Call it with appropriate arguments\n\n\
                 SAFETY-FIRST DISCOVERY:\n\
                 When exploring unfamiliar capabilities:\n\
                 1. Find read-only tools first:\n\
                    introspection_list_tools({\"read_only\": true})\n\
                    Safe to experiment without side effects\n\n\
                 2. Avoid destructive tools initially:\n\
                    Skip tools with destructive: true\n\
                    Until you understand them\n\n\
                 3. Prefer idempotent tools:\n\
                    introspection_list_tools({\"idempotent\": true})\n\
                    Safe to retry on errors\n\n\
                 DISCOVERY TIPS:\n\
                 - Always read tool descriptions carefully\n\
                 - Check tool properties before using\n\
                 - Start with read-only tools for safety\n\
                 - Use category filters for domain-specific tools\n\
                 - Use search for keyword-based discovery\n\
                 - Combine filters to narrow results\n\
                 - Explore related tools in same category\n\n\
                 CAPABILITY MAPPING:\n\
                 Task → Category → Tools:\n\
                 - File I/O → filesystem → fs_read_file, fs_write_file\n\
                 - Code search → filesystem → fs_search\n\
                 - Git ops → git → git_clone, git_commit, git_branch\n\
                 - GitHub search → github → github_search_code\n\
                 - Run commands → terminal → terminal\n\
                 - Process mgmt → process → process_list, process_kill\n\
                 - Web scraping → browser → browser tools\n\
                 - Monitoring → introspection → introspection_get_events\n\n\
                 Remember: The right tool depends on your specific task. Use filters and search to narrow down options, then read descriptions to find the perfect match!",
            ),
        },
    ]
}

/// Comprehensive guide covering all features and patterns
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to discovering and understanding available tools.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The introspection_list_tools tool is your complete catalog of available capabilities. Use it to discover what you can do.\n\n\
                 BASIC USAGE:\n\
                 List all tools:\n\
                 introspection_list_tools({})\n\n\
                 =============================================================================\n\
                 FILTERING OPTIONS\n\
                 =============================================================================\n\n\
                 BY CATEGORY:\n\
                 introspection_list_tools({\"category\": \"filesystem\"})\n\
                 introspection_list_tools({\"category\": \"git\"})\n\
                 introspection_list_tools({\"category\": \"terminal\"})\n\
                 introspection_list_tools({\"category\": \"github\"})\n\
                 introspection_list_tools({\"category\": \"browser\"})\n\
                 introspection_list_tools({\"category\": \"database\"})\n\
                 introspection_list_tools({\"category\": \"introspection\"})\n\n\
                 BY PROPERTIES:\n\
                 introspection_list_tools({\"read_only\": true})      // Safe tools\n\
                 introspection_list_tools({\"destructive\": true})    // Dangerous tools\n\
                 introspection_list_tools({\"idempotent\": true})     // Retry-safe tools\n\n\
                 BY SEARCH:\n\
                 introspection_list_tools({\"search\": \"read\"})\n\
                 introspection_list_tools({\"search\": \"search\"})\n\
                 introspection_list_tools({\"search\": \"delete\"})\n\n\
                 COMBINED FILTERS:\n\
                 introspection_list_tools({\n\
                     \"category\": \"filesystem\",\n\
                     \"read_only\": true\n\
                 })\n\n\
                 =============================================================================\n\
                 TOOL STRUCTURE\n\
                 =============================================================================\n\n\
                 Each tool includes:\n\
                 {\n\
                   \"name\": \"fs_read_file\",           // Tool identifier\n\
                   \"category\": \"filesystem\",         // Logical grouping\n\
                   \"description\": \"Read file...\",    // What it does\n\
                   \"read_only\": true,                 // Only reads data\n\
                   \"destructive\": false,              // Won't delete data\n\
                   \"idempotent\": true                 // Same result every time\n\
                 }\n\n\
                 =============================================================================\n\
                 TOOL CATEGORIES\n\
                 =============================================================================\n\n\
                 FILESYSTEM:\n\
                 - fs_read_file: Read single file\n\
                 - fs_read_multiple_files: Read multiple files\n\
                 - fs_write_file: Write/append to file\n\
                 - fs_search: Search file contents with ripgrep\n\
                 - fs_edit_block: Surgical text replacements\n\
                 - fs_list_directory: List directory contents\n\
                 - fs_create_directory: Create directories\n\
                 - fs_delete_file: Delete files\n\
                 - fs_delete_directory: Delete directories\n\
                 - fs_move_file: Move/rename files\n\
                 - fs_get_file_info: Get file metadata\n\n\
                 GIT:\n\
                 - git_clone: Clone repositories\n\
                 - git_diff: Show differences\n\
                 - git_commit: Commit changes\n\
                 - git_branch: Manage branches\n\
                 - git_merge: Merge branches\n\
                 - git_worktree: Manage worktrees\n\n\
                 GITHUB:\n\
                 - github_search_code: Search code on GitHub\n\
                 - github_search_issues: Search issues/PRs\n\
                 - github_search_repositories: Search repos\n\
                 - github_get_file_contents: Get file from repo\n\n\
                 TERMINAL:\n\
                 - terminal: Execute shell commands in persistent sessions\n\n\
                 PROCESS:\n\
                 - process_list: List running processes\n\
                 - process_kill: Terminate processes\n\n\
                 BROWSER:\n\
                 - Browser tools for web scraping and automation\n\n\
                 DATABASE:\n\
                 - Database query and schema tools\n\n\
                 INTROSPECTION:\n\
                 - introspection_list_tools: List available tools\n\
                 - introspection_get_events: Get execution history\n\
                 - introspection_inspect_tool_calls: Inspect tool calls\n\
                 - introspection_inspect_usage_stats: Get usage statistics\n\n\
                 =============================================================================\n\
                 DISCOVERY WORKFLOWS\n\
                 =============================================================================\n\n\
                 1. EXPLORE ALL CAPABILITIES:\n\
                 introspection_list_tools({})\n\
                 Review all available tools and categories\n\n\
                 2. DOMAIN-SPECIFIC DISCOVERY:\n\
                 introspection_list_tools({\"category\": \"filesystem\"})\n\
                 See all tools for a specific domain\n\n\
                 3. SAFE EXPLORATION:\n\
                 introspection_list_tools({\"read_only\": true})\n\
                 Find tools safe to experiment with\n\n\
                 4. TASK-BASED SEARCH:\n\
                 introspection_list_tools({\"search\": \"search\"})\n\
                 Find tools matching a keyword\n\n\
                 5. DANGER ZONE REVIEW:\n\
                 introspection_list_tools({\"destructive\": true})\n\
                 Identify tools that can delete data\n\n\
                 =============================================================================\n\
                 TOOL PROPERTIES EXPLAINED\n\
                 =============================================================================\n\n\
                 READ_ONLY:\n\
                 - true: Tool only reads, never modifies\n\
                 - false: Tool can create/modify/delete\n\
                 - Use read_only tools for safe exploration\n\n\
                 DESTRUCTIVE:\n\
                 - true: Tool can permanently delete data\n\
                 - false: Tool won't destroy data\n\
                 - Use caution with destructive tools\n\n\
                 IDEMPOTENT:\n\
                 - true: Same input → same output every time\n\
                 - false: Results may vary\n\
                 - Idempotent tools safe to retry\n\n\
                 =============================================================================\n\
                 COMMON QUESTIONS\n\
                 =============================================================================\n\n\
                 \"What can I do with files?\":\n\
                 introspection_list_tools({\"category\": \"filesystem\"})\n\n\
                 \"How do I search for code?\":\n\
                 introspection_list_tools({\"search\": \"search\"})\n\
                 → fs_search or github_search_code\n\n\
                 \"What git operations are available?\":\n\
                 introspection_list_tools({\"category\": \"git\"})\n\n\
                 \"Which tools are safe to experiment with?\":\n\
                 introspection_list_tools({\"read_only\": true})\n\n\
                 \"What tools might delete my data?\":\n\
                 introspection_list_tools({\"destructive\": true})\n\n\
                 \"How do I run shell commands?\":\n\
                 introspection_list_tools({\"category\": \"terminal\"})\n\
                 → terminal tool\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. DISCOVERY:\n\
                    - Start with introspection_list_tools({})\n\
                    - Review categories field\n\
                    - Explore categories one by one\n\
                    - Read tool descriptions carefully\n\n\
                 2. SAFETY:\n\
                    - Check read_only flag before using\n\
                    - Avoid destructive tools until understood\n\
                    - Test on non-critical data first\n\
                    - Prefer idempotent tools for reliability\n\n\
                 3. SEARCH:\n\
                    - Use category for domain filtering\n\
                    - Use search for keyword matching\n\
                    - Combine filters for precision\n\
                    - Review multiple options before choosing\n\n\
                 4. LEARNING:\n\
                    - Explore one category at a time\n\
                    - Try read_only tools first\n\
                    - Read descriptions thoroughly\n\
                    - Note tool names for later use\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 All tools: introspection_list_tools({})\n\
                 By category: introspection_list_tools({\"category\": \"filesystem\"})\n\
                 Safe tools: introspection_list_tools({\"read_only\": true})\n\
                 Search tools: introspection_list_tools({\"search\": \"read\"})\n\
                 Dangerous: introspection_list_tools({\"destructive\": true})\n\n\
                 Remember: introspection_list_tools is your map to all capabilities. Use it to discover what you can do, then use the actual tools to do it!",
            ),
        },
    ]
}
