//! Prompt messages for github_push_file tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GithubPushFilePromptArgs;

/// Prompt provider for github_push_file tool
///
/// This is the ONLY way to provide prompts for github_push_file - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GithubPushFilePrompts;

impl PromptProvider for GithubPushFilePrompts {
    type PromptArgs = GithubPushFilePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basics") => prompt_basics(),
            Some("create") => prompt_create(),
            Some("branches") => prompt_branches(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basics, create, branches, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GITHUB_PUSH_FILE
// ============================================================================

/// Basic usage and fundamental concepts
fn prompt_basics() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_push_file to create or update files in a GitHub repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_push_file tool creates or updates a single file directly via the GitHub API without cloning the repository locally.\n\n\
                 BASIC USAGE:\n\
                 github_push_file({\n\
                   \"owner\": \"username\",\n\
                   \"repo\": \"project\",\n\
                   \"path\": \"docs/README.md\",\n\
                   \"content\": \"# Documentation\\n\\nWelcome to the docs.\",\n\
                   \"message\": \"Add documentation\"\n\
                 })\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner (username or organization)\n\
                 - repo: Repository name\n\
                 - path: File path within repository (relative to root)\n\
                 - content: File content as string (NOT base64)\n\
                 - message: Commit message describing the change\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - branch: Target branch (default: repository default branch)\n\
                 - sha: SHA of file being replaced (for updates, prevents conflicts)\n\n\
                 RESPONSE:\n\
                 Returns JSON with:\n\
                 - content: Object with path, sha, and other file metadata\n\
                 - commit: Object with sha, message, author, committer details\n\n\
                 Example response:\n\
                 {\n\
                   \"content\": {\n\
                     \"path\": \"docs/README.md\",\n\
                     \"sha\": \"abc1234...\",\n\
                     \"size\": 45\n\
                   },\n\
                   \"commit\": {\n\
                     \"sha\": \"def5678...\",\n\
                     \"message\": \"Add documentation\",\n\
                     \"author\": {...},\n\
                     \"committer\": {...}\n\
                   }\n\
                 }\n\n\
                 WHEN TO USE:\n\
                 - Creating new configuration files\n\
                 - Adding documentation\n\
                 - Quick single-file updates\n\
                 - Automated file generation\n\
                 - Adding GitHub workflows\n\
                 - Creating templates or examples\n\n\
                 WHEN NOT TO USE:\n\
                 - Multiple files at once (use github_push_files instead)\n\
                 - Large binary files (>1MB)\n\
                 - Files requiring complex merge logic\n\
                 - When you need full git history locally\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with:\n\
                 - repo scope (for private repositories)\n\
                 - public_repo scope (for public repositories)\n\n\
                 ADVANTAGES:\n\
                 - No local clone needed\n\
                 - Fast for single file operations\n\
                 - Works from any environment\n\
                 - Automatic commit creation\n\
                 - Direct API access\n\n\
                 PATH REQUIREMENTS:\n\
                 - Relative to repository root\n\
                 - No leading slash\n\
                 - Use forward slashes (/), not backslashes\n\
                 - Can include directories (auto-created)\n\
                 - Examples: \"README.md\", \"src/lib.rs\", \".github/workflows/ci.yml\"\n\n\
                 CONTENT FORMAT:\n\
                 - Plain text string (UTF-8)\n\
                 - NOT base64 encoded\n\
                 - Newlines: Use \\n\n\
                 - Special characters: Properly escape in JSON\n\
                 - Maximum size: ~1MB recommended\n\n\
                 ERROR HANDLING:\n\
                 - 404: Repository or branch doesn't exist\n\
                 - 409: Conflict (file changed since last read, provide sha)\n\
                 - 422: Invalid content or path\n\
                 - 403: Insufficient permissions or branch protection\n\n\
                 BEST PRACTICES:\n\
                 1. Use descriptive commit messages\n\
                 2. Verify file doesn't exist or get its SHA first\n\
                 3. Use branches for non-trivial changes\n\
                 4. Check branch protection rules\n\
                 5. Keep content under 1MB\n\
                 6. Use proper file extensions\n\
                 7. Follow repository's contribution guidelines",
            ),
        },
    ]
}

