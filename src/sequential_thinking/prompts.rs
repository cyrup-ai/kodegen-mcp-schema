//! Prompt messages for sequential_thinking tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::SequentialThinkingPromptArgs;

/// Prompt provider for sequential_thinking tool
///
/// This is the ONLY way to provide prompts for sequential_thinking - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct SequentialThinkingPrompts;

impl PromptProvider for SequentialThinkingPrompts {
    type PromptArgs = SequentialThinkingPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("revision") => prompt_revision(),
            Some("branching") => prompt_branching(),
            Some("sessions") | Some("continuation") => prompt_continuation(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show examples for: basic, revision, branching, sessions".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE SEQUENTIAL THINKING
// ============================================================================

/// Basic sequential thinking
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use sequential thinking for step-by-step problem solving?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "SEQUENTIAL THINKING:\n\n\
                 1. Start thinking chain:\n\
                    sequential_thinking({\n\
                        \"thought\": \"Analyzing the memory leak. First, identify allocation patterns.\",\n\
                        \"thought_number\": 1,\n\
                        \"total_thoughts\": 4,\n\
                        \"next_thought_needed\": true\n\
                    })\n\n\
                 2. Continue sequence:\n\
                    sequential_thinking({\n\
                        \"thought\": \"Found Vec growing without bounds in process_events(). No cleanup.\",\n\
                        \"thought_number\": 2,\n\
                        \"total_thoughts\": 4,\n\
                        \"next_thought_needed\": true\n\
                    })\n\n\
                 3. Build on previous:\n\
                    sequential_thinking({\n\
                        \"thought\": \"The Vec stores event handlers but never removes completed ones.\",\n\
                        \"thought_number\": 3,\n\
                        \"total_thoughts\": 4,\n\
                        \"next_thought_needed\": true\n\
                    })\n\n\
                 4. Conclude:\n\
                    sequential_thinking({\n\
                        \"thought\": \"Fix: Add cleanup logic to remove completed handlers. Use retain().\",\n\
                        \"thought_number\": 4,\n\
                        \"total_thoughts\": 4,\n\
                        \"next_thought_needed\": false\n\
                    })\n\n\
                 KEY PARAMETERS:\n\
                 - thought: Your current reasoning\n\
                 - thought_number: Step in sequence\n\
                 - total_thoughts: Estimated steps needed\n\
                 - next_thought_needed: true to continue",
            ),
        },
    ]
}

/// Revising thoughts when assumptions change
fn prompt_revision() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I revise earlier thoughts when I discover new information?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "REVISING THOUGHTS:\n\n\
                 1. Initial thought:\n\
                    sequential_thinking({\n\
                        \"thought\": \"The bug is in the parser. Line 45 has wrong regex.\",\n\
                        \"thought_number\": 1,\n\
                        \"total_thoughts\": 3,\n\
                        \"next_thought_needed\": true\n\
                    })\n\n\
                 2. Realize error, revise:\n\
                    sequential_thinking({\n\
                        \"thought\": \"Wait, the regex is correct. Issue is in the tokenizer feeding wrong input.\",\n\
                        \"thought_number\": 2,\n\
                        \"total_thoughts\": 4,\n\
                        \"next_thought_needed\": true,\n\
                        \"is_revision\": true,\n\
                        \"revises_thought\": 1\n\
                    })\n\n\
                 3. Continue with corrected understanding:\n\
                    sequential_thinking({\n\
                        \"thought\": \"Tokenizer strips whitespace that parser needs. Fix tokenizer.\",\n\
                        \"thought_number\": 3,\n\
                        \"total_thoughts\": 4,\n\
                        \"next_thought_needed\": true\n\
                    })\n\n\
                 REVISION PARAMETERS:\n\
                 - is_revision: true (marks as revision)\n\
                 - revises_thought: which thought to revise\n\n\
                 WHEN TO REVISE:\n\
                 - Found incorrect assumption\n\
                 - New information changes understanding\n\
                 - Need to correct earlier reasoning",
            ),
        },
    ]
}

/// Branching to explore alternatives
fn prompt_branching() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create branches to explore multiple solution paths?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "BRANCHING THOUGHTS:\n\n\
                 1. Main thought path:\n\
                    sequential_thinking({\n\
                        \"thought\": \"Database is slow. Could be indexes or query structure.\",\n\
                        \"thought_number\": 1,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": true\n\
                    })\n\n\
                 2. Continue main path:\n\
                    sequential_thinking({\n\
                        \"thought\": \"Checking indexes first. Missing index on user_id column.\",\n\
                        \"thought_number\": 2,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": true\n\
                    })\n\n\
                 3. Branch to explore alternative:\n\
                    sequential_thinking({\n\
                        \"thought\": \"Alternative: Query uses SELECT *. Could optimize columns.\",\n\
                        \"thought_number\": 3,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": true,\n\
                        \"branch_from_thought\": 1,\n\
                        \"branch_id\": \"column-optimization\"\n\
                    })\n\n\
                 4. Continue branch:\n\
                    sequential_thinking({\n\
                        \"thought\": \"Selecting only needed columns reduces data transfer 80%.\",\n\
                        \"thought_number\": 4,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": true,\n\
                        \"branch_id\": \"column-optimization\"\n\
                    })\n\n\
                 BRANCHING PARAMETERS:\n\
                 - branch_from_thought: thought number to branch from\n\
                 - branch_id: identifier for this branch\n\n\
                 USE CASES:\n\
                 - Explore multiple hypotheses\n\
                 - Compare solutions\n\
                 - Parallel investigation paths",
            ),
        },
    ]
}

/// Continuing thoughts across calls (sessions are automatic)
fn prompt_continuation() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I continue a thinking chain across multiple calls?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "CONTINUING THOUGHTS:\n\n\
                 Sessions are managed automatically per connection. Simply continue calling:\n\n\
                 1. Start thinking:\n\
                    sequential_thinking({\n\
                        \"thought\": \"Investigating auth failure for user login flow.\",\n\
                        \"thought_number\": 1,\n\
                        \"total_thoughts\": 6,\n\
                        \"next_thought_needed\": true\n\
                    })\n\n\
                 2. Continue (same connection = same session):\n\
                    sequential_thinking({\n\
                        \"thought\": \"Token validation passes but session creation fails.\",\n\
                        \"thought_number\": 2,\n\
                        \"total_thoughts\": 6,\n\
                        \"next_thought_needed\": true\n\
                    })\n\n\
                 SESSION BEHAVIOR:\n\
                 - One session per connection (automatic)\n\
                 - State persists across all calls in same connection\n\
                 - No session_id needed - managed automatically\n\n\
                 NEEDS_MORE_THOUGHTS:\n\
                 - Use when you need more steps than initially estimated\n\
                 - Dynamically extends total_thoughts",
            ),
        },
    ]
}

