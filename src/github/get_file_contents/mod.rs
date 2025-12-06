//! GitHub get file contents tool schema

pub mod schema;
pub mod prompt_args;
pub mod prompts;

// Re-export all types for convenient access
pub use schema::*;
pub use prompt_args::*;
pub use prompts::GetFileContentsPrompts;
