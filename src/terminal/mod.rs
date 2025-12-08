//! Terminal category module

// Re-export all terminal tool name constants from kodegen_config
pub use kodegen_config::TERMINAL;

pub mod prompt_args;
pub mod prompts;
pub mod schema;

pub use prompt_args::TerminalPromptArgs;
pub use prompts::TerminalPrompts;
pub use schema::*;
