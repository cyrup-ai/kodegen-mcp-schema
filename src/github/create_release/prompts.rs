//! Prompt messages for github_create_release tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GithubCreateReleasePromptArgs;

/// Prompt provider for github_create_release tool
///
/// This is the ONLY way to provide prompts for github_create_release - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GithubCreateReleasePrompts;

impl PromptProvider for GithubCreateReleasePrompts {
    type PromptArgs = GithubCreateReleasePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("notes") => prompt_notes(),
            Some("options") => prompt_options(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, notes, options, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO CREATE GITHUB RELEASES
// ============================================================================

/// Basic release creation
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create a basic GitHub release using the github_create_release tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_create_release tool creates GitHub releases with tags and release notes. Here's how to use it for basic release creation:\n\n\
                 BASIC RELEASE CREATION:\n\
                 1. Simple release with tag:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"repository\",\n\
                        \"tag_name\": \"v1.0.0\",\n\
                        \"name\": \"Version 1.0.0\"\n\
                    })\n\n\
                 2. Release with description:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"repository\",\n\
                        \"tag_name\": \"v1.1.0\",\n\
                        \"name\": \"Version 1.1.0\",\n\
                        \"body\": \"This release includes bug fixes and performance improvements.\"\n\
                    })\n\n\
                 3. Release with formatted notes:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"repository\",\n\
                        \"tag_name\": \"v1.2.0\",\n\
                        \"name\": \"Version 1.2.0\",\n\
                        \"body\": \"## What's New\\n- Feature A\\n- Feature B\\n\\n## Bug Fixes\\n- Fixed issue #123\\n- Fixed issue #456\"\n\
                    })\n\n\
                 REQUIRED PARAMETERS:\n\
                 - owner: Repository owner (username or organization)\n\
                 - repo: Repository name\n\
                 - tag_name: Git tag for the release (e.g., \"v1.0.0\")\n\n\
                 OPTIONAL PARAMETERS:\n\
                 - name: Release title (defaults to tag_name if not provided)\n\
                 - body: Release notes/description (supports Markdown)\n\
                 - draft: true to create as draft (default: false)\n\
                 - prerelease: true to mark as pre-release (default: false)\n\
                 - target_commitish: Branch or commit SHA (defaults to default branch)\n\
                 - generate_release_notes: true to auto-generate notes from PRs\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"id\": 12345678,\n\
                   \"tag_name\": \"v1.0.0\",\n\
                   \"name\": \"Version 1.0.0\",\n\
                   \"url\": \"https://github.com/owner/repo/releases/tag/v1.0.0\",\n\
                   \"html_url\": \"https://github.com/owner/repo/releases/tag/v1.0.0\",\n\
                   \"draft\": false,\n\
                   \"prerelease\": false,\n\
                   \"created_at\": \"2025-12-05T10:30:00Z\",\n\
                   \"published_at\": \"2025-12-05T10:30:00Z\"\n\
                 }\n\n\
                 TAG NAMING CONVENTIONS:\n\
                 - Semantic versioning: v1.0.0, v1.2.3, v2.0.0-beta.1\n\
                 - Without 'v' prefix: 1.0.0, 2.0.0\n\
                 - Date-based: 2025.12.05, 2025-12-05\n\
                 - Custom: release-2025-q4, stable-2025\n\n\
                 IMPORTANT NOTES:\n\
                 - Tag must already exist in the repository OR will be created if target_commitish is provided\n\
                 - If tag doesn't exist and no target_commitish, the API will create tag on default branch HEAD\n\
                 - Release names should be human-readable (e.g., \"Version 1.0.0\" not just \"v1.0.0\")\n\
                 - Use Markdown formatting in body for better readability\n\
                 - Once published (not draft), releases are public and visible to all\n\n\
                 COMMON PATTERNS:\n\
                 1. Quick release:\n\
                    github_create_release({\"owner\": \"user\", \"repo\": \"app\", \"tag_name\": \"v1.0.0\"})\n\n\
                 2. Release with title:\n\
                    github_create_release({\"owner\": \"user\", \"repo\": \"app\", \"tag_name\": \"v1.0.0\", \"name\": \"First Stable Release\"})\n\n\
                 3. Release with notes:\n\
                    github_create_release({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"app\",\n\
                        \"tag_name\": \"v1.0.0\",\n\
                        \"name\": \"Version 1.0.0\",\n\
                        \"body\": \"Initial stable release with core features.\"\n\
                    })\n\n\
                 ERROR HANDLING:\n\
                 - Tag already has a release: You'll get an error. Use update instead or delete old release\n\
                 - Invalid tag name: Use valid Git tag format\n\
                 - Repository not found: Check owner/repo spelling and access permissions\n\
                 - Unauthorized: Ensure you have push access to the repository",
            ),
        },
    ]
}

