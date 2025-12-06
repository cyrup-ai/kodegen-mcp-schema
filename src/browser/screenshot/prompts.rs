//! Prompt messages for browser_screenshot tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::BrowserScreenshotPromptArgs;

/// Prompt provider for browser_screenshot tool
///
/// This is the ONLY way to provide prompts for browser_screenshot - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ScreenshotPrompts;

impl PromptProvider for ScreenshotPrompts {
    type PromptArgs = BrowserScreenshotPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_debugging()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

// ============================================================================
// DEBUGGING SCENARIO - PRIMARY USE CASE FOR BROWSER SCREENSHOTS
// ============================================================================

/// Debugging with screenshots - understanding page state when things go wrong
fn prompt_debugging() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "When should I use screenshots for debugging browser automation?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Screenshots are essential for understanding what's actually happening on the page when browser automation doesn't work as expected.\n\n\
                 WHEN TO TAKE DEBUG SCREENSHOTS:\n\
                 1. After navigation failure or unexpected behavior\n\
                 2. Before attempting to click an element\n\
                 3. After an action produces no visible result\n\
                 4. When extracted text doesn't match expectations\n\
                 5. When you need to verify page state\n\n\
                 DEBUGGING WORKFLOW:\n\
                 Step 1: Action fails or produces unexpected result\n\
                 Step 2: Take screenshot to see actual page state\n\
                 Step 3: Analyze visual state (what page, what's visible)\n\
                 Step 4: Identify issue (wrong page, overlay, element missing)\n\
                 Step 5: Adjust approach based on what you see\n\n\
                 EXAMPLE 1: Navigation Failure\n\
                 browser_navigate({\"url\": \"https://app.example.com/dashboard\"})\n\
                 // Try to click login button but it fails\n\
                 browser_click({\"selector\": \"#submit-btn\"})\n\
                 // ERROR: Element not found\n\
                 // Take screenshot to see why:\n\
                 browser_screenshot({})\n\
                 // Discovery: Page redirected to login page, not dashboard\n\
                 // Solution: Handle login first, then navigate to dashboard\n\n\
                 EXAMPLE 2: Action Had No Effect\n\
                 browser_click({\"selector\": \"#generate-report\"})\n\
                 // Expected report to appear, but nothing happened\n\
                 browser_screenshot({})\n\
                 // Discovery: Button triggered loading spinner\n\
                 // Solution: Wait for loading to complete\n\
                 // Take another screenshot after waiting\n\n\
                 EXAMPLE 3: Verify Current Page State\n\
                 // After complex sequence of actions:\n\
                 browser_click({\"selector\": \".next-step\"})\n\
                 browser_type_text({\"selector\": \"#email\", \"text\": \"user@example.com\"})\n\
                 browser_click({\"selector\": \".submit\"})\n\
                 // Verify we're on the right page:\n\
                 browser_screenshot({})\n\
                 // Check: Did we advance to confirmation page or get error?\n\n\
                 COMMON DISCOVERIES FROM DEBUG SCREENSHOTS:\n\
                 - Login redirect: Page required authentication you didn't detect\n\
                 - Modal/overlay: Dialog or cookie banner covering target element\n\
                 - Element outside viewport: Need to scroll before interacting\n\
                 - Page still loading: Content not fully rendered yet\n\
                 - Different page version: Mobile vs desktop layout\n\
                 - Error message: Page displayed error you didn't catch\n\
                 - Wrong page entirely: Navigation didn't go where expected\n\
                 - JavaScript not executed: Content requires JS that didn't run\n\n\
                 SCREENSHOT TIMING FOR DEBUGGING:\n\
                 - BEFORE action: Verify element is visible and clickable\n\
                 - AFTER action: Confirm the action had desired effect\n\
                 - ON ERROR: Immediately screenshot to see what went wrong\n\
                 - AFTER WAIT: Verify that waiting produced expected state\n\n\
                 BEST PRACTICES:\n\
                 - Take screenshots early and often during debugging\n\
                 - Use viewport screenshots for speed (default)\n\
                 - Don't assume - verify with visual evidence\n\
                 - Screenshot before and after problem area\n\
                 - Compare screenshots to identify what changed\n\
                 - Use element selectors to focus on problem area",
            ),
        },
    ]
}








