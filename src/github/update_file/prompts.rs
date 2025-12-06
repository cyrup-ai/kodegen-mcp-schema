//! Prompt messages for github_update_file tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GithubUpdateFilePromptArgs;

/// Prompt provider for github_update_file tool
///
/// This is the ONLY way to provide prompts for github_update_file - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GithubUpdateFilePrompts;

impl PromptProvider for GithubUpdateFilePrompts {
    type PromptArgs = GithubUpdateFilePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("workflow") => prompt_workflow(),
            Some("branches") => prompt_branches(),
            Some("advanced") => prompt_advanced(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, workflow, branches, advanced, comprehensive)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO UPDATE GITHUB FILES
// ============================================================================

/// Basic file update with SHA requirement
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I update an existing file in a GitHub repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_update_file tool updates existing files in a GitHub repository. Here's how to use it:\n\n\
                 BASIC FILE UPDATE:\n\
                 1. Update existing file:\n\
                    github_update_file({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"config.json\",\n\
                        \"content\": \"{\\\"version\\\": \\\"2.0\\\", \\\"debug\\\": false}\",\n\
                        \"message\": \"Update version to 2.0\",\n\
                        \"sha\": \"abc123def456...\"\n\
                    })\n\n\
                 SHA REQUIREMENT (CRITICAL!):\n\
                 - MUST provide current file SHA (commit hash)\n\
                 - SHA ensures you're updating the correct version\n\
                 - Prevents accidental overwrites of concurrent changes\n\
                 - Get SHA from github_get_file_contents first\n\
                 - GitHub API rejects updates without matching SHA\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner (username or organization)\n\
                 - repo: Repository name\n\
                 - path: File path in repository (e.g., \"src/main.rs\", \"docs/README.md\")\n\
                 - content: New file content (complete file, not diff)\n\
                 - message: Commit message describing the change\n\
                 - sha: Current file SHA (40-character hex string)\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - branch: Target branch (default: repository default branch)\n\
                 - committer_name: Name for commit author\n\
                 - committer_email: Email for commit author\n\n\
                 RESPONSE:\n\
                 - commit: New commit information with SHA\n\
                 - content: Updated file metadata\n\
                 - sha: New file SHA after update\n\n\
                 COMMON EXAMPLES:\n\
                 1. Update configuration:\n\
                    github_update_file({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"backend\",\n\
                        \"path\": \"config/production.json\",\n\
                        \"content\": \"{\\\"timeout\\\": 30}\",\n\
                        \"message\": \"Increase timeout to 30s\",\n\
                        \"sha\": \"a1b2c3d4e5f6...\"\n\
                    })\n\n\
                 2. Update documentation:\n\
                    github_update_file({\n\
                        \"owner\": \"docs-team\",\n\
                        \"repo\": \"guides\",\n\
                        \"path\": \"README.md\",\n\
                        \"content\": \"# Updated Guide\\n\\nNew content...\",\n\
                        \"message\": \"Update installation instructions\",\n\
                        \"sha\": \"f6e5d4c3b2a1...\"\n\
                    })\n\n\
                 3. Update source code:\n\
                    github_update_file({\n\
                        \"owner\": \"company\",\n\
                        \"repo\": \"app\",\n\
                        \"path\": \"src/version.rs\",\n\
                        \"content\": \"pub const VERSION: &str = \\\"2.0.0\\\";\",\n\
                        \"message\": \"Bump version to 2.0.0\",\n\
                        \"sha\": \"1234567890ab...\"\n\
                    })\n\n\
                 ERROR HANDLING:\n\
                 - 404: File not found or repository doesn't exist\n\
                 - 409: SHA mismatch (file changed since you retrieved it)\n\
                 - 422: Invalid parameters or content\n\
                 - Solution for 409: Get latest file with github_get_file_contents and retry\n\n\
                 CONTENT ENCODING:\n\
                 - Content should be plain text string\n\
                 - Tool handles base64 encoding automatically\n\
                 - No need to encode content yourself\n\
                 - Binary files: use base64-encoded string\n\n\
                 BEST PRACTICES:\n\
                 - Always get current SHA with github_get_file_contents first\n\
                 - Write descriptive commit messages\n\
                 - Verify content is complete (not just a diff)\n\
                 - Handle SHA mismatch errors by refetching and retrying\n\
                 - Use branches for experimental changes",
            ),
        },
    ]
}

