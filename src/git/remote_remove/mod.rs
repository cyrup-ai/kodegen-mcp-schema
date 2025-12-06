//! Git remote_remove tool schema, prompts, and prompt arguments

pub mod schema;
pub mod prompt_args;
pub mod prompts;

// Re-export for convenient access
pub use schema::*;
pub use prompt_args::*;
pub use prompts::*;
