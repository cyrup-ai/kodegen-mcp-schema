use rmcp::handler::server::tool::schema_for_type;
use rmcp::model::{CallToolResult, Content, Meta, PromptArgument, PromptMessage};
use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

// Import log for schema validation logging
use log;

use super::error::McpError;

// Re-export ToolArgs from parent crate
pub use crate::ToolArgs;

// ============================================================================
// SEALED PROMPT PROVIDER TRAIT
// ============================================================================

/// Sealed module - ONLY kodegen-mcp-schema can implement Sealed trait
mod sealed {
    /// Sealed trait that prevents external crates from implementing PromptProvider
    pub trait Sealed {}
}

/// Trait that ONLY kodegen-mcp-schema can implement.
///
/// This trait provides prompt generation for tools. Tools CANNOT implement this
/// trait directly - they must reference a PromptProvider implementation from
/// the schema package.
///
/// This enforces architectural constraint: ALL prompt logic MUST be centralized
/// in kodegen-mcp-schema, tools cannot implement prompts inline.
pub trait PromptProvider: sealed::Sealed + Send + Sync + 'static {
    /// Prompt arguments type - what customization does the prompt accept?
    type PromptArgs: DeserializeOwned + JsonSchema + Send + 'static;

    /// Generate prompt messages - ONLY schema can implement.
    ///
    /// Returns a conversation showing agents how/when to use the tool.
    /// Should include examples, common patterns, gotchas, requirements.
    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage>;

    /// What arguments does the teaching prompt accept?
    ///
    /// These let agents customize what they want to learn about.
    /// Example: "repo" (which repo?), "shallow" (learn about shallow clones?)
    fn prompt_arguments() -> Vec<PromptArgument>;
}

/// Re-export sealed trait for schema package to implement
///
/// Only kodegen-mcp-schema can `impl SealedPromptProvider for MyPrompts {}`.
/// Tool packages cannot implement this - it's sealed to enforce centralization.
pub use sealed::Sealed as SealedPromptProvider;

// ============================================================================
// BRANDED DISPLAY LINE
// ============================================================================

/// Tool execution status for branded line coloring
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolStatus {
    /// Tool executed successfully
    Success,
    /// Tool execution failed
    Error,
}

/// Add branded display line to CallToolResult as new Content[0].
///
/// Inserts a branded line at the beginning of the content vector:
/// - New Content[0]: Branded line (ⓚ icon tool_name duration)
/// - Content[1]: Original Content[0] (display)
/// - Content[2]: Original Content[1] (metadata)
/// - etc.
///
/// # Arguments
/// - `call_result`: The CallToolResult to modify (Success case)
/// - `tool_name`: Name of the tool being executed
/// - `icon`: Tool icon character
/// - `duration_ms`: Execution duration in milliseconds
/// - `status`: Success or Error status for coloring
pub fn add_branded_line_to_result(
    call_result: &mut CallToolResult,
    tool_name: &str,
    icon: char,
    duration_ms: u64,
    status: ToolStatus,
) {
    // Calculate duration in seconds (ceil to >= 1s)
    let duration_s = ((duration_ms as f64 / 1000.0).ceil() as u64).max(1);

    // Select timing color based on status
    // Success: green (35 #00af5f)
    // Error: red (204 #ff5f87)
    let timing_color = match status {
        ToolStatus::Success => 35,
        ToolStatus::Error => 204,
    };

    // Format branded line with ANSI colors
    // ⓚ brand symbol: color 132 (#af5f87)
    // icon + tool_name: color 32 (#0087d7)
    // duration: status-based color
    let branded_line = format!(
        "\x1b[38;5;132mⓚ\x1b[0m   \x1b[38;5;32m{} {}\x1b[0m   \x1b[38;5;{}m{}s\x1b[0m",
        icon, tool_name, timing_color, duration_s
    );

    // Insert branded content at position 0, shifting all existing content
    let branded_content = Content::text(branded_line);
    call_result.content.insert(0, branded_content);
}

#[derive(Debug, Clone)]
pub struct ToolResponse<M> {
    /// Human-readable display output - goes to Content[0].
    ///
    /// This is the PRIMARY output that humans read:
    /// - Terminal: the full output of the last command (stdout/stderr)
    /// - File read: the file contents
    /// - Search: formatted human-readable results
    /// - Database query: formatted table output
    ///
    /// Always present - every tool has human-readable output.
    /// Use empty string for tools with no display output.
    pub display: String,

