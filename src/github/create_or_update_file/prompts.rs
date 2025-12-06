//! Prompt messages for github_create_or_update_file tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::CreateOrUpdateFilePromptArgs;

/// Prompt provider for create_or_update_file tool
///
/// This is the ONLY way to provide prompts for create_or_update_file - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct CreateOrUpdateFilePrompts;

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create or update files in a GitHub repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_create_or_update_file tool creates a new file or updates an existing file in a GitHub repository via a commit.\n\n\
                 BASIC EXAMPLES:\n\
                 1. Create new file:\n\
                    github_create_or_update_file({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"path\": \"src/new_module.rs\", \"content\": \"cHViIGZuIG5ld19mdW5jdGlvbigpIHt9\", \"message\": \"Add new module\", \"branch\": \"main\"})\n\n\
                 2. Update existing file:\n\
                    github_create_or_update_file({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"path\": \"README.md\", \"content\": \"IyBSdXN0\\n\\nUpdated\", \"message\": \"Update README\", \"branch\": \"main\"})\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - path (required): File path in repository (e.g., \"src/main.rs\")\n\
                 - content (required): Base64-encoded file content\n\
                 - message (required): Commit message describing the change\n\
                 - branch (required): Branch name to commit to\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo (for private repositories)\n\
                 - public_repo (for public repositories only)\n\n\
                 RESPONSE FORMAT:\n\
                 Returns JSON with:\n\
                 - success: true/false\n\
                 - owner, repo: Repository identifiers\n\
                 - path: File path modified\n\
                 - commit_sha: Created commit SHA\n\
                 - content_sha: New file content SHA\n\
                 - html_url: Link to file on GitHub\n\
                 - message: Status message\n\
                 - operation: \"created\" or \"updated\"\n\n\
                 WHEN TO USE BASIC APPROACH:\n\
                 - Single file operations without conflict concerns\n\
                 - Direct commits to branches where conflicts are unlikely\n\
                 - Creating new files that don't exist yet\n\
                 - Simple documentation or configuration updates\n\n\
                 COMMON SINGLE-FILE WORKFLOWS:\n\
                 1. Simple file creation:\n\
                    - Generate content (e.g., new source file)\n\
                    - Base64-encode the content\n\
                    - Commit directly to target branch\n\
                 2. Updating README or docs:\n\
                    - Modify documentation content\n\
                    - Encode and commit with descriptive message\n\
                    - Triggers GitHub Pages rebuild if configured\n\
                 3. Committing configuration changes:\n\
                    - Update config file (e.g., .github/workflows/ci.yml)\n\
                    - Commit to trigger CI/CD pipeline\n\n\
                 BEST PRACTICES:\n\
                 - Always base64-encode content before sending\n\
                 - Use descriptive commit messages\n\
                 - Test content encoding/decoding locally first\n\
                 - Verify branch protection rules before committing",
            ),
        },
    ]
}

fn prompt_sha_handling() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I safely update files and prevent SHA conflicts?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "SHA (Secure Hash Algorithm) handling prevents concurrent modification conflicts when updating existing files in GitHub repositories.\n\n\
                 SHA PARAMETER EXPLANATION:\n\
                 The sha parameter is the current file content SHA (blob SHA). When provided:\n\
                 - GitHub verifies the file hasn't changed since you read it\n\
                 - If SHA matches: update proceeds\n\
                 - If SHA doesn't match: 409 Conflict error returned\n\
                 - Prevents accidental overwriting of concurrent changes\n\n\
                 WHY SHA MATTERS:\n\
                 Files may change between when you read them and when you update them:\n\
                 - Another developer commits changes\n\
                 - Automated bot updates the file\n\
                 - CI/CD pipeline modifies content\n\
                 SHA ensures atomicity: your update applies to the exact version you read.\n\n\
                 COMPLETE WORKFLOW EXAMPLE:\n\
                 Step 1 - Read current file and get SHA:\n\
                 result = github_get_file_contents({\"owner\": \"rust-lang\", \"repo\": \"rust\", \"path\": \"Cargo.toml\", \"branch\": \"main\"})\n\
                 # Returns: {\"content\": \"W3BhY2thZ2Vd...\", \"sha\": \"abc123def456...\", ...}\n\n\
                 Step 2 - Modify content locally:\n\
                 decoded = base64_decode(result.content)\n\
                 modified = decoded.replace('version = \"1.0.0\"', 'version = \"1.1.0\"')\n\
                 encoded = base64_encode(modified)\n\n\
                 Step 3 - Update with SHA from step 1:\n\
                 github_create_or_update_file({\n\
                   \"owner\": \"rust-lang\",\n\
                   \"repo\": \"rust\",\n\
                   \"path\": \"Cargo.toml\",\n\
                   \"content\": encoded,\n\
                   \"message\": \"Bump version to 1.1.0\",\n\
                   \"branch\": \"main\",\n\
                   \"sha\": \"abc123def456...\"  # SHA from step 1\n\
                 })\n\n\
                 If the file changed between step 1 and step 3, GitHub returns 409 Conflict.\n\n\
                 ERROR HANDLING FOR 409 CONFLICT:\n\
                 When you receive 409 Conflict:\n\
                 1. The file SHA changed since you read it\n\
                 2. Someone else committed changes to the same file\n\
                 3. Your local modification is now based on outdated content\n\n\
                 How to fix:\n\
                 - Call github_get_file_contents again to get latest SHA\n\
                 - Re-apply your modifications to the new content\n\
                 - Retry github_create_or_update_file with new SHA\n\
                 - Consider merging logic if changes overlap\n\n\
                 When this happens in real workflows:\n\
                 - Multiple developers editing same file\n\
                 - Long-running operations (file changed during processing)\n\
                 - Concurrent CI/CD jobs updating configuration\n\n\
                 BEST PRACTICES FOR UPDATES:\n\
                 - Always fetch current SHA before updating (use github_get_file_contents)\n\
                 - Implement retry logic with exponential backoff for conflicts\n\
                 - Use meaningful commit messages describing what changed and why\n\
                 - For complex merges with conflicts, consider using git tool instead\n\
                 - In automated systems, add conflict detection and alert mechanisms\n\
                 - Keep modification operations fast to minimize conflict window\n\
                 - Log SHA values for debugging failed updates\n\n\
                 WHEN TO USE SHA HANDLING:\n\
                 - Multi-step operations: read → process → update\n\
                 - Concurrent access scenarios: multiple agents/users\n\
                 - CI/CD workflows where files change frequently\n\
                 - Critical files where data loss is unacceptable\n\
                 - Automated bots making incremental updates\n\n\
                 WHEN SHA IS NOT NEEDED:\n\
                 - Creating new files (no prior version exists)\n\
                 - You have exclusive access to the repository\n\
                 - Content is completely replaced (not modified)\n\
                 - Working on isolated feature branches with single author",
            ),
        },
    ]
}

