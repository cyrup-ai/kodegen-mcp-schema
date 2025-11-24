//! GitHub tools Args types
//!
//! This module contains all Args and PromptArgs types for GitHub tools.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// TOOL NAME CONSTANTS
// ============================================================================
// Define each tool name exactly once to eliminate string duplication across
// tool implementations and metadata. These constants are imported by:
// - kodegen-tools-github/src/tool/*.rs (for Tool::name() implementation)
// - kodegen/src/stdio/metadata/category_metadata/version_control.rs (for routing)

// Issue Operations
pub const GITHUB_ADD_ISSUE_COMMENT: &str = "github_add_issue_comment";
pub const GITHUB_CREATE_ISSUE: &str = "github_create_issue";
pub const GITHUB_GET_ISSUE: &str = "github_get_issue";
pub const GITHUB_GET_ISSUE_COMMENTS: &str = "github_get_issue_comments";
pub const GITHUB_LIST_ISSUES: &str = "github_list_issues";
pub const GITHUB_SEARCH_ISSUES: &str = "github_search_issues";
pub const GITHUB_UPDATE_ISSUE: &str = "github_update_issue";

// Pull Request Operations
pub const GITHUB_ADD_PULL_REQUEST_REVIEW_COMMENT: &str = "github_add_pull_request_review_comment";
pub const GITHUB_CREATE_PULL_REQUEST: &str = "github_create_pull_request";
pub const GITHUB_CREATE_PULL_REQUEST_REVIEW: &str = "github_create_pull_request_review";
pub const GITHUB_GET_PULL_REQUEST_FILES: &str = "github_get_pull_request_files";
pub const GITHUB_GET_PULL_REQUEST_REVIEWS: &str = "github_get_pull_request_reviews";
pub const GITHUB_GET_PULL_REQUEST_STATUS: &str = "github_get_pull_request_status";
pub const GITHUB_LIST_PULL_REQUESTS: &str = "github_list_pull_requests";
pub const GITHUB_MERGE_PULL_REQUEST: &str = "github_merge_pull_request";
pub const GITHUB_REQUEST_COPILOT_REVIEW: &str = "github_request_copilot_review";
pub const GITHUB_UPDATE_PULL_REQUEST: &str = "github_update_pull_request";

// Repository Operations
pub const GITHUB_CREATE_BRANCH: &str = "github_create_branch";
pub const GITHUB_CREATE_OR_UPDATE_FILE: &str = "github_create_or_update_file";
pub const GITHUB_CREATE_REPOSITORY: &str = "github_create_repository";
pub const GITHUB_DELETE_BRANCH: &str = "github_delete_branch";
pub const GITHUB_FORK_REPOSITORY: &str = "github_fork_repository";
pub const GITHUB_GET_COMMIT: &str = "github_get_commit";
pub const GITHUB_GET_FILE_CONTENTS: &str = "github_get_file_contents";
pub const GITHUB_LIST_BRANCHES: &str = "github_list_branches";
pub const GITHUB_LIST_COMMITS: &str = "github_list_commits";
pub const GITHUB_PUSH_FILES: &str = "github_push_files";

// Search Operations
pub const GITHUB_SEARCH_CODE: &str = "github_search_code";
pub const GITHUB_SEARCH_REPOSITORIES: &str = "github_search_repositories";
pub const GITHUB_SEARCH_USERS: &str = "github_search_users";

// Security Operations
pub const GITHUB_CODE_SCANNING_ALERTS: &str = "github_code_scanning_alerts";
pub const GITHUB_SECRET_SCANNING_ALERTS: &str = "github_secret_scanning_alerts";

// User Operations
pub const GITHUB_GET_ME: &str = "github_get_me";

// ============================================================================
// CREATE_PULL_REQUEST
// ============================================================================

/// Arguments for creating a pull request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePullRequestArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Title of the pull request
    pub title: String,
    /// Body/description of the pull request (optional)
    #[serde(default)]
    pub body: Option<String>,
    /// The name of the branch where your changes are implemented (head branch)
    pub head: String,
    /// The name of the branch you want the changes pulled into (base branch)
    pub base: String,
    /// Whether to create the pull request as a draft (optional, defaults to false)
    #[serde(default)]
    pub draft: Option<bool>,
    /// Whether maintainers can modify the pull request (optional, defaults to true)
    #[serde(default)]
    pub maintainer_can_modify: Option<bool>,
}

// ============================================================================
// CREATE_ISSUE
// ============================================================================

