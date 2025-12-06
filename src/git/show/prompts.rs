//! Prompt messages for git_show tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitShowPromptArgs;

/// Prompt provider for git_show tool
///
/// This is the ONLY way to provide prompts for git_show - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ShowPrompts;

impl PromptProvider for ShowPrompts {
    type PromptArgs = GitShowPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("commits") => prompt_commits(),
            Some("tags") => prompt_tags(),
            Some("files") => prompt_files(),
            Some("formatting") => prompt_formatting(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (commits, tags, files, formatting)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GIT_SHOW
// ============================================================================

/// Showing commits
fn prompt_commits() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I view commit details with git_show?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_show to display detailed information about commits including metadata and full diffs.\n\n\
                 SHOWING COMMITS:\n\n\
                 1. Show specific commit:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"abc1234\"\n\
                    })\n\
                    Displays complete commit information\n\n\
                 2. Show HEAD (current commit):\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"HEAD\"\n\
                    })\n\
                    Shows the most recent commit on current branch\n\n\
                 3. Show parent commit:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"HEAD~1\"\n\
                    })\n\
                    Previous commit (one before HEAD)\n\n\
                 4. Show ancestor:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"HEAD~5\"\n\
                    })\n\
                    Five commits before HEAD\n\n\
                 5. Show specific commit with abbreviated hash:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"a1b2c3d\"\n\
                    })\n\
                    7+ character abbreviated hash works\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"commit\": \"abc1234567890abcdef1234567890abcdef12345\",\n\
                   \"author\": \"John Doe <john@example.com>\",\n\
                   \"author_date\": \"2024-01-15T10:30:00Z\",\n\
                   \"committer\": \"John Doe <john@example.com>\",\n\
                   \"commit_date\": \"2024-01-15T10:30:00Z\",\n\
                   \"message\": \"Add user authentication\\n\\nImplemented JWT-based auth system with refresh tokens.\",\n\
                   \"diff\": \"diff --git a/src/auth.rs b/src/auth.rs\\nnew file mode 100644\\nindex 0000000..abc1234\\n--- /dev/null\\n+++ b/src/auth.rs\\n@@ -0,0 +1,50 @@\\n+use jwt::Token;\\n+...\\n\"\n\
                 }\n\n\
                 WHAT'S SHOWN IN COMMIT:\n\
                 - Full commit hash (40-character SHA-1)\n\
                 - Author name and email\n\
                 - Author date (when changes were made)\n\
                 - Committer name and email (usually same as author)\n\
                 - Commit date (when commit was created)\n\
                 - Complete commit message (subject and body)\n\
                 - Full diff showing all changes in the commit\n\n\
                 COMMIT MESSAGE STRUCTURE:\n\
                 Messages typically contain:\n\
                 - Subject line: Brief summary (first line)\n\
                 - Body: Detailed explanation (after blank line)\n\
                 - Footers: Issue references, breaking changes, etc.\n\n\
                 Example message:\n\
                 \"Add user authentication\\n\\n\
                 Implemented JWT-based authentication system:\\n\
                 - Added token generation and validation\\n\
                 - Implemented refresh token mechanism\\n\
                 - Added middleware for protected routes\\n\\n\
                 Fixes #123\\n\
                 Breaking change: Auth endpoints now require API version header\"\n\n\
                 DIFF INTERPRETATION:\n\
                 The diff shows all changes made in this commit:\n\
                 - New files: \"new file mode 100644\"\n\
                 - Modified files: \"index abc1234..def5678\"\n\
                 - Deleted files: \"deleted file mode 100644\"\n\
                 - Lines added: Start with '+'\n\
                 - Lines removed: Start with '-'\n\
                 - Context lines: Start with ' ' (space)\n\n\
                 COMMIT REFERENCES:\n\
                 Ways to specify commits:\n\n\
                 Hash-based:\n\
                 - Full: \"abc1234567890abcdef1234567890abcdef12345\"\n\
                 - Abbreviated: \"abc1234\" (7+ characters)\n\
                 - git_show({\"path\": \"/project\", \"object\": \"abc1234\"})\n\n\
                 Symbolic:\n\
                 - HEAD: Current commit\n\
                 - branch_name: Latest commit on branch\n\
                 - git_show({\"path\": \"/project\", \"object\": \"main\"})\n\n\
                 Relative:\n\
                 - HEAD~1: Parent of HEAD\n\
                 - HEAD~5: 5 commits before HEAD\n\
                 - main~3: 3 commits before main branch tip\n\
                 - git_show({\"path\": \"/project\", \"object\": \"HEAD~2\"})\n\n\
                 Parent selection (merge commits):\n\
                 - HEAD^1: First parent\n\
                 - HEAD^2: Second parent\n\
                 - git_show({\"path\": \"/project\", \"object\": \"HEAD^2\"})\n\n\
                 WHEN TO USE git_show FOR COMMITS:\n\n\
                 Code review:\n\
                 - Review specific commit in detail\n\
                 - Understand what changed and why\n\
                 - See complete context of changes\n\n\
                 Investigation:\n\
                 - Identify when bug was introduced\n\
                 - Track down who made specific change\n\
                 - Understand reasoning from commit message\n\n\
                 Documentation:\n\
                 - Generate release notes from commits\n\
                 - Document major changes\n\
                 - Reference specific implementations\n\n\
                 Learning:\n\
                 - Study how feature was implemented\n\
                 - Learn from historical changes\n\
                 - Understand codebase evolution\n\n\
                 PRACTICAL EXAMPLES:\n\n\
                 Example 1 - Review last commit:\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD\"})\n\
                 See what was just committed\n\n\
                 Example 2 - Investigate specific change:\n\
                 git_show({\"path\": \"/project\", \"object\": \"a1b2c3d\"})\n\
                 Review commit that introduced feature\n\n\
                 Example 3 - Check merge commit:\n\
                 git_show({\"path\": \"/project\", \"object\": \"merge_commit_hash\"})\n\
                 See what was merged (shows combined changes)\n\n\
                 Example 4 - Review recent commits:\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD~1\"})\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD~2\"})\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD~3\"})\n\
                 Walk back through recent history\n\n\
                 WORKFLOW INTEGRATION:\n\n\
                 After committing:\n\
                 1. git_commit({\"path\": \"/project\", \"message\": \"Add feature\"})\n\
                 2. git_show({\"path\": \"/project\", \"object\": \"HEAD\"})\n\
                 3. Verify commit looks correct\n\n\
                 Before pushing:\n\
                 1. git_log({\"path\": \"/project\", \"max_count\": 5})\n\
                 2. git_show({\"path\": \"/project\", \"object\": \"commit_hash\"})\n\
                 3. Review each commit for quality\n\n\
                 During debugging:\n\
                 1. git_log({\"path\": \"/project\", \"file\": \"buggy_file.rs\"})\n\
                 2. git_show({\"path\": \"/project\", \"object\": \"suspect_commit\"})\n\
                 3. Identify problematic change\n\n\
                 AUTHOR vs COMMITTER:\n\
                 Understanding the difference:\n\n\
                 Author:\n\
                 - Person who wrote the code\n\
                 - Original creation date\n\
                 - Usually remains unchanged\n\n\
                 Committer:\n\
                 - Person who committed the code\n\
                 - When it was committed to repository\n\
                 - Can differ if commit is amended, rebased, or cherry-picked\n\n\
                 Common scenarios where they differ:\n\
                 - Applying patches from others\n\
                 - Rebasing branches (committer date updates)\n\
                 - Cherry-picking commits\n\
                 - Amending commits\n\n\
                 COMMIT HASH DISCOVERY:\n\
                 How to find commits to show:\n\n\
                 1. Use git_log:\n\
                    git_log({\"path\": \"/project\", \"max_count\": 10})\n\
                    Returns recent commits with hashes\n\n\
                 2. Use git_branch_list:\n\
                    git_branch_list({\"path\": \"/project\"})\n\
                    Shows branch tips (latest commits)\n\n\
                 3. Search git_log:\n\
                    git_log({\"path\": \"/project\", \"grep\": \"authentication\"})\n\
                    Find commits mentioning specific topics\n\n\
                 BEST PRACTICES:\n\
                 - Use git_show to understand individual commits\n\
                 - Review commit messages for context\n\
                 - Check author/committer for attribution\n\
                 - Examine diff to understand actual changes\n\
                 - Use with git_log for historical exploration\n\
                 - Verify commits before pushing\n\
                 - Document important commits for team reference",
            ),
        },
    ]
}

