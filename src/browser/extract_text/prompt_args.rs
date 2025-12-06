//! Prompt argument types for browser_extract_text tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for browser_extract_text tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserExtractTextPromptArgs {
    /// Scenario to show examples for
    /// - "page_content": Reading full page or sections
    /// - "specific_elements": Targeting specific data
    /// - "structured_data": Extracting tables, lists, forms
    /// - "verification": Checking expected content exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
