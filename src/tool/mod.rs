//! Core Tool trait and error types for MCP tool implementations
//!
//! This crate provides the fundamental abstractions for building MCP tools:
//! - The `Tool` trait that defines tool behavior and RMCP integration
//! - The `McpError` type for tool execution errors
//! - The `tool_history` module for tracking tool call history
//!
//! # Example
//!
//! ```rust
//! use kodegen_mcp_schema::{Tool, McpError};
//! use serde::{Deserialize, Serialize};
//! use schemars::JsonSchema;
//! use serde_json::Value;
//! use rmcp::model::{PromptArgument, PromptMessage};
//!
//! pub struct MyTool;
//!
//! #[derive(Deserialize, Serialize, JsonSchema)]
//! pub struct MyArgs {
//!     message: String,
//! }
//!
//! #[derive(Deserialize, Serialize, JsonSchema)]
//! pub struct MyPromptArgs {}
//!
//! impl Tool for MyTool {
//!     type Args = MyArgs;
//!     type PromptArgs = MyPromptArgs;
//!     
//!     fn name() -> &'static str { "my_tool" }
//!     fn description() -> &'static str { "Example tool" }
//!     
//!     async fn execute(&self, args: Self::Args) -> Result<Value, McpError> {
//!         Ok(serde_json::json!({ "message": args.message }))
//!     }
//!     
//!     fn prompt_arguments() -> Vec<PromptArgument> {
//!         vec![]
//!     }
//!     
//!     async fn prompt(&self, _args: Self::PromptArgs) -> Result<Vec<PromptMessage>, McpError> {
//!         Ok(vec![])
//!     }
//! }
//! ```

pub mod error;
pub mod traits;
pub mod tool_history;

// Re-export the main types for convenience
pub use error::McpError;
pub use traits::{Tool, ToolArgs, ToolExecutionContext, ToolResponse, PromptProvider, SealedPromptProvider, ToolStatus, add_branded_line_to_result};
pub use tool_history::ToolCallRecord;
