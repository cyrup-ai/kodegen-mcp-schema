//! Prompt messages for browser_navigate tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::BrowserNavigatePromptArgs;

/// Prompt provider for browser_navigate tool
///
/// This is the ONLY way to provide prompts for browser_navigate - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct NavigatePrompts;

impl PromptProvider for NavigatePrompts {
    type PromptArgs = BrowserNavigatePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            _ => prompt_basic(),
        }
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

/// Basic URL navigation patterns
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I navigate to URLs using browser_navigate?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The browser_navigate tool navigates to URLs and waits for page load. Here's how to use it for basic navigation:\n\n\
                 BASIC NAVIGATION:\n\n\
                 1. Simple URL:\n\
                    browser_navigate({\"url\": \"https://github.com\"})\n\n\
                 2. With path:\n\
                    browser_navigate({\"url\": \"https://github.com/anthropics/claude-code\"})\n\n\
                 3. Relative navigation (if already on site):\n\
                    browser_navigate({\"url\": \"/settings\"})\n\
                    browser_navigate({\"url\": \"../dashboard\"})\n\n\
                 URL REQUIREMENTS:\n\
                 - Must include protocol (https://)\n\
                 - Path should be properly encoded\n\
                 - Fragments (#section) work for anchors\n\
                 - Query parameters can be included in URL string\n\n\
                 RESPONSE STRUCTURE:\n\
                 The tool returns BrowserNavigateOutput with:\n\
                 - final_url: Where browser ended up (may differ due to redirects)\n\
                 - status_code: HTTP status (200, 404, 500, etc.)\n\
                 - title: Page title from <title> tag\n\
                 - load_time_ms: How long page took to load\n\n\
                 EXAMPLE RESPONSE:\n\
                 {\n\
                   \"final_url\": \"https://github.com/anthropics/claude-code\",\n\
                   \"status_code\": 200,\n\
                   \"title\": \"GitHub - anthropics/claude-code: Official Claude CLI\",\n\
                   \"load_time_ms\": 1234\n\
                 }\n\n\
                 WHEN TO USE BROWSER_NAVIGATE:\n\
                 - Loading a new page or URL\n\
                 - Following links to different pages\n\
                 - Checking if a URL is accessible\n\
                 - Starting a browsing session\n\
                 - Navigating after form submission\n\n\
                 URL ENCODING:\n\
                 - Spaces should be %20 or +\n\
                 - Special characters should be URL encoded\n\
                 - Already encoded URLs work as-is\n\
                 - Don't double-encode URLs\n\n\
                 COMMON PATTERNS:\n\
                 1. Check if page exists:\n\
                    browser_navigate({\"url\": \"https://example.com/page\"})\n\
                    // Check status_code in response\n\n\
                 2. Navigate to anchor:\n\
                    browser_navigate({\"url\": \"https://example.com/docs#installation\"})\n\
                    // Scrolls to #installation section\n\n\
                 3. Navigate to subdomain:\n\
                    browser_navigate({\"url\": \"https://api.example.com\"})\n\n\
                 ERROR HANDLING:\n\
                 - 404: Page not found\n\
                 - 500: Server error\n\
                 - Timeout: Page took too long to load\n\
                 - DNS errors: Invalid domain\n\
                 - Check status_code and final_url in response",
            ),
        },
    ]
}
