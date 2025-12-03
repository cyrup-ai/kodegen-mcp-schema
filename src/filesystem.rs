//! Filesystem tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

/// Canonical tool names for all filesystem tools
/// These constants are the single source of truth for filesystem tool names.
/// All tool implementations, metadata, and utilities MUST reference these constants.
pub const FS_CREATE_DIRECTORY: &str = "fs_create_directory";
pub const FS_DELETE_DIRECTORY: &str = "fs_delete_directory";
pub const FS_DELETE_FILE: &str = "fs_delete_file";
pub const FS_EDIT_BLOCK: &str = "fs_edit_block";
pub const FS_GET_FILE_INFO: &str = "fs_get_file_info";
pub const FS_LIST_DIRECTORY: &str = "fs_list_directory";
pub const FS_MOVE_FILE: &str = "fs_move_file";
pub const FS_READ_FILE: &str = "fs_read_file";
pub const FS_READ_MULTIPLE_FILES: &str = "fs_read_multiple_files";
pub const FS_SEARCH: &str = "fs_search";
pub const FS_WRITE_FILE: &str = "fs_write_file";

// ============================================================================
// READ FILE
// ============================================================================

/// Arguments for `fs_read_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsReadFileArgs {
    /// Path to the file to read (or URL if `is_url` is true)
    pub path: String,

    /// Line offset to start reading from (0-based)
    /// Positive: Start from line N (0-based indexing)
    /// Negative: Read last N lines from end (tail behavior)
    #[serde(default)]
    pub offset: i64,

    /// Maximum number of lines to read (None = use tool's default)
    /// Ignored when offset is negative
    #[serde(default)]
    pub length: Option<usize>,

    /// Whether the path is a URL (auto-detected if not specified)
    #[serde(default)]
    pub is_url: bool,
}

/// Prompt arguments for `fs_read_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsReadFilePromptArgs {
    /// Optional: specific file type to focus examples on
    #[serde(default)]
    pub file_type: Option<String>,
}

// ============================================================================
// READ MULTIPLE FILES
// ============================================================================

/// Arguments for `fs_read_multiple_files` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsReadMultipleFilesArgs {
    /// List of file paths to read
    /// 
    /// Accepts both single string and array: `paths: "file.txt"` or `paths: ["file1.txt", "file2.txt"]`
    #[serde(deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub paths: Vec<String>,

    /// Line offset for all files (optional)
    /// Positive: Start from line N (0-based indexing)
    /// Negative: Read last N lines from end (tail behavior)
    #[serde(default)]
    pub offset: i64,

    /// Max lines to read per file (optional)
    /// Ignored when offset is negative
    #[serde(default)]
    pub length: Option<usize>,
}

/// Prompt arguments for `fs_read_multiple_files` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsReadMultipleFilesPromptArgs {
    /// Optional: focus examples on specific file type (e.g., 'json', 'log', 'rust')
    /// Helps tailor examples to agent's domain context
    #[serde(default)]
    pub file_type: Option<String>,
}

// ============================================================================
// WRITE FILE
// ============================================================================

/// Arguments for `fs_write_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsWriteFileArgs {
    /// Path to the file to write
    pub path: String,

    /// Content to write to the file
    pub content: String,

    /// Write mode: "rewrite" (default) or "append"
    #[serde(default = "default_mode")]
    pub mode: String,
}

fn default_mode() -> String {
    "rewrite".to_string()
}

/// Prompt arguments for `fs_write_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsWriteFilePromptArgs {
    #[serde(default)]
    pub example_type: Option<String>,
}

// ============================================================================
// MOVE FILE
// ============================================================================

/// Arguments for `fs_move_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsMoveFileArgs {
    /// Source path (file or directory to move)
    pub source: String,

    /// Destination path (where to move it)
    pub destination: String,
}

/// Prompt arguments for `fs_move_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsMoveFilePromptArgs {
    /// Optional focus area for the teaching prompt
    /// Valid values: "rename", "move_directory", "atomic_behavior", "edge_cases", "best_practices"
    #[serde(default)]
    pub operation_focus: Option<String>,
}

// ============================================================================
// DELETE FILE
// ============================================================================

/// Arguments for `fs_delete_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsDeleteFileArgs {
    /// Path to the file to delete
    pub path: String,
}

