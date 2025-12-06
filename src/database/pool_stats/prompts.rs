//! Basic prompt message for db_pool_stats tool
//!
//! Provides single scenario covering core usage patterns,
//! pool health assessment, and troubleshooting basics.

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetPoolStatsPromptArgs;

/// Prompt provider for db_pool_stats tool
///
/// This is the ONLY way to provide prompts for db_pool_stats - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct PoolStatsPrompts;

impl PromptProvider for PoolStatsPrompts {
    type PromptArgs = GetPoolStatsPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE DB POOL STATS
// ============================================================================

/// Basic pool status check
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I check database connection pool statistics?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use db_pool_stats to monitor real-time connection pool health.\n\n\
                 BASIC USAGE:\n\
                 db_pool_stats({\"connection\": \"main\"})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"pool_size\": 10,\n\
                   \"active_connections\": 3,\n\
                   \"idle_connections\": 7,\n\
                   \"waiting_requests\": 0,\n\
                   \"max_lifetime_ms\": 1800000,\n\
                   \"idle_timeout_ms\": 600000\n\
                 }\n\n\
                 KEY METRICS:\n\
                 - pool_size: Maximum connections (10 connections total)\n\
                 - active_connections: Currently executing queries (3 active)\n\
                 - idle_connections: Ready for immediate use (7 idle)\n\
                 - waiting_requests: Queries waiting for connection (0 waiting)\n\
                 - max_lifetime_ms: Max age before connection refresh (30 minutes)\n\
                 - idle_timeout_ms: How long idle connections stay alive (10 minutes)\n\n\
                 QUICK REFERENCE - Pool Health:\n\
                 HEALTHY: idle > 0, waiting = 0, utilization < 80%\n\
                 BUSY: idle = 1-2, waiting = 0, needs monitoring\n\
                 EXHAUSTED: idle = 0, waiting > 0, increase pool_size immediately\n\n\
                 COMMON PATTERNS:\n\
                 // Before heavy operation - verify idle connections exist\n\
                 db_pool_stats({\"connection\": \"main\"})  // Check idle_connections > 0\n\n\
                 // Monitor during load - watch for waiting requests\n\
                 db_pool_stats({\"connection\": \"main\"})  // Check waiting_requests = 0\n\n\
                 // Diagnose slow queries - check active connection count\n\
                 db_pool_stats({\"connection\": \"main\"})  // Sustained high active = slow queries\n\n\
                 PARAMETERS:\n\
                 - connection: Named connection to check (required)\n\
                   Examples: \"main\", \"replica\", \"analytics\"\n\n\
                 WHEN TO USE:\n\
                 - Regular health checks\n\
                 - Before/after major operations\n\
                 - Investigating performance issues\n\
                 - Capacity planning",
            ),
        },
    ]
}



