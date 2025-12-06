//! GitHub list issues tool schema

pub mod prompt_args;
pub mod prompts;
pub mod schema;

// Re-export all types for convenient access
pub use prompt_args::*;
pub use prompts::*;
pub use schema::*;