/// Showing tags
fn prompt_tags() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I view tag information with git_show?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_show to display tag information, including annotated tag details or the commit a tag points to.\n\n\
                 SHOWING TAGS:\n\n\
                 1. Show annotated tag:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"v1.0.0\"\n\
                    })\n\
                    Displays tag metadata and referenced commit\n\n\
                 2. Show release tag:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"v2.1.3\"\n\
                    })\n\
                    View specific release information\n\n\
                 3. Show pre-release tag:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"v1.0.0-beta.1\"\n\
                    })\n\
                    Check pre-release versions\n\n\
                 4. Show latest tag:\n\
                    First find latest tag, then show it:\n\
                    git_show({\"path\": \"/project\", \"object\": \"latest_tag_name\"})\n\n\
                 ANNOTATED TAG RESPONSE:\n\
                 {\n\
                   \"tag\": \"v1.0.0\",\n\
                   \"tagger\": \"John Doe <john@example.com>\",\n\
                   \"tagger_date\": \"2024-01-15T14:30:00Z\",\n\
                   \"message\": \"Release 1.0.0\\n\\nFirst stable release with:\\n- User authentication\\n- API endpoints\\n- Documentation\",\n\
                   \"commit\": \"abc1234567890abcdef1234567890abcdef12345\",\n\
                   \"commit_message\": \"Finalize 1.0.0 release\",\n\
                   \"commit_author\": \"John Doe <john@example.com>\",\n\
                   \"commit_date\": \"2024-01-15T14:00:00Z\"\n\
                 }\n\n\
                 LIGHTWEIGHT TAG RESPONSE:\n\
                 {\n\
                   \"tag\": \"v0.9.0\",\n\
                   \"commit\": \"def5678901234567890abcdef1234567890abcdef\",\n\
                   \"commit_message\": \"Prepare for 0.9.0 release\",\n\
                   \"commit_author\": \"Jane Smith <jane@example.com>\",\n\
                   \"commit_date\": \"2024-01-10T10:00:00Z\"\n\
                 }\n\
                 Note: No tagger or tag message for lightweight tags\n\n\
                 ANNOTATED vs LIGHTWEIGHT TAGS:\n\
                 Understanding the difference:\n\n\
                 Annotated tags (recommended for releases):\n\
                 - Full git objects with metadata\n\
                 - Include tagger name and email\n\
                 - Include tagger date (when tag was created)\n\
                 - Include tag message (release notes, changelog)\n\
                 - Can be signed with GPG for verification\n\
                 - Created with: git tag -a v1.0.0 -m \"message\"\n\
                 - Show full information with git_show\n\n\
                 Lightweight tags (simple pointers):\n\
                 - Just a reference to a commit\n\
                 - No tagger information\n\
                 - No tag message\n\
                 - No signing capability\n\
                 - Created with: git tag v0.9.0\n\
                 - git_show displays the commit directly\n\n\
                 TAG INFORMATION BREAKDOWN:\n\n\
                 tag:\n\
                 - Tag name (usually semantic version: v1.2.3)\n\
                 - Follows versioning convention\n\
                 - May include pre-release: v1.0.0-alpha.1\n\
                 - May include build metadata: v1.0.0+20240115\n\n\
                 tagger:\n\
                 - Person who created the tag\n\
                 - Name and email\n\
                 - Only present for annotated tags\n\
                 - Important for release accountability\n\n\
                 tagger_date:\n\
                 - When tag was created\n\
                 - May differ from commit date\n\
                 - Useful for release timeline tracking\n\n\
                 message:\n\
                 - Tag annotation message\n\
                 - Often contains release notes\n\
                 - Changelog summary\n\
                 - Breaking changes\n\
                 - Migration instructions\n\n\
                 commit:\n\
                 - SHA-1 hash of commit this tag points to\n\
                 - The actual code snapshot for this version\n\
                 - Use to checkout specific version\n\n\
                 SEMANTIC VERSIONING:\n\
                 Common tag naming patterns:\n\n\
                 Standard releases:\n\
                 - v1.0.0: Major.Minor.Patch\n\
                 - v2.1.0: Minor version increment\n\
                 - v2.1.1: Patch version increment\n\
                 - v3.0.0: Major version (breaking changes)\n\n\
                 Pre-releases:\n\
                 - v1.0.0-alpha.1: Alpha release\n\
                 - v1.0.0-beta.2: Beta release\n\
                 - v1.0.0-rc.1: Release candidate\n\n\
                 Build metadata:\n\
                 - v1.0.0+20240115: With build date\n\
                 - v1.0.0+abc1234: With commit hash\n\n\
                 WHEN TO USE git_show FOR TAGS:\n\n\
                 Release verification:\n\
                 - Check what's in a release\n\
                 - Verify release notes\n\
                 - Confirm release author and date\n\
                 - Review tagged commit\n\n\
                 Version comparison:\n\
                 - See differences between versions\n\
                 - Track feature additions\n\
                 - Document breaking changes\n\
                 - Generate changelogs\n\n\
                 Deployment:\n\
                 - Verify version before deploying\n\
                 - Check release stability (alpha/beta/stable)\n\
                 - Confirm correct commit is tagged\n\
                 - Review release message for instructions\n\n\
                 Debugging:\n\
                 - Check what code was in specific version\n\
                 - Identify when feature was released\n\
                 - Trace bug to specific release\n\
                 - Verify fix is in tagged version\n\n\
                 PRACTICAL EXAMPLES:\n\n\
                 Example 1 - Review latest release:\n\
                 git_show({\"path\": \"/project\", \"object\": \"v2.0.0\"})\n\
                 Check release notes and tagged commit\n\n\
                 Example 2 - Verify release before deployment:\n\
                 git_show({\"path\": \"/project\", \"object\": \"v1.5.3\"})\n\
                 Confirm this is the version to deploy\n\n\
                 Example 3 - Check pre-release:\n\
                 git_show({\"path\": \"/project\", \"object\": \"v2.0.0-beta.1\"})\n\
                 Review beta release details\n\n\
                 Example 4 - Compare tag to current:\n\
                 git_show({\"path\": \"/project\", \"object\": \"v1.0.0\"})\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD\"})\n\
                 See changes since release\n\n\
                 TAG DISCOVERY:\n\
                 How to find tags to show:\n\n\
                 1. List all tags:\n\
                    git_tag({\"path\": \"/project\"})\n\
                    Returns all tags in repository\n\n\
                 2. Search for version:\n\
                    git_tag({\"path\": \"/project\", \"pattern\": \"v1.*\"})\n\
                    Find all v1.x.x releases\n\n\
                 3. Recent tags:\n\
                    git_tag({\"path\": \"/project\", \"sort\": \"-creatordate\"})\n\
                    Most recent tags first\n\n\
                 RELEASE WORKFLOW:\n\n\
                 Creating release:\n\
                 1. Commit final changes\n\
                 2. Create annotated tag with release notes\n\
                 3. git_show({\"path\": \"/project\", \"object\": \"v1.0.0\"})\n\
                 4. Verify tag information is correct\n\
                 5. Push tag to remote\n\n\
                 Verifying release:\n\
                 1. git_show({\"path\": \"/project\", \"object\": \"v1.0.0\"})\n\
                 2. Check tagger and date\n\
                 3. Review release message\n\
                 4. Verify commit hash matches expected\n\
                 5. Confirm all features are included\n\n\
                 RELEASE NOTES IN TAG MESSAGES:\n\
                 Good tag messages include:\n\n\
                 Summary:\n\
                 \"Release 1.0.0 - First Stable Release\"\n\n\
                 Features:\n\
                 \"New features:\\n\
                 - User authentication with JWT\\n\
                 - RESTful API endpoints\\n\
                 - Comprehensive documentation\"\n\n\
                 Bug fixes:\n\
                 \"Bug fixes:\\n\
                 - Fixed memory leak in connection pool\\n\
                 - Resolved race condition in cache\\n\
                 - Corrected timezone handling\"\n\n\
                 Breaking changes:\n\
                 \"BREAKING CHANGES:\\n\
                 - API endpoints now require authentication\\n\
                 - Configuration format changed to TOML\\n\
                 - Minimum Node.js version is now 18\"\n\n\
                 Migration instructions:\n\
                 \"Migration from 0.9.x:\\n\
                 1. Update configuration files to TOML format\\n\
                 2. Add authentication tokens to API calls\\n\
                 3. Run database migration script\"\n\n\
                 SIGNED TAGS:\n\
                 For security-critical releases:\n\
                 - Tags can be GPG-signed\n\
                 - Verifies tag authenticity\n\
                 - Ensures tag wasn't tampered with\n\
                 - git_show displays signature information\n\
                 - Important for supply chain security\n\n\
                 BEST PRACTICES:\n\
                 - Always use annotated tags for releases\n\
                 - Include comprehensive release notes\n\
                 - Follow semantic versioning\n\
                 - Verify tags before deploying\n\
                 - Sign important release tags\n\
                 - Document breaking changes clearly\n\
                 - Reference issue numbers in tag messages\n\
                 - Keep tag messages focused and actionable",
            ),
        },
    ]
}

