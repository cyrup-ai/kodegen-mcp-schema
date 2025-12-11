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

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("feature_branch") => prompt_feature_branch(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]  // No customization arguments for this tool
    }
}

fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I push multiple files to a repository branch at once?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_push_files tool commits multiple files atomically to a branch in a single commit. \
                 Use this for batch updates, generated code, or documentation updates where all files should be committed together.\n\n\
                 BASIC USAGE:\n\
                 1. Push multiple documentation files:\n\
                    github_push_files({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"branch\": \"feature/docs\",\n\
                      \"message\": \"docs: Add API documentation\",\n\
                      \"files\": {\n\
                        \"docs/guide.md\": \"IyBHdWlkZQ==\",\n\
                        \"docs/api.md\": \"IyBBUEk=\",\n\
                        \"docs/examples.md\": \"IyBFeGFtcGxlcw==\"\n\
                      }\n\
                    })\n\n\
                 2. Push configuration batch:\n\
                    github_push_files({\n\
                      \"owner\": \"actix\",\n\
                      \"repo\": \"actix-web\",\n\
                      \"branch\": \"main\",\n\
                      \"message\": \"chore: Update project configs\",\n\
                      \"files\": {\n\
                        \"Cargo.toml\": \"W3BhY2thZ2Vd...\",\n\
                        \".github/workflows/ci.yml\": \"bmFtZTogQ0k...\",\n\
                        \"rustfmt.toml\": \"ZWRpdGlvbj0y...\"\n\
                      }\n\
                    })\n\n\
                 3. Push generated code:\n\
                    github_push_files({\n\
                      \"owner\": \"serde-rs\",\n\
                      \"repo\": \"serde\",\n\
                      \"branch\": \"feature/codegen\",\n\
                      \"message\": \"feat: Generate type bindings\",\n\
                      \"files\": {\n\
                        \"src/gen/mod.rs\": \"cHVi...\",\n\
                        \"src/gen/types.rs\": \"dXNl...\",\n\
                        \"src/gen/client.rs\": \"c3RydWN0...\"\n\
                      }\n\
                    })\n\n\
                 PARAMETERS:\n\
                 All parameters are required:\n\
                 - owner: Repository owner (username or organization name)\n\
                 - repo: Repository name\n\
                 - branch: Target branch name (branch must already exist)\n\
                 - message: Commit message that applies to all files in the batch\n\
                 - files: Map of file paths (relative to repo root) to base64-encoded content\n\n\
                 Each file path is relative to the repository root with no leading slash.\n\
                 Example paths: \"README.md\", \"src/lib.rs\", \"docs/guide.md\"\n\n\
                 FILE ENCODING REQUIREMENT:\n\
                 ALL file content MUST be base64-encoded before passing to github_push_files.\n\
                 The tool validates that all content is valid base64 - non-encoded content will be rejected.\n\n\
                 Common mistake: Forgetting to encode file content before calling the tool.\n\
                 Always encode: file_content → base64_encode(file_content) → pass to tool\n\n\
                 Example encoding workflow:\n\
                 1. Read file content: \"# Hello World\"\n\
                 2. Base64 encode: \"IyBIZWxsbyBXb3JsZA==\"\n\
                 3. Add to files map: {\"README.md\": \"IyBIZWxsbyBXb3JsZA==\"}\n\n\
                 DOCUMENTATION UPDATE WORKFLOW:\n\
                 Use case: Generate API documentation and push entire docs directory atomically.\n\n\
                 1. Generate documentation from code (external tool):\n\
                    - Run rustdoc, typedoc, or similar documentation generator\n\
                    - Collect all generated .md or .html files\n\n\
                 2. Base64-encode all documentation files:\n\
                    - Read each file's content\n\
                    - Apply base64 encoding to each\n\
                    - Build files map with path → encoded content\n\n\
                 3. Push to repository:\n\
                    github_push_files({\n\
                      \"owner\": \"tokio-rs\",\n\
                      \"repo\": \"tokio\",\n\
                      \"branch\": \"main\",\n\
                      \"message\": \"docs: Update API documentation\",\n\
                      \"files\": {\n\
                        \"docs/tokio/index.html\": \"[base64]\",\n\
                        \"docs/tokio/runtime.html\": \"[base64]\",\n\
                        \"docs/tokio/sync.html\": \"[base64]\",\n\
                        \"docs/search-index.js\": \"[base64]\"\n\
                      }\n\
                    })\n\n\
                 All four files are committed together in a single atomic commit.\n\
                 If any file fails validation, none are committed (atomic operation).\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with appropriate scopes:\n\
                 - \"repo\" scope for private repositories (full access)\n\
                 - \"public_repo\" scope for public repositories only (limited access)\n\n\
                 Never embed credentials directly in tool calls.\n\
                 The tool automatically reads GITHUB_TOKEN from the environment.\n\n\
                 ERROR HANDLING:\n\
                 Common errors and solutions:\n\n\
                 1. 404 Not Found - Branch doesn't exist:\n\
                    - Verify branch name is correct\n\
                    - Create branch first using git_branch_create or github_create_branch\n\
                    - Ensure repository owner and name are correct\n\n\
                 2. 422 Unprocessable Entity - Invalid base64 encoding:\n\
                    - Verify all file content is properly base64-encoded\n\
                    - Check for file paths with leading slashes (should be relative)\n\
                    - Validate base64 encoding before calling tool\n\n\
                 Fix validation errors before retrying to avoid rate limit impacts.\n\n\
                 BEST PRACTICES:\n\
                 1. Always base64-encode file content - the tool requires and validates this format\n\n\
                 2. Use for multiple files only - single file updates should use github_create_or_update_file\n\n\
                 3. File paths are relative to repository root with no leading slash:\n\
                    - Correct: \"src/lib.rs\", \"README.md\", \"docs/guide.md\"\n\
                    - Wrong: \"/src/lib.rs\", \"./README.md\"\n\n\
                 4. Atomic operation - all files are committed together in one commit:\n\
                    - If any file fails validation, none are committed\n\
                    - All files share the same commit message\n\
                    - Single commit SHA is returned for the entire batch\n\n\
                 5. Files map overwrites existing files but doesn't delete missing ones:\n\
                    - Pushing {\"README.md\": \"...\"} updates README.md only\n\
                    - Other files in the repository remain unchanged\n\
                    - To delete files, use separate delete operations",
            ),
        },
    ]
}

