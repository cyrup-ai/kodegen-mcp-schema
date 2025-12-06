//! Tool name → deserializer mapping registry (macro-generated)

use super::{error::DeserializeError, types::AnyToolOutput};
use crate::{browser, citescrape, claude_agent, config, database, filesystem, git, github, introspection, memory, process, prompt, reasoning, terminal};

/// Macro to generate the tool deserialization registry.
///
/// This eliminates 800+ lines of repetitive match arms by declaring the
/// mapping between tool name constants and Output types declaratively.
macro_rules! tool_registry {
    (
        $(
            $module:ident :: $constant:ident => $variant:ident ( $output:ty )
        ),* $(,)?
    ) => {
        /// Deserialize typed output based on tool name.
        ///
        /// Takes the raw JSON string from content[1] and the tool name,
        /// returning the appropriate AnyToolOutput variant.
        pub fn deserialize_by_tool_name(
            tool_name: &str,
            json_str: &str,
        ) -> Result<AnyToolOutput, DeserializeError> {
            match tool_name {
                $(
                    $module::$constant => {
                        serde_json::from_str::<$output>(json_str)
                            .map(AnyToolOutput::$variant)
                            .map_err(|e| DeserializeError::JsonError {
                                tool: tool_name.to_string(),
                                source: e,
                            })
                    }
                )*
                _ => Err(DeserializeError::UnknownTool(tool_name.to_string())),
            }
        }
    };
}

