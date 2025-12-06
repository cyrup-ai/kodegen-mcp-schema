//! Prompt messages for browser_type_text tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::BrowserTypeTextPromptArgs;

/// Prompt provider for browser_type_text tool
///
/// This is the ONLY way to provide prompts for browser_type_text - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct TypeTextPrompts;

impl PromptProvider for TypeTextPrompts {
    type PromptArgs = BrowserTypeTextPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        // All scenarios default to basic - single comprehensive scenario
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

/// Basic text input patterns
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I type text into input fields and textareas using browser_type_text?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The browser_type_text tool types text into input elements. Here's how to use it for basic text input:\n\n\
                 BASIC TEXT INPUT:\n\n\
                 1. Type into input by selector:\n\
                    browser_type_text({\"selector\": \"#search\", \"text\": \"rust async tutorial\"})\n\n\
                 2. Type into input by name:\n\
                    browser_type_text({\"selector\": \"input[name='email']\", \"text\": \"user@example.com\"})\n\n\
                 3. Type into textarea:\n\
                    browser_type_text({\"selector\": \"textarea#message\", \"text\": \"Hello, this is my message.\"})\n\n\
                 4. Type into contenteditable:\n\
                    browser_type_text({\"selector\": \"div[contenteditable='true']\", \"text\": \"Editable content\"})\n\n\
                 SELECTOR STRATEGIES FOR INPUTS:\n\
                 - By ID: #email, #password (most reliable)\n\
                 - By name: input[name='username']\n\
                 - By placeholder: input[placeholder='Search']\n\
                 - By label: Often need to find associated input\n\
                 - By type: input[type='email'], input[type='text']\n\
                 - By class: .search-input, .form-control\n\
                 - By CSS path: form#login input[type='email']\n\n\
                 COMMON INPUT SELECTORS:\n\
                 - Search: #search, input[type='search'], .search-input\n\
                 - Email: #email, input[type='email'], input[name='email']\n\
                 - Password: #password, input[type='password']\n\
                 - Username: #username, input[name='username'], input[name='login']\n\
                 - Message/Comments: textarea#message, textarea[name='comment']\n\
                 - Phone: input[type='tel'], input[name='phone']\n\
                 - URL: input[type='url'], input[name='website']\n\n\
                 INPUT TYPES SUPPORTED:\n\
                 - text: Standard text input\n\
                 - email: Email addresses\n\
                 - password: Password fields (text is hidden)\n\
                 - search: Search queries\n\
                 - tel: Telephone numbers\n\
                 - url: Website addresses\n\
                 - textarea: Multi-line text\n\
                 - contenteditable: Rich text editors\n\
                 - number: Numeric values (type as string)\n\
                 - date: Date strings (format: YYYY-MM-DD)\n\n\
                 SELECTOR BEST PRACTICES:\n\
                 1. Prefer IDs when available (#username)\n\
                 2. Use name attributes for form fields (input[name='email'])\n\
                 3. Combine type with other attributes (input[type='email'][name='user_email'])\n\
                 4. Use specific CSS paths for ambiguous elements (form.login input[type='text'])\n\
                 5. Avoid fragile selectors like nth-child unless necessary\n\n\
                 TIMING AND WAITING:\n\
                 The tool automatically waits for elements to be:\n\
                 - Present in DOM\n\
                 - Visible on page\n\
                 - Enabled (not disabled)\n\
                 - Interactable (not covered by other elements)\n\n\
                 Default timeout is 30 seconds. For slow-loading pages, increase timeout:\n\
                 browser_type_text({\"selector\": \"#dynamic-input\", \"text\": \"content\", \"timeout_ms\": 60000})\n\n\
                 WHAT HAPPENS WHEN YOU TYPE:\n\
                 1. Element is located using selector\n\
                 2. Element is scrolled into view if needed\n\
                 3. Element is clicked to focus\n\
                 4. Existing text is cleared (by default)\n\
                 5. New text is typed character by character\n\
                 6. Input events are triggered (input, change, keydown, keyup)\n\
                 7. Any JavaScript handlers are activated naturally\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"selector\": \"#email\",\n\
                   \"text_length\": 17,\n\
                   \"message\": \"Successfully typed 17 characters into input#email\"\n\
                 }\n\n\
                 ERROR HANDLING:\n\
                 - Element not found: Check selector syntax\n\
                 - Element not visible: Wait for page to load or scroll\n\
                 - Element disabled: Cannot type into disabled inputs\n\
                 - Timeout: Increase timeout_ms parameter",
            ),
        },
    ]
}
