//! GitHub API tools schema module

// Re-export all github tool name constants from kodegen_config
pub use kodegen_config::{
    GITHUB_ACCEPT_REPO_INVITATION, GITHUB_ADD_ISSUE_COMMENT, GITHUB_ADD_PULL_REQUEST_REVIEW_COMMENT,
    GITHUB_CODE_SCANNING_ALERTS, GITHUB_CREATE_BRANCH, GITHUB_CREATE_ISSUE,
    GITHUB_CREATE_OR_UPDATE_FILE, GITHUB_CREATE_PULL_REQUEST, GITHUB_CREATE_PULL_REQUEST_REVIEW,
    GITHUB_CREATE_RELEASE, GITHUB_CREATE_REPOSITORY, GITHUB_DELETE_BRANCH, GITHUB_DELETE_FILE,
    GITHUB_FORK_REPOSITORY, GITHUB_GET_COMMIT, GITHUB_GET_FILE_CONTENTS, GITHUB_GET_ISSUE,
    GITHUB_GET_ISSUE_COMMENTS, GITHUB_GET_ME, GITHUB_GET_PULL_REQUEST_FILES,
    GITHUB_GET_PULL_REQUEST_REVIEWS, GITHUB_GET_PULL_REQUEST_STATUS, GITHUB_LIST_BRANCHES,
    GITHUB_LIST_COMMITS, GITHUB_LIST_ISSUES, GITHUB_LIST_PULL_REQUESTS, GITHUB_LIST_REPOS,
    GITHUB_MERGE_PULL_REQUEST, GITHUB_PENDING_INVITATIONS, GITHUB_PUSH_FILE, GITHUB_PUSH_FILES,
    GITHUB_REQUEST_COPILOT_REVIEW, GITHUB_SEARCH_CODE, GITHUB_SEARCH_ISSUES,
    GITHUB_SEARCH_REPOSITORIES, GITHUB_SEARCH_USERS, GITHUB_SECRET_SCANNING_ALERTS,
    GITHUB_UPDATE_ISSUE, GITHUB_UPDATE_PULL_REQUEST,
};

// Search operations
pub mod search_code;
pub mod search_repositories;
pub mod search_issues;
pub mod search_users;

// Repository operations
pub mod list_repos;
pub mod create_repository;
pub mod fork_repository;
pub mod get_file_contents;
pub mod create_or_update_file;
pub mod delete_file;
pub mod push_file;
pub mod push_files;

// Release operations
pub mod create_release;
pub mod list_releases;

// Branch operations
pub mod list_branches;
pub mod create_branch;
pub mod delete_branch;
pub mod list_commits;

// Issue operations
pub mod list_issues;
pub mod get_issue;
pub mod create_issue;
pub mod update_issue;
pub mod get_issue_comments;
pub mod add_issue_comment;

// Pull request operations
pub mod list_pull_requests;
pub mod create_pull_request;
pub mod update_pull_request;
pub mod merge_pull_request;
pub mod get_pull_request_status;
pub mod get_pull_request_files;
pub mod get_pull_request_reviews;
pub mod create_pull_request_review;
pub mod add_pull_request_review_comment;
pub mod request_copilot_review;

// Security operations
pub mod code_scanning_alerts;
pub mod secret_scanning_alerts;

// User & commit operations
pub mod get_me;
pub mod get_commit;

// Collaboration operations
pub mod accept_repo_invitation;
pub mod pending_invitations;

// Re-export all tool types for convenient access at crate root
// Search operations
// Re-export search_code tool
pub use search_code::{
    SearchCodeArgs,
    GitHubSearchCodeOutput,
    GitHubCodeSearchResult,
    SearchCodePrompts,
};

// Re-export search_repositories tool
pub use search_repositories::{
    SearchRepositoriesArgs,
    GitHubSearchReposOutput,
    GitHubRepoSearchResult,
    SearchRepositoriesPromptArgs,
    SearchRepositoriesPrompts,
};

