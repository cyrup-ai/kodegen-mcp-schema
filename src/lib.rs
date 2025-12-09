//! Lightweight MCP tool schema definitions
//! 
//! This package contains ONLY Args and PromptArgs type definitions
//! with zero heavy dependencies. It serves as the single source of
//! truth for all tool schemas in the kodegen ecosystem.

use serde::{Serialize, de::DeserializeOwned};
use schemars::JsonSchema;
use rmcp::model::{PromptArgument, PromptMessage};
use serde_json::Value;

// ============================================================================
// AUTOMATIC TOOL DISCOVERY
// ============================================================================

/// Tool metadata for automatic discovery.
///
/// Every tool in kodegen-mcp-schema registers exactly one ToolMetadata via
/// the #[tool_metadata] proc macro. This enables automatic discovery in kodegen.
pub struct ToolMetadata {
    pub name: &'static str,
    pub category: &'static kodegen_config::Category,
    pub description: &'static str,
    pub args_schema: fn() -> Value,
    pub output_schema: fn() -> Value,
    pub prompt_arguments: fn() -> Vec<PromptArgument>,
    /// Generate prompt messages from JSON arguments.
    /// Takes PromptArgs as JSON Value, deserializes internally, calls generate_prompts.
    pub generate_prompts: fn(&Value) -> Vec<PromptMessage>,
}

// Collect all ToolMetadata at link time
inventory::collect!(ToolMetadata);

// Re-export proc macro for convenient use
pub use kodegen_mcp_schema_macros::tool_metadata;

// ============================================================================
// TOOL INFRASTRUCTURE (moved from kodegen-mcp-tool)
// ============================================================================

pub mod tool;

// Re-export tool infrastructure for convenience
pub use tool::{
    Tool, ToolResponse, ToolExecutionContext,
    McpError, ToolCallRecord
};

// ============================================================================
// TOOL ARGS TRAIT (Args→Output Mapping)
// ============================================================================

/// Trait for tool argument types that enforces Args→Output mapping.
///
/// This trait is implemented in the schema package for each tool's Args type,
/// binding it to the correct Output type. The Tool trait then derives the
/// output type from Args, providing compile-time enforcement.
///
/// # Why This Is In The Schema Package
///
/// Only `kodegen-mcp-schema` implements this trait. External crates cannot
/// implement it for their own types, ensuring all tools use official schemas.
/// The output type is LOCKED to the Args type - tools cannot choose wrong output.
///
/// # Example
///
/// ```rust
/// // In kodegen-mcp-schema
/// impl ToolArgs for TerminalInput {
///     type Output = TerminalOutput;
/// }
///
/// // In tool implementation - compiler enforces TerminalOutput
/// impl Tool for TerminalTool {
///     type Args = TerminalInput;
///     // execute() MUST return ToolResponse<TerminalOutput>
///     // because TerminalInput::Output = TerminalOutput
/// }
/// ```
pub trait ToolArgs: DeserializeOwned + Serialize + JsonSchema + Send + 'static {
    /// The output type for tools using these args.
    /// Defined here in schema package, not in tool impl - provides compile-time enforcement.
    type Output: Serialize + JsonSchema + Send + 'static;

    /// Prompt provider - MUST be from schema package (sealed trait).
    /// This enforces that all prompt logic is centralized in kodegen-mcp-schema.
    type Prompts: tool::PromptProvider;

    /// Tool name (unique identifier)
    const NAME: &'static str;

    /// Tool category for organization - structured data with name and icon
    const CATEGORY: &'static kodegen_config::Category;

    /// Human-readable tool description
    const DESCRIPTION: &'static str;

    /// Tool icon character (UTF-8)
    ///
    /// Default implementation returns icon from category.
    /// Individual tools can override for tool-specific icons.
    fn icon() -> char {
        Self::CATEGORY.icon
    }
}

pub mod filesystem;
pub mod terminal;
pub mod serde_helpers;
pub mod git;
pub mod github;
pub mod browser;
pub mod citescrape;
pub mod database;
pub mod reasoner;
pub mod sequential_thinking;
pub mod claude_agent;
pub mod memory;
pub mod prompt;
pub mod introspection;
pub mod process;
pub mod config;
pub mod deserialize;
pub mod web;

