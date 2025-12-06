//! Prompt messages for git_add tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitAddPromptArgs;

/// Prompt provider for git_add tool
///
/// This is the ONLY way to provide prompts for git_add - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct AddPrompts;

impl PromptProvider for AddPrompts {
    type PromptArgs = GitAddPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("patterns") => prompt_patterns(),
            _ => prompt_specific_files(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (specific_files, patterns)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Adding specific individual files
fn prompt_specific_files() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I add specific files to the staging area?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Stage specific files for your next commit using git_add. This gives you precise control over what gets committed.\n\n\
                 STAGING SPECIFIC FILES:\n\n\
                 1. Add single file:\n\
                    git_add({\n\
                        \"path\": \"/project\",\n\
                        \"files\": [\"src/main.rs\"]\n\
                    })\n\n\
                 2. Add multiple files:\n\
                    git_add({\n\
                        \"path\": \"/project\",\n\
                        \"files\": [\n\
                            \"src/lib.rs\",\n\
                            \"src/utils.rs\",\n\
                            \"Cargo.toml\"\n\
                        ]\n\
                    })\n\n\
                 3. Add new directory:\n\
                    git_add({\n\
                        \"path\": \"/project\",\n\
                        \"files\": [\"src/handlers/\"]\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"staged_files\": [\"src/main.rs\"],\n\
                   \"success\": true\n\
                 }\n\n\
                 WHEN TO USE SPECIFIC FILES:\n\
                 - Precise control over what's committed\n\
                 - Separating changes into logical commits\n\
                 - Avoiding accidental staging of unrelated work\n\
                 - Creating focused, reviewable commits\n\n\
                 FILE PATH RULES:\n\
                 - Paths are relative to repository root\n\
                 - Use forward slashes (/) even on Windows\n\
                 - Directories should end with /\n\
                 - Can use absolute paths within repository\n\n\
                 BEST PRACTICES:\n\
                 - Add only related files together\n\
                 - Use git_status before staging to see what changed\n\
                 - Stage feature files separately from bug fixes\n\
                 - Keep commits atomic (one logical change)\n\
                 - Review changes before staging with git_diff",
            ),
        },
    ]
}

/// Adding files by patterns
fn prompt_patterns() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I add files using patterns or wildcards?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use glob patterns to stage multiple files efficiently. Patterns help you add files by type, directory, or naming convention.\n\n\
                 STAGING BY PATTERN:\n\n\
                 1. All Rust files:\n\
                    git_add({\n\
                        \"path\": \"/project\",\n\
                        \"files\": [\"*.rs\"]\n\
                    })\n\n\
                 2. All files in directory:\n\
                    git_add({\n\
                        \"path\": \"/project\",\n\
                        \"files\": [\"src/handlers/*\"]\n\
                    })\n\n\
                 3. Multiple patterns:\n\
                    git_add({\n\
                        \"path\": \"/project\",\n\
                        \"files\": [\"*.rs\", \"*.toml\"]\n\
                    })\n\n\
                 4. Recursive pattern:\n\
                    git_add({\n\
                        \"path\": \"/project\",\n\
                        \"files\": [\"src/**/*.rs\"]\n\
                    })\n\n\
                 PATTERN SYNTAX:\n\
                 - *: Match any characters in filename\n\
                   Example: *.rs matches main.rs, lib.rs\n\
                 - **: Match any subdirectories\n\
                   Example: src/**/*.rs matches all Rust files in src/ tree\n\
                 - ?: Match single character\n\
                   Example: test?.rs matches test1.rs, test2.rs\n\
                 - [abc]: Match character class\n\
                   Example: file[123].rs matches file1.rs, file2.rs, file3.rs\n\n\
                 USEFUL PATTERNS:\n\
                 - \"src/*.rs\": All Rust files directly in src/\n\
                 - \"**/*_test.rs\": All test files anywhere\n\
                 - \"*.{rs,toml}\": All Rust and TOML files\n\
                 - \"docs/**/*.md\": All Markdown files in docs/ tree\n\
                 - \"src/*/mod.rs\": All mod.rs files in src/ subdirectories\n\n\
                 PRACTICAL EXAMPLES:\n\n\
                 Stage all source code:\n\
                 git_add({\n\
                     \"path\": \"/project\",\n\
                     \"files\": [\"src/**/*.rs\", \"tests/**/*.rs\"]\n\
                 })\n\n\
                 Stage all configuration:\n\
                 git_add({\n\
                     \"path\": \"/project\",\n\
                     \"files\": [\"*.toml\", \"*.yaml\", \".config/*\"]\n\
                 })\n\n\
                 Stage all documentation:\n\
                 git_add({\n\
                     \"path\": \"/project\",\n\
                     \"files\": [\"*.md\", \"docs/**/*\"]\n\
                 })\n\n\
                 PATTERN SAFETY:\n\
                 - Always review with git_status before using broad patterns\n\
                 - Patterns respect .gitignore (won't stage ignored files)\n\
                 - Be careful with * at root level (might stage too much)\n\
                 - Test patterns with git_status to see what matches",
            ),
        },
    ]
}