/// Read-modify-update workflow pattern
fn prompt_workflow() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What's the complete workflow for reading, modifying, and updating a file?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The read-modify-update pattern is the recommended workflow for updating files. This ensures you have the correct SHA and current content.\n\n\
                 READ-MODIFY-UPDATE WORKFLOW:\n\
                 Step 1: READ - Get current file content and SHA\n\
                 github_get_file_contents({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"config.json\"\n\
                 })\n\
                 // Returns: {\"content\": \"...\", \"sha\": \"abc123...\", ...}\n\n\
                 Step 2: MODIFY - Parse and modify the content\n\
                 // Example: Parse JSON, modify values, serialize back\n\
                 let data = JSON.parse(content);\n\
                 data.version = \"2.0\";\n\
                 data.features.push(\"new-feature\");\n\
                 let modified_content = JSON.stringify(data, null, 2);\n\n\
                 Step 3: UPDATE - Write back with SHA from step 1\n\
                 github_update_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"config.json\",\n\
                     \"content\": modified_content,\n\
                     \"message\": \"Update version and add new feature\",\n\
                     \"sha\": \"abc123...\"  // From step 1\n\
                 })\n\n\
                 COMPLETE EXAMPLES:\n\
                 1. Update JSON configuration:\n\
                    // Step 1: Get current file\n\
                    result = github_get_file_contents({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"api\",\n\
                        \"path\": \"config/settings.json\"\n\
                    })\n\
                    \n\
                    // Step 2: Modify\n\
                    config = JSON.parse(result.content)\n\
                    config.api_timeout = 60\n\
                    config.retry_count = 3\n\
                    new_content = JSON.stringify(config, null, 2)\n\
                    \n\
                    // Step 3: Update\n\
                    github_update_file({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"api\",\n\
                        \"path\": \"config/settings.json\",\n\
                        \"content\": new_content,\n\
                        \"message\": \"Increase API timeout and retry count\",\n\
                        \"sha\": result.sha\n\
                    })\n\n\
                 2. Update YAML file:\n\
                    // Step 1: Read\n\
                    file = github_get_file_contents({\n\
                        \"owner\": \"devops\",\n\
                        \"repo\": \"configs\",\n\
                        \"path\": \".github/workflows/ci.yml\"\n\
                    })\n\
                    \n\
                    // Step 2: Modify\n\
                    yaml = parse_yaml(file.content)\n\
                    yaml.jobs.test.strategy.matrix.node_version = [\"18\", \"20\"]\n\
                    updated_yaml = stringify_yaml(yaml)\n\
                    \n\
                    // Step 3: Update\n\
                    github_update_file({\n\
                        \"owner\": \"devops\",\n\
                        \"repo\": \"configs\",\n\
                        \"path\": \".github/workflows/ci.yml\",\n\
                        \"content\": updated_yaml,\n\
                        \"message\": \"Update Node versions in CI\",\n\
                        \"sha\": file.sha\n\
                    })\n\n\
                 3. Update Markdown documentation:\n\
                    // Step 1: Read\n\
                    doc = github_get_file_contents({\n\
                        \"owner\": \"docs\",\n\
                        \"repo\": \"wiki\",\n\
                        \"path\": \"api/endpoints.md\"\n\
                    })\n\
                    \n\
                    // Step 2: Modify\n\
                    lines = doc.content.split(\"\\n\")\n\
                    // Add new section\n\
                    lines.push(\"## New Endpoint\")\n\
                    lines.push(\"Description of new endpoint...\")\n\
                    updated_doc = lines.join(\"\\n\")\n\
                    \n\
                    // Step 3: Update\n\
                    github_update_file({\n\
                        \"owner\": \"docs\",\n\
                        \"repo\": \"wiki\",\n\
                        \"path\": \"api/endpoints.md\",\n\
                        \"content\": updated_doc,\n\
                        \"message\": \"Document new API endpoint\",\n\
                        \"sha\": doc.sha\n\
                    })\n\n\
                 4. Update version file:\n\
                    // Step 1: Read\n\
                    version_file = github_get_file_contents({\n\
                        \"owner\": \"company\",\n\
                        \"repo\": \"product\",\n\
                        \"path\": \"VERSION\"\n\
                    })\n\
                    \n\
                    // Step 2: Modify (simple text)\n\
                    current = version_file.content.trim()\n\
                    parts = current.split(\".\")\n\
                    parts[2] = (parseInt(parts[2]) + 1).toString()\n\
                    new_version = parts.join(\".\")\n\
                    \n\
                    // Step 3: Update\n\
                    github_update_file({\n\
                        \"owner\": \"company\",\n\
                        \"repo\": \"product\",\n\
                        \"path\": \"VERSION\",\n\
                        \"content\": new_version,\n\
                        \"message\": `Bump version to ${new_version}`,\n\
                        \"sha\": version_file.sha\n\
                    })\n\n\
                 HANDLING CONCURRENT CHANGES:\n\
                 If file changes between read and update:\n\
                 \n\
                 try {\n\
                     github_update_file({...})\n\
                 } catch (error) {\n\
                     if (error.status === 409) {  // SHA mismatch\n\
                         // File was modified - refetch and retry\n\
                         file = github_get_file_contents({...})\n\
                         // Reapply your modifications\n\
                         modified = apply_changes(file.content)\n\
                         // Retry with new SHA\n\
                         github_update_file({..., \"sha\": file.sha})\n\
                     }\n\
                 }\n\n\
                 WHY THIS PATTERN?\n\
                 - Ensures you see current content before modifying\n\
                 - Prevents blind overwrites of others' changes\n\
                 - Provides SHA required for update\n\
                 - Allows intelligent conflict resolution\n\
                 - Maintains data integrity\n\n\
                 BEST PRACTICES:\n\
                 - Always read before update (never update blind)\n\
                 - Keep read-update cycle short to minimize conflicts\n\
                 - Handle 409 errors with retry logic\n\
                 - Preserve formatting when modifying structured files\n\
                 - Test modifications locally before updating",
            ),
        },
    ]
}

