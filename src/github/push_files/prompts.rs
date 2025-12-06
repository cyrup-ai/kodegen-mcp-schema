//! Prompt messages for github_push_files tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::PushFilesPromptArgs;

/// Prompt provider for push_files tool
///
/// This is the ONLY way to provide prompts for push_files - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct PushFilesPrompts;

impl PromptProvider for PushFilesPrompts {
    type PromptArgs = PushFilesPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_push_files to commit multiple files at once?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_push_files tool commits and pushes multiple files to a branch in a single atomic commit, ideal for batch updates.\n\n\
                 BASIC USAGE:\n\
                 1. Push single file:\n\
                    github_push_files({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"branch\": \"main\", \"message\": \"Update README\", \"files\": {\"README.md\": \"IyBUb2tpbw==\"}})\n\n\
                 2. Push multiple files:\n\
                    github_push_files({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"branch\": \"feature/docs\", \"message\": \"Add documentation\", \"files\": {\"docs/guide.md\": \"IyBHdWlkZQ==\", \"docs/api.md\": \"IyBBUEk=\"}})\n\n\
                 3. Update configuration:\n\
                    github_push_files({\"owner\": \"actix\", \"repo\": \"actix-web\", \"branch\": \"main\", \"message\": \"chore: Update configs\", \"files\": {\"Cargo.toml\": \"W3BhY2thZ2Vd\", \".github/workflows/ci.yml\": \"bmFtZTogQ0k=\"}})\n\n\
                 4. Batch code generation:\n\
                    github_push_files({\"owner\": \"serde-rs\", \"repo\": \"serde\", \"branch\": \"feature/codegen\", \"message\": \"feat: Generate bindings\", \"files\": {\"src/gen/mod.rs\": \"cHVi\", \"src/gen/types.rs\": \"dXNl\"}})\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - branch (required): Branch name to push to\n\
                 - message (required): Commit message for all files\n\
                 - files (required): Map of file paths to base64-encoded content\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo (for private repositories)\n\
                 - public_repo (for public repositories only)\n\n\
                 RESPONSE:\n\
                 Returns JSON with:\n\
                 - success: true/false\n\
                 - owner, repo, branch: Identifiers\n\
                 - files_pushed: Count of files committed\n\
                 - commit_sha: Created commit SHA\n\
                 - commit_message: Commit message used\n\
                 - html_url: Link to commit on GitHub\n\n\
                 COMMON WORKFLOWS:\n\
                 1. Documentation generation:\n\
                    - Generate API docs from code\n\
                    - Base64-encode all doc files\n\
                    - Push entire docs/ directory\n\
                    - Single commit for consistency\n\
                 2. Code generation:\n\
                    - Run code generator (OpenAPI, GraphQL, etc.)\n\
                    - Collect all generated files\n\
                    - Push to feature branch atomically\n\
                    - Create PR for review\n\
                 3. Batch configuration updates:\n\
                    - Modify multiple config files\n\
                    - Encode each file content\n\
                    - Push all changes together\n\
                    - Maintain atomic consistency\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Unauthenticated: 60 requests/hour\n\
                 - Check X-RateLimit-Remaining header\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found: Repository or branch doesn't exist\n\
                    Fix: Create branch first with github_create_branch\n\
                 2. 422 Unprocessable: Invalid file paths or base64 encoding\n\
                    Fix: Verify paths have no leading /; content is valid base64\n\
                 3. 409 Conflict: Branch protected or concurrent updates\n\
                    Fix: Check branch protection; retry with latest branch state\n\n\
                 BEST PRACTICES:\n\
                 - Always base64-encode file content before sending\n\
                 - Use for multiple files; single files use github_create_or_update_file\n\
                 - File paths are relative to repository root (no leading /)\n\
                 - All files committed atomically in single commit\n\
                 - Use descriptive commit messages (follows conventional commits)\n\
                 - Prefer pushing to feature branches, not main directly\n\
                 - Verify branch exists or create it first\n\
                 - Check branch protection rules before pushing\n\
                 - Maximum file size typically 100MB per file\n\
                 - Use for generated code, docs, or batch updates\n\
                 - Files map overwrites existing files, doesn't delete missing ones",
            ),
        },    ]
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]  // No customization arguments for this tool
    }
}