/// Writing comprehensive release notes
fn prompt_notes() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How should I write comprehensive release notes when creating a GitHub release?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Release notes are critical for communicating changes to users. Here's how to write effective release notes:\n\n\
                 COMPREHENSIVE RELEASE NOTES:\n\
                 1. Full-featured release:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v2.0.0\",\n\
                        \"name\": \"Version 2.0.0 - Major Update\",\n\
                        \"body\": \"## 🚀 New Features\\n\\n\
- **Dark Mode**: Full dark theme support across all pages\\n\
- **Export to PDF**: Export documents with custom formatting\\n\
- **Real-time Collaboration**: Multi-user editing with live cursors\\n\\n\
## 🐛 Bug Fixes\\n\\n\
- Fixed crash on startup when config file missing (#234)\\n\
- Fixed memory leak in background sync process (#245)\\n\
- Resolved timezone issues in date picker (#267)\\n\\n\
## 💥 Breaking Changes\\n\\n\
- **API v1 deprecated**: All API v1 endpoints removed. Migrate to v2.\\n\
- **Config format changed**: Old config files will be auto-migrated on first run.\\n\
- **Minimum Node.js version**: Now requires Node.js 18+ (was 16+)\\n\\n\
## ⚡ Performance Improvements\\n\\n\
- 3x faster initial load time\\n\
- 50% reduction in memory usage\\n\
- Improved rendering performance for large documents\\n\\n\
## 🔧 Maintenance\\n\\n\
- Updated all dependencies to latest versions\\n\
- Improved error messages and logging\\n\
- Enhanced documentation with more examples\\n\\n\
## 📝 Upgrade Notes\\n\\n\
See [MIGRATION.md](https://github.com/username/project/blob/main/MIGRATION.md) for detailed migration guide from v1.x to v2.0.\\\"\n\
                    })\n\n\
                 2. Auto-generated release notes:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v1.5.0\",\n\
                        \"name\": \"Version 1.5.0\",\n\
                        \"generate_release_notes\": true\n\
                    })\n\
                    // GitHub automatically generates notes from merged PRs\n\n\
                 3. Combining auto-generated with custom notes:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v1.5.0\",\n\
                        \"name\": \"Version 1.5.0\",\n\
                        \"body\": \"## Highlights\\n\\n\
This release focuses on performance and stability.\\n\\n\
## 📊 Statistics\\n\\n\
- 50+ commits\\n\
- 15 contributors\\n\
- 25 issues closed\\n\\n\",\n\
                        \"generate_release_notes\": true\n\
                    })\n\
                    // Your custom content appears first, then auto-generated content\n\n\
                 RELEASE NOTE SECTIONS:\n\n\
                 Essential sections:\n\
                 - **New Features** (🚀): Major new functionality\n\
                 - **Bug Fixes** (🐛): Bugs resolved in this release\n\
                 - **Breaking Changes** (💥): Changes that break backward compatibility\n\n\
                 Optional sections:\n\
                 - **Performance Improvements** (⚡): Speed and efficiency gains\n\
                 - **Deprecations** (⚠️): Features being phased out\n\
                 - **Security** (🔒): Security fixes (be careful not to expose vulnerabilities)\n\
                 - **Documentation** (📚): Docs improvements\n\
                 - **Maintenance** (🔧): Internal improvements, dependency updates\n\
                 - **Known Issues** (⚠️): Bugs not yet fixed\n\
                 - **Upgrade Notes** (📝): Migration instructions\n\
                 - **Contributors** (👥): Acknowledge contributors\n\n\
                 MARKDOWN FORMATTING:\n\
                 - Use ## for section headers\n\
                 - Use - for bullet points\n\
                 - Use **bold** for emphasis\n\
                 - Link to issues: #123 or https://github.com/owner/repo/issues/123\n\
                 - Link to PRs: #456 or https://github.com/owner/repo/pull/456\n\
                 - Link to commits: SHA or https://github.com/owner/repo/commit/abc123\n\
                 - Code blocks: ```language\\ncode\\n```\n\
                 - Inline code: `code`\n\n\
                 BEST PRACTICES:\n\n\
                 1. **Start with highlights**: Lead with most important changes\n\
                 2. **Be specific**: \"Fixed crash on startup\" not \"Fixed bugs\"\n\
                 3. **Include issue numbers**: Link to issues/PRs for context (#123)\n\
                 4. **Explain breaking changes**: Tell users exactly what broke and how to fix\n\
                 5. **Group related changes**: Keep similar items together\n\
                 6. **Use emojis sparingly**: They help scanning but don't overuse\n\
                 7. **Link to docs**: Provide migration guides, tutorials, examples\n\
                 8. **Acknowledge contributors**: @mention or list contributors\n\
                 9. **Be user-focused**: Write for users, not developers\n\
                 10. **Keep it scannable**: Use clear headers and bullet points\n\n\
                 EXAMPLES BY RELEASE TYPE:\n\n\
                 Major release (v2.0.0):\n\
                 - Comprehensive notes with all sections\n\
                 - Detailed breaking changes section\n\
                 - Migration guide link\n\
                 - Highlight major features\n\n\
                 Minor release (v1.5.0):\n\
                 - Focus on new features\n\
                 - Bug fixes\n\
                 - No breaking changes\n\n\
                 Patch release (v1.4.1):\n\
                 - Focus on bug fixes\n\
                 - Maybe small improvements\n\
                 - Keep it brief\n\n\
                 Pre-release (v2.0.0-beta.1):\n\
                 - Note it's a beta/alpha/RC\n\
                 - Known issues section\n\
                 - How to report bugs\n\
                 - Testing instructions\n\n\
                 SECURITY FIXES:\n\
                 When including security fixes:\n\
                 - Mention CVE numbers if assigned\n\
                 - Describe impact level (Critical, High, Medium, Low)\n\
                 - Don't include exploit details\n\
                 - Link to security advisory\n\
                 - Encourage users to update immediately\n\n\
                 Example:\n\
                 \"## 🔒 Security\\n\\n\
- **Critical**: Fixed SQL injection vulnerability (CVE-2025-1234). All users should update immediately.\\n\
- See [security advisory](https://github.com/owner/repo/security/advisories/GHSA-xxxx) for details.\"\n\n\
                 AUTO-GENERATED NOTES:\n\
                 When using generate_release_notes: true:\n\
                 - GitHub extracts PR titles between this and previous release\n\
                 - Groups by PR labels (feature, bug, documentation, etc.)\n\
                 - Lists new contributors\n\
                 - Includes \"Full Changelog\" link\n\
                 - You can prepend custom content by providing body parameter\n\n\
                 KEEP A CHANGELOG:\n\
                 Consider maintaining a CHANGELOG.md file:\n\
                 - Follows keepachangelog.com format\n\
                 - Copy sections to release notes\n\
                 - Provides version history in repository\n\
                 - Easier to review changes over time",
            ),
        },
    ]
}