/// Updates on branches and branch-specific workflows
fn prompt_branches() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I update files on specific branches?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use the branch parameter to update files on non-default branches. This is essential for feature development and maintaining multiple versions.\n\n\
                 UPDATES ON BRANCHES:\n\
                 1. Update on specific branch:\n\
                    github_update_file({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"path\": \"config.json\",\n\
                        \"content\": \"{\\\"feature\\\": true}\",\n\
                        \"message\": \"Enable feature flag\",\n\
                        \"sha\": \"abc123...\",\n\
                        \"branch\": \"develop\"\n\
                    })\n\n\
                 BRANCH PARAMETER:\n\
                 - Specifies which branch to update\n\
                 - Default: repository's default branch (usually \"main\" or \"master\")\n\
                 - Branch must exist before updating\n\
                 - SHA must match file's SHA on that specific branch\n\n\
                 GETTING SHA FOR SPECIFIC BRANCH:\n\
                 Always specify branch when reading to get correct SHA:\n\
                 \n\
                 // Step 1: Get file from specific branch\n\
                 file = github_get_file_contents({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"config.json\",\n\
                     \"ref\": \"develop\"  // Same branch you'll update\n\
                 })\n\
                 \n\
                 // Step 2: Modify content\n\
                 modified = modify(file.content)\n\
                 \n\
                 // Step 3: Update on same branch\n\
                 github_update_file({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"config.json\",\n\
                     \"content\": modified,\n\
                     \"message\": \"Update config\",\n\
                     \"sha\": file.sha,\n\
                     \"branch\": \"develop\"\n\
                 })\n\n\
                 COMMON BRANCH WORKFLOWS:\n\
                 1. Feature branch development:\n\
                    // Update feature flag on feature branch\n\
                    file = github_get_file_contents({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"app\",\n\
                        \"path\": \"src/features.json\",\n\
                        \"ref\": \"feature/new-ui\"\n\
                    })\n\
                    \n\
                    features = JSON.parse(file.content)\n\
                    features.new_ui = {\"enabled\": true, \"beta\": false}\n\
                    \n\
                    github_update_file({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"app\",\n\
                        \"path\": \"src/features.json\",\n\
                        \"content\": JSON.stringify(features, null, 2),\n\
                        \"message\": \"Enable new UI feature\",\n\
                        \"sha\": file.sha,\n\
                        \"branch\": \"feature/new-ui\"\n\
                    })\n\n\
                 2. Release branch updates:\n\
                    // Update version on release branch\n\
                    version = github_get_file_contents({\n\
                        \"owner\": \"company\",\n\
                        \"repo\": \"product\",\n\
                        \"path\": \"package.json\",\n\
                        \"ref\": \"release/v2.0\"\n\
                    })\n\
                    \n\
                    pkg = JSON.parse(version.content)\n\
                    pkg.version = \"2.0.1\"\n\
                    \n\
                    github_update_file({\n\
                        \"owner\": \"company\",\n\
                        \"repo\": \"product\",\n\
                        \"path\": \"package.json\",\n\
                        \"content\": JSON.stringify(pkg, null, 2),\n\
                        \"message\": \"Release version 2.0.1\",\n\
                        \"sha\": version.sha,\n\
                        \"branch\": \"release/v2.0\"\n\
                    })\n\n\
                 3. Development vs production configs:\n\
                    // Update dev config on develop branch\n\
                    dev_config = github_get_file_contents({\n\
                        \"owner\": \"org\",\n\
                        \"repo\": \"service\",\n\
                        \"path\": \"config/app.yml\",\n\
                        \"ref\": \"develop\"\n\
                    })\n\
                    \n\
                    config = parse_yaml(dev_config.content)\n\
                    config.debug = true\n\
                    config.log_level = \"trace\"\n\
                    \n\
                    github_update_file({\n\
                        \"owner\": \"org\",\n\
                        \"repo\": \"service\",\n\
                        \"path\": \"config/app.yml\",\n\
                        \"content\": stringify_yaml(config),\n\
                        \"message\": \"Enable debug logging in dev\",\n\
                        \"sha\": dev_config.sha,\n\
                        \"branch\": \"develop\"\n\
                    })\n\
                    \n\
                    // Production config stays on main branch unchanged\n\n\
                 4. Hotfix workflow:\n\
                    // Create hotfix directly on main\n\
                    bug_file = github_get_file_contents({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"backend\",\n\
                        \"path\": \"src/auth.rs\",\n\
                        \"ref\": \"main\"\n\
                    })\n\
                    \n\
                    fixed_code = fix_security_issue(bug_file.content)\n\
                    \n\
                    github_update_file({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"backend\",\n\
                        \"path\": \"src/auth.rs\",\n\
                        \"content\": fixed_code,\n\
                        \"message\": \"Fix: Security vulnerability in auth\",\n\
                        \"sha\": bug_file.sha,\n\
                        \"branch\": \"main\"\n\
                    })\n\n\
                 MULTI-BRANCH UPDATES:\n\
                 When updating same file on multiple branches:\n\
                 \n\
                 branches = [\"main\", \"develop\", \"staging\"]\n\
                 \n\
                 for (branch of branches) {\n\
                     // Get current file from this branch\n\
                     file = github_get_file_contents({\n\
                         \"owner\": \"org\",\n\
                         \"repo\": \"repo\",\n\
                         \"path\": \"SECURITY.md\",\n\
                         \"ref\": branch\n\
                     })\n\
                     \n\
                     // Apply same update to all branches\n\
                     updated = add_security_policy(file.content)\n\
                     \n\
                     github_update_file({\n\
                         \"owner\": \"org\",\n\
                         \"repo\": \"repo\",\n\
                         \"path\": \"SECURITY.md\",\n\
                         \"content\": updated,\n\
                         \"message\": \"Update security policy\",\n\
                         \"sha\": file.sha,\n\
                         \"branch\": branch\n\
                     })\n\
                 }\n\n\
                 USE CASES:\n\
                 - Feature flag management per environment\n\
                 - Configuration updates for different deployments\n\
                 - Version bumps on release branches\n\
                 - Hotfixes on production branch\n\
                 - Documentation updates across versions\n\
                 - Experimental changes on feature branches\n\n\
                 BRANCH NAMING CONVENTIONS:\n\
                 - main/master: Production code\n\
                 - develop: Integration branch\n\
                 - feature/*: Feature development\n\
                 - release/*: Release preparation\n\
                 - hotfix/*: Critical fixes\n\
                 - staging: Pre-production testing\n\n\
                 BEST PRACTICES:\n\
                 - Always read from same branch you'll update\n\
                 - Keep branch configurations in sync when needed\n\
                 - Use descriptive branch names\n\
                 - Document which branches contain which configs\n\
                 - Consider branch protection rules\n\
                 - Test changes on feature branches first",
            ),
        },
    ]
}

