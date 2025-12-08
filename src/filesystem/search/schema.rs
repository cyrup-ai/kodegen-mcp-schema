//! Schema types for fs_search tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_FILESYSTEM, FS_SEARCH};
use crate::{ToolArgs, tool_metadata};
use crate::filesystem::shared::*;
use super::prompts::SearchPrompts;

// ============================================================================
// SEARCH ARGS
// ============================================================================

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

// ============================================================================
// SEARCH OUTPUT
// ============================================================================

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

// ============================================================================
// SUPPORTING TYPES (for future compatibility)
// ============================================================================

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
// TOOL ARGS TRAIT IMPLEMENTATION
// ============================================================================

#[tool_metadata(
    description = "Blazing-fast content and filename search powered by ripgrep. Respects .gitignore automatically. Supports regex, case modes, file filtering, and background execution"
)]
impl ToolArgs for FsSearchArgs {
    type Output = FsSearchOutput;
    type Prompts = SearchPrompts;

    const NAME: &'static str = FS_SEARCH;
    const CATEGORY: &'static str = CATEGORY_FILESYSTEM;
    const DESCRIPTION: &'static str = "Blazing-fast content and filename search powered by ripgrep. Respects .gitignore automatically. Supports regex, case modes, file filtering, and background execution";
}
