//! Core deserialization types

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::{browser, citescrape, claude_agent, config, database, filesystem, git, github, introspection, process, prompt, reasoning, terminal};

/// Combined result containing both display text and typed output.
///
/// Returned when deserializing full MCP tool responses. Provides both
/// human-readable display text (content[0]) and structured typed metadata (content[1]).
#[derive(Debug, Clone)]
pub struct ToolOutputResult {
    /// Human-readable display output from content[0].
    pub display: String,

    /// Typed metadata from content[1], deserialized based on tool_name.
    pub typed: AnyToolOutput,
}

/// Enum representing all 111 MCP tool output types across all categories.
///
/// Automatically deserialized from content[1] based on tool_name routing.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum AnyToolOutput {
    // ========================================================================
    // FILESYSTEM TOOLS (11 tools)
    // ========================================================================
    FsReadFile(filesystem::FsReadFileOutput),
    FsReadMultipleFiles(filesystem::FsReadMultipleFilesOutput),
    FsWriteFile(filesystem::FsWriteFileOutput),
    FsEditBlock(filesystem::FsEditBlockOutput),
    FsCreateDirectory(filesystem::FsCreateDirectoryOutput),
    FsDeleteDirectory(filesystem::FsDeleteDirectoryOutput),
    FsDeleteFile(filesystem::FsDeleteFileOutput),
    FsMoveFile(filesystem::FsMoveFileOutput),
    FsListDirectory(filesystem::FsListDirectoryOutput),
    FsGetFileInfo(filesystem::FsGetFileInfoOutput),
    FsSearch(filesystem::FsSearchOutput),

    // ========================================================================
    // GIT TOOLS (31 tools)
    // ========================================================================
    GitInit(git::GitInitOutput),
    GitOpen(git::GitOpenOutput),
    GitClone(git::GitCloneOutput),
    GitDiscover(git::GitDiscoverOutput),
    GitAdd(git::GitAddOutput),
    GitCommit(git::GitCommitOutput),
    GitLog(git::GitLogOutput),
    GitHistory(git::GitHistoryOutput),
    GitDiff(git::GitDiffOutput),
    GitBranchCreate(git::GitBranchCreateOutput),
    GitBranchDelete(git::GitBranchDeleteOutput),
    GitBranchList(git::GitBranchListOutput),
    GitBranchRename(git::GitBranchRenameOutput),
    GitCheckout(git::GitCheckoutOutput),
    GitFetch(git::GitFetchOutput),
    GitMerge(git::GitMergeOutput),
    GitWorktreeAdd(git::GitWorktreeAddOutput),
    GitWorktreeList(git::GitWorktreeListOutput),
    GitWorktreeLock(git::GitWorktreeLockOutput),
    GitWorktreeUnlock(git::GitWorktreeUnlockOutput),
    GitWorktreePrune(git::GitWorktreePruneOutput),
    GitWorktreeRemove(git::GitWorktreeRemoveOutput),
    GitPull(git::GitPullOutput),
    GitPush(git::GitPushOutput),
    GitRemoteAdd(git::GitRemoteAddOutput),
    GitRemoteList(git::GitRemoteListOutput),
    GitRemoteRemove(git::GitRemoteRemoveOutput),
    GitReset(git::GitResetOutput),
    GitStatus(git::GitStatusOutput),
    GitStash(git::GitStashOutput),
    GitTag(git::GitTagOutput),

    // ========================================================================
    // GITHUB TOOLS (40 tools)
    // ========================================================================
    GitHubGetIssue(github::GitHubGetIssueOutput),
    GitHubCreateIssue(github::GitHubCreateIssueOutput),
    GitHubListIssues(github::GitHubListIssuesOutput),
    GitHubSearchIssues(github::GitHubSearchIssuesOutput),
    GitHubUpdateIssue(github::GitHubUpdateIssueOutput),
    GitHubCreatePr(github::GitHubCreatePrOutput),
    GitHubMergePr(github::GitHubMergePrOutput),
    GitHubUpdatePr(github::GitHubUpdatePrOutput),
    GitHubPrReviews(github::GitHubPrReviewsOutput),
    GitHubGetIssueComments(github::GitHubGetIssueCommentsOutput),
    GitHubAddIssueComment(github::GitHubAddIssueCommentOutput),
    GitHubCreateRepo(github::GitHubCreateRepoOutput),
    GitHubForkRepo(github::GitHubForkRepoOutput),
    GitHubSearchRepos(github::GitHubSearchReposOutput),
    GitHubSearchCode(github::GitHubSearchCodeOutput),
    GitHubGetFileContents(github::GitHubGetFileContentsOutput),
    GitHubCreateOrUpdateFile(github::GitHubCreateOrUpdateFileOutput),
    GitHubPushFiles(github::GitHubPushFilesOutput),
    GitHubListCommits(github::GitHubListCommitsOutput),
    GitHubGetCommit(github::GitHubGetCommitOutput),
    GitHubListBranches(github::GitHubListBranchesOutput),
    GitHubCreateBranch(github::GitHubCreateBranchOutput),
    GitHubDeleteBranch(github::GitHubDeleteBranchOutput),
    GitHubSearchUsers(github::GitHubSearchUsersOutput),
    GitHubGetMe(github::GitHubGetMeOutput),
    GitHubCodeScanningAlerts(github::GitHubCodeScanningAlertsOutput),
    GitHubSecretScanningAlerts(github::GitHubSecretScanningAlertsOutput),
    GitHubGetPrFiles(github::GitHubGetPrFilesOutput),
    GitHubGetPrStatus(github::GitHubGetPrStatusOutput),
    GitHubCreatePrReview(github::GitHubCreatePrReviewOutput),
    GitHubAddPrReviewComment(github::GitHubAddPrReviewCommentOutput),
    GitHubRequestCopilotReview(github::GitHubRequestCopilotReviewOutput),

    // ========================================================================
    // BROWSER TOOLS (11 tools)
    // ========================================================================
    BrowserAgent(browser::BrowserAgentOutput),
    BrowserAgentKill(browser::BrowserAgentKillOutput),
    BrowserResearch(browser::BrowserResearchOutput),
    BrowserWebSearch(browser::BrowserWebSearchOutput),
    BrowserNavigate(browser::BrowserNavigateOutput),
    BrowserScreenshot(browser::BrowserScreenshotOutput),
    BrowserClick(browser::BrowserClickOutput),
    BrowserType(browser::BrowserTypeOutput),
    BrowserScroll(browser::BrowserScrollOutput),
    BrowserEval(browser::BrowserEvalOutput),
    BrowserExtractText(browser::BrowserExtractTextOutput),

    // ========================================================================
    // DATABASE TOOLS (7 tools)
    // ========================================================================
    DbListSchemas(database::ListSchemasOutput),
    DbListTables(database::ListTablesOutput),
    DbExecuteSQL(database::ExecuteSQLOutput),
    DbGetTableSchema(database::GetTableSchemaOutput),
    DbGetTableIndexes(database::GetTableIndexesOutput),
    DbGetStoredProcedures(database::GetStoredProceduresOutput),
    DbGetPoolStats(database::GetPoolStatsOutput),

    // ========================================================================
    // CITESCRAPE TOOLS (2 tools)
    // ========================================================================
    ScrapeUrl(citescrape::ScrapeUrlOutput),
    WebSearch(citescrape::WebSearchOutput),

    // ========================================================================
    // PROCESS TOOLS (2 tools)
    // ========================================================================
    ProcessList(process::ProcessListOutput),
    ProcessKill(process::ProcessKillOutput),

    // ========================================================================
    // TERMINAL TOOLS (1 tool)
    // ========================================================================
    Terminal(terminal::TerminalOutput),

    // ========================================================================
    // CLAUDE AGENT TOOLS (5 tools)
    // ========================================================================
    ClaudeAgent(claude_agent::ClaudeAgentOutput),
    MemoryMemorize(claude_agent::MemorizeOutput),
    MemoryRecall(claude_agent::RecallOutput),
    MemoryListLibraries(claude_agent::ListMemoryLibrariesOutput),
    MemoryCheckMemorizeStatus(claude_agent::CheckMemorizeStatusOutput),

    // ========================================================================
    // CONFIG TOOLS (2 tools)
    // ========================================================================
    ConfigGet(config::ConfigGetOutput),
    ConfigSet(config::ConfigSetOutput),

    // ========================================================================
    // PROMPT TOOLS (4 tools)
    // ========================================================================
    PromptAdd(prompt::PromptAddOutput),
    PromptGet(prompt::PromptGetOutput),
    PromptDelete(prompt::PromptDeleteOutput),
    PromptEdit(prompt::PromptEditOutput),

    // ========================================================================
    // REASONING TOOLS (2 tools)
    // ========================================================================
    Reasoner(reasoning::ReasonerOutput),
    SequentialThinking(reasoning::SequentialThinkingOutput),

    // ========================================================================
    // INTROSPECTION TOOLS (2 tools)
    // ========================================================================
    InspectToolCalls(introspection::InspectToolCallsOutput),
    InspectUsage(introspection::InspectUsageOutput),
}
