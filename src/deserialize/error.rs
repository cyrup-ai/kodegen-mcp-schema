//! Deserialization error types

use std::fmt;

/// Errors that can occur during tool output deserialization
#[derive(Debug)]
pub enum DeserializeError {
    /// Unknown tool name provided
    UnknownTool(String),

    /// JSON deserialization failed for known tool
    JsonError {
        tool: String,
        source: serde_json::Error,
    },

    /// MCP response structure invalid (missing content[0] or content[1])
    InvalidMcpResponse {
        reason: String,
    },

    /// content[0] or content[1] was not text content
    InvalidContentType {
        index: usize,
    },
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTool(tool) => write!(f, "Unknown tool: {}", tool),
            Self::JsonError { tool, source } => {
                write!(f, "JSON deserialization failed for tool '{}': {}", tool, source)
            }
            Self::InvalidMcpResponse { reason } => {
                write!(f, "Invalid MCP response structure: {}", reason)
            }
            Self::InvalidContentType { index } => {
                write!(f, "content[{}] is not text content", index)
            }
        }
    }
}

impl std::error::Error for DeserializeError {}