    /// Typed, schema-enforced metadata - goes to Content[1].
    ///
    /// This is SEPARATE from display - NO DUPLICATION.
    /// Contains only structured data (exit_code, duration_ms, etc.)
    /// NOT the display content.
    pub metadata: M,
}

impl<M> ToolResponse<M> {
    /// Create response with both display and metadata.
    ///
    /// # Arguments
    /// - `display`: Human-readable output (full command output, file contents, etc.)
    /// - `metadata`: Typed output struct (derived from Args::Output)
    #[inline]
    pub fn new(display: impl Into<String>, metadata: M) -> Self {
        Self {
            display: display.into(),
            metadata,
        }
    }

    /// Create response with empty display.
    ///
    /// Use when there's no human-readable output but metadata is present.
    #[inline]
    pub fn empty_display(metadata: M) -> Self {
        Self {
            display: String::new(),
            metadata,
        }
    }
}

impl<M: Serialize> ToolResponse<M> {
    /// Convert to CallToolResult for MCP response.
    ///
    /// # Content Layout
    /// - `content[0]`: Human-readable display (always present, may be empty)
    /// - `content[1]`: Typed metadata as pretty-printed JSON
    ///
    /// Both display and metadata are in the content Vec - no structured_content.
    pub fn into_call_tool_result(self) -> Result<CallToolResult, serde_json::Error> {
        let display_content = Content::text(self.display);
        let json = serde_json::to_string_pretty(&self.metadata)?;
        let metadata_content = Content::text(json);

        Ok(CallToolResult {
            content: vec![display_content, metadata_content],
            structured_content: None,
            is_error: None,
            meta: None,
        })
    }

    /// Get metadata as JSON Value (for history recording).
    pub fn metadata_as_json(&self) -> serde_json::Value {
        serde_json::to_value(&self.metadata).unwrap_or_else(|_| serde_json::json!({}))
    }

    /// Convert to CallToolResult with branded display line.
    ///
    /// Creates a branded line as a separate Content[0], shifting display and metadata:
    /// - `content[0]`: Branded line (ⓚ icon tool_name duration)
    /// - `content[1]`: Human-readable display (was content[0])
    /// - `content[2]`: Typed metadata as pretty-printed JSON (was content[1])
    ///
    /// # Arguments
    /// - `tool_name`: Name of the tool being executed
    /// - `icon`: Tool icon character (from metadata or fallback)
    /// - `duration_ms`: Execution duration in milliseconds
    /// - `status`: Success or Error status for coloring
    pub fn into_call_tool_result_with_branding(
        self,
        tool_name: &str,
        icon: char,
        duration_ms: u64,
        status: ToolStatus,
    ) -> Result<CallToolResult, serde_json::Error> {
        // Calculate duration in seconds (ceil to >= 1s)
        let duration_s = ((duration_ms as f64 / 1000.0).ceil() as u64).max(1);

        // Select timing color based on status
        // Success: green (35 #00af5f)
        // Error: red (204 #ff5f87)
        let timing_color = match status {
            ToolStatus::Success => 35,
            ToolStatus::Error => 204,
        };

        // Format branded line with ANSI colors
        // ⓚ brand symbol: color 132 (#af5f87)
        // icon + tool_name: color 32 (#0087d7)
        // duration: status-based color
        let branded_line = format!(
            "\x1b[38;5;132mⓚ\x1b[0m   \x1b[38;5;32m{} {}\x1b[0m   \x1b[38;5;{}m{}s\x1b[0m",
            icon, tool_name, timing_color, duration_s
        );

        // Create content vector with branded line first
        let branded_content = Content::text(branded_line);
        let display_content = Content::text(self.display);
        let json = serde_json::to_string_pretty(&self.metadata)?;
        let metadata_content = Content::text(json);

        Ok(CallToolResult {
            content: vec![branded_content, display_content, metadata_content],
            structured_content: None,
            is_error: None,
            meta: None,
        })
    }
}

// ============================================================================
// PERFORMANCE OPTIMIZATIONS
// ============================================================================

