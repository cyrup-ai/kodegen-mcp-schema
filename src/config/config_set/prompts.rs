//! Prompt messages for config_set tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::SetConfigValuePromptArgs;

/// Prompt provider for config_set tool
///
/// This is the ONLY way to provide prompts for config_set - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct SetConfigValuePrompts;

impl PromptProvider for SetConfigValuePrompts {
    type PromptArgs = SetConfigValuePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("value_types") => prompt_value_types(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, value_types)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic configuration changes with different value types
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I make basic configuration changes using config_set?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The config_set tool modifies configuration values by key. Here's how to use it for basic changes:\n\n\
                 BASIC CONFIG CHANGES:\n\n\
                 1. Set string value:\n\
                    config_set({\"key\": \"shell.default_shell\", \"value\": \"/bin/zsh\"})\n\n\
                 2. Set number value:\n\
                    config_set({\"key\": \"shell.timeout_ms\", \"value\": 600000})\n\n\
                 3. Set boolean value:\n\
                    config_set({\"key\": \"security.require_approval\", \"value\": true})\n\n\
                 4. Set array value:\n\
                    config_set({\"key\": \"security.allowed_directories\", \"value\": [\"/home/user\", \"/tmp\", \"/project\"]})\n\n\
                 PARAMETERS:\n\
                 - key: Configuration key to update (dot notation)\n\
                 - value: New value (string, number, boolean, or array)\n\n\
                 KEY FORMAT:\n\
                 - Use dot notation for nested keys\n\
                 - Examples:\n\
                   - security.allowed_directories\n\
                   - shell.timeout_ms\n\
                   - resources.max_concurrent_terminals\n\
                   - logging.verbose\n\n\
                 COMMON CONFIGURATION KEYS:\n\
                 - shell.default_shell: String - Default shell path\n\
                 - shell.timeout_ms: Number - Command timeout in milliseconds\n\
                 - security.blocked_commands: Array - Commands that cannot be executed\n\
                 - security.allowed_directories: Array - Whitelist of accessible directories\n\
                 - security.require_approval: Boolean - Require approval for sensitive operations\n\
                 - resources.max_concurrent_terminals: Number - Maximum simultaneous terminals\n\
                 - resources.max_memory_mb: Number - Memory limit in megabytes\n\
                 - logging.verbose: Boolean - Enable verbose logging\n\n\
                 RESPONSE:\n\
                 - success: true if configuration was updated\n\
                 - key: The configuration key that was modified\n\
                 - message: Confirmation message\n\n\
                 IMPORTANT NOTES:\n\
                 - Configuration changes take effect immediately\n\
                 - Some changes may require restarting services\n\
                 - Always verify changes with config_get after modifying\n\
                 - For array values, provide the COMPLETE array (not just additions)",
            ),
        },
    ]
}