/// Arguments for `create_issue` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateIssueArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Issue title
    pub title: String,
    /// Issue body/description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Labels to apply (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Assignees (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
}

/// Prompt arguments for `create_issue` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateIssuePromptArgs {
    /// Focus area for teaching examples: "basic", "labels", "assignees", "authentication", "team-collaboration"
    /// Use this to get examples tailored to specific aspects of the tool
    #[serde(default)]
    pub focus_area: Option<String>,

    /// Repository context for examples: "personal", "team", "open-source"
    /// Helps tailor examples to different collaboration scenarios
    #[serde(default)]
    pub use_case: Option<String>,
}

// ============================================================================
// LIST_ISSUES
// ============================================================================

/// Arguments for `list_issues` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListIssuesArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Filter by state: "open", "closed", or "all" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Filter by labels (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Filter by assignee username (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    /// Page number for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page, max 100 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

/// Prompt arguments for `list_issues` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListIssuesPromptArgs {
    /// Focus area for teaching: "filtering" (state/labels/assignee), "pagination" (page/per_page), 
    /// "advanced" (combined filters), or "all" (comprehensive examples)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_area: Option<String>,
}

// ============================================================================
// LIST_PULL_REQUESTS
// ============================================================================

/// Arguments for `list_pull_requests` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListPullRequestsArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Filter by state: "open", "closed", or "all" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Filter by labels (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Page number for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page, max 100 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

/// Prompt arguments for `list_pull_requests` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListPullRequestsPromptArgs {
    /// Focus area for teaching: "overview", "filtering", "pagination", "advanced"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_area: Option<String>,
}

// ============================================================================
// GET_ISSUE
// ============================================================================

/// Arguments for `get_issue` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetIssueArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Issue number
    pub issue_number: u64,
}

/// Prompt arguments for `get_issue` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetIssuePromptArgs {
    /// What aspect to focus teaching on: "basic" (minimal usage), "advanced" (complex patterns), "pr" (pull request specific)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_focus: Option<String>,
}

// ============================================================================
// UPDATE_ISSUE
// ============================================================================

/// Arguments for `update_issue` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateIssueArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Issue number to update
    pub issue_number: u64,
    /// New title (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// New body/description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// New state: "open" or "closed" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Replace labels (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Replace assignees (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
}

/// Prompt arguments for `update_issue` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateIssuePromptArgs {
    /// Optional scope: focus on specific update types
    /// Examples: "state" (open/closed), "labels", "assignees", "title_body", or "all" (default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// Optional detail level: "basic" for simple examples, "advanced" for edge cases
    /// "basic" (default): Single example per update type
    /// "advanced": Multiple scenarios including partial updates and edge cases
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_level: Option<String>,

    /// Optional include_warnings: whether to include important notes about field behavior
    /// Default is true. Emphasizes that labels/assignees are replacement operations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_warnings: Option<bool>,
}

// ============================================================================
// ADD_ISSUE_COMMENT
// ============================================================================

/// Arguments for `add_issue_comment` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddIssueCommentArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Issue number to comment on
    pub issue_number: u64,
    /// Comment text (Markdown supported)
    pub body: String,
}

/// Prompt arguments for `add_issue_comment` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddIssueCommentPromptArgs {
    /// Optional comment style/type to focus examples on
    /// Examples: "acknowledgment", "suggestion", "summary", "feedback", "question"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_style: Option<String>,

    /// Optional features to emphasize in examples
    /// Examples: "markdown", "mentions", "references", "reactions", "all"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_features: Option<String>,
}

// ============================================================================
// GET_ISSUE_COMMENTS
// ============================================================================

/// Arguments for `get_issue_comments` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetIssueCommentsArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Issue number to fetch comments for
    pub issue_number: u64,
}

/// Prompt arguments for `get_issue_comments` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetIssueCommentsPromptArgs {}

// ============================================================================
// CREATE_PULL_REQUEST_REVIEW
// ============================================================================

/// Arguments for `create_pull_request_review` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePullRequestReviewArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pull_number: u64,
    /// Review action: "APPROVE", "REQUEST_CHANGES", or "COMMENT"
    pub event: String,
    /// Review comment/body text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Specific commit SHA to review (optional, defaults to latest)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
}

/// Prompt arguments for `create_pull_request_review` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePullRequestReviewPromptArgs {
    /// Focus area for teaching: "approve", "request_changes", "comment", or "general"
    /// Tailor examples and explanations to specific review workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_area: Option<String>,

    /// Skill level for explanations: "beginner", "intermediate", or "advanced"
    /// Adjusts depth of explanation and complexity of examples
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_level: Option<String>,
}