/// Prompt arguments for `fs_delete_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsDeleteFilePromptArgs {}

// ============================================================================
// DELETE DIRECTORY
// ============================================================================

/// Arguments for `fs_delete_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsDeleteDirectoryArgs {
    /// Path to the directory to delete
    pub path: String,

    /// Confirm recursive deletion (must be true)
    #[serde(default)]
    pub recursive: bool,
}

/// Prompt arguments for `fs_delete_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsDeleteDirectoryPromptArgs {}

// ============================================================================
// LIST DIRECTORY
// ============================================================================

/// Arguments for `fs_list_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsListDirectoryArgs {
    /// Path to the directory to list
    pub path: String,

    /// Include hidden files (starting with .)
    #[serde(default)]
    pub include_hidden: bool,
}

/// Prompt arguments for `fs_list_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsListDirectoryPromptArgs {
    #[serde(default)]
    pub show_advanced: Option<bool>,
}

// ============================================================================
// CREATE DIRECTORY
// ============================================================================

/// Arguments for `fs_create_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsCreateDirectoryArgs {
    /// Path to the directory to create
    pub path: String,
}

/// Prompt arguments for `fs_create_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsCreateDirectoryPromptArgs {
    /// Optional: Use case for customized examples
    /// - "basic": Single directory creation
    /// - "nested": Creating deep directory hierarchies
    /// - "idempotence": Safe repeated calls with same path
    /// - "validation": Path normalization and security behavior
    ///
    /// Default if omitted: comprehensive overview covering all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}

// ============================================================================
// GET FILE INFO
// ============================================================================

/// Arguments for `fs_get_file_info` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsGetFileInfoArgs {
    /// Path to the file or directory
    pub path: String,
}

/// Prompt arguments for `fs_get_file_info` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsGetFileInfoPromptArgs {
    /// Focus area for examples: 'permissions', 'timestamps', 'sizes', 'line_counts', 'platform_differences', or 'all'
    #[serde(default)]
    pub focus_area: Option<String>,
}

// ============================================================================
// EDIT BLOCK
// ============================================================================

/// Arguments for `fs_edit_block` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsEditBlockArgs {
    /// Path to the file to edit
    pub path: String,

    /// The exact string to search for and replace
    pub old_string: String,

    /// The replacement string
    pub new_string: String,

    /// Expected number of replacements (defaults to 1)
    #[serde(default = "default_expected_replacements")]
    pub expected_replacements: usize,
}

fn default_expected_replacements() -> usize {
    1
}

/// Prompt arguments for `fs_edit_block` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsEditBlockPromptArgs {
    /// Optional: aspect to focus teaching on
    /// - "fuzzy-search": emphasis on fuzzy matching and similarity threshold
    /// - "expected-replacements": emphasis on count verification and error detection
    /// - "line-endings": emphasis on line ending normalization
    /// - "edge-cases": emphasis on gotchas and error conditions
    ///
    /// Default if omitted: comprehensive coverage of all aspects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_focus: Option<String>,
    
    /// Optional: include advanced topics (fuzzy matching internals, config)
    /// Default: false (focus on common usage patterns)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_advanced: Option<bool>,
}

// ============================================================================
// SEARCH TYPES AND ENUMS
// ============================================================================

/// Where to search
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum SearchIn {
    /// Search inside file contents (default, matches ripgrep default)
    #[default]
    Content,
    
    /// Search file names/paths
    Filenames,
}

/// Case matching mode for searches
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum CaseMode {
    /// Case-sensitive matching (default)
    #[default]
    Sensitive,
    /// Case-insensitive matching
    Insensitive,
    /// Smart case: insensitive if pattern is all lowercase, sensitive otherwise
    Smart,
}

/// Boundary mode for pattern matching
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryMode {
    /// Match only when pattern appears as complete line (^pattern$)
    /// Example: "error" matches "error" but not "this error happened"
    Line,
    /// Match only when pattern is surrounded by word boundaries (\bpattern\b)
    /// Example: "test" matches "`test()`" but not "testing"
    Word,
}

/// Regex engine choice for content search
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum EngineChoice {
    /// Automatically choose best engine (tries Rust, falls back to PCRE2)
    #[default]
    Auto,
    /// Use Rust regex engine only
    Rust,
    /// Use PCRE2 regex engine (supports backreferences, look-around)
    PCRE2,
}

