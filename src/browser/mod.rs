//! Browser automation and web research tools

pub mod shared;
pub mod agent;
pub mod research;
pub mod web_search;
pub mod navigate;
pub mod click;
pub mod type_text;
pub mod scroll;
pub mod screenshot;
pub mod extract_text;

// Re-export all tool types for convenient access
pub use shared::*;

// Re-export agent tool
pub use agent::{
    BROWSER_AGENT,
    BrowserAgentAction,
    BrowserAgentArgs,
    BrowserAgentOutput,
    BrowserAgentStepInfo,
    BrowserAgentKillOutput,
    BrowserAgentPromptArgs,
    AgentPrompts,
};

// Re-export research tool
pub use research::{
    BROWSER_RESEARCH,
    BrowserResearchAction,
    BrowserResearchArgs,
    BrowserResearchOutput,
    ResearchSource,
    BrowserResearchPromptArgs,
    ResearchPrompts,
};

// Re-export web_search tool
pub use web_search::{
    BROWSER_WEB_SEARCH,
    BrowserWebSearchArgs,
    BrowserWebSearchOutput,
    WebSearchResult,
    BrowserWebSearchPromptArgs,
    WebSearchPrompts,
};

// Re-export navigate tool
pub use navigate::{
    BROWSER_NAVIGATE,
    BrowserNavigateArgs,
    BrowserNavigateOutput,
    BrowserNavigatePromptArgs,
    NavigatePrompts,
};

// Re-export click tool
pub use click::{
    BROWSER_CLICK,
    BrowserClickArgs,
    BrowserClickOutput,
    BrowserClickPromptArgs,
    ClickPrompts,
};

// Re-export type_text tool
pub use type_text::{
    BROWSER_TYPE_TEXT,
    BrowserTypeTextArgs,
    BrowserTypeOutput,
    BrowserTypeTextPromptArgs,
    TypeTextPrompts,
};

// Re-export scroll tool
pub use scroll::{
    BROWSER_SCROLL,
    BrowserScrollArgs,
    BrowserScrollOutput,
    BrowserScrollPromptArgs,
    ScrollPrompts,
};

// Re-export screenshot tool
pub use screenshot::{
    BROWSER_SCREENSHOT,
    BrowserScreenshotArgs,
    BrowserScreenshotOutput,
    BrowserScreenshotPromptArgs,
    ScreenshotPrompts,
};

// Re-export extract_text tool
pub use extract_text::{
    BROWSER_EXTRACT_TEXT,
    BrowserExtractTextArgs,
    BrowserExtractTextOutput,
    BrowserExtractTextPromptArgs,
    ExtractTextPrompts,
};