/// Advanced usage: batch updates, error handling, committer info
fn prompt_advanced() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are advanced patterns for updating files, including batch updates and error handling?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Advanced file update patterns include batch operations, robust error handling, custom committer information, and atomic workflows.\n\n\
                 BATCH FILE UPDATES:\n\
                 Update multiple files in sequence:\n\
                 \n\
                 files_to_update = [\n\
                     {\"path\": \"src/version.rs\", \"new_content\": \"...\"},\n\
                     {\"path\": \"Cargo.toml\", \"new_content\": \"...\"},\n\
                     {\"path\": \"README.md\", \"new_content\": \"...\"}\n\
                 ]\n\
                 \n\
                 for (file_update of files_to_update) {\n\
                     // Get current file\n\
                     current = github_get_file_contents({\n\
                         \"owner\": \"org\",\n\
                         \"repo\": \"project\",\n\
                         \"path\": file_update.path\n\
                     })\n\
                     \n\
                     // Update file\n\
                     github_update_file({\n\
                         \"owner\": \"org\",\n\
                         \"repo\": \"project\",\n\
                         \"path\": file_update.path,\n\
                         \"content\": file_update.new_content,\n\
                         \"message\": `Update ${file_update.path} for release`,\n\
                         \"sha\": current.sha\n\
                     })\n\
                 }\n\n\
                 CUSTOM COMMITTER INFORMATION:\n\
                 Specify who made the commit:\n\
                 \n\
                 github_update_file({\n\
                     \"owner\": \"company\",\n\
                     \"repo\": \"docs\",\n\
                     \"path\": \"api-docs.md\",\n\
                     \"content\": updated_docs,\n\
                     \"message\": \"Update API documentation\",\n\
                     \"sha\": file_sha,\n\
                     \"committer_name\": \"Documentation Bot\",\n\
                     \"committer_email\": \"docs-bot@company.com\"\n\
                 })\n\
                 \n\
                 Use cases for custom committer:\n\
                 - Automated updates by bots\n\
                 - Service accounts for CI/CD\n\
                 - Attribution to specific team\n\
                 - Tracking automated changes\n\n\
                 ROBUST ERROR HANDLING:\n\
                 Handle all common failure modes:\n\
                 \n\
                 function update_with_retry(params, max_retries = 3) {\n\
                     for (let attempt = 1; attempt <= max_retries; attempt++) {\n\
                         try {\n\
                             // Get latest version\n\
                             file = github_get_file_contents({\n\
                                 \"owner\": params.owner,\n\
                                 \"repo\": params.repo,\n\
                                 \"path\": params.path,\n\
                                 \"ref\": params.branch\n\
                             })\n\
                             \n\
                             // Apply modification\n\
                             modified = params.modify_fn(file.content)\n\
                             \n\
                             // Update\n\
                             result = github_update_file({\n\
                                 \"owner\": params.owner,\n\
                                 \"repo\": params.repo,\n\
                                 \"path\": params.path,\n\
                                 \"content\": modified,\n\
                                 \"message\": params.message,\n\
                                 \"sha\": file.sha,\n\
                                 \"branch\": params.branch\n\
                             })\n\
                             \n\
                             return result  // Success!\n\
                             \n\
                         } catch (error) {\n\
                             if (error.status === 409) {\n\
                                 // SHA mismatch - file changed, retry\n\
                                 console.log(`Attempt ${attempt}: SHA mismatch, retrying...`)\n\
                                 continue\n\
                             } else if (error.status === 404) {\n\
                                 // File or repo not found\n\
                                 throw new Error(\"File not found: \" + params.path)\n\
                             } else if (error.status === 422) {\n\
                                 // Validation error - bad parameters\n\
                                 throw new Error(\"Invalid parameters: \" + error.message)\n\
                             } else {\n\
                                 // Other error\n\
                                 if (attempt < max_retries) {\n\
                                     console.log(`Attempt ${attempt}: Error, retrying...`)\n\
                                     continue\n\
                                 }\n\
                                 throw error\n\
                             }\n\
                         }\n\
                     }\n\
                     throw new Error(\"Max retries exceeded\")\n\
                 }\n\n\
                 CONDITIONAL UPDATES:\n\
                 Only update if content actually changed:\n\
                 \n\
                 file = github_get_file_contents({\n\
                     \"owner\": \"org\",\n\
                     \"repo\": \"repo\",\n\
                     \"path\": \"status.txt\"\n\
                 })\n\
                 \n\
                 new_content = generate_status_report()\n\
                 \n\
                 if (file.content !== new_content) {\n\
                     github_update_file({\n\
                         \"owner\": \"org\",\n\
                         \"repo\": \"repo\",\n\
                         \"path\": \"status.txt\",\n\
                         \"content\": new_content,\n\
                         \"message\": \"Update status report\",\n\
                         \"sha\": file.sha\n\
                     })\n\
                 } else {\n\
                     console.log(\"No changes needed\")\n\
                 }\n\n\
                 COORDINATED MULTI-FILE UPDATES:\n\
                 Update related files together:\n\
                 \n\
                 // Example: Bump version across multiple files\n\
                 new_version = \"3.0.0\"\n\
                 \n\
                 updates = [\n\
                     {\n\
                         path: \"VERSION\",\n\
                         modify: () => new_version\n\
                     },\n\
                     {\n\
                         path: \"package.json\",\n\
                         modify: (content) => {\n\
                             pkg = JSON.parse(content)\n\
                             pkg.version = new_version\n\
                             return JSON.stringify(pkg, null, 2)\n\
                         }\n\
                     },\n\
                     {\n\
                         path: \"Cargo.toml\",\n\
                         modify: (content) => {\n\
                             return content.replace(\n\
                                 /version = \"[^\"]+\"/,\n\
                                 `version = \"${new_version}\"`\n\
                             )\n\
                         }\n\
                     }\n\
                 ]\n\
                 \n\
                 for (update of updates) {\n\
                     file = github_get_file_contents({\n\
                         \"owner\": \"org\",\n\
                         \"repo\": \"project\",\n\
                         \"path\": update.path\n\
                     })\n\
                     \n\
                     modified = update.modify(file.content)\n\
                     \n\
                     github_update_file({\n\
                         \"owner\": \"org\",\n\
                         \"repo\": \"project\",\n\
                         \"path\": update.path,\n\
                         \"content\": modified,\n\
                         \"message\": `Bump version to ${new_version}`,\n\
                         \"sha\": file.sha\n\
                     })\n\
                 }\n\n\
                 VALIDATION BEFORE UPDATE:\n\
                 Validate content before updating:\n\
                 \n\
                 file = github_get_file_contents({\n\
                     \"owner\": \"org\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"config.json\"\n\
                 })\n\
                 \n\
                 config = JSON.parse(file.content)\n\
                 config.new_setting = \"value\"\n\
                 new_content = JSON.stringify(config, null, 2)\n\
                 \n\
                 // Validate before updating\n\
                 try {\n\
                     JSON.parse(new_content)  // Ensure valid JSON\n\
                     validate_config_schema(JSON.parse(new_content))  // Custom validation\n\
                     \n\
                     github_update_file({\n\
                         \"owner\": \"org\",\n\
                         \"repo\": \"project\",\n\
                         \"path\": \"config.json\",\n\
                         \"content\": new_content,\n\
                         \"message\": \"Add new setting\",\n\
                         \"sha\": file.sha\n\
                     })\n\
                 } catch (validation_error) {\n\
                     console.error(\"Validation failed:\", validation_error)\n\
                     // Don't update if validation fails\n\
                 }\n\n\
                 TRACKING UPDATE HISTORY:\n\
                 Log updates for auditing:\n\
                 \n\
                 function tracked_update(params) {\n\
                     let start_time = Date.now()\n\
                     \n\
                     try {\n\
                         result = github_update_file(params)\n\
                         \n\
                         log({\n\
                             timestamp: new Date().toISOString(),\n\
                             action: \"update_file\",\n\
                             file: params.path,\n\
                             repo: `${params.owner}/${params.repo}`,\n\
                             commit: result.commit.sha,\n\
                             duration_ms: Date.now() - start_time,\n\
                             status: \"success\"\n\
                         })\n\
                         \n\
                         return result\n\
                     } catch (error) {\n\
                         log({\n\
                             timestamp: new Date().toISOString(),\n\
                             action: \"update_file\",\n\
                             file: params.path,\n\
                             repo: `${params.owner}/${params.repo}`,\n\
                             duration_ms: Date.now() - start_time,\n\
                             status: \"error\",\n\
                             error: error.message\n\
                         })\n\
                         throw error\n\
                     }\n\
                 }\n\n\
                 RATE LIMITING CONSIDERATIONS:\n\
                 Handle GitHub API rate limits:\n\
                 \n\
                 async function update_with_rate_limit(files) {\n\
                     const delay = 1000  // 1 second between updates\n\
                     \n\
                     for (file of files) {\n\
                         try {\n\
                             current = github_get_file_contents({...})\n\
                             modified = modify(current.content)\n\
                             github_update_file({..., sha: current.sha})\n\
                             \n\
                             // Wait before next update\n\
                             await sleep(delay)\n\
                         } catch (error) {\n\
                             if (error.status === 403 && error.message.includes(\"rate limit\")) {\n\
                                 // Rate limited - wait longer\n\
                                 console.log(\"Rate limited, waiting...\")\n\
                                 await sleep(60000)  // Wait 1 minute\n\
                                 // Retry this file\n\
                                 continue\n\
                             }\n\
                             throw error\n\
                         }\n\
                     }\n\
                 }\n\n\
                 ADVANCED PATTERNS:\n\
                 1. Atomic updates: Update multiple files as a unit\n\
                 2. Rollback: Keep track of SHAs to revert changes\n\
                 3. Dry-run: Validate modifications without committing\n\
                 4. Diff preview: Show changes before applying\n\
                 5. Batch with progress: Update many files with status tracking\n\
                 6. Conflict resolution: Merge concurrent changes intelligently\n\n\
                 BEST PRACTICES:\n\
                 - Always validate content before updating\n\
                 - Implement retry logic for SHA mismatches\n\
                 - Log all updates for auditing\n\
                 - Handle rate limits gracefully\n\
                 - Use custom committer info for automation\n\
                 - Keep batch updates atomic when possible\n\
                 - Test complex modifications thoroughly",
            ),
        },
    ]
}