// Category modules are accessible via their module paths
// e.g., kodegen_mcp_schema::github::SearchCodeArgs
// e.g., kodegen_mcp_schema::git::GitInitArgs
// This prevents namespace pollution and ambiguous re-exports at the crate root

// Re-export deserialize utilities
pub use deserialize::{
    DeserializeError,
    AnyToolOutput,
    ToolOutputResult,
    deserialize_tool_output,
    deserialize_typed_only,
};

// ============================================================================
// SEALED TRAIT IMPLEMENTATIONS FOR PROMPT PROVIDERS
// ============================================================================

// Allow schema package to implement sealed PromptProvider trait
// This enables compile-time enforcement that prompts MUST be in schema package

// GitHub tools
impl tool::SealedPromptProvider for github::accept_repo_invitation::GithubAcceptRepoInvitationPrompts {}
impl tool::SealedPromptProvider for github::search_code::SearchCodePrompts {}
impl tool::SealedPromptProvider for github::add_issue_comment::AddIssueCommentPrompts {}
impl tool::SealedPromptProvider for github::add_pull_request_review_comment::AddPullRequestReviewCommentPrompts {}
impl tool::SealedPromptProvider for github::code_scanning_alerts::CodeScanningAlertsPrompts {}
impl tool::SealedPromptProvider for github::create_branch::CreateBranchPrompts {}
impl tool::SealedPromptProvider for github::create_issue::CreateIssuePrompts {}
impl tool::SealedPromptProvider for github::create_or_update_file::CreateOrUpdateFilePrompts {}
impl tool::SealedPromptProvider for github::create_pull_request::CreatePullRequestPrompts {}
impl tool::SealedPromptProvider for github::create_pull_request_review::CreatePullRequestReviewPrompts {}
impl tool::SealedPromptProvider for github::create_release::GithubCreateReleasePrompts {}
impl tool::SealedPromptProvider for github::create_repository::CreateRepositoryPrompts {}
impl tool::SealedPromptProvider for github::delete_branch::DeleteBranchPrompts {}
impl tool::SealedPromptProvider for github::delete_file::GithubDeleteFilePrompts {}
impl tool::SealedPromptProvider for github::fork_repository::ForkRepositoryPrompts {}
impl tool::SealedPromptProvider for github::get_commit::GetCommitPrompts {}
impl tool::SealedPromptProvider for github::get_file_contents::GetFileContentsPrompts {}
impl tool::SealedPromptProvider for github::get_issue::GetIssuePrompts {}
impl tool::SealedPromptProvider for github::get_issue_comments::GetIssueCommentsPrompts {}
impl tool::SealedPromptProvider for github::get_me::GetMePrompts {}
impl tool::SealedPromptProvider for github::get_pull_request_files::GetPullRequestFilesPrompts {}
impl tool::SealedPromptProvider for github::get_pull_request_reviews::GetPullRequestReviewsPrompts {}
impl tool::SealedPromptProvider for github::get_pull_request_status::GetPullRequestStatusPrompts {}
impl tool::SealedPromptProvider for github::list_branches::ListBranchesPrompts {}
impl tool::SealedPromptProvider for github::list_commits::ListCommitsPrompts {}
impl tool::SealedPromptProvider for github::list_issues::ListIssuesPrompts {}
impl tool::SealedPromptProvider for github::list_repos::GithubListReposPrompts {}
impl tool::SealedPromptProvider for github::list_pull_requests::ListPullRequestsPrompts {}
impl tool::SealedPromptProvider for github::merge_pull_request::MergePullRequestPrompts {}
impl tool::SealedPromptProvider for github::pending_invitations::GithubPendingInvitationsPrompts {}
impl tool::SealedPromptProvider for github::push_file::GithubPushFilePrompts {}
impl tool::SealedPromptProvider for github::push_files::PushFilesPrompts {}
impl tool::SealedPromptProvider for github::request_copilot_review::RequestCopilotReviewPrompts {}
impl tool::SealedPromptProvider for github::search_issues::SearchIssuesPrompts {}
impl tool::SealedPromptProvider for github::search_repositories::SearchRepositoriesPrompts {}
impl tool::SealedPromptProvider for github::search_users::SearchUsersPrompts {}
impl tool::SealedPromptProvider for github::secret_scanning_alerts::SecretScanningAlertsPrompts {}
impl tool::SealedPromptProvider for github::update_issue::UpdateIssuePrompts {}
impl tool::SealedPromptProvider for github::update_pull_request::UpdatePullRequestPrompts {}