/// Binary file handling mode (matches ripgrep's --binary and -a/--text flags)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum BinaryMode {
    /// Automatically skip binary files (default, no flag in rg)
    #[default]
    Auto,
    /// Search binary files but suppress binary content (rg --binary)
    Binary,
    /// Treat all files as text (rg -a/--text)
    Text,
}

/// Sort criterion for search results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SortBy {
    /// Sort alphabetically by file path (A-Z or Z-A)
    Path,
    /// Sort by last modified time (recent first or oldest first)
    Modified,
    /// Sort by last accessed time (if available on platform)
    Accessed,
    /// Sort by creation time (if available on platform)
    Created,
}

/// Sort direction for search results
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    /// Ascending order: oldest first (time) or A-Z (path)
    Ascending,
    /// Descending order: newest first (time) or Z-A (path)
    Descending,
}

/// What to return from search
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum ReturnMode {
    /// Full match details: file path, line number, match content (default)
    #[default]
    Matches,
    
    /// Only return unique file paths (like rg -l)
    Paths,
    
    /// Return match counts per file (like rg -c)
    Counts,
}

// ============================================================================
// FS_SEARCH - Elite Terminal Pattern
// ============================================================================

const fn zero() -> u32 {
    0
}

const fn default_search_timeout_ms() -> u64 {
    60_000 // 1 minute default (terminal uses 5 minutes)
}

/// Search action types
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FsSearchAction {
    /// Execute a search (default) - requires `path` and `pattern` fields
    #[default]
    Search,
    /// Read current search state without re-executing
    Read,
    /// List all active searches with their current states
    List,
    /// Gracefully cancel search and cleanup all resources
    Kill,
}

