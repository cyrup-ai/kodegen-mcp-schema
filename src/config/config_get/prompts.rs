//! Prompt messages for config_get tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ConfigGetPromptArgs;

/// Prompt provider for config_get tool
pub struct ConfigGetPrompts;

impl PromptProvider for ConfigGetPrompts {
    type PromptArgs = ConfigGetPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

/// Basic config retrieval - teaches AI how to use config_get
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I retrieve and understand the server configuration?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The config_get tool retrieves complete server configuration with no parameters required.\n\n\
                 BASIC USAGE:\n\
                 config_get({})\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"config\": {\n\
                     \"security\": {\n\
                       \"blocked_commands\": [\"rm -rf /\", \"sudo\", \"chmod\", ...],\n\
                       \"allowed_directories\": [\"/home/user\", \"/tmp\", ...],\n\
                       \"max_file_size_bytes\": 10485760\n\
                     },\n\
                     \"shell\": {\n\
                       \"default_shell\": \"/bin/bash\",\n\
                       \"timeout_ms\": 300000\n\
                     },\n\
                     \"resources\": {\n\
                       \"max_concurrent_terminals\": 10,\n\
                       \"max_memory_mb\": 1024,\n\
                       \"file_read_line_limit\": 1000,\n\
                       \"file_write_line_limit\": 50\n\
                     },\n\
                     \"system_info\": {\n\
                       \"platform\": \"linux\",\n\
                       \"arch\": \"x86_64\",\n\
                       \"os_version\": \"Ubuntu 22.04\",\n\
                       \"cpu_count\": 8,\n\
                       \"memory\": {\"total_mb\": \"16384 MB\", \"available_mb\": \"8192 MB\", \"used_mb\": \"8192 MB\"}\n\
                     }\n\
                   }\n\
                 }\n\n\
                 WHEN TO CHECK CONFIG:\n\
                 - Before operations that might be blocked\n\
                 - Understanding what's allowed in the environment\n\
                 - Debugging permission or access errors\n\
                 - Verifying environment setup\n\
                 - Checking resource limits before large operations\n\n\
                 COMMON USE CASES:\n\
                 1. Check before file operations: config = config_get({}); check allowed_directories\n\
                 2. Verify command availability: config = config_get({}); check blocked_commands\n\
                 3. Check resource limits: config = config_get({}); check file_read_line_limit\n\
                 4. System diagnostics: config = config_get({}); check system_info for platform details\n\n\
                 KEY CONSTRAINTS:\n\
                 - config_get takes no input parameters\n\
                 - Always returns the complete, current configuration\n\
                 - Configuration is live and may change between calls\n\
                 - Use config_set to modify configuration values\n\n\
                 INTERPRETATION:\n\
                 - allowed_directories [] = no restrictions (full access)\n\
                 - allowed_directories [paths] = only listed paths accessible\n\
                 - blocked_commands [] = no command restrictions\n\
                 - blocked_commands [cmds] = these specific commands are blocked\n\
                 - max_file_size_bytes typically 10485760 (10MB)\n\n\
                 Always verify assumptions with config_get before critical operations.",
            ),
        },
    ]
}