/// Type alias for the schema cache to reduce complexity
type SchemaCache =
    parking_lot::RwLock<HashMap<&'static str, std::sync::Arc<serde_json::Map<String, Value>>>>;

/// Schema cache to avoid repeated serialization
static SCHEMA_CACHE: std::sync::LazyLock<SchemaCache> =
    std::sync::LazyLock::new(|| parking_lot::RwLock::new(HashMap::new()));

// ============================================================================
// CORE TRAIT
// ============================================================================

/// Core trait that all tools must implement
///
/// Tools are STRUCTS that hold their own dependencies (`GitClient`, `GitHubClient`, etc.)
/// The trait is generic and knows nothing about specific services.
/// Every method (except execute/prompt) has a sensible default.
pub trait Tool: Send + Sync + Sized + 'static {
    /// Tool execution arguments - ALSO determines output type via ToolArgs::Output.
    ///
    /// The output type is DERIVED from Args - tools cannot choose wrong output type.
    /// This binding is defined in `kodegen-mcp-schema` and enforced at compile time.
    type Args: ToolArgs;

    /// Prompt provider (MUST be from schema - tools cannot implement PromptProvider).
    ///
    /// Tools declare which PromptProvider they use, but cannot implement it themselves.
    /// PromptProvider is sealed - only kodegen-mcp-schema can implement it.
    /// This enforces that all prompt logic is centralized in the schema package.
    type Prompts: PromptProvider;

    // ========================================================================
    // IDENTITY (Required)
    // ========================================================================

    /// Unique tool name (e.g., "`git_clone`")
    fn name() -> &'static str;

    /// Human-readable description of what this tool does
    fn description() -> &'static str;

    // ========================================================================
    // SCHEMA (Auto-generated with caching)
    // ========================================================================

    /// Validate that schema generation works for this tool's Args type.
    /// 
    /// This method attempts to generate the JSON schema and catches any panics
    /// that occur during schema generation. Should be called during tool 
    /// registration to catch schema issues early.
    ///
    /// # Returns
    /// - `Ok(())` if schema generation succeeds
    /// - `Err(String)` with detailed error message if schema generation fails
    fn validate_schema() -> Result<(), String> {
        // Wrap schema_for_type in a panic catch
        let result = std::panic::catch_unwind(|| {
            let _ = schema_for_type::<Self::Args>();
        });
        
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                let error_msg = if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown panic during schema generation".to_string()
                };
                Err(format!(
                    "Schema generation failed for tool '{}': {}",
                    Self::name(),
                    error_msg
                ))
            }
        }
    }

    /// Input schema - AUTO-GENERATED from Args type via `JsonSchema` derive
    /// Cached for performance - schema is computed once and reused
    #[inline]
    fn input_schema() -> std::sync::Arc<serde_json::Map<String, Value>> {
        let name = Self::name();

        // Fast path: read from cache
        if let Some(schema) = SCHEMA_CACHE.read().get(name) {
            return schema.clone();
        }

        // Slow path: generate and cache
        // Log schema generation attempt
        log::debug!("Generating schema for tool: {}", name);
        
        // Validate schema generation with panic catching
        if let Err(e) = Self::validate_schema() {
            log::error!("{}", e);
            // For now, still proceed but with warning - could be made fatal in future
            log::warn!("Tool '{}' registered with potentially invalid schema", name);
        } else {
            log::info!("✓ Schema generated successfully for tool: {}", name);
        }
        
        let schema = std::sync::Arc::new(schema_for_type::<Self::Args>());
        SCHEMA_CACHE.write().insert(name, schema.clone());
        schema
    }

    /// Output schema - AUTO-GENERATED from `<Args as ToolArgs>::Output`.
    ///
    /// Unlike input_schema which is optional to override, output_schema is
    /// always derived from the Args→Output mapping in the schema package.
    /// This ensures compile-time enforcement of correct output types.
    #[must_use]
    #[inline]
    fn output_schema() -> std::sync::Arc<serde_json::Map<String, Value>> {
        // Use a separate cache namespace for output schemas
        static OUTPUT_SCHEMA_CACHE: std::sync::LazyLock<SchemaCache> =
            std::sync::LazyLock::new(|| parking_lot::RwLock::new(HashMap::new()));

        let name = Self::name();
        let cache_key = Box::leak(format!("{}_output", name).into_boxed_str());

        // Fast path: read from cache
        if let Some(schema) = OUTPUT_SCHEMA_CACHE.read().get(cache_key) {
            return schema.clone();
        }

        // Slow path: generate and cache from Args::Output
        let schema = std::sync::Arc::new(
            schema_for_type::<<Self::Args as ToolArgs>::Output>()
        );
        OUTPUT_SCHEMA_CACHE.write().insert(cache_key, schema.clone());
        schema
    }

    // ========================================================================
    // BEHAVIOR ANNOTATIONS (Tool IS its behavior)
    // ========================================================================

    /// Does this tool only read (never modify) state?
    ///
    /// true = read-only (safe, can't break things)
    /// false = writes/modifies state (requires caution)
    ///
    /// Default: true (assumes read-only by default - safe default)
    #[must_use]
    #[inline]
    fn read_only() -> bool {
        true
    }

    /// Can this tool delete or overwrite existing data?
    ///
    /// Only meaningful when `read_only` = false.
    /// true = can delete/overwrite (dangerous)
    /// false = only adds/creates (safer)
    ///
    /// Default: false (assumes non-destructive by default - safe default)
    #[must_use]
    #[inline]
    fn destructive() -> bool {
        false
    }

    /// Is calling this tool repeatedly with same args safe/idempotent?
    ///
    /// Only meaningful when `read_only` = false.
    /// true = safe to retry (same result every time)
    /// false = each call has different effect
    ///
    /// Default: true (assumes idempotent by default - safe default)
    #[must_use]
    #[inline]
    fn idempotent() -> bool {
        true
    }

    /// Does this tool interact with external systems (network, filesystem outside repo)?
    ///
    /// true = open world (network calls, external APIs, can fail due to external factors)
    /// false = closed world (only local operations, deterministic)
    ///
    /// Default: false (assumes local operations by default - safe default)
    #[must_use]
    #[inline]
    fn open_world() -> bool {
        false
    }

    // ========================================================================
    // EXECUTION (Required)
    // ========================================================================

    /// Execute the tool with given arguments.
    ///
    /// Return type is `ToolResponse<<Self::Args as ToolArgs>::Output>`.
    /// The output type is DERIVED from Args - compiler enforces correct type.
    ///
    /// # Example
    ///
    /// ```rust
    /// impl Tool for TerminalTool {
    ///     type Args = TerminalInput;
    ///     // TerminalInput::Output = TerminalOutput (defined in schema)
    ///     // So execute() MUST return ToolResponse<TerminalOutput>
    ///
    ///     async fn execute(&self, args: Self::Args, ctx: ToolExecutionContext)
    ///         -> Result<ToolResponse<TerminalOutput>, McpError>
    ///     {
    ///         Ok(ToolResponse::new(output_text, TerminalOutput { ... }))
    ///     }
    /// }
    /// ```
    fn execute(
        &self,
        args: Self::Args,
        ctx: ToolExecutionContext,
    ) -> impl std::future::Future<Output = Result<
        ToolResponse<<Self::Args as ToolArgs>::Output>,
        McpError
    >> + Send;

    // ========================================================================
    // RMCP INTEGRATION (Default implementations)
    // ========================================================================

    /// Prompt name (defaults to "{`tool_name`}_help")
    #[must_use]
    #[inline]
    fn prompt_name() -> Cow<'static, str> {
        Cow::Owned(format!("{}_help", Self::name()))
    }

    /// Prompt description (defaults to tool description)
    #[must_use]
    #[inline]
    fn prompt_description() -> &'static str {
        Self::description()
    }

    /// Convert this tool into an RMCP `ToolRoute`
    ///
    /// This default implementation builds the route from trait methods.
    /// Tools get this for free - no need to implement `IntoToolRoute` manually.
    fn into_tool_route<S>(self) -> rmcp::handler::server::router::tool::ToolRoute<S>
    where
        S: Send + Sync + 'static,
    {
        use rmcp::handler::server::router::tool::ToolRoute;
        use rmcp::model::{Tool as RmcpTool, ToolAnnotations, Meta};
        use std::sync::Arc;

        // Build annotations from trait methods
        let annotations = ToolAnnotations::new()
            .read_only(Self::read_only())
            .destructive(Self::destructive())
            .idempotent(Self::idempotent())
            .open_world(Self::open_world());

        // Store icon in meta field
        let mut meta = Meta::new();
        meta.0.insert("icon".to_string(), serde_json::json!(<Self::Args as ToolArgs>::icon().to_string()));

        // Build RMCP Tool metadata
        let metadata = RmcpTool {
            name: Self::name().into(),
            title: None,
            description: Some(Self::description().into()),
            input_schema: Self::input_schema(),
            output_schema: Some(Self::output_schema()),
            annotations: Some(annotations),
            icons: None,
            meta: Some(meta),
        };

        // Create handler with ToolHandler wrapper (HRTB-compatible, zero-cost)
        let handler = ToolHandler {
            tool: Arc::new(self),
        };

        // Use ToolRoute::new() - handles HRTB internally
        ToolRoute::new(metadata, handler)
    }

    /// Convert this tool into an RMCP `PromptRoute`
    ///
    /// This default implementation builds the route from trait methods.
    /// Tools get this for free - no need to implement `IntoPromptRoute` manually.
    fn into_prompt_route<S>(self) -> rmcp::handler::server::router::prompt::PromptRoute<S>
    where
        S: Send + Sync + 'static,
    {
        use rmcp::handler::server::router::prompt::PromptRoute;
        use rmcp::handler::server::wrapper::Parameters;
        use rmcp::model::{GetPromptResult, Prompt as RmcpPrompt};

        // Build meta following the same pattern as tools (see line 434-435)
        let mut meta = Meta::new();
        meta.0.insert("category".to_string(), serde_json::json!(Self::Args::CATEGORY.name));
        meta.0.insert("icon".to_string(), serde_json::json!(Self::Args::CATEGORY.icon.to_string()));

        // Build RMCP Prompt metadata using PromptProvider
        let metadata = RmcpPrompt {
            name: Self::prompt_name().into_owned(),
            title: None,
            description: Some(Self::prompt_description().to_string()),
            arguments: Some(<Self as ToolPrompts>::prompt_arguments()),
            icons: None,
            meta: Some(meta),
        };

        // Handler calls static PromptProvider methods (no tool instance needed)
        let handler = move |Parameters(args): Parameters<<Self::Prompts as PromptProvider>::PromptArgs>| {
            async move {
                let messages = <Self as ToolPrompts>::prompt(args)?;

                Ok(GetPromptResult {
                    description: Some(Self::prompt_description().to_string()),
                    messages,
                })
            }
        };

        PromptRoute::new(metadata, handler)
    }

    /// Convert Arc-wrapped tool into an RMCP `ToolRoute` (optimized - no extra Arc allocation)
    ///
    /// This is more efficient than `into_tool_route(self)` when the tool is already wrapped in Arc.
    /// The tool is used directly without creating an additional Arc wrapper.
    fn arc_into_tool_route<S>(self: Arc<Self>) -> rmcp::handler::server::router::tool::ToolRoute<S>
    where
        S: Send + Sync + 'static,
    {
        use rmcp::handler::server::router::tool::ToolRoute;
        use rmcp::model::{Tool as RmcpTool, ToolAnnotations, Meta};

        // Build annotations from trait methods
        let annotations = ToolAnnotations::new()
            .read_only(Self::read_only())
            .destructive(Self::destructive())
            .idempotent(Self::idempotent())
            .open_world(Self::open_world());

        // Store icon in meta field
        let mut meta = Meta::new();
        meta.0.insert("icon".to_string(), serde_json::json!(<Self::Args as ToolArgs>::icon().to_string()));

        // Build RMCP Tool metadata
        let metadata = RmcpTool {
            name: Self::name().into(),
            title: None,
            description: Some(Self::description().into()),
            input_schema: Self::input_schema(),
            output_schema: Some(Self::output_schema()),
            annotations: Some(annotations),
            icons: None,
            meta: Some(meta),
        };

        // Create handler with ToolHandler wrapper (HRTB-compatible, zero-cost)
        // Use self directly (already Arc<Self>) - no extra Arc allocation
        let handler = ToolHandler {
            tool: self,
        };

        // Use ToolRoute::new() - handles HRTB internally
        ToolRoute::new(metadata, handler)
    }

    /// Convert Arc-wrapped tool into an RMCP `PromptRoute` (optimized - no extra Arc allocation)
    ///
    /// This is more efficient than `into_prompt_route(self)` when the tool is already wrapped in Arc.
    /// The tool is used directly without creating an additional Arc wrapper.
    fn arc_into_prompt_route<S>(
        self: Arc<Self>,
    ) -> rmcp::handler::server::router::prompt::PromptRoute<S>
    where
        S: Send + Sync + 'static,
    {
        use rmcp::handler::server::router::prompt::PromptRoute;
        use rmcp::handler::server::wrapper::Parameters;
        use rmcp::model::{GetPromptResult, Prompt as RmcpPrompt};

        // Build meta following the same pattern as tools (see line 434-435)
        let mut meta = Meta::new();
        meta.0.insert("category".to_string(), serde_json::json!(Self::Args::CATEGORY.name));
        meta.0.insert("icon".to_string(), serde_json::json!(Self::Args::CATEGORY.icon.to_string()));

        // Build RMCP Prompt metadata using PromptProvider
        let metadata = RmcpPrompt {
            name: Self::prompt_name().into_owned(),
            title: None,
            description: Some(Self::prompt_description().to_string()),
            arguments: Some(<Self as ToolPrompts>::prompt_arguments()),
            icons: None,
            meta: Some(meta),
        };

        // Handler calls static PromptProvider methods (no tool instance needed)
        let handler = move |Parameters(args): Parameters<<Self::Prompts as PromptProvider>::PromptArgs>| {
            async move {
                let messages = <Self as ToolPrompts>::prompt(args)?;

                Ok(GetPromptResult {
                    description: Some(Self::prompt_description().to_string()),
                    messages,
                })
            }
        };

        PromptRoute::new(metadata, handler)
    }
}