// ============================================================================
// ADD_PULL_REQUEST_REVIEW_COMMENT
// ============================================================================

/// Arguments for `add_pull_request_review_comment` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddPullRequestReviewCommentArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pull_number: u64,
    /// Comment body text
    pub body: String,
    /// Commit SHA to comment on (required for new comments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    /// File path to comment on (required for new comments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Line number in the diff to comment on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    /// Side of diff: "LEFT" or "RIGHT" (default: RIGHT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Start line for multi-line comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<u32>,
    /// Side of start line: "LEFT" or "RIGHT"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_side: Option<String>,
    /// Subject type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<String>,
    /// Comment ID to reply to (for threaded replies)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<u64>,
}

/// Prompt arguments for `add_pull_request_review_comment` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddPullRequestReviewCommentPromptArgs {}

// ============================================================================
// GET_PULL_REQUEST_REVIEWS
// ============================================================================

/// Arguments for `get_pull_request_reviews` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestReviewsArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pull_number: u64,
}

/// Prompt arguments for `get_pull_request_reviews` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestReviewsPromptArgs {
    /// Optional: specific focus area for teaching prompt
    #[serde(default)]
    pub focus_area: Option<String>,
    /// Optional: use case context for examples
    #[serde(default)]
    pub use_case: Option<String>,
}

// ============================================================================
// GET_PULL_REQUEST_STATUS
// ============================================================================

/// Arguments for getting pull request status
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestStatusArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pr_number: u64,
}

// ============================================================================
// GET_PULL_REQUEST_FILES
// ============================================================================

/// Arguments for getting pull request files
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetPullRequestFilesArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pr_number: u64,
}

// ============================================================================
// MERGE_PULL_REQUEST
// ============================================================================

/// Arguments for merging a pull request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MergePullRequestArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pr_number: u64,
    /// Title for the merge commit (optional)
    #[serde(default)]
    pub commit_title: Option<String>,
    /// Extra detail for the merge commit message (optional)
    #[serde(default)]
    pub commit_message: Option<String>,
    /// Merge method: "merge", "squash", or "rebase" (optional, defaults to repository setting)
    #[serde(default)]
    pub merge_method: Option<String>,
    /// SHA that pull request head must match to allow merge (optional, for safety)
    #[serde(default)]
    pub sha: Option<String>,
}

// ============================================================================
// UPDATE_PULL_REQUEST
// ============================================================================

/// Arguments for updating a pull request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdatePullRequestArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pr_number: u64,
    /// New title (optional)
    #[serde(default)]
    pub title: Option<String>,
    /// New body/description (optional)
    #[serde(default)]
    pub body: Option<String>,
    /// New state: "open" or "closed" (optional)
    #[serde(default)]
    pub state: Option<String>,
    /// New base branch (optional)
    #[serde(default)]
    pub base: Option<String>,
    /// Whether maintainers can modify the pull request (optional)
    #[serde(default)]
    pub maintainer_can_modify: Option<bool>,
}

/// Prompt arguments for `update_pull_request` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdatePullRequestPromptArgs {
    /// Type of example to focus on: "title", "body", "state", "base", "maintainer", or "combined"
    /// Use this to get examples tailored to specific update types
    #[serde(default)]
    pub example_type: Option<String>,

    /// Whether to include common gotchas and error cases in the prompt
    /// Set to true for deeper learning about edge cases and warnings
    #[serde(default)]
    pub show_gotchas: Option<bool>,
}

// ============================================================================
// CREATE_REPOSITORY
// ============================================================================

/// Arguments for creating a repository
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateRepositoryArgs {
    /// Repository name
    pub name: String,
    /// Repository description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub description: Option<String>,
    /// Make repository private (optional, default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub private: Option<bool>,
    /// Initialize with README (optional, default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub auto_init: Option<bool>,
    /// .gitignore template name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub gitignore_template: Option<String>,
    /// License template name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub license_template: Option<String>,
    /// Allow squash merging (optional, default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub allow_squash_merge: Option<bool>,
    /// Allow merge commits (optional, default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub allow_merge_commit: Option<bool>,
    /// Allow rebase merging (optional, default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub allow_rebase_merge: Option<bool>,
    /// Automatically delete head branches after merge (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub delete_branch_on_merge: Option<bool>,
    /// Enable issues (optional, default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub has_issues: Option<bool>,
    /// Enable projects (optional, default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub has_projects: Option<bool>,
    /// Enable wiki (optional, default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(default)]
    pub has_wiki: Option<bool>,
}


