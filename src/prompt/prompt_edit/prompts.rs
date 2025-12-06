//! Prompt messages for edit_prompt tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::EditPromptPromptArgs;

/// Prompt provider for edit_prompt tool
///
/// This is the ONLY way to provide prompts for edit_prompt - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct PromptEditPrompts;

impl PromptProvider for PromptEditPrompts {
    type PromptArgs = EditPromptPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("refinement") => prompt_refinement(),
            Some("versioning") => prompt_versioning(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, refinement, versioning, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO EDIT PROMPTS
// ============================================================================

/// Basic prompt editing operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I edit existing prompt templates?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "EDITING PROMPTS:\n\n\
                 1. Update content:\n\
                    edit_prompt({\n\
                        \"name\": \"code-review\",\n\
                        \"content\": \"Improved review prompt:\\n\\n{{ code }}\\n\\nFocus on: {{ focus | default('general quality') }}\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"name\": \"code-review\",\n\
                   \"updated\": true,\n\
                   \"previous_content\": \"Old template...\"\n\
                 }\n\n\
                 2. Update description:\n\
                    edit_prompt({\n\
                        \"name\": \"code-review\",\n\
                        \"description\": \"Enhanced code review with focus areas\"\n\
                    })\n\n\
                 3. Update both:\n\
                    edit_prompt({\n\
                        \"name\": \"code-review\",\n\
                        \"content\": \"New template...\",\n\
                        \"description\": \"Updated description\"\n\
                    })\n\n\
                 EDIT BEHAVIOR:\n\
                 - Only specified fields updated\n\
                 - Unspecified fields preserved\n\
                 - Returns previous content for reference",
            ),
        },
    ]
}

/// Iterative refinement patterns
fn prompt_refinement() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I iteratively refine a prompt template?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "ITERATIVE REFINEMENT:\n\n\
                 1. Create initial prompt:\n\
                    add_prompt({\n\
                        \"name\": \"bug-analysis\",\n\
                        \"content\": \"Analyze bug:\\n{{ error }}\"\n\
                    })\n\n\
                 2. Test and identify issues:\n\
                    get_prompt({\n\
                        \"name\": \"bug-analysis\",\n\
                        \"variables\": { \"error\": \"null pointer\" }\n\
                    })\n\
                    // Realizes prompt needs more context\n\n\
                 3. Refine:\n\
                    edit_prompt({\n\
                        \"name\": \"bug-analysis\",\n\
                        \"content\": \"Analyze bug:\\n\\nError: {{ error }}\\n\\nContext: {{ context | default('none') }}\\n\\nProvide:\\n1. Root cause\\n2. Fix suggestion\"\n\
                    })\n\n\
                 4. Test again:\n\
                    get_prompt({\n\
                        \"name\": \"bug-analysis\",\n\
                        \"variables\": {\n\
                            \"error\": \"null pointer\",\n\
                            \"context\": \"line 45 in main.rs\"\n\
                        }\n\
                    })\n\n\
                 5. Further refine if needed:\n\
                    edit_prompt({\n\
                        \"name\": \"bug-analysis\",\n\
                        \"content\": \"...\"\n\
                    })\n\n\
                 REFINEMENT CYCLE:\n\
                 - Create → Test → Refine → Repeat\n\
                 - Keep improving until satisfied\n\
                 - Use get_prompt to validate changes",
            ),
        },
    ]
}

/// Version management strategies
fn prompt_versioning() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I manage different versions of prompt templates?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "VERSION MANAGEMENT:\n\n\
                 1. Version via naming:\n\
                    // Original\n\
                    add_prompt({\n\
                        \"name\": \"review-v1\",\n\
                        \"content\": \"Basic review...\"\n\
                    })\n\n\
                    // After improvements\n\
                    add_prompt({\n\
                        \"name\": \"review-v2\",\n\
                        \"content\": \"Improved review...\"\n\
                    })\n\n\
                 2. Track changes in description:\n\
                    edit_prompt({\n\
                        \"name\": \"code-review\",\n\
                        \"content\": \"New improved template...\",\n\
                        \"description\": \"v2: Added focus areas and security checks\"\n\
                    })\n\n\
                 3. Maintain backward compatibility:\n\
                    // Keep old name working\n\
                    edit_prompt({\n\
                        \"name\": \"legacy-review\",\n\
                        \"content\": \"{{ code }}\"  // Simple for old integrations\n\
                    })\n\n\
                    // New version with features\n\
                    add_prompt({\n\
                        \"name\": \"review-enhanced\",\n\
                        \"content\": \"{{ code }}\\n{% if security %}Security focus{% endif %}\"\n\
                    })\n\n\
                 VERSIONING STRATEGIES:\n\
                 - Suffix: prompt-v1, prompt-v2\n\
                 - Dated: prompt-2024-01\n\
                 - Feature: prompt-basic, prompt-enhanced",
            ),
        },
    ]
}

/// Edit workflows and patterns
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows for editing prompt templates?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "EDIT WORKFLOWS:\n\n\
                 1. Fix template bug:\n\
                    // Discover issue\n\
                    get_prompt({\n\
                        \"name\": \"broken-prompt\",\n\
                        \"variables\": { \"x\": \"test\" }\n\
                    })\n\
                    // Error: undefined variable 'y'\n\n\
                    // Fix the template\n\
                    edit_prompt({\n\
                        \"name\": \"broken-prompt\",\n\
                        \"content\": \"Fixed: {{ x | default('') }}\"\n\
                    })\n\n\
                 2. Add new feature:\n\
                    // Get current\n\
                    get_prompt({ \"name\": \"review\" })\n\
                    // See: \"Review {{ code }}\"\n\n\
                    // Add feature\n\
                    edit_prompt({\n\
                        \"name\": \"review\",\n\
                        \"content\": \"Review {{ code }}\\n\\n{% if checklist %}Checklist:\\n{% for item in checklist %}- {{ item }}\\n{% endfor %}{% endif %}\"\n\
                    })\n\n\
                 3. Simplify complex prompt:\n\
                    edit_prompt({\n\
                        \"name\": \"over-engineered\",\n\
                        \"content\": \"Simplified: {{ main_input }}\"\n\
                    })\n\n\
                 4. Update for new requirements:\n\
                    edit_prompt({\n\
                        \"name\": \"code-review\",\n\
                        \"content\": \"Review with NEW framework guidelines:\\n\\n{{ code }}\",\n\
                        \"description\": \"Updated for 2024 guidelines\"\n\
                    })\n\n\
                 WORKFLOW PATTERNS:\n\
                 - Bug fix: Identify → Edit → Verify\n\
                 - Enhancement: Plan → Edit → Test\n\
                 - Simplification: Review → Edit → Confirm",
            ),
        },
    ]
}