/// Release options - drafts, pre-releases, targeting specific commits
fn prompt_options() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are the different release options available when creating a GitHub release?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "GitHub releases support various options for different release scenarios. Here's how to use them:\n\n\
                 DRAFT RELEASES:\n\
                 1. Create draft release:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v1.0.0\",\n\
                        \"name\": \"Version 1.0.0\",\n\
                        \"body\": \"Release notes here...\",\n\
                        \"draft\": true\n\
                    })\n\
                    // Not visible to public until published\n\n\
                 2. Draft use cases:\n\
                    - Prepare release notes before publishing\n\
                    - Review with team before making public\n\
                    - Wait for CI/CD to build release assets\n\
                    - Stage release for scheduled announcement\n\n\
                 3. Draft behavior:\n\
                    - Only visible to users with push access\n\
                    - Not shown in releases list for public\n\
                    - Can be edited freely\n\
                    - Tag is created but release is hidden\n\
                    - Must be published separately (via GitHub UI or API update)\n\n\
                 PRE-RELEASES:\n\
                 1. Create pre-release:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v2.0.0-beta.1\",\n\
                        \"name\": \"Version 2.0.0 Beta 1\",\n\
                        \"body\": \"## Beta Release\\n\\nThis is a pre-release for testing. Please report issues!\",\n\
                        \"prerelease\": true\n\
                    })\n\n\
                 2. Pre-release use cases:\n\
                    - Alpha releases for early testing\n\
                    - Beta releases for wider testing\n\
                    - Release candidates (RC) before stable\n\
                    - Nightly/canary builds\n\
                    - Experimental features\n\n\
                 3. Pre-release behavior:\n\
                    - Publicly visible but marked as pre-release\n\
                    - Not shown as \"Latest\" release\n\
                    - Users can opt-in to pre-release channel\n\
                    - Clearly labeled with yellow badge on GitHub\n\
                    - Package managers may skip unless explicitly requested\n\n\
                 4. Pre-release tag conventions:\n\
                    - Alpha: v1.0.0-alpha.1, v1.0.0-alpha.2\n\
                    - Beta: v1.0.0-beta.1, v1.0.0-beta.2\n\
                    - RC: v1.0.0-rc.1, v1.0.0-rc.2\n\
                    - Nightly: v1.0.0-nightly.20251205\n\n\
                 TARGETING SPECIFIC COMMITS:\n\
                 1. Release from specific commit:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v1.0.0\",\n\
                        \"name\": \"Version 1.0.0\",\n\
                        \"target_commitish\": \"abc123def456\"\n\
                    })\n\
                    // Creates release (and tag if it doesn't exist) at specific commit\n\n\
                 2. Release from branch:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v1.0.0\",\n\
                        \"name\": \"Version 1.0.0\",\n\
                        \"target_commitish\": \"release-1.0\"\n\
                    })\n\
                    // Uses HEAD of specified branch\n\n\
                 3. When to specify target_commitish:\n\
                    - Releasing from non-default branch\n\
                    - Hotfix release from specific commit\n\
                    - Tag doesn't exist yet\n\
                    - Creating historical release\n\n\
                 COMBINING OPTIONS:\n\
                 1. Draft pre-release:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v2.0.0-beta.1\",\n\
                        \"name\": \"Version 2.0.0 Beta 1\",\n\
                        \"draft\": true,\n\
                        \"prerelease\": true,\n\
                        \"target_commitish\": \"develop\"\n\
                    })\n\
                    // Prepare beta release before publishing\n\n\
                 2. Pre-release from feature branch:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v2.0.0-alpha.1\",\n\
                        \"name\": \"Version 2.0.0 Alpha 1\",\n\
                        \"prerelease\": true,\n\
                        \"target_commitish\": \"feature/new-ui\"\n\
                    })\n\n\
                 LATEST RELEASE BEHAVIOR:\n\
                 - By default, most recent non-prerelease becomes \"Latest\"\n\
                 - Pre-releases don't become \"Latest\" automatically\n\
                 - Draft releases are not considered for \"Latest\"\n\
                 - You can manually set \"Latest\" via GitHub UI\n\n\
                 Example sequence:\n\
                 1. v1.0.0 (stable) - Latest ✓\n\
                 2. v1.1.0-beta.1 (prerelease) - Latest stays v1.0.0\n\
                 3. v1.1.0 (stable) - Latest becomes v1.1.0\n\n\
                 RELEASE LIFECYCLE:\n\
                 Draft → Pre-release → Stable:\n\
                 1. Create draft: draft: true\n\
                 2. Publish as pre-release: Edit to remove draft, keep prerelease: true\n\
                 3. Promote to stable: Edit to remove prerelease: false\n\n\
                 DECISION TREE:\n\n\
                 Need to prepare privately? → draft: true\n\
                 - Team review\n\
                 - Wait for assets\n\
                 - Schedule announcement\n\n\
                 Testing/experimental? → prerelease: true\n\
                 - Alpha/Beta/RC\n\
                 - Not production ready\n\
                 - Early adopters only\n\n\
                 Stable production release? → draft: false, prerelease: false (defaults)\n\
                 - Main release\n\
                 - Production ready\n\
                 - All users\n\n\
                 Specific commit/branch? → target_commitish: \"...\" \n\
                 - Hotfix from old version\n\
                 - Release branch\n\
                 - Tag doesn't exist\n\n\
                 COMMON WORKFLOWS:\n\n\
                 1. Standard stable release:\n\
                    github_create_release({\"owner\": \"user\", \"repo\": \"app\", \"tag_name\": \"v1.0.0\", \"name\": \"Version 1.0.0\"})\n\n\
                 2. Beta testing:\n\
                    github_create_release({\"owner\": \"user\", \"repo\": \"app\", \"tag_name\": \"v2.0.0-beta.1\", \"prerelease\": true})\n\n\
                 3. Prepare then publish:\n\
                    github_create_release({\"owner\": \"user\", \"repo\": \"app\", \"tag_name\": \"v1.0.0\", \"draft\": true})\n\
                    // Review, add assets, then publish via GitHub UI or API update\n\n\
                 4. Hotfix from specific commit:\n\
                    github_create_release({\"owner\": \"user\", \"repo\": \"app\", \"tag_name\": \"v1.0.1\", \"target_commitish\": \"abc123\"})\n\n\
                 TIPS:\n\
                 - Use draft for internal preparation\n\
                 - Use prerelease for testing versions\n\
                 - Omit both for stable releases\n\
                 - Specify target_commitish only when needed\n\
                 - Follow semantic versioning for tag names\n\
                 - Pre-release tags should include alpha/beta/rc suffix",
            ),
        },
    ]
}