// ============================================================================
// FORK_REPOSITORY
// ============================================================================

/// Arguments for forking a repository
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ForkRepositoryArgs {
    /// Repository owner to fork from
    pub owner: String,
    /// Repository name to fork
    pub repo: String,
    /// Organization to fork to (optional, defaults to user)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}

/// Prompt arguments for customizing fork_repository teaching examples
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GitHubForkRepositoryPromptArgs {
    /// Forking scenario to focus on: "personal-account", "organization", or "all" (optional, default: "all")
    /// - "personal-account": Focus on forking to your personal GitHub account
    /// - "organization": Focus on forking to an organization you belong to
    /// - "all": Include both scenarios with comprehensive examples
    #[serde(default)]
    pub scenario: Option<String>,

    /// Learning depth level: "basic", "detailed", or "advanced" (optional, default: "detailed")
    /// - "basic": Simplified explanation suitable for new GitHub users
    /// - "detailed": Comprehensive explanation with workflows and best practices (default)
    /// - "advanced": Deep dive including advanced fork management, syncing strategies, and edge cases
    #[serde(default)]
    pub depth: Option<String>,

    /// Include troubleshooting and common pitfalls section (optional, default: true)
    /// - When true: Adds comprehensive troubleshooting for common forking issues
    /// - When false: Focuses on happy path and best practices only
    #[serde(default)]
    pub include_troubleshooting: Option<bool>,
}

// ============================================================================
// CREATE_BRANCH
// ============================================================================

/// Arguments for creating a branch
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateBranchArgs {
    /// Repository owner
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// New branch name
    pub branch_name: String,
    /// SHA to create branch from
    pub sha: String,
}

/// Prompt arguments for `create_branch` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateBranchPromptArgs {
    /// Customize teaching examples based on use case
    /// Options: "basic" (simple feature branch), "advanced" (git workflows), "troubleshooting" (common errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<String>,
}

// ============================================================================
// DELETE_BRANCH
// ============================================================================

/// Arguments for deleting a branch
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeleteBranchArgs {
    /// Repository owner
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Branch name to delete
    pub branch_name: String,
}

/// Prompt arguments for customizing delete_branch teaching examples
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeleteBranchPromptArgs {
    /// Scenario to focus on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,

    /// Include permissions section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_permissions: Option<bool>,

    /// Include recovery section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_recovery: Option<bool>,
}

// ============================================================================
// LIST_BRANCHES
// ============================================================================

/// Arguments for listing branches
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListBranchesArgs {
    /// Repository owner
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Page number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page (optional, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u8>,
}

// ============================================================================
// GET_COMMIT
// ============================================================================

/// Arguments for getting a commit
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetCommitArgs {
    /// Repository owner
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Commit SHA
    pub commit_sha: String,
    /// Page number for files (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page (optional, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u8>,
}

/// Prompt arguments for `get_commit` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetCommitPromptArgs {
    /// Focus on response structure and data fields (true/false)
    #[serde(default)]
    pub explain_response: Option<bool>,

    /// Focus on handling large commits with many file changes (true/false)
    #[serde(default)]
    pub explain_pagination: Option<bool>,

    /// Focus on understanding diff format and patch content (true/false)
    #[serde(default)]
    pub explain_diffs: Option<bool>,

    /// Focus on common use cases and workflow patterns (true/false)
    #[serde(default)]
    pub explain_use_cases: Option<bool>,
}

// ============================================================================
// LIST_COMMITS
// ============================================================================

/// Arguments for listing commits
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListCommitsArgs {
    /// Repository owner
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// SHA or branch to start listing from (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    /// Filter by file path (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Filter by author (GitHub login or email) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Only commits after this date (ISO 8601) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// Only commits before this date (ISO 8601) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    /// Page number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page (optional, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u8>,
}

// ============================================================================
// GET_FILE_CONTENTS
// ============================================================================

/// Arguments for getting file or directory contents
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetFileContentsArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// File or directory path
    pub path: String,
    /// Branch, tag, or commit (optional, defaults to default branch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
}

// ============================================================================
// CREATE_OR_UPDATE_FILE
// ============================================================================

/// Arguments for creating or updating a file
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateOrUpdateFileArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// File path
    pub path: String,
    /// Commit message
    pub message: String,
    /// File content (plain text, will be base64 encoded automatically)
    pub content: String,
    /// Branch name (optional, defaults to default branch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// File SHA for updates (optional, omit for creation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
}

