//! Prompt messages for browser_extract_text tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::BrowserExtractTextPromptArgs;

/// Prompt provider for browser_extract_text tool
///
/// This is the ONLY way to provide prompts for browser_extract_text - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ExtractTextPrompts;

impl PromptProvider for ExtractTextPrompts {
    type PromptArgs = BrowserExtractTextPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("page_content") => prompt_page_content(),
            Some("specific_elements") => prompt_specific_elements(),
            _ => prompt_page_content(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Extraction scenario: page_content (basic text extraction), specific_elements (targeted by CSS selector)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Reading full page content vs targeted sections
fn prompt_page_content() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I extract page content effectively using browser_extract_text?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The browser_extract_text tool extracts visible text from page elements using CSS selectors. Start broad, then refine.\n\n\
                 READING PAGE CONTENT:\n\n\
                 1. Full page text (all visible content):\n\
                    browser_extract_text({})\n\
                    // Returns all visible text on the page\n\
                    // Includes navigation, headers, content, footers\n\n\
                 2. Main content area (excluding nav/footer):\n\
                    browser_extract_text({\"selector\": \"main\"})\n\
                    browser_extract_text({\"selector\": \"#content\"})\n\
                    browser_extract_text({\"selector\": \"article\"})\n\
                    // Returns only the main content section\n\n\
                 3. Multiple content sections:\n\
                    browser_extract_text({\"selector\": \"main, article, .content\"})\n\
                    // Comma-separated selectors extract from all matching elements\n\n\
                 4. Specific content container:\n\
                    browser_extract_text({\"selector\": \".post-content\"})\n\
                    browser_extract_text({\"selector\": \"#article-body\"})\n\
                    browser_extract_text({\"selector\": \"div.prose\"})\n\n\
                 WHEN TO READ FULL PAGE:\n\
                 - Understanding overall page structure\n\
                 - Finding where specific content is located\n\
                 - Debugging: \"What does the page actually show?\"\n\
                 - Searching for keywords across entire page\n\
                 - Initial page exploration\n\n\
                 WHEN TO USE SELECTORS:\n\
                 - Large pages where you only need a section\n\
                 - Avoiding repetitive navigation/footer text\n\
                 - Extracting specific content regions\n\
                 - When you know the page structure\n\
                 - Reducing noise in extraction results\n\n\
                 PROGRESSIVE REFINEMENT STRATEGY:\n\
                 1. Start broad:\n\
                    browser_extract_text({})\n\
                    // See everything, understand page structure\n\n\
                 2. Identify main content area:\n\
                    browser_extract_text({\"selector\": \"main\"})\n\
                    // Extract just the main content\n\n\
                 3. Further refine if needed:\n\
                    browser_extract_text({\"selector\": \"main .article-body\"})\n\
                    // Get specific subsection within main\n\n\
                 COMMON CONTENT SELECTORS:\n\
                 - Main content: main, article, #content, .content, .main\n\
                 - Article body: .article-body, .post-content, .entry-content\n\
                 - Documentation: .documentation, .docs-content, .doc-body\n\
                 - Blog posts: article.post, .blog-post, .entry\n\
                 - Product info: .product-details, .product-description\n\n\
                 HANDLING DYNAMIC CONTENT:\n\
                 If content is loaded dynamically:\n\
                 1. Navigate to the page\n\
                 2. Wait for content to load (use appropriate wait conditions)\n\
                 3. Extract after page is fully rendered\n\
                 4. Extract returns current page state\n\n\
                 EXCLUDING CONTENT:\n\
                 While you can't exclude with this tool directly, you can:\n\
                 - Target specific sections (main, not nav)\n\
                 - Use more specific selectors\n\
                 - Extract multiple sections separately\n\n\
                 Example workflow:\n\
                 browser_extract_text({\"selector\": \"main\"})\n\
                 // Gets main content without nav/footer\n\n\
                 READING STRATEGY:\n\
                 1. First visit: Extract full page to understand structure\n\
                 2. Identify: Find the selector for content you need\n\
                 3. Extract: Use specific selector for focused extraction\n\
                 4. Verify: Check if extraction captured what you needed\n\
                 5. Refine: Adjust selector if needed\n\n\
                 SELECTOR PRECEDENCE:\n\
                 More specific selectors override general ones:\n\
                 - #id (most specific)\n\
                 - .class\n\
                 - element (least specific)\n\
                 - Combinations: div.class#id (very specific)\n\n\
                 Remember: The tool extracts text content only, not HTML structure or attributes.",
            ),
        },
    ]
}

