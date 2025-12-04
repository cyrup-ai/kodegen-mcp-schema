//! Config tool argument schemas

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
pub use rmcp::model::Implementation as ClientInfo;

use crate::ToolArgs;

// ============================================================================
// CANONICAL TOOL NAMES
// ============================================================================

/// Canonical name for the config_get tool
pub const CONFIG_GET: &str = "config_get";

/// Canonical name for the config_set tool
pub const CONFIG_SET: &str = "config_set";

// ============================================================================
// CONFIG VALUE TYPE
// ============================================================================

/// Configuration value that can be string, number, boolean, or array of strings
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ConfigValue {
    String(String),
    Number(i64),
    Boolean(bool),
    Array(Vec<String>),
}

impl ConfigValue {
    /// Convert to string value
    pub fn into_string(self) -> Result<String, String> {
        match self {
            ConfigValue::String(s) => Ok(s),
            _ => Err("Expected string".to_string()),
        }
    }

    /// Convert to number value
    pub fn into_number(self) -> Result<i64, String> {
        match self {
            ConfigValue::Number(n) => Ok(n),
            _ => Err("Expected number".to_string()),
        }
    }

    /// Convert to boolean value
    pub fn into_boolean(self) -> Result<bool, String> {
        match self {
            ConfigValue::Boolean(b) => Ok(b),
            _ => Err("Expected boolean".to_string()),
        }
    }

    /// Convert to array value
    pub fn into_array(self) -> Result<Vec<String>, String> {
        match self {
            ConfigValue::Array(arr) => Ok(arr),
            ConfigValue::String(s) if s.starts_with('[') => {
                serde_json::from_str(&s).map_err(|_| "Invalid array format".to_string())
            }
            _ => Err("Expected array".to_string()),
        }
    }
}

// ============================================================================
// DEFAULT VALUE FUNCTIONS
// ============================================================================

// Default value functions (required for serde default attributes)
fn default_fuzzy_search_threshold() -> f64 {
    0.7
}

fn default_http_connection_timeout_secs() -> u64 {
    5
}

fn default_path_validation_timeout_ms() -> u64 {
    30_000
}

// ============================================================================
// TYPED CONFIGURATION STRUCTURES
// ============================================================================

/// Complete server configuration with security settings, resource limits, and diagnostics
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServerConfig {
    /// Commands that cannot be executed
    pub blocked_commands: Vec<String>,

    /// Default shell for command execution
    pub default_shell: String,

    /// Directories the server can access (empty = full access)
    pub allowed_directories: Vec<String>,

    /// Directories the server cannot access
    pub denied_directories: Vec<String>,

    /// Max lines for file read operations
    pub file_read_line_limit: usize,

    /// Max lines per file write operation
    pub file_write_line_limit: usize,

    /// Minimum similarity ratio (0.0-1.0) for fuzzy search suggestions
    #[serde(default = "default_fuzzy_search_threshold")]
    pub fuzzy_search_threshold: f64,

    /// HTTP connection timeout in seconds
    #[serde(default = "default_http_connection_timeout_secs")]
    pub http_connection_timeout_secs: u64,

    /// Path validation timeout in milliseconds (for slow network filesystems)
    #[serde(default = "default_path_validation_timeout_ms")]
    pub path_validation_timeout_ms: u64,

    /// Currently connected client (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_client: Option<ClientInfo>,

    /// History of all clients that have connected
    #[serde(default)]
    pub client_history: Vec<ClientRecord>,

    /// System diagnostic information (refreshed on every get_config call)
    pub system_info: SystemInfo,

    /// Total config save failures since server start
    #[serde(default)]
    pub save_error_count: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            blocked_commands: vec![
                "rm".to_string(),
                "rmdir".to_string(),
                "del".to_string(),
                "format".to_string(),
                "dd".to_string(),
                "shred".to_string(),
                "sudo".to_string(),
                "su".to_string(),
                "passwd".to_string(),
                "useradd".to_string(),
                "userdel".to_string(),
                "chmod".to_string(),
                "chown".to_string(),
                "shutdown".to_string(),
                "reboot".to_string(),
                "halt".to_string(),
                "poweroff".to_string(),
            ],
            default_shell: if cfg!(windows) {
                "powershell.exe".to_string()
            } else {
                "/bin/sh".to_string()
            },
            allowed_directories: Vec::new(),
            denied_directories: Vec::new(),
            file_read_line_limit: 1000,
            file_write_line_limit: 50,
            fuzzy_search_threshold: 0.7,
            http_connection_timeout_secs: 5,
            path_validation_timeout_ms: 30_000,
            current_client: None,
            client_history: Vec::new(),
            system_info: SystemInfo::default(),
            save_error_count: 0,
        }
    }
}

