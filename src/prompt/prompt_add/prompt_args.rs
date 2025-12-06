//! Prompt argument types for add_prompt tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for add_prompt tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddPromptPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Creating simple prompts
    /// - "templating": Using Jinja2 templates
    /// - "organization": Prompt naming and organization
    /// - "workflows": Prompt creation workflows
    /// - "comprehensive": All scenarios combined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