// ============================================================================
// TOOL PROMPTS EXTENSION TRAIT
// ============================================================================

/// Extension trait providing prompt methods via PromptProvider.
///
/// This is implemented automatically for all Tool implementations via blanket impl.
/// Tools get prompt() and prompt_arguments() methods for free by specifying
/// their Prompts associated type.
///
/// The methods delegate to the PromptProvider, which only schema can implement.
pub trait ToolPrompts: Tool {
    /// Get prompt arguments (calls schema's PromptProvider)
    #[inline]
    fn prompt_arguments() -> Vec<PromptArgument> {
        <Self::Prompts as PromptProvider>::prompt_arguments()
    }

    /// Generate prompts (calls schema's PromptProvider)
    ///
    /// Takes PromptArgs from the PromptProvider, not from Self.
    /// This ensures tools cannot override prompt generation.
    #[inline]
    fn prompt(args: <Self::Prompts as PromptProvider>::PromptArgs) -> Result<Vec<PromptMessage>, McpError> {
        Ok(<Self::Prompts as PromptProvider>::generate_prompts(&args))
    }
}

/// Blanket implementation - all Tools automatically get ToolPrompts
impl<T: Tool> ToolPrompts for T {}

// ============================================================================
// PROGRESS NOTIFICATION CONTEXT
// ============================================================================