// Filesystem tools
impl tool::SealedPromptProvider for filesystem::create_directory::CreateDirectoryPrompts {}
impl tool::SealedPromptProvider for filesystem::delete_directory::DeleteDirectoryPrompts {}
impl tool::SealedPromptProvider for filesystem::delete_file::DeleteFilePrompts {}
impl tool::SealedPromptProvider for filesystem::edit_block::EditBlockPrompts {}
impl tool::SealedPromptProvider for filesystem::get_file_info::GetFileInfoPrompts {}
impl tool::SealedPromptProvider for filesystem::list_directory::ListDirectoryPrompts {}
impl tool::SealedPromptProvider for filesystem::move_file::MoveFilePrompts {}
impl tool::SealedPromptProvider for filesystem::read_file::ReadFilePrompts {}
impl tool::SealedPromptProvider for filesystem::read_multiple_files::ReadMultipleFilesPrompts {}
impl tool::SealedPromptProvider for filesystem::search::SearchPrompts {}
impl tool::SealedPromptProvider for filesystem::write_file::WriteFilePrompts {}

// Database tools
impl tool::SealedPromptProvider for database::list_schemas::ListSchemasPrompts {}
impl tool::SealedPromptProvider for database::list_tables::ListTablesPrompts {}
impl tool::SealedPromptProvider for database::table_schema::TableSchemaPrompts {}
impl tool::SealedPromptProvider for database::table_indexes::TableIndexesPrompts {}
impl tool::SealedPromptProvider for database::stored_procedures::StoredProceduresPrompts {}
impl tool::SealedPromptProvider for database::execute_sql::DbExecuteSqlPrompts {}
impl tool::SealedPromptProvider for database::pool_stats::PoolStatsPrompts {}

// Terminal tool
impl tool::SealedPromptProvider for terminal::TerminalPrompts {}

// Introspection tools
impl tool::SealedPromptProvider for introspection::get_events::IntrospectionGetEventsPrompts {}
impl tool::SealedPromptProvider for introspection::inspect_tool_calls::InspectToolCallsPrompts {}
impl tool::SealedPromptProvider for introspection::inspect_usage_stats::InspectUsageStatsPrompts {}
impl tool::SealedPromptProvider for introspection::list_tools::IntrospectionListToolsPrompts {}

// Reasoner tool
impl tool::SealedPromptProvider for reasoner::ReasonerPrompts {}

// Sequential thinking tool
impl tool::SealedPromptProvider for sequential_thinking::SequentialThinkingPrompts {}

// Prompt management tools
impl tool::SealedPromptProvider for prompt::prompt_add::PromptAddPrompts {}
impl tool::SealedPromptProvider for prompt::prompt_get::PromptGetPrompts {}
impl tool::SealedPromptProvider for prompt::prompt_edit::PromptEditPrompts {}
impl tool::SealedPromptProvider for prompt::prompt_delete::PromptDeletePrompts {}

// Agent tools
impl tool::SealedPromptProvider for claude_agent::agent::ClaudeAgentPrompts {}

// Browser tools
impl tool::SealedPromptProvider for browser::navigate::NavigatePrompts {}
impl tool::SealedPromptProvider for browser::click::ClickPrompts {}
impl tool::SealedPromptProvider for browser::type_text::TypeTextPrompts {}
impl tool::SealedPromptProvider for browser::scroll::ScrollPrompts {}
impl tool::SealedPromptProvider for browser::screenshot::ScreenshotPrompts {}
impl tool::SealedPromptProvider for browser::extract_text::ExtractTextPrompts {}
impl tool::SealedPromptProvider for browser::web_search::WebSearchPrompts {}
impl tool::SealedPromptProvider for browser::research::ResearchPrompts {}
impl tool::SealedPromptProvider for browser::agent::AgentPrompts {}

// Citescrape tools
impl tool::SealedPromptProvider for citescrape::fetch::FetchPrompts {}
impl tool::SealedPromptProvider for citescrape::scrape_url::ScrapeUrlPrompts {}
impl tool::SealedPromptProvider for citescrape::web_search::WebSearchPrompts {}

