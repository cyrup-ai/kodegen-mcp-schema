//! Prompt messages for browser_scroll tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::BrowserScrollPromptArgs;

/// Prompt provider for browser_scroll tool
///
/// This is the ONLY way to provide prompts for browser_scroll - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ScrollPrompts;

impl PromptProvider for ScrollPrompts {
    type PromptArgs = BrowserScrollPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        // Always return basic scenario - all others have been trimmed
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic page scrolling with x/y deltas
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I scroll a page up, down, left, or right using browser_scroll?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The browser_scroll tool scrolls the page using pixel-based x/y coordinates. Positive values scroll right/down, negative values scroll left/up.\n\n\
                 BASIC PAGE SCROLLING:\n\n\
                 1. Scroll down by pixels:\n\
                    browser_scroll({\"y\": 500})\n\
                    // Scrolls down 500 pixels\n\n\
                 2. Scroll up:\n\
                    browser_scroll({\"y\": -500})\n\
                    // Negative values scroll up 500 pixels\n\n\
                 3. Scroll horizontally right:\n\
                    browser_scroll({\"x\": 300})\n\
                    // Scrolls right 300 pixels\n\n\
                 4. Scroll horizontally left:\n\
                    browser_scroll({\"x\": -300})\n\
                    // Negative values scroll left 300 pixels\n\n\
                 5. Diagonal scroll:\n\
                    browser_scroll({\"x\": 100, \"y\": 300})\n\
                    // Scrolls right 100px and down 300px\n\n\
                 SCROLL AMOUNTS:\n\
                 - Small adjustment: 100-200 pixels\n\
                 - Half viewport: ~400-500 pixels\n\
                 - Full viewport: ~800-1000 pixels\n\
                 - Large jump: 2000+ pixels\n\n\
                 WHEN TO SCROLL:\n\
                 - Element below viewport (can't click/see it)\n\
                 - Need to reveal more content\n\
                 - Looking for something further down page\n\
                 - Loading infinite scroll content\n\
                 - Navigating long forms or documents\n\n\
                 SCROLL DIRECTION REFERENCE:\n\
                 - y > 0: Scroll DOWN (content moves up, you see lower content)\n\
                 - y < 0: Scroll UP (content moves down, you see higher content)\n\
                 - x > 0: Scroll RIGHT (content moves left, you see rightward content)\n\
                 - x < 0: Scroll LEFT (content moves right, you see leftward content)\n\n\
                 COMMON PATTERNS:\n\
                 1. Scroll to bottom of page:\n\
                    browser_scroll({\"y\": 10000})\n\
                    // Large value ensures you reach bottom\n\n\
                 2. Scroll to top of page:\n\
                    browser_scroll({\"y\": -10000})\n\
                    // Large negative value ensures you reach top\n\n\
                 3. Progressive scrolling:\n\
                    browser_scroll({\"y\": 1000})\n\
                    // Check content\n\
                    browser_scroll({\"y\": 1000})\n\
                    // Continue scrolling\n\n\
                 4. Precise navigation:\n\
                    browser_scroll({\"y\": 250})\n\
                    // Small increments for careful positioning\n\n\
                 OUTPUT STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"direction\": \"down\",        // Scroll direction: up, down, left, right\n\
                   \"amount\": 500,              // Pixels scrolled\n\
                   \"message\": \"Scrolled down 500 pixels\"\n\
                 }\n\n\
                 TROUBLESHOOTING:\n\
                 - If scroll doesn't work: Page might be at top/bottom already\n\
                 - If element still not visible: Scroll more or use selector method\n\
                 - If page seems stuck: Try screenshot to verify current position\n\
                 - If scrolling too much: Reduce pixel values (try 200-300)\n\n\
                 BEST PRACTICES:\n\
                 - Start with moderate scroll amounts (500-800px)\n\
                 - Use multiple small scrolls for better control\n\
                 - Verify content with screenshot or extract_text after scrolling\n\
                 - For specific elements, use selector method instead",
            ),
        },
    ]
}
