//! MCP content extraction utilities

use super::error::DeserializeError;

/// Extract display text from MCP content array.
///
/// Reads from `content[0]` which contains human-readable text.
pub fn extract_display(content: &[rmcp::model::Content]) -> Result<String, DeserializeError> {
    content
        .first()
        .ok_or_else(|| DeserializeError::InvalidMcpResponse {
            reason: "Missing content[0] (display text)".to_string(),
        })
        .and_then(|c| {
            if let rmcp::model::RawContent::Text(text_content) = &**c {
                Ok(text_content.text.clone())
            } else {
                Err(DeserializeError::InvalidContentType { index: 0 })
            }
        })
}

/// Extract typed JSON string from MCP content array.
///
/// Reads from `content[1]` which contains serialized Output struct.
pub fn extract_typed_json(content: &[rmcp::model::Content]) -> Result<String, DeserializeError> {
    content
        .get(1)
        .ok_or_else(|| DeserializeError::InvalidMcpResponse {
            reason: "Missing content[1] (typed output)".to_string(),
        })
        .and_then(|c| {
            if let rmcp::model::RawContent::Text(text_content) = &**c {
                Ok(text_content.text.clone())
            } else {
                Err(DeserializeError::InvalidContentType { index: 1 })
            }
        })
}

/// Parse MCP JSON response string into content array.
///
/// Accepts the full MCP response JSON as a string and parses it into
/// the expected Vec<rmcp::model::Content> structure.
pub fn parse_mcp_response(json_str: &str) -> Result<Vec<rmcp::model::Content>, DeserializeError> {
    serde_json::from_str(json_str).map_err(|e| DeserializeError::InvalidMcpResponse {
        reason: format!("Failed to parse MCP response JSON: {}", e),
    })
}