/// Arguments for `fs_search` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsSearchArgs {
    /// Action to perform - defaults to SEARCH for backward compatibility
    #[serde(default)]
    pub action: FsSearchAction,

    /// Search instance number (0, 1, 2...) - defaults to 0
    /// Searches are reusable and stateful - use different numbers for parallel work
    #[serde(default = "zero")]
    pub search: u32,

    /// Maximum time in milliseconds to wait for search completion (default 60000ms = 1 minute)
    ///
    /// - On timeout: returns current results snapshot, search continues in background
    /// - Special value 0: fire-and-forget background task (returns immediately)
    /// - Search continues running after timeout - use action=READ to check progress
    #[serde(default = "default_search_timeout_ms")]
    pub await_completion_ms: u64,

    // ========================================================================
    // SEARCH-SPECIFIC FIELDS
    // ========================================================================
    
    /// Root directory to search (required for SEARCH, ignored for READ/LIST/KILL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Pattern to search for (required for SEARCH, ignored for READ/LIST/KILL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    /// Where to search: "content" (inside files) or "filenames" (file paths)
    /// Default: "content" (matches ripgrep default behavior)
    #[serde(default)]
    pub search_in: SearchIn,

    /// What to return: "matches" (full details), "paths" (file paths only), or "counts" (match counts)
    /// Default: "matches" (matches ripgrep default behavior)
    #[serde(default)]
    pub return_only: ReturnMode,

    /// Glob pattern to filter files (e.g. "*.js", "*.{ts,tsx}") - maps to rg --glob
    #[serde(default)]
    pub file_pattern: Option<String>,

    /// File types to include using ripgrep's built-in definitions (rg --type)
    /// 
    /// Accepts both single string and array: `type: "rust"` or `type: ["rust", "python"]`
    #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub r#type: Vec<String>,

    /// File types to exclude using ripgrep's built-in definitions (rg --type-not)
    /// 
    /// Accepts both single string and array: `type_not: "test"` or `type_not: ["test", "json"]`
    #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub type_not: Vec<String>,

    /// DEPRECATED: Use `case_mode` instead. Provided for backward compatibility.
    #[serde(default)]
    pub ignore_case: Option<bool>,

    /// Case matching mode: "sensitive", "insensitive", or "smart" (default: "sensitive")
    #[serde(default)]
    pub case_mode: CaseMode,

    /// Maximum number of results
    #[serde(default)]
    pub max_results: Option<u32>,

    /// Include hidden files
    #[serde(default)]
    pub include_hidden: bool,

    /// Disable all ignore files (.gitignore, .ignore, etc.)
    #[serde(default)]
    pub no_ignore: bool,

    /// Number of context lines (rg -C / rg --context)
    #[serde(default)]
    pub context: u32,

    /// Number of lines before each match (rg -B / rg --before-context)
    #[serde(default)]
    pub before_context: Option<u32>,

    /// Number of lines after each match (rg -A / rg --after-context)
    #[serde(default)]
    pub after_context: Option<u32>,

    /// Timeout in milliseconds
    #[serde(default)]
    pub timeout_ms: Option<u64>,

    /// Stop early when exact filename match found (files only)
    #[serde(default)]
    pub early_termination: Option<bool>,

    /// Force literal string matching instead of regex (default: false)
    #[serde(default)]
    pub literal_search: bool,

    /// DEPRECATED: Use `boundary_mode="word"` instead. Provided for backward compatibility.
    #[serde(default)]
    pub word_boundary: Option<bool>,

    /// Boundary mode for pattern matching: "word", "line", or null (default: null)
    #[serde(default)]
    pub boundary_mode: Option<String>,

    /// Invert match - show lines/files that DON'T match the pattern
    #[serde(default)]
    pub invert_match: bool,

    /// Regex engine choice: "auto", "rust", or "pcre2" (default: "auto")
    #[serde(default)]
    pub engine: EngineChoice,

    /// Preprocessor command to run on files before searching
    #[serde(default)]
    pub preprocessor: Option<String>,

    /// Glob patterns for files to run through preprocessor
    /// 
    /// Accepts both single string and array: `preprocessor_globs: "*.txt"` or `preprocessor_globs: ["*.txt", "*.md"]`
    #[serde(default, deserialize_with = "crate::serde_helpers::string_or_vec")]
    pub preprocessor_globs: Vec<String>,

    /// Enable searching inside compressed files (.gz, .zip, .bz2, .xz)
    #[serde(default)]
    pub search_zip: bool,

    /// Binary file handling mode (default: Auto)
    #[serde(default)]
    pub binary_mode: BinaryMode,

    /// Enable multiline pattern matching (rg --multiline)
    #[serde(default)]
    pub multiline: bool,

    /// Skip files larger than this size in bytes (None = unlimited)
    #[serde(default)]
    pub max_filesize: Option<u64>,

    /// Maximum directory depth to traverse (None = unlimited)
    #[serde(default)]
    pub max_depth: Option<usize>,

    /// Return only the matched portion of text, not the entire line
    #[serde(default)]
    pub only_matching: bool,

    /// Sort results by specified criterion (None = no sorting, filesystem order)
    #[serde(default)]
    pub sort_by: Option<SortBy>,

    /// Sort direction (default: Ascending if `sort_by` is specified)
    #[serde(default)]
    pub sort_direction: Option<SortDirection>,

    /// Text encoding (None = auto-detect)
    #[serde(default)]
    pub encoding: Option<String>,
}

/// Prompt arguments for `fs_search` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct FsSearchPromptArgs {}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

use crate::ToolArgs;

/// Output from `fs_read_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsReadFileOutput {
    pub success: bool,
    pub path: String,
    pub mime_type: String,
    pub is_image: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_lines: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_read: Option<u64>,
    pub is_partial: bool,
    pub content: String,
}

/// Output from `fs_read_multiple_files` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsReadMultipleFilesOutput {
    pub success: bool,
    pub files_requested: usize,
    pub files_read: usize,
    pub files_failed: usize,
    pub results: Vec<FileReadResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileReadResult {
    pub path: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

/// Output from `fs_write_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsWriteFileOutput {
    pub success: bool,
    pub path: String,
    pub bytes_written: u64,
    pub lines_written: u64,
    pub mode: String,
}

/// Output from `fs_edit_block` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsEditBlockOutput {
    pub success: bool,
    pub path: String,
    pub replacements_made: u32,
    pub message: String,
}

/// Output from `fs_create_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsCreateDirectoryOutput {
    pub success: bool,
    pub path: String,
    pub created: bool,
    pub message: String,
}

/// Output from `fs_delete_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsDeleteDirectoryOutput {
    pub success: bool,
    pub path: String,
    pub message: String,
}

/// Output from `fs_delete_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsDeleteFileOutput {
    pub success: bool,
    pub path: String,
    pub message: String,
}