// Re-export search_issues tool
pub use search_issues::{
    SearchIssuesArgs,
    GitHubSearchIssuesOutput,
    SearchIssuesPromptArgs,
    SearchIssuesPrompts,
    // Note: GitHubIssueSummary is exported from list_issues to avoid duplicate exports
};

// Re-export search_users tool
pub use search_users::{
    SearchUsersArgs,
    GitHubSearchUsersOutput,
    GitHubUserSearchResult,
    SearchUsersPrompts,
};

// Repository operations
// Re-export list_repos tool
pub use list_repos::{
    GithubListReposPromptArgs,
    GithubListReposPrompts,
};

// Re-export create_repository tool
pub use create_repository::{
    CreateRepositoryArgs,
    GitHubCreateRepoOutput,
    CreateRepositoryPrompts,
};

// Re-export fork_repository tool
pub use fork_repository::{
    ForkRepositoryArgs,
    GitHubForkRepoOutput,
    GitHubForkRepositoryPromptArgs,
    ForkRepositoryPrompts,
};

// Re-export get_file_contents tool
pub use get_file_contents::{
    GetFileContentsArgs,
    GitHubGetFileContentsOutput,
    GitHubFileContent,
    GitHubDirectoryEntry,
    GetFileContentsPrompts,
};

// Re-export create_or_update_file tool
pub use create_or_update_file::{
    CreateOrUpdateFileArgs,
    GitHubCreateOrUpdateFileOutput,
    CreateOrUpdateFilePrompts,
};

// Re-export delete_file tool
pub use delete_file::{
    GithubDeleteFilePromptArgs,
    GithubDeleteFilePrompts,
};

// Re-export push_file tool
pub use push_file::{
    GithubPushFilePromptArgs,
    GithubPushFilePrompts,
};

// Re-export push_files tool
pub use push_files::{
    PushFilesArgs,
    GitHubPushFilesOutput,
    PushFilesPrompts,
};

// Branch operations
// Re-export list_branches tool
pub use list_branches::{
    ListBranchesArgs,
    GitHubListBranchesOutput,
    GitHubBranch,
    ListBranchesPrompts,
};

// Re-export create_branch tool
pub use create_branch::{
    CreateBranchArgs,
    GitHubCreateBranchOutput,
    CreateBranchPromptArgs,
    CreateBranchPrompts,
};

// Re-export delete_branch tool
pub use delete_branch::{
    DeleteBranchArgs,
    GitHubDeleteBranchOutput,
    DeleteBranchPromptArgs,
    DeleteBranchPrompts,
};

// Re-export list_commits tool
pub use list_commits::{
    ListCommitsArgs,
    GitHubListCommitsOutput,
    GitHubCommitSummary,
    ListCommitsPrompts,
};

// Issue operations
// Re-export list_issues tool
pub use list_issues::{
    ListIssuesArgs,
    GitHubListIssuesOutput,
    GitHubIssueSummary,
    GithubListIssuesPromptArgs,
    ListIssuesPrompts,
};

// Re-export get_issue tool
pub use get_issue::{
    GetIssueArgs,
    GitHubGetIssueOutput,
    GitHubIssue,
    GetIssuePromptArgs,
    GetIssuePrompts,
};

// Re-export create_issue tool
pub use create_issue::{
    CreateIssueArgs,
    GitHubCreateIssueOutput,
    CreateIssuePromptArgs,
    CreateIssuePrompts,
};

// Re-export update_issue tool
pub use update_issue::{
    UpdateIssueArgs,
    GitHubUpdateIssueOutput,
    UpdateIssuePromptArgs,
    UpdateIssuePrompts,
};

// Re-export get_issue_comments tool
pub use get_issue_comments::{
    GetIssueCommentsArgs,
    GitHubGetIssueCommentsOutput,
    GitHubComment,
    GetIssueCommentsPromptArgs,
    GetIssueCommentsPrompts,
};

// Re-export add_issue_comment tool
pub use add_issue_comment::{
    AddIssueCommentArgs,
    GitHubAddIssueCommentOutput,
    AddIssueCommentPromptArgs,
    AddIssueCommentPrompts,
};

