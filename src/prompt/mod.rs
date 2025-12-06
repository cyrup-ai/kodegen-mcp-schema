//! Prompt category module

pub mod prompt_add;
pub mod prompt_delete;
pub mod prompt_edit;
pub mod prompt_get;

// Re-export prompt_get tool (includes all shared types)
pub use prompt_get::{
    PROMPT_GET,
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
    PROMPT_ADD,
    AddPromptArgs,
    PromptAddOutput,
    AddPromptPromptArgs,
    PromptAddPrompts,
};

// Re-export prompt_delete tool
pub use prompt_delete::{
    PROMPT_DELETE,
    DeletePromptArgs,
    PromptDeleteOutput,
    DeletePromptPromptArgs,
    PromptDeletePrompts,
};

// Re-export prompt_edit tool
pub use prompt_edit::{
    PROMPT_EDIT,
    EditPromptArgs,
    PromptEditOutput,
    EditPromptPromptArgs,
    PromptEditPrompts,
};
