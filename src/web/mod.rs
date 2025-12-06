//! Web tools

pub mod scrape_url;
pub mod web_search;

// Re-export scrape_url tool types
pub use scrape_url::{
    ScrapeUrlPromptArgs,
    ScrapeUrlPrompts,
};

// Re-export web_search tool types
pub use web_search::{
    WebSearchPromptArgs,
    WebSearchPrompts,
};