/// Execution context provided to tools for progress notifications and cancellation.
///
/// Supports three patterns:
/// 1. Stream text messages: `ctx.stream("output\n")`
/// 2. Report numeric progress: `ctx.progress(50, 100)`  
/// 3. Combined: `ctx.update(50, 100, "Processing file 50/100")`
#[derive(Clone)]
pub struct ToolExecutionContext {
    /// Peer interface for sending progress notifications
    peer: rmcp::service::Peer<rmcp::RoleServer>,

    /// Cancellation token (tool should check periodically)
    ct: tokio_util::sync::CancellationToken,

    /// Unique request identifier (used for progress_token)
    request_id: rmcp::model::RequestId,

    /// Infrastructure context from kodegen stdio server (via HTTP headers)
    /// These are None for non-HTTP transports

    /// Connection ID - identifies the stdio connection instance
    connection_id: Option<String>,

    /// Current working directory from client environment
    pwd: Option<PathBuf>,

    /// Git repository root from client environment
    git_root: Option<PathBuf>,
}

impl ToolExecutionContext {
    /// Create a new ToolExecutionContext with the given peer, cancellation token, and request ID.
    ///
    /// This constructor is public to allow custom integration contexts (e.g., bridging
    /// to non-RMCP transports or in-process sessions). Most tools should not need to
    /// call this directly - the context is typically provided by the RMCP framework.
    ///
    /// # Arguments
    /// * `peer` - RMCP peer for sending progress notifications
    /// * `ct` - Cancellation token for this execution
    /// * `request_id` - Unique identifier for this request
    #[must_use]
    pub fn new(
        peer: rmcp::service::Peer<rmcp::RoleServer>,
        ct: tokio_util::sync::CancellationToken,
        request_id: rmcp::model::RequestId,
    ) -> Self {
        Self {
            peer,
            ct,
            request_id,
            connection_id: None,
            pwd: None,
            git_root: None,
        }
    }

