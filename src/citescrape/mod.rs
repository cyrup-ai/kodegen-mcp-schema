//! Citescrape category module

// Re-export all citescrape tool name constants from kodegen_config
pub use kodegen_config::{FETCH, SCRAPE_URL, WEB_SEARCH};

pub mod fetch;
pub mod scrape_url;
pub mod web_search;

// Re-export scrape_url tool
pub use scrape_url::{
    ScrapeUrlArgs,
    ScrapeUrlOutput,
    ScrapeUrlPromptArgs,
    ScrapeUrlPrompts,
    ScrapeAction,
    CrawlSnapshot,
    ScrapeSearchResult,
};

// Re-export web_search tool
pub use web_search::{
    WebSearchArgs,
    WebSearchOutput,
    WebSearchPromptArgs,
    WebSearchPrompts,
    WebSearchResultItem,
};

// Re-export fetch tool
pub use fetch::{
    FetchArgs,
    FetchOutput,
    FetchPromptArgs,
    FetchPrompts,
};
