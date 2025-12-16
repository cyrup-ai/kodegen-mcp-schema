//! Browser automation and web research tools

// Re-export all browser tool name constants from kodegen_config
pub use kodegen_config::{
    BROWSER_AGENT, BROWSER_CLICK, BROWSER_EXTRACT_TEXT, BROWSER_NAVIGATE,
    BROWSER_RESEARCH, BROWSER_SCREENSHOT, BROWSER_SCROLL, BROWSER_TYPE_TEXT,
};

pub mod shared;
pub mod agent;
pub mod research;
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
    BrowserResearchAction,
    BrowserResearchArgs,
    BrowserResearchOutput,
    ResearchSource,
    BrowserResearchPromptArgs,
    ResearchPrompts,
};

// Re-export navigate tool
pub use navigate::{
    BrowserNavigateArgs,
    BrowserNavigateOutput,
    BrowserNavigatePromptArgs,
    NavigatePrompts,
};

// Re-export click tool
pub use click::{
    BrowserClickArgs,
    BrowserClickOutput,
    BrowserClickPromptArgs,
    ClickPrompts,
};

// Re-export type_text tool
pub use type_text::{
    BrowserTypeTextArgs,
    BrowserTypeOutput,
    BrowserTypeTextPromptArgs,
    TypeTextPrompts,
};

// Re-export scroll tool
pub use scroll::{
    BrowserScrollArgs,
    BrowserScrollOutput,
    BrowserScrollPromptArgs,
    ScrollPrompts,
};

// Re-export screenshot tool
pub use screenshot::{
    BrowserScreenshotArgs,
    BrowserScreenshotOutput,
    BrowserScreenshotPromptArgs,
    ScreenshotPrompts,
};

// Re-export extract_text tool
pub use extract_text::{
    BrowserExtractTextArgs,
    BrowserExtractTextOutput,
    BrowserExtractTextPromptArgs,
    ExtractTextPrompts,
};