/// Creating new files
fn prompt_create() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Show me examples of creating different types of files with github_push_file.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Here are comprehensive examples of creating various file types with github_push_file:\n\n\
                 CREATING FILES:\n\n\
                 1. Create new README:\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \"docs/README.md\",\n\
                      \"content\": \"# Documentation\\n\\nThis is the documentation.\",\n\
                      \"message\": \"Add documentation\"\n\
                    })\n\n\
                 2. Create configuration file:\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \"config/settings.json\",\n\
                      \"content\": \"{\\\"version\\\": \\\"1.0\\\", \\\"debug\\\": false}\",\n\
                      \"message\": \"Add settings configuration\"\n\
                    })\n\n\
                 3. Create source code file:\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \"src/helper.js\",\n\
                      \"content\": \"export function helper() {\\n  return 'Hello';\\n}\",\n\
                      \"message\": \"Add helper function\"\n\
                    })\n\n\
                 4. Create GitHub workflow:\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \".github/workflows/ci.yml\",\n\
                      \"content\": \"name: CI\\non: [push]\\njobs:\\n  test:\\n    runs-on: ubuntu-latest\\n    steps:\\n      - uses: actions/checkout@v3\",\n\
                      \"message\": \"Add CI workflow\"\n\
                    })\n\n\
                 5. Create CODEOWNERS file:\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \".github/CODEOWNERS\",\n\
                      \"content\": \"* @team-lead\\n/docs/ @docs-team\\n*.js @javascript-team\",\n\
                      \"message\": \"Add CODEOWNERS\"\n\
                    })\n\n\
                 6. Create license file:\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \"LICENSE\",\n\
                      \"content\": \"MIT License\\n\\nCopyright (c) 2025\\n\\nPermission is hereby granted...\",\n\
                      \"message\": \"Add MIT license\"\n\
                    })\n\n\
                 7. Create issue template:\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \".github/ISSUE_TEMPLATE/bug_report.md\",\n\
                      \"content\": \"---\\nname: Bug Report\\nabout: Report a bug\\n---\\n\\n## Description\\n\\n## Steps to Reproduce\",\n\
                      \"message\": \"Add bug report template\"\n\
                    })\n\n\
                 8. Create pull request template:\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \".github/PULL_REQUEST_TEMPLATE.md\",\n\
                      \"content\": \"## Changes\\n\\n## Checklist\\n- [ ] Tests pass\\n- [ ] Documentation updated\",\n\
                      \"message\": \"Add PR template\"\n\
                    })\n\n\
                 NO CLONE NEEDED:\n\
                 - All files created directly via GitHub API\n\
                 - No local repository required\n\
                 - Instant commit creation\n\
                 - Works from any environment\n\
                 - Useful for automation and CI/CD\n\n\
                 FILE CREATION PATTERNS:\n\
                 - Configuration files: JSON, YAML, TOML, INI\n\
                 - Documentation: Markdown, plain text\n\
                 - Source code: Any programming language\n\
                 - Templates: Issue, PR, contributing guides\n\
                 - GitHub files: Workflows, CODEOWNERS, funding\n\
                 - Dotfiles: .gitignore, .editorconfig, .env.example\n\n\
                 AUTOMATIC DIRECTORY CREATION:\n\
                 - Directories in path are created automatically\n\
                 - No need to create parent directories first\n\
                 - Example: \"deep/nested/path/file.txt\" works immediately\n\n\
                 CONTENT ESCAPING:\n\
                 Remember to escape special characters in JSON:\n\
                 - Newlines: \\n\n\
                 - Tabs: \\t\n\
                 - Quotes: \\\"\n\
                 - Backslashes: \\\\\n\n\
                 RESPONSE VERIFICATION:\n\
                 Check response for:\n\
                 - content.sha: File's SHA hash (save for updates)\n\
                 - commit.sha: Commit's SHA hash\n\
                 - content.path: Confirms file location\n\
                 - commit.message: Confirms commit message\n\n\
                 USE CASES:\n\
                 - Automated documentation generation\n\
                 - CI/CD configuration deployment\n\
                 - Template repository creation\n\
                 - Configuration file management\n\
                 - Quick file additions\n\
                 - Repository setup automation",
            ),
        },
    ]
}

