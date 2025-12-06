//! Prompt messages for github_search_issues tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::SearchIssuesPromptArgs;

/// Prompt provider for search_issues tool
///
/// This is the ONLY way to provide prompts for search_issues - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct SearchIssuesPrompts;

impl PromptProvider for SearchIssuesPrompts {
    type PromptArgs = SearchIssuesPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("syntax") => prompt_syntax(),
            Some("patterns") => prompt_patterns(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, syntax, patterns)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO SEARCH GITHUB ISSUES
// ============================================================================

/// Basic issue search operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I search for GitHub issues using github_search_issues?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_search_issues tool searches issues and pull requests across GitHub using powerful query syntax. Here's how to use it for basic searches:\n\n\
                 BASIC ISSUE SEARCH:\n\
                 1. Simple keyword search:\n\
                    github_search_issues({\"query\": \"memory leak\"})\n\
                    // Searches all issues containing 'memory leak'\n\n\
                 2. Search in specific repository:\n\
                    github_search_issues({\"query\": \"memory leak repo:rust-lang/rust\"})\n\
                    // Searches only in rust-lang/rust repository\n\n\
                 3. Search by state:\n\
                    github_search_issues({\"query\": \"is:issue is:open label:bug\"})\n\
                    // Finds open issues with 'bug' label\n\n\
                 4. Search with sort and pagination:\n\
                    github_search_issues({\n\
                       \"query\": \"authentication error\",\n\
                       \"sort\": \"created\",\n\
                       \"order\": \"desc\",\n\
                       \"per_page\": 50\n\
                    })\n\
                    // Returns 50 most recently created results\n\n\
                 PARAMETERS:\n\
                 - query (required): Search query using GitHub search syntax\n\
                 - sort (optional): \"comments\", \"reactions\", \"created\", \"updated\" (default: best match)\n\
                 - order (optional): \"asc\" or \"desc\" (default: \"desc\")\n\
                 - page (optional): Page number for pagination (default: 1)\n\
                 - per_page (optional): Results per page, max 100 (default: 30)\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"total_count\": 42,         // Total matching results\n\
                   \"items\": [\n\
                     {\n\
                       \"number\": 123,          // Issue/PR number\n\
                       \"title\": \"Memory leak in handler\",\n\
                       \"state\": \"open\",        // \"open\" or \"closed\"\n\
                       \"repository_url\": \"https://api.github.com/repos/owner/repo\",\n\
                       \"html_url\": \"https://github.com/owner/repo/issues/123\",\n\
                       \"user\": {              // Author info\n\
                         \"login\": \"username\"\n\
                       },\n\
                       \"labels\": [{           // Array of labels\n\
                         \"name\": \"bug\"\n\
                       }],\n\
                       \"created_at\": \"2024-01-15T10:30:00Z\",\n\
                       \"updated_at\": \"2024-01-20T14:45:00Z\",\n\
                       \"comments\": 5\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 COMMON BASIC SEARCHES:\n\
                 1. Find bugs:\n\
                    github_search_issues({\"query\": \"is:issue is:open label:bug\"})\n\n\
                 2. Find your issues:\n\
                    github_search_issues({\"query\": \"is:issue author:your-username\"})\n\n\
                 3. Find unassigned issues:\n\
                    github_search_issues({\"query\": \"is:issue is:open no:assignee\"})\n\n\
                 4. Find issues by keyword:\n\
                    github_search_issues({\"query\": \"database connection timeout\"})\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with any valid scope.\n\n\
                 PAGINATION:\n\
                 For more results, use page parameter:\n\
                 github_search_issues({\"query\": \"bug\", \"page\": 2, \"per_page\": 50})\n\
                 Max 100 results per page, up to 1000 total results per query.\n\n\
                 BEST PRACTICES:\n\
                 - Use is:issue or is:pr to specify type\n\
                 - Combine with repo: for targeted searches\n\
                 - Use sort and order for organized results\n\
                 - Respect rate limits: 30 requests/minute for search API\n\
                 - Use specific queries to reduce result count",
            ),
        },
    ]
}

