//! Prompt messages for process_list tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ProcessListPromptArgs;

/// Prompt provider for process_list tool
///
/// This is the ONLY way to provide prompts for process_list - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ProcessListPrompts;

impl PromptProvider for ProcessListPrompts {
    type PromptArgs = ProcessListPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("filtering") => prompt_filtering(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, filtering)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic process listing
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list all running processes using the process_list tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The process_list tool shows all running processes with PID, command name, CPU usage, and memory consumption.\n\n\
                 LISTING PROCESSES:\n\n\
                 1. List all processes:\n\
                    process_list({})\n\
                    // Returns all running processes on the system\n\n\
                 2. Limit results by count:\n\
                    process_list({\n\
                        \"limit\": 10\n\
                    })\n\
                    // Returns only the first 10 processes\n\n\
                 3. Get top 20 processes:\n\
                    process_list({\n\
                        \"limit\": 20\n\
                    })\n\
                    // Useful for quick overview without overwhelming output\n\n\
                 4. Large set for analysis:\n\
                    process_list({\n\
                        \"limit\": 100\n\
                    })\n\
                    // Get first 100 processes for detailed analysis\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"processes\": [\n\
                     {\n\
                       \"pid\": 1234,\n\
                       \"name\": \"node\",\n\
                       \"cpu_percent\": 15.2,\n\
                       \"memory_mb\": 256.5\n\
                     },\n\
                     {\n\
                       \"pid\": 5678,\n\
                       \"name\": \"postgres\",\n\
                       \"cpu_percent\": 2.1,\n\
                       \"memory_mb\": 128.3\n\
                     },\n\
                     {\n\
                       \"pid\": 9012,\n\
                       \"name\": \"systemd\",\n\
                       \"cpu_percent\": 0.0,\n\
                       \"memory_mb\": 12.8\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 FIELD DESCRIPTIONS:\n\
                 - pid: Process ID - unique identifier for the process\n\
                 - name: Command/executable name - the process name as it appears in the system\n\
                 - cpu_percent: CPU usage percentage - current CPU utilization (0.0 to 100.0+)\n\
                 - memory_mb: Memory in megabytes - RAM consumption of the process\n\n\
                 FIELD BEHAVIOR:\n\
                 - cpu_percent may be 0.0 for idle processes\n\
                 - memory_mb shows current RAM usage, not potential maximum\n\
                 - name is typically the executable name, may include full path\n\
                 - All running processes are shown unless limit is specified\n\n\
                 PARAMETER OPTIONS:\n\
                 - No parameters: List all processes (can be hundreds)\n\
                 - limit: Integer - maximum number of processes to return\n\
                 - Use limit for performance when you only need a subset\n\n\
                 COMMON PATTERNS:\n\
                 - Quick check: limit=10 for fast overview\n\
                 - Full scan: {} for complete system view\n\
                 - Medium set: limit=50 for balanced analysis",
            ),
        },
    ]
}

/// Filtering processes by name and limiting results
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I filter processes by name and limit results?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use the filter parameter to search for specific processes by name. Filtering is case-insensitive and matches substrings. Combine with limit for refined results.\n\n\
                 FILTERING BY NAME:\n\n\
                 1. Filter by exact name:\n\
                    process_list({\n\
                        \"filter\": \"node\"\n\
                    })\n\
                    // Returns only processes with \"node\" in name\n\n\
                 2. Filter for specific application:\n\
                    process_list({\n\
                        \"filter\": \"postgres\"\n\
                    })\n\
                    // Find all PostgreSQL processes\n\n\
                 3. Case-insensitive matching:\n\
                    process_list({\n\
                        \"filter\": \"Node\"\n\
                    })\n\
                    // Matches \"node\", \"Node\", \"NODE\", \"nodejs\"\n\n\
                 4. Partial match:\n\
                    process_list({\n\
                        \"filter\": \"post\"\n\
                    })\n\
                    // Matches \"postgres\", \"postfix\", \"postgis\"\n\n\
                 COMBINING FILTER AND LIMIT:\n\n\
                 1. Top 5 matching processes:\n\
                    process_list({\n\
                        \"filter\": \"python\",\n\
                        \"limit\": 5\n\
                    })\n\
                    // Get first 5 Python processes only\n\n\
                 2. Limited service check:\n\
                    process_list({\n\
                        \"filter\": \"nginx\",\n\
                        \"limit\": 10\n\
                    })\n\
                    // Find up to 10 nginx worker processes\n\n\
                 3. Quick filtered scan:\n\
                    process_list({\n\
                        \"filter\": \"docker\",\n\
                        \"limit\": 3\n\
                    })\n\
                    // First 3 Docker-related processes\n\n\
                 USING LIMIT ALONE:\n\n\
                 1. Small result set:\n\
                    process_list({\n\
                        \"limit\": 5\n\
                    })\n\
                    // Top 5 processes from full list\n\n\
                 2. Medium result set:\n\
                    process_list({\n\
                        \"limit\": 50\n\
                    })\n\
                    // First 50 processes for analysis\n\n\
                 3. Large result set:\n\
                    process_list({\n\
                        \"limit\": 100\n\
                    })\n\
                    // Extensive view without getting everything\n\n\
                 COMMON FILTERING PATTERNS:\n\n\
                 - Find specific service:\n\
                   process_list({ \"filter\": \"nginx\" })\n\n\
                 - Find all Python processes:\n\
                   process_list({ \"filter\": \"python\" })\n\n\
                 - Find Java applications:\n\
                   process_list({ \"filter\": \"java\" })\n\n\
                 - Limited search for performance:\n\
                   process_list({ \"filter\": \"service_name\", \"limit\": 10 })\n\n\
                 - Top processes by name pattern:\n\
                   process_list({ \"filter\": \"app\", \"limit\": 20 })\n\n\
                 FILTER MATCHING RULES:\n\
                 - Substring match: filter=\"node\" matches \"node\", \"nodejs\", \"node_exporter\"\n\
                 - Case-insensitive: filter=\"Node\" matches \"node\", \"NODE\", \"Node.js\"\n\
                 - Partial words: filter=\"post\" matches \"postgres\", \"postfix\"\n\
                 - No regex: plain text matching only\n\n\
                 PERFORMANCE TIPS:\n\
                 - Use filter to narrow results before applying business logic\n\
                 - Combine filter + limit for fastest queries\n\
                 - Exact filter strings return fewer matches (faster)\n\
                 - Generic filters like \"p\" or \"a\" return many results\n\n\
                 PRACTICAL EXAMPLES:\n\n\
                 Check if service is running:\n\
                 process_list({ \"filter\": \"myapp\" })\n\
                 // If empty array, service is down\n\n\
                 Find resource-heavy Node processes:\n\
                 process_list({ \"filter\": \"node\", \"limit\": 20 })\n\
                 // Then examine cpu_percent and memory_mb fields\n\n\
                 Quick development server check:\n\
                 process_list({ \"filter\": \"webpack\", \"limit\": 5 })\n\
                 // Verify dev server is running",
            ),
        },
    ]
}






