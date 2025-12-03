//! MCP tool output deserialization
//!
//! Provides unified API for deserializing full MCP tool responses into
//! typed output structures with display text.

mod error;
mod extract;
mod registry;
mod types;

pub use error::DeserializeError;
pub use types::{AnyToolOutput, ToolOutputResult};

/// Deserialize a full MCP tool response JSON.
///
/// Takes the complete MCP response JSON string (entire Vec<Content>) and
/// the tool name, returning both display text and typed output together.
///
/// # Arguments
/// * `tool_name` - The canonical tool name (e.g., "fs_read_file", "git_add")
/// * `mcp_response_json` - Full MCP response JSON string containing content array
///
/// # Returns
/// * `Ok(ToolOutputResult)` - Successfully deserialized with display + typed output
/// * `Err(DeserializeError)` - Parsing or deserialization failed
///
/// # Example
/// ```rust
/// use kodegen_mcp_schema::deserialize::deserialize_tool_output;
///
/// let mcp_json = r#"[
///     {"type":"text","text":"📄 Read file: src/main.rs\n 📊 Content: 500 lines"},
///     {"type":"text","text":"{\"success\":true,\"path\":\"src/main.rs\",\"content\":\"...\"}"}
/// ]"#;
///
/// let result = deserialize_tool_output("fs_read_file", mcp_json)?;
///
/// println!("Display: {}", result.display);
/// match result.typed {
///     AnyToolOutput::FsReadFile(output) => {
///         println!("Path: {}", output.path);
///         println!("Lines: {:?}", output.total_lines);
///     }
///     _ => unreachable!(),
/// }
/// ```
pub fn deserialize_tool_output(
    tool_name: &str,
    mcp_response_json: &str,
) -> Result<ToolOutputResult, DeserializeError> {
    // Parse full MCP response JSON into content array
    let content = extract::parse_mcp_response(mcp_response_json)?;

    // Extract display text from content[0]
    let display = extract::extract_display(&content)?;

    // Extract typed JSON from content[1]
    let typed_json = extract::extract_typed_json(&content)?;

    // Deserialize typed output based on tool name
    let typed = registry::deserialize_by_tool_name(tool_name, &typed_json)?;

    Ok(ToolOutputResult { display, typed })
}

/// Deserialize typed output from raw JSON (content[1] only).
///
/// Legacy API for backward compatibility. Prefer `deserialize_tool_output`
/// when you have the full MCP response.
///
/// # Arguments
/// * `tool_name` - The canonical tool name
/// * `json_str` - Raw JSON string from content[1]
///
/// # Returns
/// * `Ok(AnyToolOutput)` - Successfully deserialized typed output
/// * `Err(DeserializeError)` - Deserialization failed
pub fn deserialize_typed_only(
    tool_name: &str,
    json_str: &str,
) -> Result<AnyToolOutput, DeserializeError> {
    registry::deserialize_by_tool_name(tool_name, json_str)
}