/// Search syntax reference guide
fn prompt_syntax() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What search syntax and qualifiers can I use with github_search_issues?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "GitHub issue search supports powerful query syntax with many qualifiers for filtering. Here's a complete reference:\n\n\
                 SEARCH QUALIFIERS:\n\n\
                 TYPE QUALIFIERS:\n\
                 - is:issue           Only issues (not PRs)\n\
                 - is:pr              Only pull requests\n\
                 - type:issue         Alternative syntax for is:issue\n\
                 - type:pr            Alternative syntax for is:pr\n\n\
                 STATE QUALIFIERS:\n\
                 - is:open            Open issues/PRs\n\
                 - is:closed          Closed issues/PRs\n\
                 - is:merged          Merged PRs only\n\
                 - is:unmerged        Unmerged PRs\n\
                 - is:draft           Draft PRs\n\
                 - state:open         Alternative syntax\n\
                 - state:closed       Alternative syntax\n\n\
                 REPOSITORY QUALIFIERS:\n\
                 - repo:owner/name    Specific repository\n\
                 - user:username      User's repositories\n\
                 - org:orgname        Organization's repositories\n\n\
                 AUTHOR & ASSIGNEE:\n\
                 - author:username    Created by user\n\
                 - assignee:username  Assigned to user\n\
                 - mentions:username  User mentioned\n\
                 - commenter:username User commented\n\
                 - involves:username  Author, assignee, mentions, or commenter\n\
                 - no:assignee        Not assigned to anyone\n\n\
                 LABEL QUALIFIERS:\n\
                 - label:bug          Has 'bug' label\n\
                 - label:\"good first issue\"  Label with spaces (use quotes)\n\
                 - -label:wontfix     Does NOT have 'wontfix' label\n\
                 - no:label           No labels at all\n\n\
                 DATE QUALIFIERS:\n\
                 - created:>2024-01-01        Created after date\n\
                 - created:<2024-01-01        Created before date\n\
                 - created:2024-01-01         Created on exact date\n\
                 - created:2024-01..2024-06   Date range\n\
                 - updated:>2024-01-01        Updated after date\n\
                 - closed:>2024-01-01         Closed after date\n\
                 - merged:2024-01-01..2024-06 Merged in range\n\n\
                 RELATIVE DATES:\n\
                 - created:>yesterday         Yesterday or later\n\
                 - updated:<1week             More than week ago\n\
                 - closed:>=2024-01-01        On or after date\n\n\
                 METADATA QUALIFIERS:\n\
                 - comments:>5        More than 5 comments\n\
                 - comments:10        Exactly 10 comments\n\
                 - comments:10..20    Between 10 and 20 comments\n\
                 - reactions:>10      More than 10 reactions\n\
                 - interactions:>50   More than 50 total interactions\n\n\
                 MILESTONE & PROJECT:\n\
                 - milestone:\"v1.0\"   In milestone\n\
                 - project:myproject  In project board\n\
                 - no:milestone       No milestone\n\
                 - no:project         No project\n\n\
                 STATUS CHECKS (PRs):\n\
                 - status:success     All checks passed\n\
                 - status:failure     Some checks failed\n\
                 - status:pending     Checks pending\n\n\
                 REVIEW STATUS (PRs):\n\
                 - review:required    Review required\n\
                 - review:approved    Approved by reviewer\n\
                 - review:changes_requested  Changes requested\n\
                 - review-requested:username  Review requested from user\n\
                 - reviewed-by:username       Reviewed by user\n\n\
                 TEXT SEARCH:\n\
                 - \"exact phrase\"    Match exact phrase (use quotes)\n\
                 - word1 word2        Match all words (AND)\n\
                 - word1 OR word2     Match any word\n\
                 - -word              Exclude word\n\n\
                 SEARCH IN SPECIFIC FIELDS:\n\
                 - in:title           Search in title only\n\
                 - in:body            Search in body only\n\
                 - in:comments        Search in comments\n\n\
                 COMBINING QUALIFIERS:\n\
                 Use spaces to combine (AND logic):\n\
                 github_search_issues({\n\
                   \"query\": \"is:issue is:open label:bug repo:user/project created:>2024-01-01\"\n\
                 })\n\n\
                 EXAMPLE QUERIES:\n\
                 1. Open bugs in repository:\n\
                    github_search_issues({\"query\": \"is:issue is:open label:bug repo:user/project\"})\n\n\
                 2. Your assigned issues:\n\
                    github_search_issues({\"query\": \"is:issue is:open assignee:your-username\"})\n\n\
                 3. Recent issues needing attention:\n\
                    github_search_issues({\"query\": \"is:issue created:>2024-01-01 comments:0 repo:user/project\"})\n\n\
                 4. Stale open issues:\n\
                    github_search_issues({\"query\": \"is:issue is:open updated:<2023-06-01 repo:user/project\"})\n\n\
                 5. High-priority bugs:\n\
                    github_search_issues({\"query\": \"is:issue is:open label:bug label:priority-high\"})\n\n\
                 6. PRs awaiting your review:\n\
                    github_search_issues({\"query\": \"is:pr is:open review-requested:your-username\"})\n\n\
                 7. Merged PRs in date range:\n\
                    github_search_issues({\"query\": \"is:pr is:merged merged:2024-01-01..2024-01-31\"})\n\n\
                 TIPS:\n\
                 - Combine qualifiers for precise filtering\n\
                 - Use quotes for multi-word labels or phrases\n\
                 - Date ranges are inclusive\n\
                 - Relative dates like 'yesterday' are supported\n\
                 - Use - prefix to exclude matches\n\
                 - Case-insensitive search by default",
            ),
        },
    ]
}

