//! Prompt messages for github_get_file_contents tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GetFileContentsPromptArgs;

/// Prompt provider for get_file_contents tool
///
/// This is the ONLY way to provide prompts for get_file_contents - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GetFileContentsPrompts;

impl PromptProvider for GetFileContentsPrompts {
    type PromptArgs = GetFileContentsPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("files") => prompt_files(),
            Some("branches") => prompt_branches(),
            _ => prompt_files(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (files, branches)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Reading file contents from GitHub repositories
fn prompt_files() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I read file contents from GitHub repositories?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_get_file_contents tool retrieves file contents from any GitHub repository. Use it to read source code, configs, documentation, and more.\n\n\
                 READING FILE CONTENTS:\n\n\
                 1. Basic file read (default branch):\n\
                    github_get_file_contents({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"path\": \"README.md\"\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"type\": \"file\",\n\
                   \"name\": \"README.md\",\n\
                   \"path\": \"README.md\",\n\
                   \"sha\": \"abc1234567890def...\",\n\
                   \"size\": 5432,\n\
                   \"content\": \"# The Rust Programming Language\\n\\nThis is...\",\n\
                   \"encoding\": \"base64\",\n\
                   \"html_url\": \"https://github.com/rust-lang/rust/blob/master/README.md\",\n\
                   \"download_url\": \"https://raw.githubusercontent.com/rust-lang/rust/master/README.md\"\n\
                 }\n\n\
                 2. Read source file:\n\
                    github_get_file_contents({\n\
                      \"owner\": \"tokio-rs\",\n\
                      \"repo\": \"tokio\",\n\
                      \"path\": \"tokio/src/runtime/mod.rs\"\n\
                    })\n\n\
                 3. Read configuration file:\n\
                    github_get_file_contents({\n\
                      \"owner\": \"actix\",\n\
                      \"repo\": \"actix-web\",\n\
                      \"path\": \"Cargo.toml\"\n\
                    })\n\n\
                 4. Read nested file:\n\
                    github_get_file_contents({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \"src/api/handlers/user.rs\"\n\
                    })\n\n\
                 FILE METADATA IN RESPONSE:\n\
                 - name: File name only (\"README.md\")\n\
                 - path: Full path in repository (\"docs/guide/README.md\")\n\
                 - sha: Git SHA hash of file content (use for change detection)\n\
                 - size: File size in bytes\n\
                 - content: File content (automatically decoded from base64)\n\
                 - encoding: Original encoding (\"base64\" for GitHub API)\n\
                 - html_url: Link to view file on GitHub website\n\
                 - download_url: Direct download link to raw file content\n\n\
                 CONTENT HANDLING:\n\
                 - Text files: Returned as decoded UTF-8 strings\n\
                 - Binary files: Returned as base64-encoded strings\n\
                 - Large files (>1MB): May require download_url for full content\n\
                 - Empty files: Content is empty string\n\n\
                 AUTHENTICATION:\n\
                 - Public repos: No auth required (rate limited to 60 req/hour)\n\
                 - Private repos: Requires GITHUB_TOKEN with repo scope\n\
                 - With token: 5,000 requests/hour rate limit\n\n\
                 ERROR HANDLING:\n\
                 - 404: File not found or repo doesn't exist\n\
                 - 403: Private repo without proper authentication\n\
                 - 422: Invalid path format\n\n\
                 BEST PRACTICES:\n\
                 - Use sha field to detect changes\n\
                 - Use download_url for files >1MB\n\
                 - Cache results when appropriate\n\
                 - Respect rate limits",
            ),
        },
    ]
}

/// Reading from specific branches, tags, and commits
fn prompt_branches() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I read files from specific branches, tags, or commits?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use the ref_name parameter to read files from any branch, tag, or commit. This is crucial for comparing versions or reading historical code.\n\n\
                 READING FROM BRANCHES:\n\n\
                 1. Read from specific branch:\n\
                    github_get_file_contents({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"path\": \"src/lib.rs\",\n\
                      \"ref_name\": \"master\"\n\
                    })\n\n\
                 2. Read from feature branch:\n\
                    github_get_file_contents({\n\
                      \"owner\": \"actix\",\n\
                      \"repo\": \"actix-web\",\n\
                      \"path\": \"README.md\",\n\
                      \"ref_name\": \"feature/http3\"\n\
                    })\n\n\
                 READING FROM TAGS:\n\n\
                 1. Read from release tag:\n\
                    github_get_file_contents({\n\
                      \"owner\": \"serde-rs\",\n\
                      \"repo\": \"serde\",\n\
                      \"path\": \"Cargo.toml\",\n\
                      \"ref_name\": \"v1.0.195\"\n\
                    })\n\n\
                 2. Read from version tag:\n\
                    github_get_file_contents({\n\
                      \"owner\": \"clap-rs\",\n\
                      \"repo\": \"clap\",\n\
                      \"path\": \"CHANGELOG.md\",\n\
                      \"ref_name\": \"v4.5.0\"\n\
                    })\n\n\
                 READING FROM COMMITS:\n\n\
                 Read from commit SHA (full or short):\n\
                 github_get_file_contents({\n\
                   \"owner\": \"rust-lang\",\n\
                   \"repo\": \"rust\",\n\
                   \"path\": \"README.md\",\n\
                   \"ref_name\": \"abc1234\"\n\
                 })\n\n\
                 REF_NAME PARAMETER:\n\
                 - Optional: Omit for default branch\n\
                 - Branch: \"main\", \"develop\", \"feature/new-api\"\n\
                 - Tag: \"v1.0.0\", \"release-2024-01\"\n\
                 - Commit SHA: Full or short (7+ chars)\n\n\
                 REF RESOLUTION:\n\
                 GitHub resolves in order: branch → tag → commit SHA\n\n\
                 ERROR HANDLING:\n\
                 - 404: ref_name not found\n\
                 - 422: Invalid ref_name format\n\n\
                 BEST PRACTICES:\n\
                 - Use tags for stable versions\n\
                 - Use commit SHAs for exact snapshots\n\
                 - Cache results by (owner, repo, path, ref_name)",
            ),
        },
    ]
}
