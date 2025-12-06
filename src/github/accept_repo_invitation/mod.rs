//! GitHub accept repository invitation tool schema

pub mod prompt_args;
pub mod prompts;

// Re-export all types for convenient access
pub use prompt_args::*;
pub use prompts::GithubAcceptRepoInvitationPrompts;
