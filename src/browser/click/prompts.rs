//! Prompt messages for browser_click tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::BrowserClickPromptArgs;

/// Prompt provider for browser_click tool
///
/// This is the ONLY way to provide prompts for browser_click - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ClickPrompts;

impl PromptProvider for ClickPrompts {
    type PromptArgs = BrowserClickPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("selectors") => prompt_selectors(),
            _ => prompt_waiting(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (selectors, waiting)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO CLICK ELEMENTS
// ============================================================================

/// CSS selector patterns for targeting different elements
fn prompt_selectors() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I select elements to click using CSS selectors?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "CSS selectors are the primary way to target clickable elements. Choose selectors based on reliability and specificity.\n\n\
                 SELECTOR PATTERNS:\n\n\
                 1. BY ID (MOST RELIABLE):\n\
                 browser_click({\"selector\": \"#submit-button\"})\n\
                 browser_click({\"selector\": \"#login-form button\"})\n\n\
                 2. BY ATTRIBUTE (RECOMMENDED FOR TESTING):\n\
                 browser_click({\"selector\": \"[data-testid='submit']\"})\n\
                 browser_click({\"selector\": \"[aria-label='Close']\"})\n\
                 browser_click({\"selector\": \"button[type='submit']\"})\n\n\
                 3. BY CLASS:\n\
                 browser_click({\"selector\": \".btn-primary\"})\n\
                 browser_click({\"selector\": \".btn.btn-primary.active\"})\n\n\
                 4. BY TEXT CONTENT:\n\
                 browser_click({\"selector\": \"button:contains('Submit')\"})\n\
                 Warning: Breaks with internationalization!\n\n\
                 5. HIERARCHICAL SELECTORS:\n\
                 browser_click({\"selector\": \"form > button\"})\n\
                 browser_click({\"selector\": \"#login-form .submit-btn\"})\n\n\
                 6. NTH-CHILD:\n\
                 browser_click({\"selector\": \"ul li:nth-child(3) a\"})\n\
                 browser_click({\"selector\": \"table tr:nth-child(2) button\"})\n\n\
                 SELECTOR PRIORITY (MOST TO LEAST RELIABLE):\n\
                 1. data-testid or data-cy attributes → Designed for testing\n\
                 2. id attributes (#unique-id) → Should be unique on page\n\
                 3. aria-label attributes → Accessibility-friendly\n\
                 4. Unique class names → If stable across versions\n\
                 5. Element hierarchy → Structure-dependent\n\
                 6. Text content → Can break with i18n/translations\n\n\
                 BEST PRACTICES:\n\
                 ✓ Prefer data-testid: browser_click({\"selector\": \"[data-testid='submit']\"})\n\
                 ✓ Use ID when available: browser_click({\"selector\": \"#login-button\"})\n\
                 ✓ Be specific to avoid multiple matches\n\
                 ✗ Avoid brittle selectors like: div > div > div > button",
            ),
        },
    ]
}

/// Handling dynamic content and timing
fn prompt_waiting() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I handle clicking elements that load dynamically or need timing?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Browser interactions require waiting for elements to appear or become clickable. The browser_click tool handles this automatically.\n\n\
                 AUTOMATIC WAITING:\n\
                 The tool automatically waits for:\n\
                 - Element to exist in DOM\n\
                 - Element to be visible (not hidden)\n\
                 - Element to be enabled (not disabled)\n\
                 - Element to be in viewport (scrolls if needed)\n\
                 - Overlays/modals to clear (if covering element)\n\n\
                 WAIT PARAMETERS:\n\n\
                 1. wait_timeout_ms - Maximum time to wait:\n\
                 browser_click({\"selector\": \"#button\", \"wait_timeout_ms\": 5000})\n\n\
                 2. wait_for_clickable - Wait until enabled and not covered:\n\
                 browser_click({\"selector\": \".loading-button\", \"wait_for_clickable\": true})\n\n\
                 3. wait_for_navigation - Wait for page transition:\n\
                 browser_click({\"selector\": \"a.next-page\", \"wait_for_navigation\": true})\n\n\
                 COMMON SCENARIOS:\n\n\
                 AJAX-loaded buttons:\n\
                 browser_click({\"selector\": \".load-more-button\"})\n\n\
                 Modal animations:\n\
                 browser_click({\"selector\": \".modal .confirm-button\", \"wait_for_clickable\": true})\n\n\
                 Dropdown menus:\n\
                 browser_click({\"selector\": \".dropdown-toggle\"})\n\
                 browser_click({\"selector\": \".dropdown-menu .item-3\", \"wait_timeout_ms\": 2000})\n\n\
                 Loading spinners:\n\
                 browser_click({\"selector\": \".save-button\", \"wait_for_clickable\": true, \"wait_timeout_ms\": 10000})\n\n\
                 Slow API calls:\n\
                 browser_click({\"selector\": \".slow-api-button\", \"wait_timeout_ms\": 20000})\n\n\
                 Form submissions:\n\
                 browser_click({\"selector\": \"button[type='submit']\", \"wait_for_navigation\": true})\n\n\
                 BEST PRACTICES:\n\
                 ✓ Let tool handle automatic waiting (default behavior)\n\
                 ✓ Use wait_for_clickable for dynamic enabled/disabled states\n\
                 ✓ Use wait_for_navigation for page transitions\n\
                 ✗ Don't manually sleep/wait before clicking",
            ),
        },
    ]
}