/// Common search patterns and use cases
fn prompt_patterns() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common search patterns I should know for github_search_issues?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Here are proven search patterns for common GitHub issue management tasks:\n\n\
                 TRIAGE PATTERNS:\n\n\
                 1. Unlabeled issues (need triage):\n\
                    github_search_issues({\n\
                      \"query\": \"is:issue is:open no:label repo:user/project\"\n\
                    })\n\
                    // Find issues that need categorization\n\n\
                 2. Unassigned bugs:\n\
                    github_search_issues({\n\
                      \"query\": \"is:issue is:open label:bug no:assignee repo:user/project\"\n\
                    })\n\
                    // Find bugs ready for assignment\n\n\
                 3. No comments (need attention):\n\
                    github_search_issues({\n\
                      \"query\": \"is:issue is:open comments:0 created:>1week repo:user/project\"\n\
                    })\n\
                    // Find issues with no responses\n\n\
                 PRIORITY MANAGEMENT:\n\n\
                 4. High-priority issues:\n\
                    github_search_issues({\n\
                      \"query\": \"is:issue is:open label:priority-high repo:user/project\",\n\
                      \"sort\": \"created\",\n\
                      \"order\": \"asc\"\n\
                    })\n\
                    // Oldest high-priority issues first\n\n\
                 PERSONAL WORKFLOW:\n\n\
                 5. Your assigned work:\n\
                    github_search_issues({\n\
                      \"query\": \"is:issue is:open assignee:your-username\",\n\
                      \"sort\": \"updated\",\n\
                      \"order\": \"desc\"\n\
                    })\n\
                    // Your assigned issues\n\n\
                 6. PRs awaiting your review:\n\
                    github_search_issues({\n\
                      \"query\": \"is:pr is:open review-requested:your-username\",\n\
                      \"sort\": \"created\",\n\
                      \"order\": \"asc\"\n\
                    })\n\
                    // Oldest review requests first\n\n\
                 TIPS FOR EFFECTIVE PATTERNS:\n\
                 - Combine sort with patterns for prioritization\n\
                 - Use date filters to focus on recent activity\n\
                 - Layer multiple labels for precision\n\
                 - Exclude labels with - to filter out categories\n\
                 - Use reactions/comments for popularity signals\n\
                 - Save common patterns as search bookmarks",
            ),
        },
    ]
}
