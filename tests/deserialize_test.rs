//! Integration tests for MCP tool output deserialization

use kodegen_mcp_schema::{deserialize_tool_output, AnyToolOutput};
use std::fs;

#[test]
fn test_deserialize_terminal_output() {
    // Read the full MCP response from a JSON file
    let mcp_response_json = fs::read_to_string("tests/fixtures/terminal_output.json")
        .expect("Failed to read terminal_output.json fixture");

    // Deserialize using the tool name and full MCP response
    let result = deserialize_tool_output("terminal", &mcp_response_json)
        .expect("Failed to deserialize terminal output");

    // Verify display text was extracted correctly
    assert!(result.display.contains("Terminal 0"));
    assert!(result.display.contains("Exit code: 0"));
    assert!(result.display.contains("Duration: 1523ms"));

    // Verify typed output was deserialized correctly
    match result.typed {
        AnyToolOutput::Terminal(output) => {
            assert_eq!(output.terminal, Some(0));
            assert_eq!(output.exit_code, Some(0));
            assert_eq!(output.cwd, "/home/user/project");
            assert_eq!(output.duration_ms, 1523);
            assert!(output.completed);
        }
        _ => panic!("Expected Terminal variant, got {:?}", result.typed),
    }
}

#[test]
fn test_deserialize_terminal_output_with_error() {
    // Test with a command that failed
    let mcp_response_json = r#"[
        {
            "type": "text",
            "text": "📟 Terminal 1: Command failed\n⏱️  Duration: 250ms\n📁 Working directory: /tmp\n❌ Exit code: 127"
        },
        {
            "type": "text",
            "text": "{\"terminal\":1,\"exit_code\":127,\"cwd\":\"/tmp\",\"duration_ms\":250,\"completed\":true}"
        }
    ]"#;

    let result = deserialize_tool_output("terminal", mcp_response_json)
        .expect("Failed to deserialize terminal output");

    // Verify error case
    match result.typed {
        AnyToolOutput::Terminal(output) => {
            assert_eq!(output.terminal, Some(1));
            assert_eq!(output.exit_code, Some(127)); // Command not found
            assert_eq!(output.cwd, "/tmp");
            assert_eq!(output.duration_ms, 250);
            assert!(output.completed);
        }
        _ => panic!("Expected Terminal variant"),
    }
}

#[test]
fn test_deserialize_terminal_output_still_running() {
    // Test with a command still running (no exit code)
    let mcp_response_json = r#"[
        {
            "type": "text",
            "text": "📟 Terminal 0: Command still running\n⏱️  Duration: 5000ms (timeout)\n📁 Working directory: /home/user"
        },
        {
            "type": "text",
            "text": "{\"terminal\":0,\"cwd\":\"/home/user\",\"duration_ms\":5000,\"completed\":false}"
        }
    ]"#;

    let result = deserialize_tool_output("terminal", mcp_response_json)
        .expect("Failed to deserialize terminal output");

    match result.typed {
        AnyToolOutput::Terminal(output) => {
            assert_eq!(output.terminal, Some(0));
            assert_eq!(output.exit_code, None); // Still running
            assert_eq!(output.cwd, "/home/user");
            assert_eq!(output.duration_ms, 5000);
            assert!(!output.completed);
        }
        _ => panic!("Expected Terminal variant"),
    }
}

#[test]
fn test_deserialize_invalid_tool_name() {
    let mcp_response_json = r#"[
        {"type": "text", "text": "Some output"},
        {"type": "text", "text": "{}"}
    ]"#;

    let result = deserialize_tool_output("nonexistent_tool", mcp_response_json);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(format!("{}", err).contains("Unknown tool"));
}

#[test]
fn test_deserialize_malformed_json() {
    let mcp_response_json = r#"[
        {"type": "text", "text": "Display text"},
        {"type": "text", "text": "not valid json{{{"}
    ]"#;

    let result = deserialize_tool_output("terminal", mcp_response_json);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(format!("{}", err).contains("JSON deserialization failed"));
}

#[test]
fn test_deserialize_missing_content() {
    // Missing content[1]
    let mcp_response_json = r#"[
        {"type": "text", "text": "Display text only"}
    ]"#;

    let result = deserialize_tool_output("terminal", mcp_response_json);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(format!("{}", err).contains("Missing content[1]"));
}
