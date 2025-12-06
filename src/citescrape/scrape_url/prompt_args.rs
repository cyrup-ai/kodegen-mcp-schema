//! Prompt argument types for scrape_url tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt arguments for scrape_url tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScrapeUrlPromptArgs {
    /// Scenario to show examples for
    /// - "basic": Basic URL scraping
    /// - "crawling": Multi-page crawling
    /// - "search": Searching crawled content
    /// - "background": Background crawl management
    /// - "extraction": Content extraction options
    /// - "comprehensive": All scenarios combined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
}