fn prompt_feature_branch() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use push_files to commit generated code to a feature branch?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use github_push_files for feature branch workflows when generating code, creating batch updates, or preparing pull requests. \
                 This enables atomic commits of all generated files with isolated changes for easy review.\n\n\
                 FEATURE BRANCH PATTERN:\n\
                 Feature branches must exist before pushing files. Follow this two-step pattern:\n\n\
                 Step 1 - Create the feature branch:\n\
                 git_branch_create({\n\
                   \"path\": \"/home/user/tokio\",\n\
                   \"name\": \"feature/generated-code\",\n\
                   \"checkout\": false\n\
                 })\n\n\
                 Step 2 - Push files to the feature branch:\n\
                 github_push_files({\n\
                   \"owner\": \"tokio-rs\",\n\
                   \"repo\": \"tokio\",\n\
                   \"branch\": \"feature/generated-code\",\n\
                   \"message\": \"feat: Add generated bindings\",\n\
                   \"files\": {\n\
                     \"src/generated/client.rs\": \"[base64 content]\",\n\
                     \"src/generated/models.rs\": \"[base64 content]\"\n\
                   }\n\
                 })\n\n\
                 Why separate steps?\n\
                 - github_push_files requires the branch to exist on GitHub\n\
                 - git_branch_create creates the branch locally, then you push files to it\n\
                 - This enables pushing to a new branch without checking it out locally\n\n\
                 COMMON GENERATION SCENARIOS:\n\n\
                 1. OpenAPI Client Generation:\n\
                    Use case: Generate TypeScript or Rust client from OpenAPI spec\n\n\
                    Workflow:\n\
                    - Run OpenAPI generator (external tool)\n\
                    - Collect generated client files: client.rs, models.rs, apis/\n\
                    - Base64-encode all generated files\n\
                    - Push to feature/openapi-client branch\n\n\
                    Example:\n\
                    github_push_files({\n\
                      \"owner\": \"myorg\",\n\
                      \"repo\": \"api-client\",\n\
                      \"branch\": \"feature/openapi-client\",\n\
                      \"message\": \"feat: Generate OpenAPI client from v3.1 spec\",\n\
                      \"files\": {\n\
                        \"src/client.rs\": \"[base64]\",\n\
                        \"src/models/user.rs\": \"[base64]\",\n\
                        \"src/models/organization.rs\": \"[base64]\",\n\
                        \"src/apis/users_api.rs\": \"[base64]\",\n\
                        \"src/apis/orgs_api.rs\": \"[base64]\"\n\
                      }\n\
                    })\n\n\
                 2. GraphQL Schema Generation:\n\
                    Use case: Generate GraphQL types and resolvers from schema\n\n\
                    Workflow:\n\
                    - Run GraphQL codegen (graphql-code-generator, apollo, etc.)\n\
                    - Collect schema.ts, resolvers.ts, types.ts\n\
                    - Encode and push to feature/graphql-update\n\n\
                    Example:\n\
                    github_push_files({\n\
                      \"owner\": \"myorg\",\n\
                      \"repo\": \"graphql-server\",\n\
                      \"branch\": \"feature/graphql-update\",\n\
                      \"message\": \"feat: Update GraphQL schema and types\",\n\
                      \"files\": {\n\
                        \"src/generated/schema.ts\": \"[base64]\",\n\
                        \"src/generated/resolvers.ts\": \"[base64]\",\n\
                        \"src/generated/types.ts\": \"[base64]\"\n\
                      }\n\
                    })\n\n\
                 3. Code Scaffold Generation:\n\
                    Use case: Generate project scaffolding from templates\n\n\
                    Workflow:\n\
                    - Run scaffold generator (cookiecutter, yeoman, custom tool)\n\
                    - Collect all generated project files\n\
                    - Push to feature/scaffold-[date] branch\n\n\
                    Example:\n\
                    github_push_files({\n\
                      \"owner\": \"myorg\",\n\
                      \"repo\": \"microservices\",\n\
                      \"branch\": \"feature/scaffold-2024-12-10\",\n\
                      \"message\": \"feat: Scaffold new user service\",\n\
                      \"files\": {\n\
                        \"services/user/Cargo.toml\": \"[base64]\",\n\
                        \"services/user/src/main.rs\": \"[base64]\",\n\
                        \"services/user/src/handlers.rs\": \"[base64]\",\n\
                        \"services/user/src/models.rs\": \"[base64]\",\n\
                        \"services/user/tests/integration.rs\": \"[base64]\"\n\
                      }\n\
                    })\n\n\
                 COMPLETE WORKFLOW EXAMPLE:\n\
                 End-to-end code generation pipeline with feature branch and PR creation:\n\n\
                 1. Fetch latest main branch:\n\
                    git_fetch({\n\
                      \"path\": \"/home/user/project\",\n\
                      \"remote\": \"origin\",\n\
                      \"branch\": \"main\"\n\
                    })\n\n\
                 2. Create feature branch from main:\n\
                    git_branch_create({\n\
                      \"path\": \"/home/user/project\",\n\
                      \"name\": \"feature/generated-openapi\",\n\
                      \"checkout\": false\n\
                    })\n\n\
                 3. Run code generator (external tool, not MCP):\n\
                    - Execute: openapi-generator generate -i spec.yaml -o ./generated\n\
                    - Generator creates files in ./generated directory\n\n\
                 4. Collect and base64-encode all generated files:\n\
                    - Read each file from ./generated\n\
                    - Apply base64 encoding to content\n\
                    - Build files map: {path: base64_content}\n\n\
                 5. Push all generated files to feature branch atomically:\n\
                    github_push_files({\n\
                      \"owner\": \"myorg\",\n\
                      \"repo\": \"api-client\",\n\
                      \"branch\": \"feature/generated-openapi\",\n\
                      \"message\": \"feat: Generate OpenAPI client from updated spec\",\n\
                      \"files\": {\n\
                        \"src/generated/client.rs\": \"[base64 content]\",\n\
                        \"src/generated/models.rs\": \"[base64 content]\",\n\
                        \"src/generated/apis.rs\": \"[base64 content]\"\n\
                      }\n\
                    })\n\n\
                 6. Create pull request (optional):\n\
                    github_create_pull_request({\n\
                      \"owner\": \"myorg\",\n\
                      \"repo\": \"api-client\",\n\
                      \"title\": \"Generate OpenAPI client from updated spec\",\n\
                      \"head\": \"feature/generated-openapi\",\n\
                      \"base\": \"main\",\n\
                      \"body\": \"Auto-generated client code from OpenAPI spec v3.1\"\n\
                    })\n\n\
                 7. Review, approve, and merge workflow:\n\
                    - Team reviews PR on GitHub\n\
                    - CI runs tests on feature branch\n\
                    - Approve and merge to main when ready\n\n\
                 ATOMIC OPERATION BENEFITS:\n\
                 When pushing multiple generated files to a feature branch:\n\n\
                 1. All files committed together in single commit:\n\
                    - No partial state where some files are committed and others aren't\n\
                    - Repository remains in consistent state\n\n\
                 2. Single commit message for entire generation:\n\
                    - Clear git history: one commit = one generation event\n\
                    - Easy to understand what changed and why\n\n\
                 3. Easy to revert if needed:\n\
                    - Revert one commit to undo entire generation\n\
                    - No need to track down multiple commits\n\n\
                 4. PR reviewer sees all changes at once:\n\
                    - Feature branch shows complete generated code\n\
                    - Reviewer can understand full impact of generation\n\
                    - All files appear in one PR diff\n\n\
                 BATCH UPDATES FOR MULTIPLE REPOSITORIES:\n\
                 Pattern for updating multiple repositories with same generated code:\n\n\
                 for each repository in [repo1, repo2, repo3]:\n\
                   1. Create feature branch in repository\n\
                   2. Push files with github_push_files (one call per repo)\n\
                   3. Create pull request for review\n\n\
                 Note: Atomic per-repository, not across repositories.\n\
                 Each repository gets its own feature branch and commit.\n\n\
                 Example loop:\n\
                 repos = [\"service-a\", \"service-b\", \"service-c\"]\n\
                 for repo in repos:\n\
                   github_push_files({\n\
                     \"owner\": \"myorg\",\n\
                     \"repo\": repo,\n\
                     \"branch\": \"feature/update-shared-types\",\n\
                     \"message\": \"feat: Update shared type definitions\",\n\
                     \"files\": {\n\
                       \"src/types/shared.rs\": \"[base64 same content]\"\n\
                     }\n\
                   })\n\n\
                 ERROR HANDLING:\n\
                 Common errors when working with feature branches:\n\n\
                 1. Branch not found (404):\n\
                    - Cause: Feature branch doesn't exist on GitHub yet\n\
                    - Fix: Create branch first with git_branch_create\n\
                    - Verify branch was created successfully before pushing files\n\n\
                 2. Invalid base64 encoding (422):\n\
                    - Cause: File content not properly base64-encoded\n\
                    - Fix: Verify encoding before calling github_push_files\n\
                    - Test: Decode and re-encode to validate format\n\n\
                 3. Conflict or concurrent updates (409):\n\
                    - Cause: Feature branch was updated elsewhere\n\
                    - Fix: Rebase feature branch on latest main\n\
                    - Or: Create new feature branch with updated name",
            ),
        },
    ]
}
