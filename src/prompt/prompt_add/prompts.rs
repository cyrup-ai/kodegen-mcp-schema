//! Prompt messages for add_prompt tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::AddPromptPromptArgs;

/// Prompt provider for add_prompt tool
///
/// This is the ONLY way to provide prompts for add_prompt - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct PromptAddPrompts;

impl PromptProvider for PromptAddPrompts {
    type PromptArgs = AddPromptPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            _ => prompt_templating(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, templating)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO CREATE PROMPTS
// ============================================================================

/// Basic prompt creation
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create basic prompts with add_prompt?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The add_prompt tool creates reusable prompt templates with Jinja2 templating support. Here's how to create basic prompts:\n\n\
                 CREATING PROMPTS:\n\n\
                 1. Create simple prompt:\n\
                    add_prompt({\n\
                        \"name\": \"code-review\",\n\
                        \"content\": \"Review this code for bugs and improvements:\\n\\n{{ code }}\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"name\": \"code-review\",\n\
                   \"created\": true\n\
                 }\n\n\
                 2. Create with description:\n\
                    add_prompt({\n\
                        \"name\": \"explain-error\",\n\
                        \"content\": \"Explain this error message:\\n\\n{{ error }}\",\n\
                        \"description\": \"Help understand error messages\"\n\
                    })\n\n\
                 PROMPT FIELDS:\n\
                 - name: Unique identifier\n\
                 - content: Template content\n\
                 - description: Optional documentation",
            ),
        },
    ]
}

/// Jinja2 templating features
fn prompt_templating() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use Jinja2 templating in prompts?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "add_prompt supports full Jinja2 templating syntax for dynamic, reusable prompts:\n\n\
                 JINJA2 TEMPLATING:\n\n\
                 1. Variable substitution:\n\
                    add_prompt({\n\
                        \"name\": \"greet\",\n\
                        \"content\": \"Hello {{ name }}, welcome to {{ project }}!\"\n\
                    })\n\n\
                 2. Conditional content:\n\
                    add_prompt({\n\
                        \"name\": \"code-review-conditional\",\n\
                        \"content\": \"Review this {{ language }} code:\\n\\n{{ code }}\\n\\n{% if strict %}Be strict about style.{% endif %}\"\n\
                    })\n\n\
                 3. Loops:\n\
                    add_prompt({\n\
                        \"name\": \"analyze-files\",\n\
                        \"content\": \"Analyze these files:\\n{% for file in files %}\\n- {{ file }}\\n{% endfor %}\"\n\
                    })\n\n\
                 4. Filters:\n\
                    add_prompt({\n\
                        \"name\": \"uppercase-title\",\n\
                        \"content\": \"# {{ title | upper }}\\n\\n{{ content }}\"\n\
                    })\n\n\
                 5. Default values:\n\
                    add_prompt({\n\
                        \"name\": \"optional-context\",\n\
                        \"content\": \"Task: {{ task }}\\n\\nContext: {{ context | default('No additional context') }}\"\n\
                    })\n\n\
                 TEMPLATE FEATURES:\n\
                 - {{ variable }}: Insert value\n\
                 - {% if cond %}...{% endif %}: Conditional\n\
                 - {% for x in list %}...{% endfor %}: Loop\n\
                 - {{ var | filter }}: Apply filter\n\
                 - {{ var | default('value') }}: Default",
            ),
        },
    ]
}