/// Output from `fs_move_file` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsMoveFileOutput {
    pub success: bool,
    pub source: String,
    pub destination: String,
    pub message: String,
}

/// Output from `fs_list_directory` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsListDirectoryOutput {
    pub success: bool,
    pub path: String,
    pub total_entries: usize,
    pub directories: usize,
    pub files: usize,
    pub entries: Vec<DirectoryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DirectoryEntry {
    pub name: String,
    pub is_directory: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

/// Output from `fs_get_file_info` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsGetFileInfoOutput {
    pub success: bool,
    pub path: String,
    pub exists: bool,
    pub is_file: bool,
    pub is_directory: bool,
    pub is_symlink: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_count: Option<u64>,
}

/// Result type for search operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum FsSearchResultType {
    /// File path match (filename search)
    File,
    /// Content match within a file
    Content,
    /// File list entry (paths-only mode)
    FileList,
}

/// Single search result from fs_search
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsSearchResult {
    /// File path where match was found
    pub file: String,
    /// Line number (content search only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    /// Matching line content (content search only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,
    /// Result type
    pub r#type: FsSearchResultType,
    /// True if this is a context line, false if actual match
    #[serde(default)]
    pub is_context: bool,
    /// Whether this result came from a binary file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_binary: Option<bool>,
    /// Whether binary content was suppressed in this result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_suppressed: Option<bool>,
}

/// Snapshot of a single search instance (for LIST action)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsSearchSnapshot {
    /// Search instance ID
    pub search: u32,
    /// Search pattern (if search was started)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Root path being searched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Number of matches found so far
    pub match_count: usize,
    /// Number of files searched so far
    pub files_searched: usize,
    /// Whether the search has completed
    pub completed: bool,
    /// Time elapsed in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

/// Unified output from `fs_search` tool (all actions: SEARCH, READ, LIST, KILL)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsSearchOutput {
    /// Search instance ID (None for LIST action which returns multiple)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<u32>,
    /// Human-readable status message
    pub output: String,
    /// Search pattern used (present for SEARCH/READ actions)
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pattern: String,
    /// Root path searched (present for SEARCH/READ actions)
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    /// Search results (present for SEARCH/READ actions)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<FsSearchResult>,
    /// List of all active search snapshots (present for LIST action)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub searches: Vec<FsSearchSnapshot>,
    /// Total number of matches found
    #[serde(default)]
    pub match_count: usize,
    /// Number of files that were searched
    #[serde(default)]
    pub files_searched: usize,
    /// Number of errors encountered during search
    #[serde(default)]
    pub error_count: usize,
    /// Error messages from search operation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    /// Time elapsed in milliseconds
    pub duration_ms: u64,
    /// Whether the operation has completed
    pub completed: bool,
    /// Whether the operation was successful
    pub success: bool,
    /// Exit code (0 = success, 1 = error, 130 = cancelled/killed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// Error message if operation failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchMatch {
    pub file: String,
    pub line: u32,
    #[serde(rename = "match")]
    pub match_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_context: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_context: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileMatchCount {
    pub file: String,
    pub count: u32,
}

// ============================================================================
// TOOL ARGS IMPLEMENTATION (Args→Output Binding)
// ============================================================================

impl ToolArgs for FsReadFileArgs {
    type Output = FsReadFileOutput;
}

impl ToolArgs for FsReadMultipleFilesArgs {
    type Output = FsReadMultipleFilesOutput;
}

impl ToolArgs for FsWriteFileArgs {
    type Output = FsWriteFileOutput;
}

impl ToolArgs for FsEditBlockArgs {
    type Output = FsEditBlockOutput;
}

impl ToolArgs for FsCreateDirectoryArgs {
    type Output = FsCreateDirectoryOutput;
}

impl ToolArgs for FsDeleteDirectoryArgs {
    type Output = FsDeleteDirectoryOutput;
}

impl ToolArgs for FsDeleteFileArgs {
    type Output = FsDeleteFileOutput;
}

impl ToolArgs for FsMoveFileArgs {
    type Output = FsMoveFileOutput;
}

impl ToolArgs for FsListDirectoryArgs {
    type Output = FsListDirectoryOutput;
}

impl ToolArgs for FsGetFileInfoArgs {
    type Output = FsGetFileInfoOutput;
}

impl ToolArgs for FsSearchArgs {
    type Output = FsSearchOutput;
}
