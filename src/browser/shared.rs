//! Shared types for browser tools

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Re-export tool name constants from kodegen_config
pub use kodegen_config::{BROWSER_AGENT_KILL, BROWSER_EVAL};

// ============================================================================
// WAIT CONDITION
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WaitCondition {
    /// Element exists in DOM (uses page.find_element)
    Present,

    /// Element is visible (not display:none, visibility:hidden, etc)
    Visible,

    /// Element is visible AND not disabled (ready for interaction)
    Clickable,

    /// Element text contains specific string
    TextContains,

    /// Element attribute has specific value
    AttributeIs,
}

// ============================================================================
// WAIT FOR ARGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserWaitForArgs {
    /// CSS selector for element to wait for
    pub selector: String,

    /// Condition to wait for
    pub condition: WaitCondition,

    /// Optional: timeout in milliseconds (default: 10000)
    #[serde(default)]
    pub timeout_ms: Option<u64>,

    /// Optional: expected text (required for TextContains condition)
    #[serde(default)]
    pub text: Option<String>,

    /// Optional: attribute name (required for AttributeIs condition)
    #[serde(default)]
    pub attribute_name: Option<String>,

    /// Optional: attribute value (required for AttributeIs condition)
    #[serde(default)]
    pub attribute_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserWaitForPromptArgs {}

// ============================================================================
// WAIT ARGS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserWaitArgs {
    /// Duration to wait in milliseconds
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserWaitPromptArgs {}

// ============================================================================
// JS VALUE
// ============================================================================

/// Typed JavaScript value - covers all ECMAScript primitive and complex types
/// 
/// JavaScript has 7 primitive types (undefined, null, boolean, number, string, bigint, symbol)
/// plus complex types (Object, Array, Function). This enum provides type-safe representation
/// for all possible JavaScript values that can be returned from browser_eval.
///
/// Reference: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Data_structures
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum JsValue {
    /// JavaScript `undefined` value
    /// Serializes as: {"type": "Undefined"}
    Undefined,
    
    /// JavaScript `null` value
    /// Serializes as: {"type": "Null"}
    Null,
    
    /// JavaScript boolean: `true` or `false`
    /// Serializes as: {"type": "Boolean", "value": true}
    Boolean(bool),
    
    /// JavaScript number (IEEE 754 double-precision floating-point)
    /// Includes: integers, floats, NaN, Infinity, -Infinity
    /// Serializes as: {"type": "Number", "value": 42.5}
    Number(f64),
    
    /// JavaScript string
    /// Serializes as: {"type": "String", "value": "hello"}
    String(String),
    
    /// JavaScript BigInt (arbitrary precision integers)
    /// Stored as string because BigInt can exceed i64/u64 range
    /// Example: 9007199254740991n (Number.MAX_SAFE_INTEGER + 1)
    /// Serializes as: {"type": "BigInt", "value": "9007199254740992"}
    BigInt(String),
    
    /// JavaScript Symbol
    /// Symbols are unique identifiers, represented as their string description
    /// Example: Symbol('mySymbol') -> "Symbol(mySymbol)"
    /// Serializes as: {"type": "Symbol", "value": "Symbol(mySymbol)"}
    Symbol(String),
    
    /// JavaScript Object (excluding Array and Function)
    /// Complex objects are JSON-serialized to preserve structure
    /// Example: {a: 1, b: 2} -> "{\"a\":1,\"b\":2}"
    /// Serializes as: {"type": "Object", "value": "{\"a\":1,\"b\":2}"}
    Object(String),
    
    /// JavaScript Array
    /// Arrays are JSON-serialized to preserve structure
    /// Example: [1, 2, 3] -> "[1,2,3]"
    /// Serializes as: {"type": "Array", "value": "[1,2,3]"}
    Array(String),
    
    /// JavaScript Function
    /// Functions are represented as their string representation
    /// Example: function foo() {} -> "[Function: foo]"
    /// Serializes as: {"type": "Function", "value": "[Function: foo]"}
    Function(String),
}

// ============================================================================
// BROWSER EVAL OUTPUT
// ============================================================================

/// Output from `browser_eval` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrowserEvalOutput {
    pub success: bool,
    pub result: JsValue,
}