/// Pushing to branches
fn prompt_branches() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use github_push_file with different branches?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_push_file tool supports creating and updating files on any branch. Here's how to work with branches:\n\n\
                 PUSHING TO BRANCHES:\n\n\
                 1. Push to specific existing branch:\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \"config.json\",\n\
                      \"content\": \"{\\\"version\\\": \\\"1.0\\\"}\",\n\
                      \"message\": \"Add config\",\n\
                      \"branch\": \"develop\"\n\
                    })\n\n\
                 2. Push to main branch (explicit):\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \"README.md\",\n\
                      \"content\": \"# Project\",\n\
                      \"message\": \"Update README\",\n\
                      \"branch\": \"main\"\n\
                    })\n\n\
                 3. Push to feature branch:\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \"features/new-feature.js\",\n\
                      \"content\": \"export const feature = () => {};\",\n\
                      \"message\": \"Add new feature\",\n\
                      \"branch\": \"feature/new-feature\"\n\
                    })\n\n\
                 4. Push to release branch:\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \"CHANGELOG.md\",\n\
                      \"content\": \"# v1.0.0\\n\\n- Initial release\",\n\
                      \"message\": \"Update changelog for v1.0.0\",\n\
                      \"branch\": \"release/v1.0.0\"\n\
                    })\n\n\
                 DEFAULT BRANCH BEHAVIOR:\n\
                 - If branch not specified, uses repository's default branch\n\
                 - Usually \"main\" or \"master\"\n\
                 - Check repository settings to confirm\n\n\
                 CREATING FILES IN NEW BRANCHES:\n\
                 Branch must exist before pushing to it. Create branch first:\n\
                 \n\
                 Step 1 - Create branch:\n\
                 github_create_branch({\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"branch\": \"feature/x\",\n\
                   \"from_branch\": \"main\"\n\
                 })\n\
                 \n\
                 Step 2 - Push file to new branch:\n\
                 github_push_file({\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"path\": \"feature.rs\",\n\
                   \"content\": \"// New feature code\",\n\
                   \"message\": \"Add feature implementation\",\n\
                   \"branch\": \"feature/x\"\n\
                 })\n\n\
                 BRANCH NAMING CONVENTIONS:\n\
                 - Feature branches: feature/description or feat/description\n\
                 - Bug fixes: fix/description or bugfix/description\n\
                 - Hotfixes: hotfix/description\n\
                 - Releases: release/version\n\
                 - Development: develop or dev\n\n\
                 WORKFLOW EXAMPLES:\n\n\
                 1. Feature development workflow:\n\
                    // Create feature branch\n\
                    github_create_branch({\"owner\": \"u\", \"repo\": \"r\", \"branch\": \"feature/auth\"})\n\
                    // Add authentication config\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"auth.config.js\", \"content\": \"...\", \"message\": \"Add auth config\", \"branch\": \"feature/auth\"})\n\
                    // Add authentication module\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"src/auth.js\", \"content\": \"...\", \"message\": \"Add auth module\", \"branch\": \"feature/auth\"})\n\
                    // Create pull request\n\
                    github_create_pull_request({\"owner\": \"u\", \"repo\": \"r\", \"head\": \"feature/auth\", \"base\": \"main\", \"title\": \"Add authentication\"})\n\n\
                 2. Hotfix workflow:\n\
                    // Create hotfix branch from main\n\
                    github_create_branch({\"owner\": \"u\", \"repo\": \"r\", \"branch\": \"hotfix/security\", \"from_branch\": \"main\"})\n\
                    // Apply security fix\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"src/security.js\", \"content\": \"...\", \"message\": \"Fix security vulnerability\", \"branch\": \"hotfix/security\"})\n\
                    // Create urgent PR\n\
                    github_create_pull_request({\"owner\": \"u\", \"repo\": \"r\", \"head\": \"hotfix/security\", \"base\": \"main\", \"title\": \"Security hotfix\"})\n\n\
                 3. Documentation branch:\n\
                    // Create docs branch\n\
                    github_create_branch({\"owner\": \"u\", \"repo\": \"r\", \"branch\": \"docs/api\"})\n\
                    // Add API documentation\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"docs/api.md\", \"content\": \"...\", \"message\": \"Document API endpoints\", \"branch\": \"docs/api\"})\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"docs/examples.md\", \"content\": \"...\", \"message\": \"Add examples\", \"branch\": \"docs/api\"})\n\n\
                 BRANCH PROTECTION:\n\
                 - Protected branches may require:\n\
                   - Pull request reviews\n\
                   - Status checks to pass\n\
                   - Signed commits\n\
                 - Direct pushes to protected branches will fail (403 error)\n\
                 - Solution: Create feature branch, then PR to protected branch\n\n\
                 ERROR SCENARIOS:\n\
                 - 404: Branch doesn't exist (create it first)\n\
                 - 403: Branch is protected (use PR workflow)\n\
                 - 422: Invalid branch name format\n\n\
                 BEST PRACTICES:\n\
                 1. Never commit directly to main/master\n\
                 2. Use feature branches for all changes\n\
                 3. Follow branch naming conventions\n\
                 4. Create PRs from feature branches\n\
                 5. Delete branches after merging\n\
                 6. Keep branch names descriptive\n\
                 7. Check branch protection rules first",
            ),
        },
    ]
}