/// Complete release workflows including git operations
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are the complete workflows for creating GitHub releases?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Creating releases involves multiple steps beyond just the API call. Here are complete workflows:\n\n\
                 WORKFLOW 1: STANDARD RELEASE FROM MAIN\n\
                 1. Ensure you're on main branch with latest:\n\
                    terminal({\"command\": \"git checkout main && git pull origin main\"})\n\n\
                 2. Create and push tag:\n\
                    terminal({\"command\": \"git tag -a v1.0.0 -m 'Release 1.0.0' && git push origin v1.0.0\"})\n\n\
                 3. Create release:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v1.0.0\",\n\
                        \"name\": \"Version 1.0.0\",\n\
                        \"body\": \"## Changes\\n- Feature A\\n- Bug fix B\"\n\
                    })\n\n\
                 WORKFLOW 2: RELEASE WITH VERSION BUMP\n\
                 1. Update version in files:\n\
                    fs_edit_block({\n\
                        \"path\": \"/project/package.json\",\n\
                        \"old_string\": \"\\\"version\\\": \\\"0.9.0\\\"\",\n\
                        \"new_string\": \"\\\"version\\\": \\\"1.0.0\\\"\"\n\
                    })\n\n\
                 2. Commit version bump:\n\
                    terminal({\"command\": \"git add . && git commit -m 'Bump version to 1.0.0' && git push\"})\n\n\
                 3. Create tag and release:\n\
                    terminal({\"command\": \"git tag -a v1.0.0 -m 'Release 1.0.0' && git push origin v1.0.0\"})\n\
                    github_create_release({\"owner\": \"user\", \"repo\": \"project\", \"tag_name\": \"v1.0.0\"})\n\n\
                 WORKFLOW 3: DRAFT, REVIEW, PUBLISH\n\
                 1. Create tag:\n\
                    terminal({\"command\": \"git tag -a v1.0.0 -m 'Release 1.0.0' && git push origin v1.0.0\"})\n\n\
                 2. Create draft release:\n\
                    github_create_release({\n\
                        \"owner\": \"username\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v1.0.0\",\n\
                        \"name\": \"Version 1.0.0\",\n\
                        \"body\": \"## Changes\\n- Feature A\\n- Bug fix B\",\n\
                        \"draft\": true\n\
                    })\n\n\
                 3. Review on GitHub, add assets if needed\n\n\
                 4. Publish via GitHub UI or update API call\n\n\
                 WORKFLOW 4: RELEASE FROM RELEASE BRANCH\n\
                 1. Create release branch:\n\
                    terminal({\"command\": \"git checkout -b release-1.0 && git push -u origin release-1.0\"})\n\n\
                 2. Make final changes on release branch:\n\
                    fs_edit_block({...})\n\
                    terminal({\"command\": \"git add . && git commit -m 'Final changes for 1.0' && git push\"})\n\n\
                 3. Tag and release from release branch:\n\
                    terminal({\"command\": \"git tag -a v1.0.0 -m 'Release 1.0.0' && git push origin v1.0.0\"})\n\
                    github_create_release({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v1.0.0\",\n\
                        \"target_commitish\": \"release-1.0\"\n\
                    })\n\n\
                 4. Merge release branch back to main:\n\
                    terminal({\"command\": \"git checkout main && git merge release-1.0 && git push\"})\n\n\
                 WORKFLOW 5: HOTFIX RELEASE\n\
                 1. Create hotfix branch from tag:\n\
                    terminal({\"command\": \"git checkout -b hotfix-1.0.1 v1.0.0\"})\n\n\
                 2. Make hotfix:\n\
                    fs_edit_block({...})\n\
                    terminal({\"command\": \"git add . && git commit -m 'Hotfix: critical bug' && git push -u origin hotfix-1.0.1\"})\n\n\
                 3. Tag and release hotfix:\n\
                    terminal({\"command\": \"git tag -a v1.0.1 -m 'Hotfix 1.0.1' && git push origin v1.0.1\"})\n\
                    github_create_release({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v1.0.1\",\n\
                        \"name\": \"Version 1.0.1 - Hotfix\",\n\
                        \"body\": \"## Critical Fix\\n- Fixed security vulnerability\\n\\nAll users should update immediately.\"\n\
                    })\n\n\
                 4. Merge hotfix to main and develop:\n\
                    terminal({\"command\": \"git checkout main && git merge hotfix-1.0.1 && git push\"})\n\
                    terminal({\"command\": \"git checkout develop && git merge hotfix-1.0.1 && git push\"})\n\n\
                 WORKFLOW 6: PRE-RELEASE (BETA) WORKFLOW\n\
                 1. Create beta from develop:\n\
                    terminal({\"command\": \"git checkout develop && git pull\"})\n\n\
                 2. Tag beta:\n\
                    terminal({\"command\": \"git tag -a v2.0.0-beta.1 -m 'Beta 1' && git push origin v2.0.0-beta.1\"})\n\n\
                 3. Create pre-release:\n\
                    github_create_release({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v2.0.0-beta.1\",\n\
                        \"name\": \"Version 2.0.0 Beta 1\",\n\
                        \"body\": \"## Beta Release\\n\\nPlease test and report issues!\\n\\n## New Features\\n- Feature X\\n- Feature Y\",\n\
                        \"prerelease\": true\n\
                    })\n\n\
                 4. After testing, create final release:\n\
                    terminal({\"command\": \"git checkout main && git merge develop && git push\"})\n\
                    terminal({\"command\": \"git tag -a v2.0.0 -m 'Release 2.0.0' && git push origin v2.0.0\"})\n\
                    github_create_release({\"owner\": \"user\", \"repo\": \"project\", \"tag_name\": \"v2.0.0\"})\n\n\
                 WORKFLOW 7: AUTO-GENERATED RELEASE NOTES\n\
                 1. Ensure PRs have good titles and labels\n\n\
                 2. Create tag:\n\
                    terminal({\"command\": \"git tag -a v1.0.0 -m 'Release 1.0.0' && git push origin v1.0.0\"})\n\n\
                 3. Create release with auto-generated notes:\n\
                    github_create_release({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"tag_name\": \"v1.0.0\",\n\
                        \"name\": \"Version 1.0.0\",\n\
                        \"generate_release_notes\": true\n\
                    })\n\
                    // GitHub pulls PR titles and groups by labels\n\n\
                 WORKFLOW 8: COORDINATED RELEASE (MULTIPLE REPOS)\n\
                 1. Release each component:\n\
                    // Backend\n\
                    terminal({\"terminal\": 0, \"command\": \"cd /backend && git tag v1.0.0 && git push origin v1.0.0\"})\n\
                    github_create_release({\"owner\": \"user\", \"repo\": \"backend\", \"tag_name\": \"v1.0.0\"})\n\n\
                    // Frontend\n\
                    terminal({\"terminal\": 1, \"command\": \"cd /frontend && git tag v1.0.0 && git push origin v1.0.0\"})\n\
                    github_create_release({\"owner\": \"user\", \"repo\": \"frontend\", \"tag_name\": \"v1.0.0\"})\n\n\
                 2. Create umbrella release in main repo:\n\
                    github_create_release({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"platform\",\n\
                        \"tag_name\": \"v1.0.0\",\n\
                        \"body\": \"## Platform Release 1.0.0\\n\\nComponents:\\n- Backend: v1.0.0\\n- Frontend: v1.0.0\"\n\
                    })\n\n\
                 BEST PRACTICES:\n\n\
                 1. **Semantic Versioning**:\n\
                    - Major (v2.0.0): Breaking changes\n\
                    - Minor (v1.5.0): New features, backward compatible\n\
                    - Patch (v1.0.1): Bug fixes\n\n\
                 2. **Tag Management**:\n\
                    - Create annotated tags: git tag -a (not lightweight)\n\
                    - Push tags explicitly: git push origin v1.0.0\n\
                    - Use consistent naming: always use 'v' prefix or never use it\n\n\
                 3. **Release Timing**:\n\
                    - Release during business hours (for support)\n\
                    - Avoid Fridays (weekend issues)\n\
                    - Coordinate with team\n\
                    - Monitor after release\n\n\
                 4. **Testing Before Release**:\n\
                    - Run full test suite\n\
                    - Build and test artifacts\n\
                    - Check breaking changes\n\
                    - Review changelog\n\n\
                 5. **Post-Release**:\n\
                    - Announce on social media, blog, etc.\n\
                    - Update documentation\n\
                    - Monitor issues and discussions\n\
                    - Be ready to hotfix if needed\n\n\
                 6. **Version Files**:\n\
                    Update version in:\n\
                    - package.json (Node.js)\n\
                    - Cargo.toml (Rust)\n\
                    - setup.py / pyproject.toml (Python)\n\
                    - pom.xml (Java)\n\
                    - Any version constants in code\n\n\
                 7. **Changelog**:\n\
                    - Maintain CHANGELOG.md\n\
                    - Follow keepachangelog.com format\n\
                    - Update before each release\n\
                    - Link from release notes\n\n\
                 COMMON MISTAKES TO AVOID:\n\
                 - Releasing without testing\n\
                 - Forgetting to update version numbers\n\
                 - Missing breaking change documentation\n\
                 - Poor or missing release notes\n\
                 - Not coordinating with team\n\
                 - Releasing too frequently (noise) or too rarely (risk)\n\
                 - Forgetting to merge release branch back",
            ),
        },
    ]
}

