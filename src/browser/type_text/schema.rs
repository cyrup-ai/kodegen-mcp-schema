//! Schema types for browser_type_text tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{ToolArgs, tool_metadata};
use super::prompts::TypeTextPrompts;

// ============================================================================
// CANONICAL TOOL NAME CONSTANTS
// ============================================================================

pub const BROWSER_TYPE_TEXT: &str = "browser_type_text";

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn default_clear() -> bool {
    true
}

// ============================================================================
// INPUT ARGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserTypeTextArgs {
    /// CSS selector for input element
    pub selector: String,

    /// Text to type into the element
    pub text: String,

    /// Optional: clear existing text first (default: true)
    #[serde(default = "default_clear")]
    pub clear: bool,

    /// Optional: timeout in milliseconds (default: 5000)
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

// ============================================================================
// OUTPUT
// ============================================================================

/// Output from `browser_type` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserTypeOutput {
    pub success: bool,
    pub selector: String,
    pub text_length: usize,
    pub message: String,
}

// ============================================================================
// TOOL ARGS TRAIT IMPL
// ============================================================================

#[tool_metadata(
    name = "browser_type_text",
    category = "browser",
    description = "Type text into input fields, textareas, and other editable elements with optional clear-first behavior"
)]
impl ToolArgs for BrowserTypeTextArgs {
    type Output = BrowserTypeOutput;
    type Prompts = TypeTextPrompts;

    const NAME: &'static str = BROWSER_TYPE_TEXT;
    const CATEGORY: &'static str = "browser";
    const DESCRIPTION: &'static str = "Type text into input fields, textareas, and other editable elements with optional clear-first behavior";
}