/// File creation workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows for using github_push_file?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Here are comprehensive workflows showing how to use github_push_file effectively:\n\n\
                 FILE CREATION WORKFLOWS:\n\n\
                 1. ADD GITHUB WORKFLOW FILE:\n\
                    // Create CI workflow\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \".github/workflows/ci.yml\",\n\
                      \"content\": \"name: CI\\non:\\n  push:\\n    branches: [main]\\n  pull_request:\\n    branches: [main]\\njobs:\\n  test:\\n    runs-on: ubuntu-latest\\n    steps:\\n      - uses: actions/checkout@v3\\n      - name: Run tests\\n        run: npm test\",\n\
                      \"message\": \"Add CI workflow\"\n\
                    })\n\n\
                 2. ADD CONFIGURATION FILE:\n\
                    // Add ESLint config\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \".eslintrc.json\",\n\
                      \"content\": \"{\\n  \\\"env\\\": {\\n    \\\"browser\\\": true,\\n    \\\"es2021\\\": true\\n  },\\n  \\\"extends\\\": \\\"eslint:recommended\\\",\\n  \\\"rules\\\": {}\\n}\",\n\
                      \"message\": \"Add ESLint configuration\"\n\
                    })\n\n\
                 3. ADD CODEOWNERS FILE:\n\
                    // Define code ownership\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \".github/CODEOWNERS\",\n\
                      \"content\": \"# Global owners\\n* @team-lead\\n\\n# Frontend\\n/src/ui/ @frontend-team\\n*.css @frontend-team\\n\\n# Backend\\n/src/api/ @backend-team\\n*.sql @backend-team\\n\\n# Documentation\\n/docs/ @docs-team\\n*.md @docs-team\",\n\
                      \"message\": \"Add CODEOWNERS\"\n\
                    })\n\n\
                 4. ADD CONTRIBUTING GUIDE:\n\
                    // Create contribution guidelines\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \"CONTRIBUTING.md\",\n\
                      \"content\": \"# Contributing\\n\\n## Getting Started\\n1. Fork the repository\\n2. Create a feature branch\\n3. Make your changes\\n4. Submit a pull request\\n\\n## Code Style\\nFollow the project's ESLint configuration.\\n\\n## Testing\\nEnsure all tests pass before submitting.\",\n\
                      \"message\": \"Add contributing guidelines\"\n\
                    })\n\n\
                 5. ADD SECURITY POLICY:\n\
                    // Create security policy\n\
                    github_push_file({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"path\": \"SECURITY.md\",\n\
                      \"content\": \"# Security Policy\\n\\n## Supported Versions\\n| Version | Supported |\\n|---------|-----------|\\n| 1.x     | :white_check_mark: |\\n\\n## Reporting a Vulnerability\\nEmail security@example.com with details.\",\n\
                      \"message\": \"Add security policy\"\n\
                    })\n\n\
                 6. REPOSITORY SETUP WORKFLOW:\n\
                    // Complete repository initialization\n\
                    \n\
                    // 1. Add README\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"README.md\", \"content\": \"# Project\\n\\nDescription here.\", \"message\": \"Initial README\"})\n\
                    \n\
                    // 2. Add license\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"LICENSE\", \"content\": \"MIT License\\n...\", \"message\": \"Add MIT license\"})\n\
                    \n\
                    // 3. Add gitignore\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \".gitignore\", \"content\": \"node_modules/\\n.env\\ndist/\", \"message\": \"Add gitignore\"})\n\
                    \n\
                    // 4. Add CI workflow\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \".github/workflows/ci.yml\", \"content\": \"...\", \"message\": \"Add CI\"})\n\
                    \n\
                    // 5. Add contribution guide\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"CONTRIBUTING.md\", \"content\": \"...\", \"message\": \"Add contributing guide\"})\n\n\
                 7. DOCUMENTATION WORKFLOW:\n\
                    // Create comprehensive docs\n\
                    \n\
                    // Main docs\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"docs/README.md\", \"content\": \"# Documentation\", \"message\": \"Add docs index\"})\n\
                    \n\
                    // Getting started\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"docs/getting-started.md\", \"content\": \"# Getting Started\", \"message\": \"Add getting started guide\"})\n\
                    \n\
                    // API reference\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"docs/api.md\", \"content\": \"# API Reference\", \"message\": \"Add API documentation\"})\n\
                    \n\
                    // Examples\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"docs/examples.md\", \"content\": \"# Examples\", \"message\": \"Add examples\"})\n\n\
                 8. CONFIGURATION DEPLOYMENT:\n\
                    // Deploy multiple configs\n\
                    \n\
                    // TypeScript config\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"tsconfig.json\", \"content\": \"{...}\", \"message\": \"Add TypeScript config\"})\n\
                    \n\
                    // Package config\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"package.json\", \"content\": \"{...}\", \"message\": \"Add package.json\"})\n\
                    \n\
                    // Linter config\n\
                    github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \".eslintrc.json\", \"content\": \"{...}\", \"message\": \"Add ESLint config\"})\n\n\
                 USE CASES:\n\
                 - Repository initialization and setup\n\
                 - Adding GitHub Actions workflows\n\
                 - Creating documentation files\n\
                 - Deploying configuration files\n\
                 - Adding governance files (CODEOWNERS, SECURITY.md)\n\
                 - Creating issue and PR templates\n\
                 - Quick single-file updates\n\
                 - Automated content generation\n\n\
                 WORKFLOW BEST PRACTICES:\n\
                 1. Start with essential files (README, LICENSE)\n\
                 2. Add configuration files next\n\
                 3. Set up CI/CD workflows\n\
                 4. Add governance and contribution files\n\
                 5. Create comprehensive documentation\n\
                 6. Use descriptive commit messages\n\
                 7. Follow conventional commit format\n\n\
                 AUTOMATION PATTERNS:\n\
                 - Generate docs from code → push with github_push_file\n\
                 - Template repository creation → batch file creation\n\
                 - Configuration management → automated updates\n\
                 - CI/CD pipeline → workflow file deployment\n\n\
                 WHEN TO USE BATCH OPERATIONS:\n\
                 For multiple related files, consider github_push_files:\n\
                 - Atomic commits for multiple files\n\
                 - Better for large-scale updates\n\
                 - Single commit for consistency\n\
                 \n\
                 Use github_push_file for:\n\
                 - Single file operations\n\
                 - Independent file updates\n\
                 - Quick additions\n\
                 - Iterative development",
            ),
        },
    ]
}