    /// Get connection ID from stdio server (for resource isolation)
    /// Always present for kodegen stdio connections, None for direct HTTP clients
    #[must_use]
    pub fn connection_id(&self) -> Option<&str> {
        self.connection_id.as_deref()
    }

    /// Get current working directory from client environment
    #[must_use]
    pub fn pwd(&self) -> Option<&std::path::Path> {
        self.pwd.as_deref()
    }

    /// Get git repository root from client environment
    #[must_use]
    pub fn git_root(&self) -> Option<&std::path::Path> {
        self.git_root.as_deref()
    }

    /// Get the request ID for this tool execution
    ///
    /// The request ID uniquely identifies this tool call and can be used for:
    /// - Filtering events in multi-request scenarios
    /// - Correlating logs and outputs
    /// - Tracking execution history
    #[must_use]
    pub fn request_id(&self) -> &rmcp::model::RequestId {
        &self.request_id
    }

    /// Stream a text message (for terminal output, logs, status updates).
    ///
    /// Use this for incrementally streaming text output as it becomes available.
    ///
    /// # Example
    /// ```
    /// // Terminal streaming command output
    /// ctx.stream("npm info using npm@8.19.2\n").await.ok();
    /// ctx.stream("added 234 packages in 15s\n").await.ok();
    /// ```
    pub async fn stream(&self, message: impl Into<String>) -> Result<(), McpError> {
        self.notify_internal(0.0, None, Some(message.into())).await
    }

