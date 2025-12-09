//! Schema types for config_get tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use kodegen_config::{CATEGORY_CONFIG, CONFIG_GET};
use chrono::{DateTime, Utc};
pub use rmcp::model::Implementation as ClientInfo;
use crate::{ToolArgs, tool_metadata};
use super::prompts::ConfigGetPrompts;

// ============================================================================
// CONFIG_GET TOOL
// ============================================================================

/// Arguments for `config_get` tool
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetConfigArgs {}

/// Output from `config_get` tool with fully typed configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConfigGetOutput {
    /// Whether the operation succeeded
    pub success: bool,
    /// Complete server configuration (fully typed)
    ///
    /// NOTE: This uses an opaque serde_json::Value to avoid circular dependency
    /// with kodegen_config_manager. The actual type is kodegen_config_manager::ServerConfig.
    pub config: serde_json::Value,
}

// ============================================================================
// SUPPORTING TYPES (For documentation and re-export only)
// ============================================================================
// These types are defined here for schema documentation purposes,
// but the actual runtime types come from kodegen_config_manager

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

/// Client connection record with timestamp tracking
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ClientRecord {
    pub client_info: ClientInfo,
    pub connected_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

// ============================================================================
// TOOL ARGS TRAIT IMPL
// ============================================================================

#[tool_metadata(
    description = "Get complete server configuration including security settings (blocked commands, allowed directories), shell preferences, resource limits, and live statistics"
)]
impl ToolArgs for GetConfigArgs {
    type Output = ConfigGetOutput;
    type Prompts = ConfigGetPrompts;

    const NAME: &'static str = CONFIG_GET;
    const CATEGORY: &'static kodegen_config::Category = CATEGORY_CONFIG;
    const DESCRIPTION: &'static str = "Get complete server configuration including security settings (blocked commands, allowed directories), shell preferences, resource limits, and live statistics";
}