/// Showing files at commits
fn prompt_files() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I view files at specific commits with git_show?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_show to display file contents as they existed at a specific point in time.\n\n\
                 SHOWING FILES AT COMMIT:\n\n\
                 1. File at specific commit:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"abc1234:src/main.rs\"\n\
                    })\n\
                    Shows main.rs as it was in commit abc1234\n\n\
                 2. File in HEAD (current version):\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"HEAD:Cargo.toml\"\n\
                    })\n\
                    Shows Cargo.toml in latest commit\n\n\
                 3. File in parent commit:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"HEAD~1:src/lib.rs\"\n\
                    })\n\
                    Shows lib.rs from previous commit\n\n\
                 4. File at tag:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"v1.0.0:README.md\"\n\
                    })\n\
                    Shows README as it was in v1.0.0 release\n\n\
                 5. File on different branch:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"feature/auth:src/auth.rs\"\n\
                    })\n\
                    Shows auth.rs from feature branch\n\n\
                 6. Nested file path:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"HEAD:src/handlers/user.rs\"\n\
                    })\n\
                    Shows file in subdirectory\n\n\
                 FORMAT: commit:path\n\
                 The object parameter uses colon notation:\n\
                 - Before colon: Commit reference (hash, HEAD, tag, branch)\n\
                 - After colon: File path relative to repository root\n\
                 - Examples:\n\
                   - \"abc1234:file.txt\"\n\
                   - \"HEAD:src/main.rs\"\n\
                   - \"v1.0.0:docs/api.md\"\n\
                   - \"main:config/settings.json\"\n\n\
                 FILE RESPONSE:\n\
                 {\n\
                   \"commit\": \"abc1234567890abcdef1234567890abcdef12345\",\n\
                   \"path\": \"src/main.rs\",\n\
                   \"content\": \"fn main() {\\n    println!(\\\"Hello, world!\\\");\\n}\\n\",\n\
                   \"size\": 48,\n\
                   \"mode\": \"100644\",\n\
                   \"type\": \"blob\"\n\
                 }\n\n\
                 Response fields:\n\
                 - commit: Full SHA-1 hash of the commit\n\
                 - path: File path in repository\n\
                 - content: Complete file contents as text\n\
                 - size: File size in bytes\n\
                 - mode: Unix file permissions (100644 = regular file)\n\
                 - type: Git object type (blob = file)\n\n\
                 FILE PATHS:\n\
                 Always relative to repository root:\n\n\
                 Correct:\n\
                 - \"src/main.rs\"\n\
                 - \"docs/api.md\"\n\
                 - \"config/production.json\"\n\
                 - \"tests/integration/test_api.rs\"\n\n\
                 Incorrect:\n\
                 - \"/src/main.rs\" (no leading slash)\n\
                 - \"./src/main.rs\" (no dot prefix)\n\
                 - \"../src/main.rs\" (no parent references)\n\n\
                 USE CASES:\n\n\
                 Use Case 1 - See file at point in time:\n\
                 Check how configuration looked in old release:\n\
                 git_show({\"path\": \"/project\", \"object\": \"v1.0.0:config/app.json\"})\n\
                 See original configuration settings\n\n\
                 Use Case 2 - Compare versions manually:\n\
                 Get file from two different commits:\n\
                 git_show({\"path\": \"/project\", \"object\": \"v1.0.0:src/api.rs\"})\n\
                 git_show({\"path\": \"/project\", \"object\": \"v2.0.0:src/api.rs\"})\n\
                 Compare API changes between versions\n\n\
                 Use Case 3 - Recover old file content:\n\
                 Retrieve file that was deleted or changed:\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD~5:deleted_file.rs\"})\n\
                 Get content from before deletion\n\n\
                 Use Case 4 - Check branch differences:\n\
                 See how file differs on feature branch:\n\
                 git_show({\"path\": \"/project\", \"object\": \"main:src/core.rs\"})\n\
                 git_show({\"path\": \"/project\", \"object\": \"feature/x:src/core.rs\"})\n\
                 Compare implementations\n\n\
                 Use Case 5 - Review before merge:\n\
                 Check file from pull request branch:\n\
                 git_show({\"path\": \"/project\", \"object\": \"pr-branch:new_feature.rs\"})\n\
                 Review new file before merging\n\n\
                 Use Case 6 - Verify deployment:\n\
                 Confirm deployed version matches tag:\n\
                 git_show({\"path\": \"/project\", \"object\": \"v1.5.0:src/version.rs\"})\n\
                 Check version number in release\n\n\
                 COMMIT REFERENCES FOR FILES:\n\
                 All standard git references work:\n\n\
                 Hash-based:\n\
                 - \"abc1234:file.txt\" - Specific commit\n\
                 - \"a1b2c3d4567890:src/lib.rs\" - Full or abbreviated hash\n\n\
                 Symbolic:\n\
                 - \"HEAD:config.json\" - Current commit\n\
                 - \"main:README.md\" - Latest on main branch\n\
                 - \"origin/main:package.json\" - Remote main branch\n\n\
                 Tags:\n\
                 - \"v1.0.0:Cargo.toml\" - Release version\n\
                 - \"v2.1.3:docs/api.md\" - Specific release\n\n\
                 Relative:\n\
                 - \"HEAD~1:file.rs\" - Previous commit\n\
                 - \"HEAD~10:old_file.rs\" - 10 commits ago\n\
                 - \"main~5:src/lib.rs\" - 5 commits before main\n\n\
                 Branches:\n\
                 - \"feature/auth:src/auth.rs\" - Feature branch\n\
                 - \"develop:test.rs\" - Develop branch\n\
                 - \"hotfix/bug:fix.rs\" - Hotfix branch\n\n\
                 PRACTICAL WORKFLOWS:\n\n\
                 Workflow 1 - Bug investigation:\n\
                 File works now but broke before:\n\
                 1. git_log({\"path\": \"/project\", \"file\": \"buggy.rs\"})\n\
                 2. git_show({\"path\": \"/project\", \"object\": \"last_good:buggy.rs\"})\n\
                 3. git_show({\"path\": \"/project\", \"object\": \"first_bad:buggy.rs\"})\n\
                 4. Compare to identify problem\n\n\
                 Workflow 2 - Configuration migration:\n\
                 Need to see old config format:\n\
                 1. git_show({\"path\": \"/project\", \"object\": \"v1.0.0:config/old.yml\"})\n\
                 2. Extract needed values\n\
                 3. Migrate to new format\n\n\
                 Workflow 3 - Feature comparison:\n\
                 Two branches implement same feature:\n\
                 1. git_show({\"path\": \"/project\", \"object\": \"approach-a:feature.rs\"})\n\
                 2. git_show({\"path\": \"/project\", \"object\": \"approach-b:feature.rs\"})\n\
                 3. Compare implementations\n\
                 4. Choose best approach\n\n\
                 Workflow 4 - Release verification:\n\
                 Confirm release has correct code:\n\
                 1. git_show({\"path\": \"/project\", \"object\": \"v2.0.0:src/version.rs\"})\n\
                 2. Verify version constant matches tag\n\
                 3. Check changelog is updated\n\
                 4. git_show({\"path\": \"/project\", \"object\": \"v2.0.0:CHANGELOG.md\"})\n\n\
                 Workflow 5 - Code archaeology:\n\
                 How has file evolved over time:\n\
                 1. git_show({\"path\": \"/project\", \"object\": \"HEAD~20:src/core.rs\"})\n\
                 2. git_show({\"path\": \"/project\", \"object\": \"HEAD~10:src/core.rs\"})\n\
                 3. git_show({\"path\": \"/project\", \"object\": \"HEAD:src/core.rs\"})\n\
                 4. Track evolution and refactoring\n\n\
                 FILE DISCOVERY:\n\
                 How to find files to show:\n\n\
                 1. List files in commit:\n\
                    git_show({\"path\": \"/project\", \"object\": \"commit_hash\"})\n\
                    Diff shows all changed files\n\n\
                 2. Check specific path:\n\
                    Know the file path from repository structure\n\
                    Use tab completion if available\n\n\
                 3. Search commit history:\n\
                    git_log({\"path\": \"/project\", \"file\": \"target_file.rs\"})\n\
                    Find commits that modified file\n\n\
                 BINARY FILES:\n\
                 git_show works with text files:\n\
                 - Source code (.rs, .js, .py)\n\
                 - Configuration (.json, .toml, .yml)\n\
                 - Documentation (.md, .txt)\n\
                 - Scripts (.sh, .bash)\n\n\
                 For binary files:\n\
                 - May return base64 encoded content\n\
                 - May show file metadata only\n\
                 - Check tool documentation for specifics\n\n\
                 ERROR HANDLING:\n\
                 Common errors:\n\n\
                 File doesn't exist:\n\
                 - File wasn't in that commit\n\
                 - Check path spelling\n\
                 - Verify commit reference\n\n\
                 Invalid object:\n\
                 - Commit hash doesn't exist\n\
                 - Tag name misspelled\n\
                 - Branch doesn't exist\n\n\
                 Invalid path:\n\
                 - Path is not relative to root\n\
                 - Leading slash present\n\
                 - Incorrect directory separator\n\n\
                 BEST PRACTICES:\n\
                 - Use colon notation: \"commit:path\"\n\
                 - Paths relative to repository root\n\
                 - No leading slashes or dot prefixes\n\
                 - Use tags for release comparisons\n\
                 - Use HEAD~N for recent history\n\
                 - Combine with git_log for file history\n\
                 - Verify file exists in target commit\n\
                 - Use for recovery, comparison, archaeology",
            ),
        },
    ]
}