/// Comprehensive guide covering all aspects of release creation
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to creating GitHub releases effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "GitHub releases are distribution points for your software. Here's everything you need to know:\n\n\
                 =============================================================================\n\
                 WHAT ARE GITHUB RELEASES?\n\
                 =============================================================================\n\n\
                 Releases are:\n\
                 - Git tags with additional metadata\n\
                 - Distribution points for software\n\
                 - Change documentation for versions\n\
                 - Asset hosting (binaries, archives)\n\
                 - Communication tool for users\n\n\
                 Benefits:\n\
                 - Semantic versioning support\n\
                 - Automatic changelog generation\n\
                 - Binary/asset distribution\n\
                 - User notifications\n\
                 - Release notes in repository\n\
                 - Integration with package managers\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 Minimal release:\n\
                 github_create_release({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"repository\",\n\
                     \"tag_name\": \"v1.0.0\"\n\
                 })\n\n\
                 Standard release:\n\
                 github_create_release({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"repository\",\n\
                     \"tag_name\": \"v1.0.0\",\n\
                     \"name\": \"Version 1.0.0\",\n\
                     \"body\": \"Release notes with changes...\"\n\
                 })\n\n\
                 Full-featured release:\n\
                 github_create_release({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"repository\",\n\
                     \"tag_name\": \"v1.0.0\",\n\
                     \"name\": \"Version 1.0.0 - Stable Release\",\n\
                     \"body\": \"## Changes\\n- Feature A\\n- Bug fix B\",\n\
                     \"draft\": false,\n\
                     \"prerelease\": false,\n\
                     \"target_commitish\": \"main\",\n\
                     \"generate_release_notes\": false\n\
                 })\n\n\
                 =============================================================================\n\
                 PARAMETERS REFERENCE\n\
                 =============================================================================\n\n\
                 **Required Parameters**:\n\
                 - owner: Repository owner (string)\n\
                 - repo: Repository name (string)\n\
                 - tag_name: Git tag for release (string)\n\n\
                 **Optional Parameters**:\n\
                 - name: Release title (string, defaults to tag_name)\n\
                 - body: Release notes in Markdown (string)\n\
                 - draft: Create as draft (boolean, default: false)\n\
                 - prerelease: Mark as pre-release (boolean, default: false)\n\
                 - target_commitish: Branch/commit SHA (string, defaults to default branch)\n\
                 - generate_release_notes: Auto-generate notes (boolean, default: false)\n\n\
                 =============================================================================\n\
                 RELEASE TYPES\n\
                 =============================================================================\n\n\
                 1. **Stable Release** (Production):\n\
                    github_create_release({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"app\",\n\
                        \"tag_name\": \"v1.0.0\",\n\
                        \"name\": \"Version 1.0.0\"\n\
                    })\n\
                    - Production ready\n\
                    - Becomes \"Latest\" release\n\
                    - For all users\n\n\
                 2. **Pre-release** (Beta/Alpha/RC):\n\
                    github_create_release({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"app\",\n\
                        \"tag_name\": \"v2.0.0-beta.1\",\n\
                        \"name\": \"Version 2.0.0 Beta 1\",\n\
                        \"prerelease\": true\n\
                    })\n\
                    - Testing versions\n\
                    - Not marked as \"Latest\"\n\
                    - Early adopters only\n\n\
                 3. **Draft Release** (Internal):\n\
                    github_create_release({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"app\",\n\
                        \"tag_name\": \"v1.0.0\",\n\
                        \"name\": \"Version 1.0.0\",\n\
                        \"draft\": true\n\
                    })\n\
                    - Not visible publicly\n\
                    - Team review\n\
                    - Prepare before publishing\n\n\
                 =============================================================================\n\
                 VERSIONING SCHEMES\n\
                 =============================================================================\n\n\
                 **Semantic Versioning** (Recommended):\n\
                 - Format: MAJOR.MINOR.PATCH (e.g., 1.2.3)\n\
                 - Major: Breaking changes (v2.0.0)\n\
                 - Minor: New features, compatible (v1.5.0)\n\
                 - Patch: Bug fixes (v1.0.1)\n\
                 - Pre-release: v2.0.0-alpha.1, v2.0.0-beta.1, v2.0.0-rc.1\n\n\
                 Examples:\n\
                 - v1.0.0 → v1.0.1 (patch)\n\
                 - v1.0.0 → v1.1.0 (minor)\n\
                 - v1.0.0 → v2.0.0 (major)\n\
                 - v2.0.0-beta.1 → v2.0.0-beta.2 → v2.0.0-rc.1 → v2.0.0\n\n\
                 **Calendar Versioning** (CalVer):\n\
                 - Format: YYYY.MM.DD or YYYY.MM.MICRO\n\
                 - Examples: 2025.12.05, 2025.12.1\n\
                 - Good for: Frequent releases, date-driven releases\n\n\
                 **Custom Versioning**:\n\
                 - release-2025-q4\n\
                 - stable-2025\n\
                 - sprint-52\n\n\
                 =============================================================================\n\
                 RELEASE NOTES BEST PRACTICES\n\
                 =============================================================================\n\n\
                 Structure:\n\
                 ## 🚀 New Features\n\
                 - Feature description with benefits\n\
                 - Another feature (#123)\n\n\
                 ## 🐛 Bug Fixes\n\
                 - Fixed specific bug (#456)\n\
                 - Resolved another issue\n\n\
                 ## 💥 Breaking Changes\n\
                 - What changed and why\n\
                 - Migration instructions\n\n\
                 ## 📝 Other Changes\n\
                 - Performance improvements\n\
                 - Documentation updates\n\n\
                 Tips:\n\
                 - Write for users, not developers\n\
                 - Link to issues/PRs with #number\n\
                 - Explain breaking changes clearly\n\
                 - Include migration guides\n\
                 - Use Markdown formatting\n\
                 - Keep it scannable\n\n\
                 =============================================================================\n\
                 AUTO-GENERATED RELEASE NOTES\n\
                 =============================================================================\n\n\
                 Enable auto-generation:\n\
                 github_create_release({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"app\",\n\
                     \"tag_name\": \"v1.0.0\",\n\
                     \"generate_release_notes\": true\n\
                 })\n\n\
                 What it does:\n\
                 - Extracts merged PR titles since last release\n\
                 - Groups by PR labels (feature, bug, documentation)\n\
                 - Lists new contributors\n\
                 - Generates \"Full Changelog\" link\n\n\
                 Combine with custom notes:\n\
                 github_create_release({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"app\",\n\
                     \"tag_name\": \"v1.0.0\",\n\
                     \"body\": \"## Highlights\\nMajor performance improvements!\\n\\n\",\n\
                     \"generate_release_notes\": true\n\
                 })\n\
                 // Your content appears first, then auto-generated\n\n\
                 =============================================================================\n\
                 COMPLETE WORKFLOW EXAMPLE\n\
                 =============================================================================\n\n\
                 Step 1 - Prepare:\n\
                 terminal({\"command\": \"git checkout main && git pull origin main\"})\n\n\
                 Step 2 - Update version files:\n\
                 fs_edit_block({\n\
                     \"path\": \"/project/package.json\",\n\
                     \"old_string\": \"\\\"version\\\": \\\"0.9.0\\\"\",\n\
                     \"new_string\": \"\\\"version\\\": \\\"1.0.0\\\"\"\n\
                 })\n\n\
                 Step 3 - Commit version bump:\n\
                 terminal({\"command\": \"git add . && git commit -m 'Bump version to 1.0.0' && git push\"})\n\n\
                 Step 4 - Create and push tag:\n\
                 terminal({\"command\": \"git tag -a v1.0.0 -m 'Release 1.0.0' && git push origin v1.0.0\"})\n\n\
                 Step 5 - Create release:\n\
                 github_create_release({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"project\",\n\
                     \"tag_name\": \"v1.0.0\",\n\
                     \"name\": \"Version 1.0.0 - First Stable Release\",\n\
                     \"body\": \"## 🎉 First Stable Release\\n\\nWe're excited to announce the first stable release!\\n\\n## 🚀 Features\\n- Core functionality\\n- User authentication\\n- Data persistence\\n\\n## 📝 Documentation\\nSee docs at https://docs.example.com\"\n\
                 })\n\n\
                 Step 6 - Verify:\n\
                 - Check release appears on GitHub\n\
                 - Verify tag is correct\n\
                 - Test release artifacts\n\
                 - Monitor for issues\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 **Hotfix Release**:\n\
                 1. Branch from tag: git checkout -b hotfix-1.0.1 v1.0.0\n\
                 2. Fix bug and commit\n\
                 3. Tag: git tag v1.0.1\n\
                 4. Create release with hotfix notes\n\
                 5. Merge back to main and develop\n\n\
                 **Pre-release to Stable**:\n\
                 1. Create beta: tag v2.0.0-beta.1, prerelease: true\n\
                 2. Test and iterate: v2.0.0-beta.2, v2.0.0-rc.1\n\
                 3. Final release: v2.0.0 without prerelease flag\n\n\
                 **Draft Review Workflow**:\n\
                 1. Create draft release\n\
                 2. Add release notes\n\
                 3. Upload assets (if any)\n\
                 4. Review with team\n\
                 5. Publish via GitHub UI\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. **Versioning**:\n\
                    - Follow semantic versioning\n\
                    - Be consistent with version format\n\
                    - Tag format: always use 'v' prefix or never\n\n\
                 2. **Timing**:\n\
                    - Release during business hours\n\
                    - Avoid Fridays (weekend support)\n\
                    - Coordinate with team\n\n\
                 3. **Testing**:\n\
                    - Run full test suite\n\
                    - Test in staging environment\n\
                    - Verify breaking changes\n\n\
                 4. **Documentation**:\n\
                    - Write clear release notes\n\
                    - Document breaking changes\n\
                    - Provide migration guides\n\
                    - Link to relevant docs\n\n\
                 5. **Communication**:\n\
                    - Announce on appropriate channels\n\
                    - Update documentation\n\
                    - Monitor issues after release\n\n\
                 6. **Tags**:\n\
                    - Use annotated tags: git tag -a\n\
                    - Write meaningful tag messages\n\
                    - Push tags explicitly\n\n\
                 =============================================================================\n\
                 TROUBLESHOOTING\n\
                 =============================================================================\n\n\
                 **Tag already has release**:\n\
                 - Can't create duplicate release for same tag\n\
                 - Delete old release first or use different tag\n\n\
                 **Tag doesn't exist**:\n\
                 - Provide target_commitish to create tag automatically\n\
                 - Or create tag manually first\n\n\
                 **Permission denied**:\n\
                 - Need push access to repository\n\
                 - Check authentication token\n\n\
                 **Release not showing as latest**:\n\
                 - Pre-releases don't become \"Latest\"\n\
                 - Draft releases are hidden\n\
                 - Check release settings\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Simple release:\n\
                 github_create_release({\"owner\": \"user\", \"repo\": \"app\", \"tag_name\": \"v1.0.0\"})\n\n\
                 With notes:\n\
                 github_create_release({\"owner\": \"user\", \"repo\": \"app\", \"tag_name\": \"v1.0.0\", \"body\": \"Changes...\"})\n\n\
                 Draft:\n\
                 github_create_release({\"owner\": \"user\", \"repo\": \"app\", \"tag_name\": \"v1.0.0\", \"draft\": true})\n\n\
                 Pre-release:\n\
                 github_create_release({\"owner\": \"user\", \"repo\": \"app\", \"tag_name\": \"v2.0.0-beta.1\", \"prerelease\": true})\n\n\
                 Auto-generated notes:\n\
                 github_create_release({\"owner\": \"user\", \"repo\": \"app\", \"tag_name\": \"v1.0.0\", \"generate_release_notes\": true})\n\n\
                 From specific commit:\n\
                 github_create_release({\"owner\": \"user\", \"repo\": \"app\", \"tag_name\": \"v1.0.0\", \"target_commitish\": \"abc123\"})",
            ),
        },
    ]
}