/// Targeting specific elements by ID, class, attribute
fn prompt_specific_elements() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I extract specific elements like titles, prices, or error messages?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Target specific elements using CSS selectors. Use the most specific selector that reliably identifies your target.\n\n\
                 EXTRACTING SPECIFIC ELEMENTS:\n\n\
                 1. Single element by ID (most reliable):\n\
                    browser_extract_text({\"selector\": \"#product-title\"})\n\
                    browser_extract_text({\"selector\": \"#price\"})\n\
                    browser_extract_text({\"selector\": \"#error-message\"})\n\
                    // IDs are unique - returns exactly one element\n\n\
                 2. Multiple elements by class:\n\
                    browser_extract_text({\"selector\": \".search-result\"})\n\
                    browser_extract_text({\"selector\": \".product-card\"})\n\
                    browser_extract_text({\"selector\": \".comment\"})\n\
                    // Returns text from ALL matching elements\n\
                    // Output: array of text strings\n\n\
                 3. Element with specific attribute:\n\
                    browser_extract_text({\"selector\": \"[data-testid='username']\"})\n\
                    browser_extract_text({\"selector\": \"[aria-label='Close']\"})\n\
                    browser_extract_text({\"selector\": \"[name='email']\"})\n\
                    // Attribute selectors are stable for testing\n\n\
                 4. Nested content (child selectors):\n\
                    browser_extract_text({\"selector\": \"#reviews .review-text\"})\n\
                    browser_extract_text({\"selector\": \".product-card .title\"})\n\
                    browser_extract_text({\"selector\": \"article .author-name\"})\n\
                    // Space = descendant selector (any level deep)\n\n\
                 5. Direct children only:\n\
                    browser_extract_text({\"selector\": \".container > .item\"})\n\
                    browser_extract_text({\"selector\": \"nav > a\"})\n\
                    // > = direct child only (not grandchildren)\n\n\
                 COMMON EXTRACTIONS BY TYPE:\n\n\
                 Page title:\n\
                 browser_extract_text({\"selector\": \"h1\"})\n\
                 browser_extract_text({\"selector\": \".page-title\"})\n\
                 browser_extract_text({\"selector\": \"#main-heading\"})\n\n\
                 Error messages:\n\
                 browser_extract_text({\"selector\": \".error\"})\n\
                 browser_extract_text({\"selector\": \".alert-danger\"})\n\
                 browser_extract_text({\"selector\": \".error-message, .validation-error\"})\n\
                 // Comma combines multiple selectors\n\n\
                 Success messages:\n\
                 browser_extract_text({\"selector\": \".alert-success\"})\n\
                 browser_extract_text({\"selector\": \".success-message\"})\n\
                 browser_extract_text({\"selector\": \".notification.success\"})\n\n\
                 Form field values:\n\
                 browser_extract_text({\"selector\": \"input[name='email']\"})\n\
                 browser_extract_text({\"selector\": \"#username\"})\n\
                 browser_extract_text({\"selector\": \"textarea#description\"})\n\n\
                 Buttons:\n\
                 browser_extract_text({\"selector\": \"button.submit\"})\n\
                 browser_extract_text({\"selector\": \".btn-primary\"})\n\
                 browser_extract_text({\"selector\": \"[type='submit']\"})\n\n\
                 Links:\n\
                 browser_extract_text({\"selector\": \"a.nav-link\"})\n\
                 browser_extract_text({\"selector\": \".breadcrumb a\"})\n\
                 browser_extract_text({\"selector\": \"nav a\"})\n\
                 // Returns link text, not href\n\n\
                 SELECTOR STRATEGIES:\n\n\
                 Start specific, broaden if needed:\n\
                 1. Try ID first (most specific):\n\
                    browser_extract_text({\"selector\": \"#error\"})\n\
                 2. Try class if no ID:\n\
                    browser_extract_text({\"selector\": \".error-message\"})\n\
                 3. Try attribute:\n\
                    browser_extract_text({\"selector\": \"[role='alert']\"})\n\
                 4. Try element + class:\n\
                    browser_extract_text({\"selector\": \"div.error\"})\n\n\
                 Combine selectors for multiple targets:\n\
                 browser_extract_text({\"selector\": \".error, .warning, .alert\"})\n\
                 // Gets all error, warning, and alert messages\n\n\
                 HANDLING MULTIPLE MATCHES:\n\
                 When selector matches multiple elements:\n\
                 browser_extract_text({\"selector\": \".product-title\"})\n\
                 // Returns array: [\"Product 1\", \"Product 2\", \"Product 3\"]\n\n\
                 To get specific instance, use pseudo-selector:\n\
                 browser_extract_text({\"selector\": \".product-title:first-child\"})\n\
                 // Returns just the first match\n\n\
                 DEBUGGING SELECTORS:\n\
                 If extraction returns nothing:\n\
                 1. Extract full page to see what's there\n\
                 2. Check if selector syntax is correct\n\
                 3. Try broader selector (remove class, keep element)\n\
                 4. Check if content is dynamically loaded\n\
                 5. Verify element is visible (not hidden)\n\n\
                 BEST PRACTICES:\n\
                 1. Prefer IDs for unique elements (#)\n\
                 2. Use data attributes for test stability\n\
                 3. Combine selectors when appropriate\n\
                 4. Start specific, broaden if no match\n\
                 5. Test selectors on actual pages\n\
                 6. Handle multiple matches appropriately\n\
                 7. Check if content is dynamically loaded",
            ),
        },
    ]
}