/// Value type formats and type matching
fn prompt_value_types() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What value types does config_set support and how do I format them?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The config_set tool supports multiple value types. The value must match the expected type for each configuration key.\n\n\
                 VALUE TYPE FORMATS:\n\n\
                 1. STRING VALUES:\n\
                    config_set({\"key\": \"shell.default_shell\", \"value\": \"/bin/bash\"})\n\
                    config_set({\"key\": \"logging.output_path\", \"value\": \"/var/log/app.log\"})\n\n\
                    - Use for: File paths, shell names, text settings\n\
                    - Format: Quoted string\n\
                    - Example values: \"/bin/zsh\", \"info\", \"utf-8\"\n\n\
                 2. INTEGER/NUMBER VALUES:\n\
                    config_set({\"key\": \"shell.timeout_ms\", \"value\": 300000})\n\
                    config_set({\"key\": \"resources.max_memory_mb\", \"value\": 2048})\n\
                    config_set({\"key\": \"resources.max_concurrent_terminals\", \"value\": 10})\n\n\
                    - Use for: Timeouts, limits, counts, sizes\n\
                    - Format: Unquoted number (no decimals for integers)\n\
                    - Example values: 1000, 5000, 60000\n\n\
                 3. BOOLEAN VALUES:\n\
                    config_set({\"key\": \"security.require_approval\", \"value\": true})\n\
                    config_set({\"key\": \"logging.verbose\", \"value\": false})\n\
                    config_set({\"key\": \"features.experimental_mode\", \"value\": true})\n\n\
                    - Use for: Toggles, flags, enable/disable settings\n\
                    - Format: true or false (lowercase, unquoted)\n\
                    - Example values: true, false\n\n\
                 4. ARRAY OF STRINGS:\n\
                    config_set({\n\
                        \"key\": \"security.allowed_directories\",\n\
                        \"value\": [\"/home/user\", \"/tmp\", \"/project\"]\n\
                    })\n\n\
                    config_set({\n\
                        \"key\": \"security.blocked_commands\",\n\
                        \"value\": [\"rm -rf /\", \"format\", \"mkfs\", \"dd\"]\n\
                    })\n\n\
                    config_set({\n\
                        \"key\": \"shell.environment_variables\",\n\
                        \"value\": [\"PATH=/usr/bin\", \"HOME=/home/user\"]\n\
                    })\n\n\
                    - Use for: Lists, collections, multiple values\n\
                    - Format: Bracket-enclosed, comma-separated quoted strings\n\
                    - Must provide COMPLETE array (not incremental additions)\n\
                    - Empty array: []\n\n\
                 5. FLOATING POINT VALUES:\n\
                    config_set({\"key\": \"resources.cpu_threshold\", \"value\": 0.85})\n\
                    config_set({\"key\": \"search.similarity_score\", \"value\": 0.7})\n\n\
                    - Use for: Percentages, ratios, thresholds\n\
                    - Format: Unquoted decimal number\n\
                    - Example values: 0.5, 0.95, 1.5\n\n\
                 TYPE MATCHING:\n\
                 - Value MUST match expected type for the key\n\
                 - Passing wrong type will cause error\n\
                 - Check current value type with config_get first\n\n\
                 EXAMPLES BY KEY TYPE:\n\n\
                 String keys:\n\
                 - shell.default_shell → \"/bin/bash\" (not \"bash\" or \"/bin/bash/\")\n\
                 - logging.level → \"info\" (not INFO or info without quotes)\n\n\
                 Number keys:\n\
                 - shell.timeout_ms → 600000 (not \"600000\" with quotes)\n\
                 - resources.max_memory_mb → 4096 (not 4096.0 or \"4GB\")\n\n\
                 Boolean keys:\n\
                 - security.require_approval → true (not \"true\" or 1 or True)\n\
                 - logging.verbose → false (not \"false\" or 0 or False)\n\n\
                 Array keys:\n\
                 - security.allowed_directories → [\"/home\", \"/tmp\"] (not \"/home,/tmp\")\n\
                 - security.blocked_commands → [\"rm\", \"sudo\"] (not \"rm,sudo\")\n\n\
                 COMMON TYPE ERRORS:\n\n\
                 1. Quoting numbers:\n\
                    WRONG: config_set({\"key\": \"timeout_ms\", \"value\": \"5000\"})\n\
                    RIGHT: config_set({\"key\": \"timeout_ms\", \"value\": 5000})\n\n\
                 2. Not quoting strings:\n\
                    WRONG: config_set({\"key\": \"default_shell\", \"value\": /bin/bash})\n\
                    RIGHT: config_set({\"key\": \"default_shell\", \"value\": \"/bin/bash\"})\n\n\
                 3. Wrong boolean format:\n\
                    WRONG: config_set({\"key\": \"verbose\", \"value\": \"true\"})\n\
                    WRONG: config_set({\"key\": \"verbose\", \"value\": 1})\n\
                    RIGHT: config_set({\"key\": \"verbose\", \"value\": true})\n\n\
                 4. Single value instead of array:\n\
                    WRONG: config_set({\"key\": \"allowed_directories\", \"value\": \"/home/user\"})\n\
                    RIGHT: config_set({\"key\": \"allowed_directories\", \"value\": [\"/home/user\"]})\n\n\
                 5. Incomplete array update:\n\
                    WRONG: Just adding new item (will replace entire array)\n\
                    RIGHT: Include ALL items (current + new)\n\n\
                 WORKFLOW FOR ARRAY UPDATES:\n\
                 // Want to add /project to allowed directories\n\
                 // Step 1: Get current config\n\
                 current = config_get({})\n\
                 // Step 2: Note current array: [\"/home/user\", \"/tmp\"]\n\
                 // Step 3: Create new array with ALL items (old + new)\n\
                 config_set({\n\
                     \"key\": \"security.allowed_directories\",\n\
                     \"value\": [\"/home/user\", \"/tmp\", \"/project\"]\n\
                 })\n\
                 // Step 4: Verify with config_get\n\n\
                 TYPE DISCOVERY:\n\
                 If unsure about a key's type:\n\
                 1. Use config_get to see current value\n\
                 2. Match the type of the current value\n\
                 3. Maintain same format for new value",
            ),
        },
    ]
}