/// System diagnostic information collected at runtime
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SystemInfo {
    /// Operating system family ("macos", "linux", "windows", etc.)
    pub platform: String,

    /// CPU architecture ("x86_64", "aarch64", "arm", etc.)
    pub arch: String,

    /// OS version string (e.g., "macOS 14.6", "Ubuntu 22.04")
    pub os_version: String,

    /// Kernel version (e.g., "23.6.0" for macOS, "6.5.0-1" for Linux)
    pub kernel_version: String,

    /// Machine hostname
    pub hostname: String,

    /// Kodegen server version from Cargo.toml
    pub rust_version: String,

    /// Number of logical CPU cores
    pub cpu_count: usize,

    /// Memory information
    pub memory: MemoryInfo,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            platform: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            os_version: "unknown".to_string(),
            kernel_version: "unknown".to_string(),
            hostname: "unknown".to_string(),
            rust_version: "unknown".to_string(),
            cpu_count: 0,
            memory: MemoryInfo::default(),
        }
    }
}

/// Memory usage information (values formatted as strings with units)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemoryInfo {
    /// Total memory (formatted with units, e.g., "16384 MB")
    pub total_mb: String,

    /// Available memory (formatted with units)
    pub available_mb: String,

    /// Used memory (formatted with units)
    pub used_mb: String,
}

impl Default for MemoryInfo {
    fn default() -> Self {
        Self {
            total_mb: "0 MB".to_string(),
            available_mb: "0 MB".to_string(),
            used_mb: "0 MB".to_string(),
        }
    }
}

/// Client connection record with timestamp tracking
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClientRecord {
    pub client_info: ClientInfo,
    pub connected_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

// ============================================================================
// GET CONFIG
// ============================================================================

/// Arguments for `get_config` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetConfigArgs {}

/// Prompt arguments for `get_config` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetConfigPromptArgs {}

// ============================================================================
// SET CONFIG VALUE
// ============================================================================

/// Arguments for `set_config_value` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SetConfigValueArgs {
    /// Configuration key to update
    pub key: String,

    /// New value (string, number, boolean, or array)
    pub value: ConfigValue,
}

/// Prompt arguments for `set_config_value` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SetConfigValuePromptArgs {}

// ============================================================================
// OUTPUT TYPES
// ============================================================================

/// Output from `config_get` tool with fully typed configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConfigGetOutput {
    /// Whether the operation succeeded
    pub success: bool,
    /// Complete server configuration (fully typed)
    pub config: ServerConfig,
}

/// Output from `config_set` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConfigSetOutput {
    /// Whether the operation succeeded
    pub success: bool,
    /// The key that was set
    pub key: String,
    /// Human-readable result message
    pub message: String,
}

// ============================================================================
// TOOL ARGS IMPLEMENTATION (Args→Output Binding)
// ============================================================================

impl ToolArgs for GetConfigArgs {
    type Output = ConfigGetOutput;
}

impl ToolArgs for SetConfigValueArgs {
    type Output = ConfigSetOutput;
}