// Registry declaration: ~100 lines vs 800+ lines of manual match arms
tool_registry! {
    // FILESYSTEM (11 tools)
    filesystem::FS_READ_FILE => FsReadFile(filesystem::FsReadFileOutput),
    filesystem::FS_READ_MULTIPLE_FILES => FsReadMultipleFiles(filesystem::FsReadMultipleFilesOutput),
    filesystem::FS_WRITE_FILE => FsWriteFile(filesystem::FsWriteFileOutput),
    filesystem::FS_EDIT_BLOCK => FsEditBlock(filesystem::FsEditBlockOutput),
    filesystem::FS_CREATE_DIRECTORY => FsCreateDirectory(filesystem::FsCreateDirectoryOutput),
    filesystem::FS_DELETE_DIRECTORY => FsDeleteDirectory(filesystem::FsDeleteDirectoryOutput),
    filesystem::FS_DELETE_FILE => FsDeleteFile(filesystem::FsDeleteFileOutput),
    filesystem::FS_MOVE_FILE => FsMoveFile(filesystem::FsMoveFileOutput),
    filesystem::FS_LIST_DIRECTORY => FsListDirectory(filesystem::FsListDirectoryOutput),
    filesystem::FS_GET_FILE_INFO => FsGetFileInfo(filesystem::FsGetFileInfoOutput),
    filesystem::FS_SEARCH => FsSearch(filesystem::FsSearchOutput),

    // GIT (31 tools)
    git::GIT_INIT => GitInit(git::GitInitOutput),
    git::GIT_OPEN => GitOpen(git::GitOpenOutput),
    git::GIT_CLONE => GitClone(git::GitCloneOutput),
    git::GIT_DISCOVER => GitDiscover(git::GitDiscoverOutput),
    git::GIT_ADD => GitAdd(git::GitAddOutput),
    git::GIT_COMMIT => GitCommit(git::GitCommitOutput),
    git::GIT_LOG => GitLog(git::GitLogOutput),
    git::GIT_HISTORY => GitHistory(git::GitHistoryOutput),
    git::GIT_DIFF => GitDiff(git::GitDiffOutput),
    git::GIT_BRANCH_CREATE => GitBranchCreate(git::GitBranchCreateOutput),
    git::GIT_BRANCH_DELETE => GitBranchDelete(git::GitBranchDeleteOutput),
    git::GIT_BRANCH_LIST => GitBranchList(git::GitBranchListOutput),
    git::GIT_BRANCH_RENAME => GitBranchRename(git::GitBranchRenameOutput),
    git::GIT_CHECKOUT => GitCheckout(git::GitCheckoutOutput),
    git::GIT_FETCH => GitFetch(git::GitFetchOutput),
    git::GIT_MERGE => GitMerge(git::GitMergeOutput),
    git::GIT_WORKTREE_ADD => GitWorktreeAdd(git::GitWorktreeAddOutput),
    git::GIT_WORKTREE_LIST => GitWorktreeList(git::GitWorktreeListOutput),
    git::GIT_WORKTREE_LOCK => GitWorktreeLock(git::GitWorktreeLockOutput),
    git::GIT_WORKTREE_UNLOCK => GitWorktreeUnlock(git::GitWorktreeUnlockOutput),
    git::GIT_WORKTREE_PRUNE => GitWorktreePrune(git::GitWorktreePruneOutput),
    git::GIT_WORKTREE_REMOVE => GitWorktreeRemove(git::GitWorktreeRemoveOutput),
    git::GIT_PULL => GitPull(git::GitPullOutput),
    git::GIT_PUSH => GitPush(git::GitPushOutput),
    git::GIT_REMOTE_ADD => GitRemoteAdd(git::GitRemoteAddOutput),
    git::GIT_REMOTE_LIST => GitRemoteList(git::GitRemoteListOutput),
    git::GIT_REMOTE_REMOVE => GitRemoteRemove(git::GitRemoteRemoveOutput),
    git::GIT_RESET => GitReset(git::GitResetOutput),
    git::GIT_STATUS => GitStatus(git::GitStatusOutput),
    git::GIT_STASH => GitStash(git::GitStashOutput),
    git::GIT_TAG => GitTag(git::GitTagOutput),

    // GITHUB (40 tools) - Using constants from github module
    github::GITHUB_GET_ISSUE => GitHubGetIssue(github::GitHubGetIssueOutput),
    github::GITHUB_CREATE_ISSUE => GitHubCreateIssue(github::GitHubCreateIssueOutput),
    github::GITHUB_LIST_ISSUES => GitHubListIssues(github::GitHubListIssuesOutput),
    github::GITHUB_SEARCH_ISSUES => GitHubSearchIssues(github::GitHubSearchIssuesOutput),
    github::GITHUB_UPDATE_ISSUE => GitHubUpdateIssue(github::GitHubUpdateIssueOutput),
    github::GITHUB_CREATE_PULL_REQUEST => GitHubCreatePr(github::GitHubCreatePrOutput),
    github::GITHUB_MERGE_PULL_REQUEST => GitHubMergePr(github::GitHubMergePrOutput),
    github::GITHUB_UPDATE_PULL_REQUEST => GitHubUpdatePr(github::GitHubUpdatePrOutput),
    github::GITHUB_GET_PULL_REQUEST_REVIEWS => GitHubPrReviews(github::GitHubPrReviewsOutput),
    github::GITHUB_GET_ISSUE_COMMENTS => GitHubGetIssueComments(github::GitHubGetIssueCommentsOutput),
    github::GITHUB_ADD_ISSUE_COMMENT => GitHubAddIssueComment(github::GitHubAddIssueCommentOutput),
    github::GITHUB_CREATE_REPOSITORY => GitHubCreateRepo(github::GitHubCreateRepoOutput),
    github::GITHUB_FORK_REPOSITORY => GitHubForkRepo(github::GitHubForkRepoOutput),
    github::GITHUB_SEARCH_REPOSITORIES => GitHubSearchRepos(github::GitHubSearchReposOutput),
    github::GITHUB_SEARCH_CODE => GitHubSearchCode(github::GitHubSearchCodeOutput),
    github::GITHUB_GET_FILE_CONTENTS => GitHubGetFileContents(github::GitHubGetFileContentsOutput),
    github::GITHUB_CREATE_OR_UPDATE_FILE => GitHubCreateOrUpdateFile(github::GitHubCreateOrUpdateFileOutput),
    github::GITHUB_PUSH_FILES => GitHubPushFiles(github::GitHubPushFilesOutput),
    github::GITHUB_LIST_COMMITS => GitHubListCommits(github::GitHubListCommitsOutput),
    github::GITHUB_GET_COMMIT => GitHubGetCommit(github::GitHubGetCommitOutput),
    github::GITHUB_LIST_BRANCHES => GitHubListBranches(github::GitHubListBranchesOutput),
    github::GITHUB_CREATE_BRANCH => GitHubCreateBranch(github::GitHubCreateBranchOutput),
    github::GITHUB_DELETE_BRANCH => GitHubDeleteBranch(github::GitHubDeleteBranchOutput),
    github::GITHUB_SEARCH_USERS => GitHubSearchUsers(github::GitHubSearchUsersOutput),
    github::GITHUB_GET_ME => GitHubGetMe(github::GitHubGetMeOutput),
    github::GITHUB_CODE_SCANNING_ALERTS => GitHubCodeScanningAlerts(github::GitHubCodeScanningAlertsOutput),
    github::GITHUB_SECRET_SCANNING_ALERTS => GitHubSecretScanningAlerts(github::GitHubSecretScanningAlertsOutput),
    github::GITHUB_GET_PULL_REQUEST_FILES => GitHubGetPrFiles(github::GitHubGetPrFilesOutput),
    github::GITHUB_GET_PULL_REQUEST_STATUS => GitHubGetPrStatus(github::GitHubGetPrStatusOutput),
    github::GITHUB_CREATE_PULL_REQUEST_REVIEW => GitHubCreatePrReview(github::GitHubCreatePrReviewOutput),
    github::GITHUB_ADD_PULL_REQUEST_REVIEW_COMMENT => GitHubAddPrReviewComment(github::GitHubAddPrReviewCommentOutput),
    github::GITHUB_REQUEST_COPILOT_REVIEW => GitHubRequestCopilotReview(github::GitHubRequestCopilotReviewOutput),

    // BROWSER (11 tools)
    browser::BROWSER_AGENT => BrowserAgent(browser::BrowserAgentOutput),
    browser::BROWSER_AGENT_KILL => BrowserAgentKill(browser::BrowserAgentKillOutput),
    browser::BROWSER_RESEARCH => BrowserResearch(browser::BrowserResearchOutput),
    browser::BROWSER_WEB_SEARCH => BrowserWebSearch(browser::BrowserWebSearchOutput),
    browser::BROWSER_NAVIGATE => BrowserNavigate(browser::BrowserNavigateOutput),
    browser::BROWSER_SCREENSHOT => BrowserScreenshot(browser::BrowserScreenshotOutput),
    browser::BROWSER_CLICK => BrowserClick(browser::BrowserClickOutput),
    browser::BROWSER_TYPE_TEXT => BrowserType(browser::BrowserTypeOutput),
    browser::BROWSER_SCROLL => BrowserScroll(browser::BrowserScrollOutput),
    browser::BROWSER_EVAL => BrowserEval(browser::BrowserEvalOutput),
    browser::BROWSER_EXTRACT_TEXT => BrowserExtractText(browser::BrowserExtractTextOutput),

    // DATABASE (7 tools)
    database::DB_LIST_SCHEMAS => DbListSchemas(database::ListSchemasOutput),
    database::DB_LIST_TABLES => DbListTables(database::ListTablesOutput),
    database::DB_EXECUTE_SQL => DbExecuteSQL(database::ExecuteSQLOutput),
    database::DB_TABLE_SCHEMA => DbGetTableSchema(database::GetTableSchemaOutput),
    database::DB_TABLE_INDEXES => DbGetTableIndexes(database::GetTableIndexesOutput),
    database::DB_STORED_PROCEDURES => DbGetStoredProcedures(database::GetStoredProceduresOutput),
    database::DB_POOL_STATS => DbGetPoolStats(database::GetPoolStatsOutput),

    // CITESCRAPE (2 tools)
    citescrape::SCRAPE_URL => ScrapeUrl(citescrape::ScrapeUrlOutput),
    citescrape::WEB_SEARCH => WebSearch(citescrape::WebSearchOutput),

    // PROCESS (2 tools)
    process::PROCESS_LIST => ProcessList(process::ProcessListOutput),
    process::PROCESS_KILL => ProcessKill(process::ProcessKillOutput),

    // TERMINAL (1 tool)
    terminal::TERMINAL => Terminal(terminal::TerminalOutput),

    // CLAUDE AGENT (1 tool)
    claude_agent::CLAUDE_AGENT => ClaudeAgent(claude_agent::ClaudeAgentOutput),

    // MEMORY (4 tools)
    memory::MEMORY_MEMORIZE => MemoryMemorize(memory::MemorizeOutput),
    memory::MEMORY_RECALL => MemoryRecall(memory::RecallOutput),
    memory::MEMORY_LIST_LIBRARIES => MemoryListLibraries(memory::ListMemoryLibrariesOutput),
    memory::MEMORY_CHECK_MEMORIZE_STATUS => MemoryCheckMemorizeStatus(memory::CheckMemorizeStatusOutput),

    // CONFIG (2 tools)
    config::CONFIG_GET => ConfigGet(config::ConfigGetOutput),
    config::CONFIG_SET => ConfigSet(config::ConfigSetOutput),

    // PROMPT (4 tools)
    prompt::PROMPT_ADD => PromptAdd(prompt::PromptAddOutput),
    prompt::PROMPT_GET => PromptGet(prompt::PromptGetOutput),
    prompt::PROMPT_DELETE => PromptDelete(prompt::PromptDeleteOutput),
    prompt::PROMPT_EDIT => PromptEdit(prompt::PromptEditOutput),

    // REASONING (2 tools)
    reasoning::REASONER => Reasoner(reasoning::ReasonerOutput),
    reasoning::SEQUENTIAL_THINKING => SequentialThinking(reasoning::SequentialThinkingOutput),

    // INTROSPECTION (2 tools)
    introspection::INSPECT_TOOL_CALLS => InspectToolCalls(introspection::InspectToolCallsOutput),
    introspection::INSPECT_USAGE_STATS => InspectUsage(introspection::InspectUsageOutput),
}