fn prompt_branch_targeting() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create/update files on feature branches and integrate with PRs?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The branch parameter enables you to target specific branches for file commits, supporting feature branch workflows and PR integration.\n\n\
                 BRANCH PARAMETER USAGE:\n\
                 - branch (required): Name of the branch to commit to\n\
                 - Defaults to repository's default branch if you specify \"main\" or \"master\"\n\
                 - Can target any existing branch: feature/*, fix/*, docs/*, etc.\n\
                 - Branch must exist before committing (create with git_branch_create)\n\n\
                 CREATING ON FEATURE BRANCHES:\n\
                 Committing to feature branches instead of main:\n\
                 - Isolates work-in-progress changes\n\
                 - Enables code review before merging\n\
                 - Prevents breaking main branch\n\
                 - Supports parallel development\n\n\
                 Example:\n\
                 github_create_or_update_file({\n\
                   \"owner\": \"actix\",\n\
                   \"repo\": \"actix-web\",\n\
                   \"path\": \".github/workflows/ci.yml\",\n\
                   \"content\": \"bmFtZTogQ0kKb246IFtwdXNoLCBwdWxsX3JlcXVlc3Rd\",\n\
                   \"message\": \"Add CI workflow\",\n\
                   \"branch\": \"feature/ci-setup\"  # Target feature branch, not main\n\
                 })\n\n\
                 COMPLETE WORKFLOW EXAMPLE:\n\
                 Step 1 - Create feature branch:\n\
                 git_branch_create({\n\
                   \"owner\": \"tokio-rs\",\n\
                   \"repo\": \"tokio\",\n\
                   \"branch_name\": \"feature/update-config\",\n\
                   \"from_branch\": \"main\"\n\
                 })\n\n\
                 Step 2 - Make multiple file changes on same branch:\n\
                 github_create_or_update_file({\n\
                   \"owner\": \"tokio-rs\",\n\
                   \"repo\": \"tokio\",\n\
                   \"path\": \"Cargo.toml\",\n\
                   \"content\": \"W3BhY2thZ2Vd...\",\n\
                   \"message\": \"Update dependencies\",\n\
                   \"branch\": \"feature/update-config\"\n\
                 })\n\
                 github_create_or_update_file({\n\
                   \"owner\": \"tokio-rs\",\n\
                   \"repo\": \"tokio\",\n\
                   \"path\": \"README.md\",\n\
                   \"content\": \"IyBUb2tpbw==\",\n\
                   \"message\": \"Update README for new deps\",\n\
                   \"branch\": \"feature/update-config\"\n\
                 })\n\n\
                 Step 3 - Create PR from feature branch to main:\n\
                 github_create_pull_request({\n\
                   \"owner\": \"tokio-rs\",\n\
                   \"repo\": \"tokio\",\n\
                   \"title\": \"Update configuration and documentation\",\n\
                   \"head\": \"feature/update-config\",\n\
                   \"base\": \"main\",\n\
                   \"body\": \"Updates dependencies and corresponding docs\"\n\
                 })\n\n\
                 BRANCH NAMING PATTERNS:\n\
                 Use descriptive prefixes for clarity:\n\
                 - feature/* - New functionality (feature/add-metrics)\n\
                 - fix/* - Bug fixes (fix/memory-leak)\n\
                 - docs/* - Documentation only (docs/api-guide)\n\
                 - chore/* - Maintenance tasks (chore/update-deps)\n\n\
                 INTEGRATION WITH GITHUB WORKFLOWS:\n\
                 Creating files on PR branches enables:\n\
                 1. Commit triggers CI/CD:\n\
                    - Each file commit runs tests\n\
                    - Build status updates on PR\n\
                    - Linting and security checks\n\
                 2. Review before merge:\n\
                    - Code review on changed files\n\
                    - Request changes if needed\n\
                    - Approve when ready\n\
                 3. Automated testing:\n\
                    - Integration tests run on feature branch\n\
                    - Preview deployments created\n\
                    - Performance benchmarks compared\n\n\
                 BEST PRACTICES:\n\
                 - Always create feature branches from latest main\n\
                 - Use consistent branch naming conventions\n\
                 - Keep feature branches short-lived (merge quickly)\n\
                 - Delete branches after PR merge\n\
                 - Check branch protection rules before committing\n\
                 - For multiple related files, commit them to same branch\n\
                 - Set bot author info for automated commits",
            ),
        },
    ]
}

impl PromptProvider for CreateOrUpdateFilePrompts {
    type PromptArgs = CreateOrUpdateFilePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("sha_handling") => prompt_sha_handling(),
            Some("branch_targeting") => prompt_branch_targeting(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}