/// Output formatting options
fn prompt_formatting() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What output formatting options are available with git_show?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "git_show supports various formatting options to customize output and focus on specific information.\n\n\
                 OUTPUT FORMATS:\n\n\
                 1. Stats only (no diff):\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"abc1234\",\n\
                        \"stat\": true\n\
                    })\n\
                    Shows commit info + file statistics without full diff\n\n\
                 2. Names only (changed files):\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"abc1234\",\n\
                        \"name_only\": true\n\
                    })\n\
                    Lists only file names that were changed\n\n\
                 3. Custom format string:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"abc1234\",\n\
                        \"format\": \"%h %s (%an)\"\n\
                    })\n\
                    Custom commit information format\n\n\
                 4. Specific file diff:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"abc1234\",\n\
                        \"files\": [\"src/main.rs\"]\n\
                    })\n\
                    Only show diff for specified file(s)\n\n\
                 5. Name and status:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"abc1234\",\n\
                        \"name_status\": true\n\
                    })\n\
                    Shows file names with change type (Added/Modified/Deleted)\n\n\
                 STATS OUTPUT:\n\
                 With stat: true, you get summary statistics:\n\
                 {\n\
                   \"commit\": \"abc1234...\",\n\
                   \"author\": \"John Doe <john@example.com>\",\n\
                   \"message\": \"Add user authentication\",\n\
                   \"stats\": {\n\
                     \"files_changed\": 5,\n\
                     \"insertions\": 234,\n\
                     \"deletions\": 89,\n\
                     \"files\": [\n\
                       {\"path\": \"src/auth.rs\", \"insertions\": 150, \"deletions\": 0},\n\
                       {\"path\": \"src/lib.rs\", \"insertions\": 20, \"deletions\": 5},\n\
                       {\"path\": \"tests/auth_test.rs\", \"insertions\": 64, \"deletions\": 0}\n\
                     ]\n\
                   }\n\
                 }\n\
                 No full diff content, just statistics\n\n\
                 WHEN TO USE STATS:\n\
                 - Quick overview of commit size\n\
                 - Check how many files changed\n\
                 - Identify large commits\n\
                 - Faster than full diff\n\
                 - Good for commit summaries\n\n\
                 NAME ONLY OUTPUT:\n\
                 With name_only: true, you get file list:\n\
                 {\n\
                   \"commit\": \"abc1234...\",\n\
                   \"files\": [\n\
                     \"src/auth.rs\",\n\
                     \"src/lib.rs\",\n\
                     \"src/config.rs\",\n\
                     \"tests/auth_test.rs\",\n\
                     \"Cargo.toml\"\n\
                   ]\n\
                 }\n\
                 Just file paths, no diff or statistics\n\n\
                 WHEN TO USE NAME_ONLY:\n\
                 - See which files were touched\n\
                 - Find if specific file was modified\n\
                 - Fastest option (minimal data)\n\
                 - Checking commit scope\n\
                 - Identifying file changes without details\n\n\
                 NAME STATUS OUTPUT:\n\
                 With name_status: true, you get status indicators:\n\
                 {\n\
                   \"commit\": \"abc1234...\",\n\
                   \"changes\": [\n\
                     {\"status\": \"A\", \"path\": \"src/auth.rs\"},\n\
                     {\"status\": \"M\", \"path\": \"src/lib.rs\"},\n\
                     {\"status\": \"M\", \"path\": \"tests/test.rs\"},\n\
                     {\"status\": \"D\", \"path\": \"old_file.rs\"},\n\
                     {\"status\": \"R\", \"old_path\": \"old.rs\", \"new_path\": \"new.rs\"}\n\
                   ]\n\
                 }\n\n\
                 Status codes:\n\
                 - A: Added (new file)\n\
                 - M: Modified (changed file)\n\
                 - D: Deleted (removed file)\n\
                 - R: Renamed (file moved/renamed)\n\
                 - C: Copied (file copied)\n\
                 - T: Type changed (e.g., file to symlink)\n\n\
                 CUSTOM FORMAT STRINGS:\n\
                 Format placeholders:\n\n\
                 Commit information:\n\
                 - %H: Full commit hash\n\
                 - %h: Abbreviated commit hash\n\
                 - %T: Tree hash\n\
                 - %t: Abbreviated tree hash\n\
                 - %P: Parent hashes\n\
                 - %p: Abbreviated parent hashes\n\n\
                 Author information:\n\
                 - %an: Author name\n\
                 - %ae: Author email\n\
                 - %ad: Author date\n\
                 - %ar: Author date, relative\n\
                 - %at: Author date, UNIX timestamp\n\
                 - %ai: Author date, ISO 8601\n\n\
                 Committer information:\n\
                 - %cn: Committer name\n\
                 - %ce: Committer email\n\
                 - %cd: Committer date\n\
                 - %cr: Committer date, relative\n\
                 - %ct: Committer date, UNIX timestamp\n\
                 - %ci: Committer date, ISO 8601\n\n\
                 Message:\n\
                 - %s: Subject (first line)\n\
                 - %b: Body (after first line)\n\
                 - %B: Raw body (subject and body)\n\
                 - %N: Commit notes\n\n\
                 Formatting:\n\
                 - %n: Newline\n\
                 - %x00: Null byte\n\
                 - %%: Literal percent sign\n\n\
                 CUSTOM FORMAT EXAMPLES:\n\n\
                 Example 1 - One-line summary:\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD\", \"format\": \"%h %s\"})\n\
                 Output: \"abc1234 Add user authentication\"\n\n\
                 Example 2 - With author:\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD\", \"format\": \"%h %s (%an)\"})\n\
                 Output: \"abc1234 Add user authentication (John Doe)\"\n\n\
                 Example 3 - Detailed info:\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD\", \"format\": \"%H%n%an <%ae>%n%ai%n%s\"})\n\
                 Output:\n\
                 \"abc1234567890abcdef1234567890abcdef12345\n\
                 John Doe <john@example.com>\n\
                 2024-01-15 10:30:00 +0000\n\
                 Add user authentication\"\n\n\
                 Example 4 - Machine-readable:\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD\", \"format\": \"%H|%an|%at|%s\"})\n\
                 Output: \"abc123...|John Doe|1705315800|Add authentication\"\n\n\
                 SPECIFIC FILE FILTERING:\n\
                 Show diff for only certain files:\n\n\
                 Single file:\n\
                 git_show({\n\
                     \"path\": \"/project\",\n\
                     \"object\": \"abc1234\",\n\
                     \"files\": [\"src/main.rs\"]\n\
                 })\n\
                 Only shows changes to main.rs in that commit\n\n\
                 Multiple files:\n\
                 git_show({\n\
                     \"path\": \"/project\",\n\
                     \"object\": \"abc1234\",\n\
                     \"files\": [\"src/auth.rs\", \"src/session.rs\", \"tests/auth_test.rs\"]\n\
                 })\n\
                 Shows changes to only these three files\n\n\
                 Directory:\n\
                 git_show({\n\
                     \"path\": \"/project\",\n\
                     \"object\": \"abc1234\",\n\
                     \"files\": [\"src/handlers/\"]\n\
                 })\n\
                 Shows changes to all files in handlers directory\n\n\
                 COMBINING OPTIONS:\n\
                 Multiple options can work together:\n\n\
                 Stats for specific files:\n\
                 git_show({\n\
                     \"path\": \"/project\",\n\
                     \"object\": \"abc1234\",\n\
                     \"stat\": true,\n\
                     \"files\": [\"src/\"]\n\
                 })\n\
                 Statistics for only source files\n\n\
                 Custom format with stats:\n\
                 git_show({\n\
                     \"path\": \"/project\",\n\
                     \"object\": \"abc1234\",\n\
                     \"format\": \"%h %s\",\n\
                     \"stat\": true\n\
                 })\n\
                 Custom commit info + statistics\n\n\
                 PERFORMANCE CONSIDERATIONS:\n\n\
                 Fastest to slowest:\n\
                 1. name_only: true - Just file names\n\
                 2. name_status: true - File names + status codes\n\
                 3. stat: true - Statistics without full diff\n\
                 4. files: [...] - Full diff for subset\n\
                 5. (default) - Full commit with complete diff\n\n\
                 For large commits:\n\
                 - Use stat: true to check size first\n\
                 - Use name_only to see affected files\n\
                 - Use files: [...] to review specific files\n\
                 - Avoid full diff if you only need summary\n\n\
                 PRACTICAL USE CASES:\n\n\
                 Use Case 1 - Quick commit size check:\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD\", \"stat\": true})\n\
                 See how large the commit is before viewing full diff\n\n\
                 Use Case 2 - Find if file was changed:\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"name_only\": true})\n\
                 Check if config file was modified\n\n\
                 Use Case 3 - Review specific module:\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"files\": [\"src/api/\"]})\n\
                 Only see API changes, ignore other modules\n\n\
                 Use Case 4 - Generate changelog:\n\
                 git_show({\"path\": \"/project\", \"object\": \"v1.0.0\", \"format\": \"- %s (%an)\"})\n\
                 Format for changelog entry\n\n\
                 Use Case 5 - Identify file additions:\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"name_status\": true})\n\
                 See which files were added vs modified\n\n\
                 BEST PRACTICES:\n\
                 - Use stat: true for commit summaries\n\
                 - Use name_only for quick file lists\n\
                 - Use files: [...] to focus reviews\n\
                 - Use custom formats for scripts/automation\n\
                 - Use name_status to understand change types\n\
                 - Start with stats, then get full diff if needed\n\
                 - Filter to relevant files for large commits\n\
                 - Use format strings for machine-readable output",
            ),
        },
    ]
}

