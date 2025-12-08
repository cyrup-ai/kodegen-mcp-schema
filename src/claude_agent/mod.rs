//! Claude agent category module

// Re-export all claude_agent tool name constants from kodegen_config
pub use kodegen_config::CLAUDE_AGENT;

pub mod agent;
pub mod shared;

// Re-export agent tool types for convenient access
pub use agent::*;
pub use shared::*;