/// Comprehensive guide covering all features and patterns
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to updating GitHub files effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_update_file tool provides a complete solution for updating files in GitHub repositories with conflict prevention, branch support, and robust error handling.\n\n\
                 =============================================================================\n\
                 OVERVIEW\n\
                 =============================================================================\n\n\
                 WHAT IT DOES:\n\
                 - Updates existing files in GitHub repositories\n\
                 - Creates commits with your changes\n\
                 - Supports any branch\n\
                 - Prevents conflicts with SHA verification\n\
                 - Handles text and binary files\n\n\
                 WHEN TO USE:\n\
                 - Configuration updates\n\
                 - Documentation changes\n\
                 - Version bumps\n\
                 - Automated maintenance\n\
                 - Feature flag toggling\n\
                 - Source code modifications\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 MINIMAL EXAMPLE:\n\
                 github_update_file({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"README.md\",\n\
                     \"content\": \"# Updated README\\n\\nNew content...\",\n\
                     \"message\": \"Update README\",\n\
                     \"sha\": \"abc123def456...\"  // Current file SHA\n\
                 })\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner (username or organization name)\n\
                 - repo: Repository name\n\
                 - path: File path relative to repository root\n\
                 - content: Complete new file content (not a diff)\n\
                 - message: Commit message describing the change\n\
                 - sha: Current file SHA (40-character hex hash)\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - branch: Target branch (default: repository default branch)\n\
                 - committer_name: Name for commit author (default: authenticated user)\n\
                 - committer_email: Email for commit author (default: authenticated user email)\n\n\
                 =============================================================================\n\
                 SHA REQUIREMENT - CRITICAL!\n\
                 =============================================================================\n\n\
                 WHY SHA IS REQUIRED:\n\
                 - Ensures you're updating the version you think you are\n\
                 - Prevents accidental overwrites of concurrent changes\n\
                 - GitHub API rejects updates without matching SHA\n\
                 - Acts as optimistic locking mechanism\n\n\
                 HOW TO GET SHA:\n\
                 Always call github_get_file_contents first:\n\
                 \n\
                 file = github_get_file_contents({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"config.json\"\n\
                 })\n\
                 // file.sha contains the SHA you need\n\
                 \n\
                 github_update_file({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"config.json\",\n\
                     \"content\": modified_content,\n\
                     \"message\": \"Update config\",\n\
                     \"sha\": file.sha  // Use SHA from get_file_contents\n\
                 })\n\n\
                 SHA MISMATCH ERROR (409):\n\
                 If file changed between read and update:\n\
                 - GitHub returns 409 Conflict error\n\
                 - Your SHA doesn't match current file SHA\n\
                 - Solution: Refetch file and retry with new SHA\n\
                 - Implement retry logic for robustness\n\n\
                 =============================================================================\n\
                 RECOMMENDED WORKFLOW\n\
                 =============================================================================\n\n\
                 THREE-STEP PATTERN (Read-Modify-Update):\n\
                 \n\
                 Step 1: READ current file\n\
                 file = github_get_file_contents({\n\
                     \"owner\": \"org\",\n\
                     \"repo\": \"repo\",\n\
                     \"path\": \"file.json\",\n\
                     \"ref\": \"main\"  // Optional: specify branch\n\
                 })\n\
                 // Returns: {content, sha, size, ...}\n\
                 \n\
                 Step 2: MODIFY content\n\
                 data = JSON.parse(file.content)\n\
                 data.version = \"2.0\"\n\
                 data.updated_at = new Date().toISOString()\n\
                 modified = JSON.stringify(data, null, 2)\n\
                 \n\
                 Step 3: UPDATE with SHA from step 1\n\
                 github_update_file({\n\
                     \"owner\": \"org\",\n\
                     \"repo\": \"repo\",\n\
                     \"path\": \"file.json\",\n\
                     \"content\": modified,\n\
                     \"message\": \"Update version to 2.0\",\n\
                     \"sha\": file.sha,\n\
                     \"branch\": \"main\"\n\
                 })\n\n\
                 WHY THIS PATTERN:\n\
                 - See current content before modifying\n\
                 - Get required SHA\n\
                 - Detect conflicts early\n\
                 - Make informed changes\n\n\
                 =============================================================================\n\
                 WORKING WITH BRANCHES\n\
                 =============================================================================\n\n\
                 UPDATE ON SPECIFIC BRANCH:\n\
                 github_update_file({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"config.json\",\n\
                     \"content\": new_content,\n\
                     \"message\": \"Update config\",\n\
                     \"sha\": file_sha,\n\
                     \"branch\": \"develop\"  // Specify branch\n\
                 })\n\n\
                 IMPORTANT: Branch must exist before updating\n\
                 IMPORTANT: SHA must be from that specific branch\n\n\
                 GETTING FILE FROM SPECIFIC BRANCH:\n\
                 file = github_get_file_contents({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"config.json\",\n\
                     \"ref\": \"develop\"  // Read from same branch\n\
                 })\n\
                 // Now file.sha is correct for develop branch\n\n\
                 BRANCH WORKFLOWS:\n\
                 - Feature branches: Update configs for feature development\n\
                 - Release branches: Version bumps and changelog updates\n\
                 - Hotfix branches: Critical fixes on production\n\
                 - Environment branches: Different configs per environment\n\n\
                 =============================================================================\n\
                 RESPONSE STRUCTURE\n\
                 =============================================================================\n\n\
                 SUCCESS RESPONSE:\n\
                 {\n\
                     \"commit\": {\n\
                         \"sha\": \"def456...\",         // New commit SHA\n\
                         \"url\": \"https://...\",\n\
                         \"message\": \"Update config\",\n\
                         \"author\": {...},\n\
                         \"committer\": {...}\n\
                     },\n\
                     \"content\": {\n\
                         \"name\": \"config.json\",\n\
                         \"path\": \"config.json\",\n\
                         \"sha\": \"789abc...\",         // New file SHA\n\
                         \"size\": 1234,\n\
                         \"url\": \"https://...\"\n\
                     }\n\
                 }\n\n\
                 WHAT YOU GET:\n\
                 - New commit SHA (for tracking)\n\
                 - New file SHA (for future updates)\n\
                 - File metadata (size, URL, etc.)\n\
                 - Author/committer information\n\n\
                 =============================================================================\n\
                 ERROR HANDLING\n\
                 =============================================================================\n\n\
                 COMMON ERRORS:\n\
                 \n\
                 404 - Not Found:\n\
                 - File doesn't exist at path\n\
                 - Repository doesn't exist\n\
                 - No access to repository\n\
                 - Branch doesn't exist\n\
                 Solution: Verify path, repo, and permissions\n\
                 \n\
                 409 - Conflict (SHA Mismatch):\n\
                 - File changed since you got SHA\n\
                 - Someone else updated the file\n\
                 - Concurrent modification detected\n\
                 Solution: Refetch file and retry with new SHA\n\
                 \n\
                 422 - Unprocessable Entity:\n\
                 - Invalid parameters\n\
                 - Malformed content\n\
                 - Invalid branch name\n\
                 - SHA format incorrect\n\
                 Solution: Validate all parameters\n\
                 \n\
                 403 - Forbidden:\n\
                 - No write permission\n\
                 - Branch protected\n\
                 - Rate limit exceeded\n\
                 Solution: Check permissions and rate limits\n\n\
                 RETRY LOGIC FOR SHA MISMATCH:\n\
                 \n\
                 function update_with_retry(params, max_retries = 3) {\n\
                     for (attempt = 1; attempt <= max_retries; attempt++) {\n\
                         try {\n\
                             // Get latest file\n\
                             file = github_get_file_contents({...})\n\
                             \n\
                             // Modify\n\
                             modified = modify_function(file.content)\n\
                             \n\
                             // Update\n\
                             return github_update_file({\n\
                                 ...,\n\
                                 \"content\": modified,\n\
                                 \"sha\": file.sha\n\
                             })\n\
                         } catch (error) {\n\
                             if (error.status === 409 && attempt < max_retries) {\n\
                                 console.log(\"Retry due to conflict...\")\n\
                                 continue  // Retry\n\
                             }\n\
                             throw error  // Give up\n\
                         }\n\
                     }\n\
                 }\n\n\
                 =============================================================================\n\
                 COMMON USE CASES\n\
                 =============================================================================\n\n\
                 1. UPDATE CONFIGURATION FILE:\n\
                 config = github_get_file_contents({\"owner\": \"org\", \"repo\": \"api\", \"path\": \"config.json\"})\n\
                 data = JSON.parse(config.content)\n\
                 data.timeout = 60\n\
                 github_update_file({..., \"content\": JSON.stringify(data, null, 2), \"sha\": config.sha})\n\n\
                 2. BUMP VERSION:\n\
                 version_file = github_get_file_contents({\"owner\": \"org\", \"repo\": \"app\", \"path\": \"VERSION\"})\n\
                 new_version = increment_version(version_file.content.trim())\n\
                 github_update_file({..., \"content\": new_version, \"sha\": version_file.sha})\n\n\
                 3. UPDATE DOCUMENTATION:\n\
                 readme = github_get_file_contents({\"owner\": \"org\", \"repo\": \"docs\", \"path\": \"README.md\"})\n\
                 updated = add_section(readme.content, \"## New Feature\", \"Description...\")\n\
                 github_update_file({..., \"content\": updated, \"sha\": readme.sha})\n\n\
                 4. TOGGLE FEATURE FLAG:\n\
                 flags = github_get_file_contents({\"owner\": \"org\", \"repo\": \"app\", \"path\": \"features.json\"})\n\
                 features = JSON.parse(flags.content)\n\
                 features.new_ui = true\n\
                 github_update_file({..., \"content\": JSON.stringify(features, null, 2), \"sha\": flags.sha})\n\n\
                 5. UPDATE WORKFLOW FILE:\n\
                 workflow = github_get_file_contents({\"owner\": \"org\", \"repo\": \"app\", \"path\": \".github/workflows/ci.yml\"})\n\
                 updated_yaml = modify_yaml(workflow.content)\n\
                 github_update_file({..., \"content\": updated_yaml, \"sha\": workflow.sha})\n\n\
                 =============================================================================\n\
                 BATCH UPDATES\n\
                 =============================================================================\n\n\
                 UPDATE MULTIPLE FILES:\n\
                 files = [\n\
                     {\"path\": \"src/version.rs\", \"update\": update_rust_version},\n\
                     {\"path\": \"package.json\", \"update\": update_npm_version},\n\
                     {\"path\": \"Cargo.toml\", \"update\": update_cargo_version}\n\
                 ]\n\
                 \n\
                 for (file_info of files) {\n\
                     current = github_get_file_contents({\n\
                         \"owner\": \"org\",\n\
                         \"repo\": \"project\",\n\
                         \"path\": file_info.path\n\
                     })\n\
                     \n\
                     modified = file_info.update(current.content)\n\
                     \n\
                     github_update_file({\n\
                         \"owner\": \"org\",\n\
                         \"repo\": \"project\",\n\
                         \"path\": file_info.path,\n\
                         \"content\": modified,\n\
                         \"message\": `Update ${file_info.path} for v2.0`,\n\
                         \"sha\": current.sha\n\
                     })\n\
                 }\n\n\
                 =============================================================================\n\
                 ADVANCED FEATURES\n\
                 =============================================================================\n\n\
                 CUSTOM COMMITTER:\n\
                 github_update_file({\n\
                     \"owner\": \"org\",\n\
                     \"repo\": \"project\",\n\
                     \"path\": \"status.txt\",\n\
                     \"content\": \"All systems operational\",\n\
                     \"message\": \"Update status\",\n\
                     \"sha\": file_sha,\n\
                     \"committer_name\": \"Status Bot\",\n\
                     \"committer_email\": \"bot@company.com\"\n\
                 })\n\n\
                 CONDITIONAL UPDATE:\n\
                 file = github_get_file_contents({...})\n\
                 new_content = generate_content()\n\
                 if (file.content !== new_content) {\n\
                     github_update_file({..., \"sha\": file.sha})\n\
                 }\n\n\
                 VALIDATION BEFORE UPDATE:\n\
                 file = github_get_file_contents({...})\n\
                 modified = modify(file.content)\n\
                 validate(modified)  // Throws if invalid\n\
                 github_update_file({..., \"content\": modified, \"sha\": file.sha})\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. ALWAYS READ FIRST:\n\
                    Never update without getting current content and SHA\n\
                    github_get_file_contents() → modify → github_update_file()\n\n\
                 2. DESCRIPTIVE COMMIT MESSAGES:\n\
                    Bad: \"Update file\"\n\
                    Good: \"Increase API timeout from 30s to 60s for slow endpoints\"\n\n\
                 3. VALIDATE CONTENT:\n\
                    Verify JSON/YAML syntax, schema compliance, no corruption\n\n\
                 4. HANDLE SHA MISMATCHES:\n\
                    Implement retry logic for 409 errors with exponential backoff\n\n\
                 5. USE BRANCHES:\n\
                    Test changes on feature branches before updating main\n\n\
                 6. BATCH THOUGHTFULLY:\n\
                    Update related files together, but respect rate limits\n\n\
                 7. TRACK CHANGES:\n\
                    Log updates for auditing, keep commit SHAs for rollback\n\n\
                 8. TEST MODIFICATIONS:\n\
                    Validate logic before updating production files\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Basic: github_update_file({owner, repo, path, content, message, sha})\n\
                 With branch: add \"branch\": \"develop\"\n\
                 Custom author: add \"committer_name\" and \"committer_email\"\n\
                 Get SHA: github_get_file_contents({owner, repo, path, ref: branch})\n\
                 Retry on 409: Refetch file and retry with new SHA\n\
                 Batch: Loop over files, get SHA, modify, update\n\
                 Validate: Check content before updating\n\n\
                 Remember: Always read before update, handle conflicts gracefully, validate content, and write descriptive commit messages!",
            ),
        },
    ]
}