/// Comprehensive git_show guide
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using git_show effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "git_show displays information about git objects - commits, tags, trees, and files at specific points in time.\n\n\
                 CORE CONCEPTS:\n\n\
                 What git_show displays:\n\
                 - Commit details: Metadata, message, and full diff\n\
                 - Tag information: Annotated tag details and referenced commit\n\
                 - File contents: Files as they existed at specific commits\n\
                 - Tree listings: Directory structure at commits (advanced)\n\n\
                 Basic syntax:\n\
                 git_show({\n\
                     \"path\": \"/repository\",\n\
                     \"object\": \"reference\"\n\
                 })\n\n\
                 =============================================================================\n\
                 SECTION 1: SHOWING COMMITS\n\
                 =============================================================================\n\n\
                 Display commit information with full diff:\n\n\
                 Specific commit:\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234\"})\n\
                 Shows complete commit details\n\n\
                 Current commit:\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD\"})\n\
                 Latest commit on current branch\n\n\
                 Previous commits:\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD~1\"})  // One back\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD~5\"})  // Five back\n\
                 Walk through recent history\n\n\
                 Branch tip:\n\
                 git_show({\"path\": \"/project\", \"object\": \"main\"})\n\
                 Latest commit on main branch\n\n\
                 Commit response includes:\n\
                 - Full commit hash (SHA-1)\n\
                 - Author name and email\n\
                 - Author date (when changes were made)\n\
                 - Committer name and email\n\
                 - Commit date (when committed)\n\
                 - Complete commit message (subject + body)\n\
                 - Full diff showing all file changes\n\n\
                 =============================================================================\n\
                 SECTION 2: SHOWING TAGS\n\
                 =============================================================================\n\n\
                 Display tag information and referenced commits:\n\n\
                 Annotated tag:\n\
                 git_show({\"path\": \"/project\", \"object\": \"v1.0.0\"})\n\
                 Shows tag metadata + commit\n\n\
                 Tag response includes:\n\
                 - Tag name\n\
                 - Tagger name and email (annotated tags only)\n\
                 - Tag date (when tag was created)\n\
                 - Tag message / release notes (annotated tags only)\n\
                 - Referenced commit hash\n\
                 - Commit message and details\n\n\
                 Annotated vs lightweight:\n\
                 - Annotated: Full metadata with message\n\
                 - Lightweight: Just points to commit (shows commit directly)\n\n\
                 =============================================================================\n\
                 SECTION 3: SHOWING FILES AT COMMITS\n\
                 =============================================================================\n\n\
                 Display file contents as they existed at specific points:\n\n\
                 Format: \"commit:path\"\n\
                 Use colon notation to specify file at commit\n\n\
                 File at commit:\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234:src/main.rs\"})\n\
                 Shows main.rs from that commit\n\n\
                 File in HEAD:\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD:config.json\"})\n\
                 Current version in repository\n\n\
                 File at tag:\n\
                 git_show({\"path\": \"/project\", \"object\": \"v1.0.0:README.md\"})\n\
                 README from release v1.0.0\n\n\
                 File on branch:\n\
                 git_show({\"path\": \"/project\", \"object\": \"feature/auth:src/auth.rs\"})\n\
                 File from feature branch\n\n\
                 File response includes:\n\
                 - Commit hash\n\
                 - File path\n\
                 - Complete file contents\n\
                 - File size in bytes\n\
                 - File mode (permissions)\n\
                 - Object type (blob)\n\n\
                 =============================================================================\n\
                 SECTION 4: OUTPUT FORMATTING\n\
                 =============================================================================\n\n\
                 Customize output to focus on specific information:\n\n\
                 Statistics only:\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"stat\": true})\n\
                 Commit info + file statistics, no full diff\n\
                 - Faster than full diff\n\
                 - Shows files changed, insertions, deletions\n\
                 - Good for commit summaries\n\n\
                 File names only:\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"name_only\": true})\n\
                 Just list changed file paths\n\
                 - Fastest option\n\
                 - See which files were touched\n\
                 - No diff content or statistics\n\n\
                 File names with status:\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"name_status\": true})\n\
                 File paths with change indicators\n\
                 - A: Added, M: Modified, D: Deleted\n\
                 - R: Renamed, C: Copied\n\
                 - Understand change types\n\n\
                 Custom format:\n\
                 git_show({\"path\": \"/project\", \"object\": \"HEAD\", \"format\": \"%h %s (%an)\"})\n\
                 Custom commit information format\n\
                 - %h: Short hash\n\
                 - %H: Full hash\n\
                 - %s: Subject line\n\
                 - %an: Author name\n\
                 - %ae: Author email\n\
                 - %ad: Author date\n\
                 - %cn: Committer name\n\
                 - %cd: Committer date\n\n\
                 Specific files only:\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"files\": [\"src/api.rs\"]})\n\
                 Only show diff for specified files\n\
                 - Filter large commits\n\
                 - Focus on relevant changes\n\
                 - Can specify multiple files or directories\n\n\
                 =============================================================================\n\
                 SECTION 5: OBJECT REFERENCES\n\
                 =============================================================================\n\n\
                 Ways to specify objects in git_show:\n\n\
                 Commit hashes:\n\
                 - \"abc1234567890abcdef...\" (full 40-character SHA-1)\n\
                 - \"abc1234\" (abbreviated, 7+ characters)\n\
                 - git_show({\"path\": \"/project\", \"object\": \"abc1234\"})\n\n\
                 Symbolic references:\n\
                 - \"HEAD\" - Current commit\n\
                 - \"main\" - Tip of main branch\n\
                 - \"origin/main\" - Remote main branch\n\
                 - \"feature/auth\" - Feature branch tip\n\
                 - git_show({\"path\": \"/project\", \"object\": \"main\"})\n\n\
                 Tags:\n\
                 - \"v1.0.0\" - Release tag\n\
                 - \"v2.1.3\" - Specific version\n\
                 - \"v1.0.0-beta.1\" - Pre-release tag\n\
                 - git_show({\"path\": \"/project\", \"object\": \"v1.0.0\"})\n\n\
                 Relative references:\n\
                 - \"HEAD~1\" - One commit before HEAD (parent)\n\
                 - \"HEAD~5\" - Five commits before HEAD\n\
                 - \"HEAD^\" - Parent of HEAD (same as HEAD~1)\n\
                 - \"main~3\" - Three commits before main\n\
                 - git_show({\"path\": \"/project\", \"object\": \"HEAD~2\"})\n\n\
                 Parent selection (merge commits):\n\
                 - \"HEAD^1\" - First parent\n\
                 - \"HEAD^2\" - Second parent\n\
                 - git_show({\"path\": \"/project\", \"object\": \"HEAD^2\"})\n\n\
                 File at commit (colon notation):\n\
                 - \"commit:path\" - File at specific point\n\
                 - \"HEAD:file.rs\" - File in current commit\n\
                 - \"v1.0.0:README.md\" - File in release\n\
                 - \"main:config.json\" - File on branch\n\
                 - git_show({\"path\": \"/project\", \"object\": \"HEAD:src/main.rs\"})\n\n\
                 =============================================================================\n\
                 SECTION 6: COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 Workflow 1 - Review recent commits:\n\
                 1. git_show({\"path\": \"/project\", \"object\": \"HEAD\"})\n\
                 2. git_show({\"path\": \"/project\", \"object\": \"HEAD~1\"})\n\
                 3. git_show({\"path\": \"/project\", \"object\": \"HEAD~2\"})\n\
                 Walk back through history\n\n\
                 Workflow 2 - Verify release:\n\
                 1. git_show({\"path\": \"/project\", \"object\": \"v1.0.0\"})\n\
                 2. Review tag message and commit\n\
                 3. Check release notes\n\
                 4. Verify tagged commit is correct\n\n\
                 Workflow 3 - Compare file versions:\n\
                 1. git_show({\"path\": \"/project\", \"object\": \"v1.0.0:config.json\"})\n\
                 2. git_show({\"path\": \"/project\", \"object\": \"HEAD:config.json\"})\n\
                 3. Compare configurations manually\n\
                 4. Identify changes between versions\n\n\
                 Workflow 4 - Investigate bug:\n\
                 1. git_log({\"path\": \"/project\", \"file\": \"buggy.rs\"})\n\
                 2. git_show({\"path\": \"/project\", \"object\": \"suspect_commit\"})\n\
                 3. Review changes in that commit\n\
                 4. Identify problematic code\n\n\
                 Workflow 5 - Pre-push review:\n\
                 1. git_show({\"path\": \"/project\", \"object\": \"HEAD\", \"stat\": true})\n\
                 2. Check commit size\n\
                 3. git_show({\"path\": \"/project\", \"object\": \"HEAD\"})\n\
                 4. Review full changes\n\
                 5. Verify quality before pushing\n\n\
                 Workflow 6 - Recover deleted file:\n\
                 1. git_log({\"path\": \"/project\", \"file\": \"deleted.rs\"})\n\
                 2. Find last commit with file\n\
                 3. git_show({\"path\": \"/project\", \"object\": \"last_commit:deleted.rs\"})\n\
                 4. Copy contents to restore file\n\n\
                 =============================================================================\n\
                 SECTION 7: USE CASES\n\
                 =============================================================================\n\n\
                 Code Review:\n\
                 - Review specific commits in detail\n\
                 - Examine commit messages for context\n\
                 - Verify changes before approving\n\
                 - Check author and timestamps\n\n\
                 Release Management:\n\
                 - Verify tagged releases\n\
                 - Review release notes in tag messages\n\
                 - Confirm correct commit is tagged\n\
                 - Generate changelogs from tags\n\n\
                 Debugging:\n\
                 - Identify when bugs were introduced\n\
                 - Track down problematic changes\n\
                 - Review file history\n\
                 - Compare working vs broken versions\n\n\
                 File Recovery:\n\
                 - Retrieve deleted files\n\
                 - Restore old versions\n\
                 - Compare configuration versions\n\
                 - Recover accidentally overwritten code\n\n\
                 Learning:\n\
                 - Study how features were implemented\n\
                 - Learn from historical changes\n\
                 - Understand codebase evolution\n\
                 - Review best practices in commits\n\n\
                 Documentation:\n\
                 - Generate release notes\n\
                 - Document major changes\n\
                 - Reference specific implementations\n\
                 - Create changelogs\n\n\
                 =============================================================================\n\
                 SECTION 8: PERFORMANCE TIPS\n\
                 =============================================================================\n\n\
                 For large commits:\n\
                 1. Start with stat: true\n\
                    git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"stat\": true})\n\
                    Check size before loading full diff\n\n\
                 2. Use name_only for quick check:\n\
                    git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"name_only\": true})\n\
                    See which files changed\n\n\
                 3. Filter to specific files:\n\
                    git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"files\": [\"src/\"]})\n\
                    Only review relevant parts\n\n\
                 Speed comparison (fastest to slowest):\n\
                 - name_only: Fastest, just file names\n\
                 - name_status: File names + status codes\n\
                 - stat: Statistics without full diff\n\
                 - files: Full diff for file subset\n\
                 - (default): Complete commit with all changes\n\n\
                 =============================================================================\n\
                 SECTION 9: BEST PRACTICES\n\
                 =============================================================================\n\n\
                 General:\n\
                 - Use git_show to understand individual commits thoroughly\n\
                 - Always review commit messages for context\n\
                 - Check author/committer for attribution\n\
                 - Examine diffs to understand actual changes\n\
                 - Combine with git_log for historical exploration\n\n\
                 Commits:\n\
                 - Verify commits before pushing\n\
                 - Review recent commits regularly\n\
                 - Document important commits for team\n\
                 - Use abbreviated hashes (7 chars) for readability\n\n\
                 Tags:\n\
                 - Always check annotated tags for releases\n\
                 - Read tag messages for release notes\n\
                 - Verify tagged commit matches expectations\n\
                 - Use for deployment verification\n\n\
                 Files:\n\
                 - Use colon notation: \"commit:path\"\n\
                 - Paths relative to repository root\n\
                 - No leading slashes or dot prefixes\n\
                 - Good for file comparison across versions\n\
                 - Useful for recovery and archaeology\n\n\
                 Formatting:\n\
                 - Use stat for quick summaries\n\
                 - Use name_only when you just need file lists\n\
                 - Use files parameter to focus large commits\n\
                 - Use custom formats for automation\n\
                 - Start with stats, get full diff if needed\n\n\
                 =============================================================================\n\
                 DECISION TREE: WHAT TO SHOW?\n\
                 =============================================================================\n\n\
                 Need to review a specific commit? → Show commit\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234\"})\n\n\
                 Need to see release information? → Show tag\n\
                 git_show({\"path\": \"/project\", \"object\": \"v1.0.0\"})\n\n\
                 Need file from the past? → Show file at commit\n\
                 git_show({\"path\": \"/project\", \"object\": \"v1.0.0:README.md\"})\n\n\
                 Commit too large? → Use formatting options\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"stat\": true})\n\n\
                 Just need file list? → Use name_only\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"name_only\": true})\n\n\
                 Need specific files only? → Use files parameter\n\
                 git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"files\": [\"src/\"]})\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Show commit: git_show({\"path\": \"/project\", \"object\": \"abc1234\"})\n\
                 Show HEAD: git_show({\"path\": \"/project\", \"object\": \"HEAD\"})\n\
                 Show tag: git_show({\"path\": \"/project\", \"object\": \"v1.0.0\"})\n\
                 Show file: git_show({\"path\": \"/project\", \"object\": \"HEAD:file.rs\"})\n\
                 Stats only: git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"stat\": true})\n\
                 Names only: git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"name_only\": true})\n\
                 Specific files: git_show({\"path\": \"/project\", \"object\": \"abc1234\", \"files\": [\"src/\"]})\n\n\
                 Remember: git_show is your window into repository history. Use it to understand commits, verify releases, recover files, and explore how your codebase evolved!",
            ),
        },
    ]
}
