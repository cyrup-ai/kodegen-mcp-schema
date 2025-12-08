use serde::{Deserialize, Serialize};

/// Single tool call record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRecord {
    /// ISO 8601 timestamp (UTC)
    pub timestamp: String,

    /// Name of the tool that was called
    pub tool_name: String,

    /// Arguments passed to the tool (serialized JSON string)
    pub args_json: String,

    /// Output returned by the tool (serialized JSON string)
    pub output_json: String,

    /// Execution duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}