    /// Report numeric progress (for progress bars, counters).
    ///
    /// Use this when you have a known total and current progress value.
    ///
    /// # Example
    /// ```
    /// // Processing 50 out of 100 files
    /// ctx.progress(50.0, 100.0).await.ok();
    /// ```
    pub async fn progress(&self, current: f64, total: f64) -> Result<(), McpError> {
        self.notify_internal(current, Some(total), None).await
    }

    /// Report both numeric progress and a descriptive message.
    ///
    /// Use this when you want both a progress bar AND status text.
    ///
    /// # Example
    /// ```
    /// ctx.update(50.0, 100.0, "Generating embeddings 50/100").await.ok();
    /// ```
    pub async fn update(
        &self,
        current: f64,
        total: f64,
        message: impl Into<String>
    ) -> Result<(), McpError> {
        self.notify_internal(current, Some(total), Some(message.into())).await
    }

    /// Advanced: Full control over progress notification fields.
    ///
    /// Use this when you need fine-grained control (e.g., unknown total).
    ///
    /// # Example
    /// ```
    /// // Unknown total, just report current count
    /// ctx.notify(lines_read as f64, None, Some("Reading file...")).await.ok();
    /// ```
    pub async fn notify(
        &self,
        progress: f64,
        total: Option<f64>,
        message: Option<String>
    ) -> Result<(), McpError> {
        self.notify_internal(progress, total, message).await
    }

    /// Internal implementation - sends the actual notification
    async fn notify_internal(
        &self,
        progress: f64,
        total: Option<f64>,
        message: Option<String>
    ) -> Result<(), McpError> {
        use rmcp::model::{ProgressNotificationParam, ProgressToken, NumberOrString};

        // Generate unique progress token from request ID
        let progress_token = ProgressToken(NumberOrString::String(
            format!("tool_{}", match &self.request_id {
                NumberOrString::Number(n) => n.to_string(),
                NumberOrString::String(s) => s.to_string(),
            }).into()
        ));

        let params = ProgressNotificationParam {
            progress_token,
            progress,
            total,
            message,
        };

        self.peer
            .notify_progress(params)
            .await
            .map_err(|e| McpError::Other(anyhow::anyhow!(
                "Failed to send progress notification: {}", e
            )))
    }

