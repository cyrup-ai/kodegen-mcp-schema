//! Prompt category module

// Re-export all prompt tool name constants from kodegen_config
pub use kodegen_config::{PROMPT_ADD, PROMPT_DELETE, PROMPT_EDIT, PROMPT_GET};

pub mod prompt_add;
pub mod prompt_delete;
pub mod prompt_edit;
pub mod prompt_get;

// Re-export prompt_get tool (includes all shared types)
pub use prompt_get::{
    GetPromptArgs,
    GetPromptAction,
    PromptGetOutput,
    GetPromptPromptArgs,
    PromptGetPrompts,
    // Shared types
    TemplateParamValue,
    CategoryInfo,
    PromptCategoriesResult,
    PromptParameterDef,
    PromptParameterType,
    PromptSummary,
    PromptListResult,
    PromptMetadataOutput,
    PromptContentResult,
    PromptRenderedResult,
    PromptResult,
};

// Re-export prompt_add tool
pub use prompt_add::{
    AddPromptArgs,
    PromptAddOutput,
    AddPromptPromptArgs,
    PromptAddPrompts,
};

// Re-export prompt_delete tool
pub use prompt_delete::{
    DeletePromptArgs,
    PromptDeleteOutput,
    DeletePromptPromptArgs,
    PromptDeletePrompts,
};

// Re-export prompt_edit tool
pub use prompt_edit::{
    EditPromptArgs,
    PromptEditOutput,
    EditPromptPromptArgs,
    PromptEditPrompts,
};