// Pull request operations
// Re-export list_pull_requests tool
pub use list_pull_requests::{
    ListPullRequestsArgs,
    GitHubListPrsOutput,
    GitHubPrSummary,
    ListPullRequestsPromptArgs,
    ListPullRequestsPrompts,
};

// Re-export create_pull_request tool
pub use create_pull_request::{
    CreatePullRequestArgs,
    GitHubCreatePrOutput,
    CreatePullRequestPrompts,
};

// Re-export update_pull_request tool
pub use update_pull_request::{
    UpdatePullRequestArgs,
    GitHubUpdatePrOutput,
    UpdatePullRequestPromptArgs,
    UpdatePullRequestPrompts,
};

// Re-export merge_pull_request tool
pub use merge_pull_request::{
    MergePullRequestArgs,
    GitHubMergePrOutput,
    MergePullRequestPrompts,
};

// Re-export get_pull_request_status tool
pub use get_pull_request_status::{
    GetPullRequestStatusArgs,
    GitHubGetPrStatusOutput,
    GetPullRequestStatusPrompts,
};

// Re-export get_pull_request_files tool
pub use get_pull_request_files::{
    GetPullRequestFilesArgs,
    GitHubGetPrFilesOutput,
    GitHubPrFile,
    GetPullRequestFilesPrompts,
};

// Re-export get_pull_request_reviews tool
pub use get_pull_request_reviews::{
    GetPullRequestReviewsArgs,
    GitHubPrReviewsOutput,
    GitHubReview,
    GetPullRequestReviewsPromptArgs,
    GetPullRequestReviewsPrompts,
};

// Re-export create_pull_request_review tool
pub use create_pull_request_review::{
    CreatePullRequestReviewArgs,
    GitHubCreatePrReviewOutput,
    CreatePullRequestReviewPromptArgs,
    CreatePullRequestReviewPrompts,
};

// Re-export add_pull_request_review_comment tool
pub use add_pull_request_review_comment::{
    AddPullRequestReviewCommentArgs,
    GitHubAddPrReviewCommentOutput,
    AddPullRequestReviewCommentPromptArgs,
    AddPullRequestReviewCommentPrompts,
};

// Re-export request_copilot_review tool
pub use request_copilot_review::{
    RequestCopilotReviewArgs,
    GitHubRequestCopilotReviewOutput,
    RequestCopilotReviewPromptArgs,
    RequestCopilotReviewPrompts,
};

// Security operations
// Re-export code_scanning_alerts tool
pub use code_scanning_alerts::{
    CodeScanningAlertsArgs,
    GitHubCodeScanningAlertsOutput,
    GitHubCodeScanningAlert,
    CodeScanningAlertsPrompts,
};

// Re-export secret_scanning_alerts tool
pub use secret_scanning_alerts::{
    SecretScanningAlertsArgs,
    GitHubSecretScanningAlertsOutput,
    GitHubSecretScanningAlert,
    SecretScanningAlertsPrompts,
};

// User & commit operations
// Re-export get_me tool
pub use get_me::{
    GetMeArgs,
    GitHubGetMeOutput,
    GetMePrompts,
};

// Re-export get_commit tool
pub use get_commit::{
    GetCommitArgs,
    GitHubGetCommitOutput,
    GitHubCommitDetail,
    GitHubCommitStats,
    GitHubCommitFile,
    GetCommitPromptArgs,
    GetCommitPrompts,
};

// Release operations
// Re-export create_release tool
pub use create_release::{
    GithubCreateReleasePromptArgs,
    GithubCreateReleasePrompts,
};

// Re-export list_releases tool
pub use list_releases::{
    ListReleasesPromptArgs,
    ListReleasesPrompts,
};

// Collaboration operations
// Re-export accept_repo_invitation tool
pub use accept_repo_invitation::{
    GithubAcceptRepoInvitationPromptArgs,
    GithubAcceptRepoInvitationPrompts,
};

// Re-export pending_invitations tool
pub use pending_invitations::{
    GithubPendingInvitationsPromptArgs,
    GithubPendingInvitationsPrompts,
};