// ============================================================================
// PUSH_FILES
// ============================================================================

/// Arguments for push_files tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PushFilesArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Branch name
    pub branch: String,
    /// Map of file paths to base64-encoded content
    pub files: HashMap<String, String>,
    /// Commit message
    pub message: String,
}

/// Prompt arguments for push_files tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PushFilesPromptArgs {
    /// Optional use case to focus examples on (e.g., 'bulk_setup', 'binary_files', 'encoding')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case: Option<String>,
    /// Optional feature to focus on (e.g., 'atomicity', 'base64', 'permissions')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<String>,
}

// ============================================================================
// SEARCH_CODE
// ============================================================================

/// Arguments for searching code
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchCodeArgs {
    /// Search query using GitHub code search syntax
    pub query: String,
    /// Sort by: "indexed" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Order: "asc" or "desc" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Page number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page (optional, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u8>,
    /// Enrich results with star counts (default: false)
    #[serde(default)]
    pub enrich_stars: bool,
}

// ============================================================================
// SEARCH_ISSUES
// ============================================================================

/// Arguments for `search_issues` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchIssuesArgs {
    /// GitHub search query (supports complex syntax)
    pub query: String,
    /// Sort results by: "comments", "reactions", "created", "updated" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Sort order: "asc" or "desc" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Page number for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page, max 100 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

/// Prompt arguments for `search_issues` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchIssuesPromptArgs {
    /// Focus area for teaching: "basic", "filters", "advanced", "pagination", "all"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_area: Option<String>,
    /// Include comprehensive examples or concise explanations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_examples: Option<bool>,
}

// ============================================================================
// SEARCH_REPOSITORIES
// ============================================================================

/// Arguments for searching repositories
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchRepositoriesArgs {
    /// Search query using GitHub repository search syntax
    pub query: String,
    /// Sort by: "stars", "forks", or "updated" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Order: "asc" or "desc" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Page number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page (optional, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u8>,
}

/// Prompt arguments for `search_repositories` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchRepositoriesPromptArgs {
    /// Optional: specific programming language to focus examples on (e.g., "rust", "python", "javascript")
    #[serde(default)]
    pub language: Option<String>,
    
    /// Optional: use case to tailor examples to (e.g., "discovery", "research", "trending", "contribution", "evaluation")
    #[serde(default)]
    pub use_case: Option<String>,
    
    /// Optional: level of detail for examples (e.g., "brief", "detailed")
    #[serde(default)]
    pub depth: Option<String>,
}

// ============================================================================
// SEARCH_USERS
// ============================================================================

/// Arguments for searching users
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchUsersArgs {
    /// Search query using GitHub user search syntax
    pub query: String,
    /// Sort by: "followers", "repositories", or "joined" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Order: "asc" or "desc" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Page number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Results per page (optional, max 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u8>,
}

// ============================================================================
// REQUEST_COPILOT_REVIEW
// ============================================================================

/// Arguments for `request_copilot_review` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequestCopilotReviewArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Pull request number
    pub pull_number: u64,
}

/// Prompt arguments for `request_copilot_review` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequestCopilotReviewPromptArgs {
    /// Focus area for the review (e.g., "security", "performance", "style") - optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_area: Option<String>,
    
    /// Depth of explanation (e.g., "basic", "detailed") - optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<String>,
}

// ============================================================================
// CODE_SCANNING_ALERTS
// ============================================================================

/// Arguments for code_scanning_alerts tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CodeScanningAlertsArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Filter by state: "open", "closed", or "dismissed" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Filter by branch/ref (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    /// Filter by tool name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    /// Filter by severity: "critical", "high", "medium", "low", "warning", "note", "error" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

/// Prompt arguments for code_scanning_alerts tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CodeScanningAlertsPromptArgs {}

// ============================================================================
// SECRET_SCANNING_ALERTS
// ============================================================================

/// Arguments for secret_scanning_alerts tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SecretScanningAlertsArgs {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Filter by state: "open" or "resolved" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Filter by secret type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_type: Option<String>,
    /// Filter by resolution: "false_positive", "wont_fix", "revoked", "used_in_tests" (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
}

/// Prompt arguments for secret_scanning_alerts tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SecretScanningAlertsPromptArgs {
    /// Optional use case context for customizing teaching focus
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case: Option<String>,
}

// ============================================================================
// GET_ME
// ============================================================================

/// Arguments for get_me tool (no arguments needed)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetMeArgs {
    // No fields - uses GITHUB_TOKEN for authentication
}

/// Prompt arguments for get_me tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetMePromptArgs {}
