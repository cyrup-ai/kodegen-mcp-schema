//! Citescrape category module

pub mod fetch;
pub mod scrape_url;
pub mod web_search;

// Re-export scrape_url tool
pub use scrape_url::{
    SCRAPE_URL,
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
    WEB_SEARCH,
    WebSearchArgs,
    WebSearchOutput,
    WebSearchPromptArgs,
    WebSearchPrompts,
    WebSearchResultItem,
};

// Re-export fetch tool
pub use fetch::{
    FETCH,
    FetchArgs,
    FetchOutput,
    FetchPromptArgs,
    FetchPrompts,
};
