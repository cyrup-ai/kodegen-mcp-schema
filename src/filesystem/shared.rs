//! Shared types used across filesystem tools

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// SEARCH-SPECIFIC ENUMS (used by fs_search)
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

/// Pattern matching mode for filename search
/// 
/// When specified as INPUT: Forces the pattern to be interpreted as specified type.
/// When returned as OUTPUT: Indicates how the pattern was actually interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum FsPatternMode {
    /// Regular expression pattern
    /// Detected by: ^, $, \., \d, \w, \s, .*, .+, (?...), |, [...]+ 
    Regex,
    /// Glob/shell wildcard pattern  
    /// Detected by: *, ?, **, {a,b}, [abc]
    Glob,
    /// Plain substring match (default when no special characters detected)
    /// Also used when literal_search=true
    #[default]
    Substring,
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Default value for search instance number
pub const fn zero() -> u32 {
    0
}

/// Default timeout for search operations (1 minute)
pub const fn default_search_timeout_ms() -> u64 {
    60_000 // 1 minute default (terminal uses 5 minutes)
}
