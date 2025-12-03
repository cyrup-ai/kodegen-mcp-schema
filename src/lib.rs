//! Lightweight MCP tool schema definitions
//! 
//! This package contains ONLY Args and PromptArgs type definitions
//! with zero heavy dependencies. It serves as the single source of
//! truth for all tool schemas in the kodegen ecosystem.

use serde::{Serialize, de::DeserializeOwned};
use schemars::JsonSchema;

// ============================================================================
// TOOL ARGS TRAIT (Args→Output Mapping)
// ============================================================================

/// Trait for tool argument types that enforces Args→Output mapping.
///
/// This trait is implemented in the schema package for each tool's Args type,
/// binding it to the correct Output type. The Tool trait then derives the
/// output type from Args, providing compile-time enforcement.
///
/// # Why This Is In The Schema Package
///
/// Only `kodegen-mcp-schema` implements this trait. External crates cannot
/// implement it for their own types, ensuring all tools use official schemas.
/// The output type is LOCKED to the Args type - tools cannot choose wrong output.
///
/// # Example
///
/// ```rust
/// // In kodegen-mcp-schema
/// impl ToolArgs for TerminalInput {
///     type Output = TerminalOutput;
/// }
///
/// // In tool implementation - compiler enforces TerminalOutput
/// impl Tool for TerminalTool {
///     type Args = TerminalInput;
///     // execute() MUST return ToolResponse<TerminalOutput>
///     // because TerminalInput::Output = TerminalOutput
/// }
/// ```
pub trait ToolArgs: DeserializeOwned + Serialize + JsonSchema + Send + 'static {
    /// The output type for tools using these args.
    /// Defined here in schema package, not in tool impl - provides compile-time enforcement.
    type Output: Serialize + JsonSchema + Send + 'static;
}

pub mod filesystem;
pub mod terminal;
pub mod serde_helpers;
pub mod git;
pub mod github;
pub mod browser;
pub mod citescrape;
pub mod database;
pub mod reasoning;
pub mod claude_agent;
pub mod prompt;
pub mod introspection;
pub mod process;
pub mod config;

// Re-export all types at crate root for convenience
// Note: ToolArgs trait is already defined in this file, no need to re-export
pub use filesystem::*;
pub use terminal::*;
pub use git::*;
pub use github::*;
pub use browser::*;
pub use citescrape::*;
pub use database::*;
pub use reasoning::*;
pub use claude_agent::*;
pub use prompt::*;
pub use introspection::*;
pub use process::*;
pub use config::*;
