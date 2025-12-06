//! Prompt messages for reasoner tool

use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent};
use super::prompt_args::ReasonerPromptArgs;

/// Generate prompt messages for reasoner tool
pub fn reasoner_prompt_messages(args: &ReasonerPromptArgs) -> Vec<PromptMessage> {
    match args.strategy_focus.as_deref() {
        Some("beam_search") => prompt_beam_search(),
        Some("mcts") => prompt_mcts(),
        Some("mcts_002_alpha") => prompt_mcts_high_exploration(),
        Some("mcts_002alt_alpha") => prompt_mcts_length_rewarding(),
        _ => prompt_basic_usage(),
    }
}

fn prompt_basic_usage() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use the reasoner tool to explore multiple solution approaches for a complex problem?"
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "```typescript\n\
                 // Step 1: Start reasoning with initial thought\n\
                 {\n\
                   \"thought\": \"First, I need to understand the problem constraints\",\n\
                   \"thought_number\": 1,\n\
                   \"total_thoughts\": 5,\n\
                   \"next_thought_needed\": true\n\
                 }\n\
