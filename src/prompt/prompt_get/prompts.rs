//! Prompt messages for get_prompt tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetPromptPromptArgs;

/// Prompt provider for get_prompt tool
pub struct PromptGetPrompts;

impl PromptProvider for PromptGetPrompts {
    type PromptArgs = GetPromptPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("variables") => prompt_variables(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show examples for (basic, variables)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GET_PROMPT
// ============================================================================

/// Basic prompt retrieval
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I retrieve prompts using get_prompt?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "RETRIEVING PROMPTS:\n\n\
                 1. Get prompt without variables:\n\
                    get_prompt({\n\
                        \"name\": \"code-review\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"name\": \"code-review\",\n\
                   \"content\": \"Review this code for bugs and improvements:\\n\\n{{ code }}\",\n\
                   \"rendered\": null\n\
                 }\n\n\
                 2. Get with variables:\n\
                    get_prompt({\n\
                        \"name\": \"code-review\",\n\
                        \"variables\": {\n\
                            \"code\": \"fn main() { println!(\\\"Hello\\\"); }\"\n\
                        }\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"name\": \"code-review\",\n\
                   \"content\": \"Review this code...\",\n\
                   \"rendered\": \"Review this code for bugs and improvements:\\n\\nfn main() { println!(\\\"Hello\\\"); }\"\n\
                 }\n\n\
                 OUTPUT FIELDS:\n\
                 - name: Prompt identifier\n\
                 - content: Raw template\n\
                 - rendered: Rendered with variables (if provided)\n\n\
                 Use get_prompt to retrieve and optionally render prompt templates with Jinja2 variable substitution."
            ),
        },
    ]
}

/// Variable substitution patterns
fn prompt_variables() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use variables in get_prompt?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "VARIABLE SUBSTITUTION:\n\n\
                 1. Simple variables:\n\
                    get_prompt({\n\
                        \"name\": \"greet\",\n\
                        \"variables\": {\n\
                            \"name\": \"Alice\",\n\
                            \"project\": \"KODEGEN\"\n\
                        }\n\
                    })\n\
                    // \"Hello Alice, welcome to KODEGEN!\"\n\n\
                 2. Object variables:\n\
                    get_prompt({\n\
                        \"name\": \"user-summary\",\n\
                        \"variables\": {\n\
                            \"user\": {\n\
                                \"name\": \"Bob\",\n\
                                \"role\": \"Developer\"\n\
                            }\n\
                        }\n\
                    })\n\
                    // Template: \"{{ user.name }} is a {{ user.role }}\"\n\n\
                 3. List variables:\n\
                    get_prompt({\n\
                        \"name\": \"file-list\",\n\
                        \"variables\": {\n\
                            \"files\": [\"main.rs\", \"lib.rs\", \"mod.rs\"]\n\
                        }\n\
                    })\n\
                    // Template: \"{% for f in files %}{{ f }}\\n{% endfor %}\"\n\n\
                 4. Optional variables:\n\
                    get_prompt({\n\
                        \"name\": \"optional-prompt\",\n\
                        \"variables\": {\n\
                            \"required\": \"value\"\n\
                            // \"optional\" not provided, uses default\n\
                        }\n\
                    })\n\n\
                 VARIABLE TYPES:\n\
                 - Strings: \"value\"\n\
                 - Numbers: 42, 3.14\n\
                 - Booleans: true, false\n\
                 - Objects: { \"key\": \"value\" }\n\
                 - Arrays: [\"a\", \"b\", \"c\"]\n\n\
                 Variables are passed as JSON values and substituted into Jinja2 templates using {{ variable }} syntax."
            ),
        },
    ]
}