/// Comprehensive guide covering all aspects
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using the github_push_file tool effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_push_file tool provides a powerful way to create or update single files in GitHub repositories via the API, without local cloning.\n\n\
                 =============================================================================\n\
                 COMPLETE GITHUB_PUSH_FILE GUIDE\n\
                 =============================================================================\n\n\
                 TOOL PURPOSE:\n\
                 Create or update a single file in a GitHub repository via the GitHub API.\n\
                 No local repository clone required. Fast, direct, API-based file operations.\n\n\
                 BASIC SYNTAX:\n\
                 github_push_file({\n\
                   \"owner\": \"username\",      // Repository owner\n\
                   \"repo\": \"project\",        // Repository name\n\
                   \"path\": \"file.txt\",       // File path in repository\n\
                   \"content\": \"content\",     // File content (plain text)\n\
                   \"message\": \"commit msg\",  // Commit message\n\
                   \"branch\": \"main\"          // Optional: target branch\n\
                 })\n\n\
                 =============================================================================\n\
                 PARAMETERS EXPLAINED\n\
                 =============================================================================\n\n\
                 REQUIRED PARAMETERS:\n\
                 \n\
                 1. owner (string, required)\n\
                    - Repository owner: username or organization\n\
                    - Examples: \"torvalds\", \"rust-lang\", \"microsoft\"\n\
                    \n\
                 2. repo (string, required)\n\
                    - Repository name\n\
                    - Examples: \"linux\", \"rust\", \"vscode\"\n\
                    \n\
                 3. path (string, required)\n\
                    - File path relative to repository root\n\
                    - No leading slash\n\
                    - Use forward slashes\n\
                    - Examples: \"README.md\", \"src/lib.rs\", \".github/workflows/ci.yml\"\n\
                    \n\
                 4. content (string, required)\n\
                    - File content as plain text string\n\
                    - NOT base64 encoded\n\
                    - UTF-8 encoding\n\
                    - Escape special characters in JSON (\\n, \\\", \\\\)\n\
                    \n\
                 5. message (string, required)\n\
                    - Commit message describing the change\n\
                    - Should be clear and descriptive\n\
                    - Examples: \"Add README\", \"Update configuration\", \"Fix typo in docs\"\n\n\
                 OPTIONAL PARAMETERS:\n\
                 \n\
                 1. branch (string, optional)\n\
                    - Target branch name\n\
                    - If omitted, uses repository default branch\n\
                    - Branch must exist before pushing\n\
                    - Examples: \"main\", \"develop\", \"feature/auth\"\n\
                    \n\
                 2. sha (string, optional)\n\
                    - SHA of file being replaced\n\
                    - Required for updates to prevent conflicts\n\
                    - Get from github_get_file_contents first\n\
                    - Omit when creating new files\n\n\
                 =============================================================================\n\
                 RESPONSE STRUCTURE\n\
                 =============================================================================\n\n\
                 Returns JSON object with two main sections:\n\
                 \n\
                 1. content: File information\n\
                    - path: File path in repository\n\
                    - sha: New SHA hash of file\n\
                    - size: File size in bytes\n\
                    - name: File name\n\
                    - type: \"file\"\n\
                    \n\
                 2. commit: Commit information\n\
                    - sha: Commit SHA hash\n\
                    - message: Commit message\n\
                    - author: Author details (name, email, date)\n\
                    - committer: Committer details\n\
                    - html_url: GitHub URL to view commit\n\n\
                 Example response:\n\
                 {\n\
                   \"content\": {\n\
                     \"path\": \"docs/README.md\",\n\
                     \"sha\": \"abc123...\",\n\
                     \"size\": 245,\n\
                     \"name\": \"README.md\",\n\
                     \"type\": \"file\"\n\
                   },\n\
                   \"commit\": {\n\
                     \"sha\": \"def456...\",\n\
                     \"message\": \"Add documentation\",\n\
                     \"author\": {...},\n\
                     \"committer\": {...},\n\
                     \"html_url\": \"https://github.com/owner/repo/commit/def456...\"\n\
                   }\n\
                 }\n\n\
                 =============================================================================\n\
                 COMMON EXAMPLES\n\
                 =============================================================================\n\n\
                 1. CREATE NEW README:\n\
                 github_push_file({\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"path\": \"README.md\",\n\
                   \"content\": \"# My Project\\n\\nWelcome to my project.\",\n\
                   \"message\": \"Add README\"\n\
                 })\n\n\
                 2. ADD GITHUB WORKFLOW:\n\
                 github_push_file({\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"path\": \".github/workflows/test.yml\",\n\
                   \"content\": \"name: Tests\\non: [push]\\njobs:\\n  test:\\n    runs-on: ubuntu-latest\\n    steps:\\n      - uses: actions/checkout@v3\",\n\
                   \"message\": \"Add test workflow\"\n\
                 })\n\n\
                 3. ADD CONFIGURATION FILE:\n\
                 github_push_file({\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"path\": \"config.json\",\n\
                   \"content\": \"{\\\"version\\\": \\\"1.0\\\", \\\"debug\\\": false}\",\n\
                   \"message\": \"Add configuration\"\n\
                 })\n\n\
                 4. CREATE IN FEATURE BRANCH:\n\
                 github_push_file({\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"path\": \"src/feature.js\",\n\
                   \"content\": \"export function feature() {}\\n\",\n\
                   \"message\": \"Implement new feature\",\n\
                   \"branch\": \"feature/new-feature\"\n\
                 })\n\n\
                 5. ADD CODEOWNERS:\n\
                 github_push_file({\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"path\": \".github/CODEOWNERS\",\n\
                   \"content\": \"* @team-lead\\n/docs/ @docs-team\",\n\
                   \"message\": \"Add code ownership\"\n\
                 })\n\n\
                 =============================================================================\n\
                 WORKING WITH BRANCHES\n\
                 =============================================================================\n\n\
                 DEFAULT BRANCH:\n\
                 - Omit branch parameter to use repository default\n\
                 - Usually \"main\" or \"master\"\n\
                 \n\
                 SPECIFIC BRANCH:\n\
                 - Specify branch parameter for non-default branches\n\
                 - Branch must already exist\n\
                 - Create branch first if needed:\n\
                   github_create_branch({\"owner\": \"u\", \"repo\": \"r\", \"branch\": \"feature/x\"})\n\
                   github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"...\", \"branch\": \"feature/x\", ...})\n\n\
                 BRANCH WORKFLOWS:\n\
                 \n\
                 Feature development:\n\
                 1. Create feature branch\n\
                 2. Push files to feature branch\n\
                 3. Create pull request\n\
                 4. Review and merge\n\
                 \n\
                 Direct to main (not recommended):\n\
                 - Only for urgent fixes or initial setup\n\
                 - Check if branch is protected first\n\
                 - Protected branches require PRs\n\n\
                 =============================================================================\n\
                 UPDATING EXISTING FILES\n\
                 =============================================================================\n\n\
                 To update existing file:\n\
                 \n\
                 Step 1 - Get current file SHA:\n\
                 github_get_file_contents({\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"path\": \"file.txt\"\n\
                 })\n\
                 // Response includes sha field\n\
                 \n\
                 Step 2 - Update with SHA:\n\
                 github_push_file({\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"path\": \"file.txt\",\n\
                   \"content\": \"updated content\",\n\
                   \"message\": \"Update file\",\n\
                   \"sha\": \"abc123...\"  // From step 1\n\
                 })\n\
                 \n\
                 WHY SHA IS NEEDED:\n\
                 - Prevents accidental overwrites\n\
                 - Ensures you're updating expected version\n\
                 - GitHub rejects updates without SHA for existing files\n\
                 - Acts as optimistic locking mechanism\n\n\
                 =============================================================================\n\
                 AUTHENTICATION\n\
                 =============================================================================\n\n\
                 GITHUB_TOKEN ENVIRONMENT VARIABLE:\n\
                 - Required for authentication\n\
                 - Personal access token or GitHub App token\n\
                 \n\
                 REQUIRED SCOPES:\n\
                 - repo: Full repository access (private repos)\n\
                 - public_repo: Public repository access only\n\
                 \n\
                 CREATE TOKEN:\n\
                 1. GitHub Settings → Developer settings → Personal access tokens\n\
                 2. Generate new token\n\
                 3. Select required scopes\n\
                 4. Save token securely\n\
                 5. Set GITHUB_TOKEN environment variable\n\n\
                 =============================================================================\n\
                 ERROR HANDLING\n\
                 =============================================================================\n\n\
                 COMMON ERRORS:\n\
                 \n\
                 404 Not Found:\n\
                 - Repository doesn't exist\n\
                 - Branch doesn't exist\n\
                 - No access to private repository\n\
                 Fix: Verify owner/repo/branch, check permissions\n\
                 \n\
                 409 Conflict:\n\
                 - File changed since SHA was obtained\n\
                 - Concurrent modifications\n\
                 Fix: Get fresh SHA and retry\n\
                 \n\
                 422 Unprocessable Entity:\n\
                 - Invalid path format\n\
                 - Invalid content\n\
                 - Invalid parameters\n\
                 Fix: Check path format, content encoding\n\
                 \n\
                 403 Forbidden:\n\
                 - Insufficient permissions\n\
                 - Branch is protected\n\
                 - Repository is archived\n\
                 Fix: Check token scopes, use PR workflow for protected branches\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. COMMIT MESSAGES:\n\
                    - Use descriptive messages\n\
                    - Follow conventional commits format\n\
                    - Examples: \"feat: Add feature\", \"fix: Fix bug\", \"docs: Update README\"\n\
                    \n\
                 2. BRANCH STRATEGY:\n\
                    - Use feature branches for changes\n\
                    - Avoid direct commits to main\n\
                    - Create PRs for review\n\
                    \n\
                 3. FILE UPDATES:\n\
                    - Always get SHA before updating\n\
                    - Handle 409 conflicts gracefully\n\
                    - Retry with fresh SHA\n\
                    \n\
                 4. CONTENT SIZE:\n\
                    - Keep files under 1MB\n\
                    - Use git LFS for large files\n\
                    - Consider chunking large content\n\
                    \n\
                 5. PATH FORMAT:\n\
                    - Use forward slashes\n\
                    - No leading slash\n\
                    - Relative to repository root\n\
                    \n\
                 6. AUTOMATION:\n\
                    - Check branch protection rules\n\
                    - Handle rate limits (5000 req/hour)\n\
                    - Implement retry logic\n\
                    - Log commit SHAs for tracking\n\n\
                 =============================================================================\n\
                 USE CASES\n\
                 =============================================================================\n\n\
                 IDEAL FOR:\n\
                 - Automated documentation generation\n\
                 - Configuration file deployment\n\
                 - GitHub Actions workflow management\n\
                 - Issue/PR template creation\n\
                 - Single file updates\n\
                 - Quick additions without local clone\n\
                 - CI/CD pipeline integration\n\
                 - Repository initialization\n\
                 \n\
                 NOT IDEAL FOR:\n\
                 - Multiple related files (use github_push_files)\n\
                 - Large binary files (use git LFS)\n\
                 - Complex merge scenarios\n\
                 - Operations requiring full git history locally\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Create file:\n\
                 github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"f\", \"content\": \"c\", \"message\": \"m\"})\n\
                 \n\
                 Create in branch:\n\
                 github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"f\", \"content\": \"c\", \"message\": \"m\", \"branch\": \"b\"})\n\
                 \n\
                 Update file:\n\
                 github_push_file({\"owner\": \"u\", \"repo\": \"r\", \"path\": \"f\", \"content\": \"c\", \"message\": \"m\", \"sha\": \"s\"})\n\
                 \n\
                 Remember: This tool is for single file operations. For multiple files, use github_push_files for atomic commits!",
            ),
        },
    ]
}