    /// Check if tool execution was cancelled by the client.
    ///
    /// Tools should check this periodically during long operations.
    ///
    /// # Example
    /// ```
    /// for item in items {
    ///     if ctx.is_cancelled() {
    ///         return Err(McpError::cancelled("Operation cancelled"));
    ///     }
    ///     process_item(item).await?;
    /// }
    /// ```
    pub fn is_cancelled(&self) -> bool {
        self.ct.is_cancelled()
    }

    /// Get the cancellation token for use with `tokio::select!` or custom logic.
    pub fn cancellation_token(&self) -> &tokio_util::sync::CancellationToken {
        &self.ct
    }
}

// ============================================================================
// FromContextPart implementation for ToolExecutionContext
// ============================================================================

impl<S> rmcp::handler::server::common::FromContextPart<rmcp::handler::server::tool::ToolCallContext<'_, S>>
    for ToolExecutionContext
where
    S: Send + Sync + 'static,
{
    fn from_context_part(
        context: &mut rmcp::handler::server::tool::ToolCallContext<'_, S>
    ) -> Result<Self, rmcp::ErrorData> {
        // Extract HTTP request Parts (automatically injected by rmcp)
        let parts = context.request_context.extensions.get::<http::request::Parts>();

        // Extract kodegen headers from Parts
        use kodegen_config::{X_KODEGEN_CONNECTION_ID, X_KODEGEN_PWD, X_KODEGEN_GITROOT};

        let (connection_id, pwd, git_root) = if let Some(parts) = parts {
            let conn_id = parts.headers.get(X_KODEGEN_CONNECTION_ID)
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let pwd_val = parts.headers.get(X_KODEGEN_PWD)
                .and_then(|v| v.to_str().ok())
                .map(PathBuf::from);

            let git_root_val = parts.headers.get(X_KODEGEN_GITROOT)
                .and_then(|v| v.to_str().ok())
                .map(PathBuf::from);

            (conn_id, pwd_val, git_root_val)
        } else {
            // Non-HTTP transport (direct stdio, child process)
            (None, None, None)
        };

        Ok(ToolExecutionContext {
            peer: context.request_context.peer.clone(),
            ct: context.request_context.ct.clone(),
            request_id: context.request_context.id.clone(),
            connection_id,
            pwd,
            git_root,
        })
    }
}

// ============================================================================
// ToolHandler - Zero-cost wrapper for CallToolHandler implementation
// ============================================================================

/// Wrapper struct that holds a tool and implements CallToolHandler.
/// This enables HRTB-compatible tool routing without closure lifetime issues.
struct ToolHandler<T: Tool> {
    tool: Arc<T>,
}

impl<T: Tool> Clone for ToolHandler<T> {
    fn clone(&self) -> Self {
        Self {
            tool: self.tool.clone(),
        }
    }
}

impl<T, S> rmcp::handler::server::tool::CallToolHandler<S, ()> for ToolHandler<T>
where
    T: Tool,
    S: Send + Sync + 'static,
{
    fn call(
        self,
        mut context: rmcp::handler::server::tool::ToolCallContext<'_, S>,
    ) -> futures::future::BoxFuture<'_, Result<rmcp::model::CallToolResult, rmcp::ErrorData>> {
        use rmcp::handler::server::wrapper::Parameters;
        use rmcp::handler::server::common::FromContextPart;

        Box::pin(async move {
            // Extract arguments and execution context
            let Parameters(args) = Parameters::<T::Args>::from_context_part(&mut context)?;
            let exec_ctx = ToolExecutionContext::from_context_part(&mut context)?;

            // Execute tool - returns ToolResponse<<T::Args as ToolArgs>::Output>
            let result = self.tool.execute(args, exec_ctx).await;

            match result {
                Ok(response) => {
                    // Convert ToolResponse to CallToolResult with structured validation
                    let result = response.into_call_tool_result()
                        .map_err(|e| rmcp::ErrorData::internal_error(
                            format!("Failed to serialize tool output: {}", e),
                            None
                        ))?;

                    Ok(result)
                }
                Err(e) => {
                    Err(rmcp::ErrorData::from(e))
                }
            }
        })
    }
}