// Git tools - Part 1 (Basic Operations)
impl tool::SealedPromptProvider for git::init::InitPrompts {}
impl tool::SealedPromptProvider for git::clone::ClonePrompts {}
impl tool::SealedPromptProvider for git::open::OpenPrompts {}
impl tool::SealedPromptProvider for git::discover::DiscoverPrompts {}
impl tool::SealedPromptProvider for git::add::AddPrompts {}
impl tool::SealedPromptProvider for git::commit::CommitPrompts {}
impl tool::SealedPromptProvider for git::status::StatusPrompts {}
impl tool::SealedPromptProvider for git::log::LogPrompts {}
impl tool::SealedPromptProvider for git::diff::DiffPrompts {}
impl tool::SealedPromptProvider for git::branch_list::BranchListPrompts {}
impl tool::SealedPromptProvider for git::branch_create::BranchCreatePrompts {}
impl tool::SealedPromptProvider for git::branch_delete::BranchDeletePrompts {}
impl tool::SealedPromptProvider for git::branch_rename::BranchRenamePrompts {}
impl tool::SealedPromptProvider for git::checkout::GitCheckoutPrompts {}
impl tool::SealedPromptProvider for git::config_set::ConfigSetPrompts {}
impl tool::SealedPromptProvider for git::fetch::FetchPrompts {}

// Git tools - Part 2 (Advanced Operations)
impl tool::SealedPromptProvider for git::push::PushPrompts {}
impl tool::SealedPromptProvider for git::pull::PullPrompts {}
impl tool::SealedPromptProvider for git::merge::MergePrompts {}
impl tool::SealedPromptProvider for git::reset::ResetPrompts {}
impl tool::SealedPromptProvider for git::stash::StashPrompts {}
impl tool::SealedPromptProvider for git::stash_apply::StashApplyPrompts {}
impl tool::SealedPromptProvider for git::remote_add::RemoteAddPrompts {}
impl tool::SealedPromptProvider for git::remote_list::RemoteListPrompts {}
impl tool::SealedPromptProvider for git::remote_remove::RemoteRemovePrompts {}
impl tool::SealedPromptProvider for git::tag::TagPrompts {}
impl tool::SealedPromptProvider for git::history::HistoryPrompts {}
impl tool::SealedPromptProvider for git::worktree_add::WorktreeAddPrompts {}
impl tool::SealedPromptProvider for git::worktree_list::WorktreeListPrompts {}
impl tool::SealedPromptProvider for git::worktree_lock::WorktreeLockPrompts {}
impl tool::SealedPromptProvider for git::worktree_unlock::WorktreeUnlockPrompts {}
impl tool::SealedPromptProvider for git::worktree_prune::WorktreePrunePrompts {}
impl tool::SealedPromptProvider for git::worktree_remove::WorktreeRemovePrompts {}

// Git tools - Part 3 (Additional Operations)
impl tool::SealedPromptProvider for git::cherry_pick::CherryPickPrompts {}
impl tool::SealedPromptProvider for git::config_get::ConfigGetPrompts {}
impl tool::SealedPromptProvider for git::rebase::RebasePrompts {}
impl tool::SealedPromptProvider for git::revert::RevertPrompts {}
impl tool::SealedPromptProvider for git::show::ShowPrompts {}
impl tool::SealedPromptProvider for git::stash_list::StashListPrompts {}
impl tool::SealedPromptProvider for git::stash_pop::GitStashPopPrompts {}
impl tool::SealedPromptProvider for git::tag_list::GitTagListPrompts {}

// Configuration tools
impl tool::SealedPromptProvider for config::config_get::ConfigGetPrompts {}
impl tool::SealedPromptProvider for config::config_set::SetConfigValuePrompts {}

// Process management tools
impl tool::SealedPromptProvider for process::process_list::ProcessListPrompts {}
impl tool::SealedPromptProvider for process::process_kill::ProcessKillPrompts {}

// Memory tools
impl tool::SealedPromptProvider for memory::list_libraries::prompts::MemoryListLibrariesPrompts {}
impl tool::SealedPromptProvider for memory::memorize::MemorizePrompts {}
impl tool::SealedPromptProvider for memory::recall::MemoryRecallPrompts {}
impl tool::SealedPromptProvider for memory::check_memorize_status::CheckMemorizeStatusPrompts {}

// Web tools
impl tool::SealedPromptProvider for web::scrape_url::ScrapeUrlPrompts {}
impl tool::SealedPromptProvider for web::web_search::WebSearchPrompts {}
