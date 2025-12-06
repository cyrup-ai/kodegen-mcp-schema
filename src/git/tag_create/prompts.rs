//! Prompt messages for git_tag_create tool

use crate::tool::{PromptProvider, SealedPromptProvider};
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitTagCreatePromptArgs;

/// Prompt provider for git_tag_create tool
///
/// This is the ONLY way to provide prompts for git_tag_create - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct TagCreatePrompts;

// Implement the sealed trait to allow PromptProvider implementation
impl SealedPromptProvider for TagCreatePrompts {}

impl PromptProvider for TagCreatePrompts {
    type PromptArgs = GitTagCreatePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("annotated") => prompt_annotated(),
            Some("lightweight") => prompt_lightweight(),
            Some("versioning") => prompt_versioning(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (annotated, lightweight, versioning, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO CREATE GIT TAGS
// ============================================================================

/// Annotated tags with detailed messages and metadata
fn prompt_annotated() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create annotated tags in Git?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Annotated tags are full objects in Git that store the tagger's name, email, date, and can include a message. They're recommended for all release tags.\n\n\
                 CREATING ANNOTATED TAGS:\n\
                 1. Basic annotated tag:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.0.0\",\n\
                        \"message\": \"Release version 1.0.0\"\n\
                    })\n\n\
                 2. Tag with detailed message:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v2.0.0\",\n\
                        \"message\": \"Release 2.0.0\\n\\nMajor Changes:\\n- New REST API\\n- Breaking changes to config format\\n- 50% performance improvement\\n\\nBug Fixes:\\n- Fixed memory leak in parser\\n- Corrected timezone handling\"\n\
                    })\n\n\
                 3. Tag specific commit:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.5.0\",\n\
                        \"target\": \"abc1234\",\n\
                        \"message\": \"Backport release 1.5.0\"\n\
                    })\n\n\
                 4. Force overwrite existing tag:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.0.0\",\n\
                        \"message\": \"Release 1.0.0 (corrected)\",\n\
                        \"force\": true\n\
                    })\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"operation\": \"create\",\n\
                   \"name\": \"v1.0.0\",\n\
                   \"is_annotated\": true,\n\
                   \"target_commit\": \"abc123456789\",\n\
                   \"message\": \"Release version 1.0.0\"\n\
                 }\n\n\
                 ANNOTATED TAG ADVANTAGES:\n\
                 - Stores tagger name, email, and timestamp\n\
                 - Contains a complete message describing the tag\n\
                 - Can be GPG-signed for verification\n\
                 - Full Git objects (have their own SHA-1)\n\
                 - Recommended for all public releases\n\
                 - Preserved when pushed to remote repositories\n\
                 - Can be verified and audited\n\n\
                 MESSAGE FORMATTING:\n\
                 - First line: Brief summary (like commit message)\n\
                 - Blank line separator\n\
                 - Detailed description with markdown-style formatting\n\
                 - Include: features, breaking changes, bug fixes\n\
                 - Use \\n for line breaks in JSON\n\n\
                 WHEN TO USE ANNOTATED TAGS:\n\
                 - Production releases (v1.0.0, v2.1.3)\n\
                 - Beta/RC releases (v2.0.0-beta.1)\n\
                 - Major milestones\n\
                 - Any tag that needs documentation\n\
                 - Tags that will be shared publicly\n\
                 - When you need traceability (who tagged when)\n\n\
                 TARGET COMMIT:\n\
                 - If omitted: tags current HEAD\n\
                 - Can specify: full SHA, short SHA, branch name, or HEAD~N\n\
                 - Examples: \"abc1234\", \"main\", \"HEAD~5\"\n\
                 - Useful for tagging historical commits\n\n\
                 FORCE FLAG:\n\
                 - Use with caution - overwrites existing tag\n\
                 - Required if tag name already exists\n\
                 - Can cause confusion for other developers\n\
                 - Better to use new tag name if possible\n\
                 - Common use: fixing incorrect release tag immediately after creation\n\n\
                 BEST PRACTICES:\n\
                 - Always use annotated tags for releases\n\
                 - Write clear, descriptive messages\n\
                 - Follow semantic versioning in tag names\n\
                 - Include changelog summary in message\n\
                 - Tag from clean working directory\n\
                 - Push tags explicitly after creation: git_push({\"path\": \"/project\", \"tags\": true})\n\
                 - Don't reuse tag names (use force sparingly)\n\
                 - Document breaking changes prominently",
            ),
        },
    ]
}

/// Lightweight tags for temporary markers
fn prompt_lightweight() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "When should I use lightweight tags versus annotated tags?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Lightweight tags are simple pointers to commits without extra metadata. They're useful for temporary markers and personal bookmarks.\n\n\
                 CREATING LIGHTWEIGHT TAGS:\n\
                 1. Simple lightweight tag:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"checkpoint-before-refactor\"\n\
                    })\n\
                    // No message = lightweight tag\n\n\
                 2. Tag for quick reference:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"working-state\"\n\
                    })\n\n\
                 3. Temporary marker:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"before-merge\",\n\
                        \"target\": \"feature-branch\"\n\
                    })\n\n\
                 4. Personal bookmark:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"tested-good\"\n\
                    })\n\n\
                 LIGHTWEIGHT TAG CHARACTERISTICS:\n\
                 - Just a pointer to a commit (like a branch that doesn't move)\n\
                 - No tagger name, email, or date stored\n\
                 - No message or description\n\
                 - Not a full Git object\n\
                 - Minimal storage footprint\n\
                 - Fast to create and delete\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"operation\": \"create\",\n\
                   \"name\": \"checkpoint-before-refactor\",\n\
                   \"is_annotated\": false,\n\
                   \"target_commit\": \"abc123456789\"\n\
                 }\n\n\
                 WHEN TO USE LIGHTWEIGHT TAGS:\n\
                 - Temporary markers during development\n\
                 - Personal bookmarks (not shared with team)\n\
                 - Quick reference points for testing\n\
                 - Marking specific states for comparison\n\
                 - Internal checkpoints\n\
                 - When you'll delete the tag soon\n\
                 - Local-only tags that won't be pushed\n\n\
                 WHEN NOT TO USE LIGHTWEIGHT TAGS:\n\
                 - Public releases - always use annotated\n\
                 - Shared team tags - others need context\n\
                 - Long-term markers - need documentation\n\
                 - Production deployments - need audit trail\n\
                 - Tags that will be pushed to remote\n\n\
                 COMPARISON: LIGHTWEIGHT vs ANNOTATED:\n\n\
                 Lightweight:\n\
                 + Fast and simple\n\
                 + Minimal storage\n\
                 + Good for temporary use\n\
                 - No metadata (who, when, why)\n\
                 - No message or description\n\
                 - Cannot be GPG-signed\n\
                 - Less suitable for collaboration\n\n\
                 Annotated:\n\
                 + Full Git object with SHA-1\n\
                 + Stores tagger, date, message\n\
                 + Can be GPG-signed\n\
                 + Better for releases and shared tags\n\
                 + Provides complete audit trail\n\
                 - Slightly more storage\n\
                 - Requires message (more typing)\n\n\
                 CREATING DECISION TREE:\n\
                 Is this for a release? → Use ANNOTATED\n\
                 Will others need context? → Use ANNOTATED\n\
                 Need to know who/when? → Use ANNOTATED\n\
                 Pushing to remote? → Use ANNOTATED\n\
                 Just a personal bookmark? → Use LIGHTWEIGHT\n\
                 Temporary marker? → Use LIGHTWEIGHT\n\
                 Will delete soon? → Use LIGHTWEIGHT\n\n\
                 COMMON LIGHTWEIGHT TAG PATTERNS:\n\
                 1. Before risky operation:\n\
                    git_tag_create({\"path\": \"/project\", \"name\": \"before-rebase\"})\n\
                 2. Known good state:\n\
                    git_tag_create({\"path\": \"/project\", \"name\": \"tests-pass\"})\n\
                 3. Experiment baseline:\n\
                    git_tag_create({\"path\": \"/project\", \"name\": \"baseline\"})\n\
                 4. Quick reference:\n\
                    git_tag_create({\"path\": \"/project\", \"name\": \"demo-ready\"})\n\n\
                 CLEANUP:\n\
                 Lightweight tags are easy to delete when no longer needed:\n\
                 git_tag({\"path\": \"/project\", \"operation\": \"delete\", \"name\": \"checkpoint-before-refactor\"})\n\n\
                 BEST PRACTICES:\n\
                 - Use descriptive names even for temporary tags\n\
                 - Clean up old lightweight tags regularly\n\
                 - Don't push lightweight tags to shared repositories\n\
                 - Prefix with your initials for personal tags (e.g., \"jd-checkpoint\")\n\
                 - Convert to annotated if tag becomes important\n\
                 - Document your lightweight tag naming convention",
            ),
        },
    ]
}

/// Semantic versioning patterns for releases
fn prompt_versioning() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I properly version my releases using semantic versioning with Git tags?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Semantic Versioning (SemVer) uses MAJOR.MINOR.PATCH format to communicate the nature of changes. Git tags are the perfect way to mark these versions.\n\n\
                 SEMANTIC VERSIONING FORMAT:\n\
                 vMAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]\n\n\
                 MAJOR: Breaking changes (incompatible API changes)\n\
                 MINOR: New features (backward-compatible)\n\
                 PATCH: Bug fixes (backward-compatible)\n\
                 PRERELEASE: alpha, beta, rc.1 (optional)\n\
                 BUILD: Build metadata (optional)\n\n\
                 MAJOR VERSION (Breaking Changes):\n\
                 1. Increment when making incompatible API changes:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v2.0.0\",\n\
                        \"message\": \"Major release 2.0.0\\n\\nBREAKING CHANGES:\\n- Removed deprecated API endpoints\\n- Changed config file format\\n- Renamed core functions\\n\\nMigration Guide:\\n- Update config from TOML to YAML\\n- Replace old_function() with new_function()\\n- See MIGRATION.md for details\"\n\
                    })\n\n\
                 2. First stable release:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.0.0\",\n\
                        \"message\": \"First stable release 1.0.0\\n\\nInitial stable API includes:\\n- Core functionality\\n- Documentation\\n- Test coverage\\n- Production-ready\"\n\
                    })\n\n\
                 MINOR VERSION (New Features):\n\
                 1. Add backward-compatible functionality:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.1.0\",\n\
                        \"message\": \"Minor release 1.1.0\\n\\nNew Features:\\n- Added export to CSV\\n- New reporting dashboard\\n- REST API pagination\\n\\nAll existing functionality preserved\"\n\
                    })\n\n\
                 2. Significant improvements:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.2.0\",\n\
                        \"message\": \"Minor release 1.2.0\\n\\nEnhancements:\\n- 3x faster query performance\\n- New search filters\\n- Batch operations support\"\n\
                    })\n\n\
                 PATCH VERSION (Bug Fixes):\n\
                 1. Backward-compatible bug fixes:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.0.1\",\n\
                        \"message\": \"Patch release 1.0.1\\n\\nBug Fixes:\\n- Fixed crash on empty input\\n- Corrected date formatting\\n- Fixed memory leak in worker threads\"\n\
                    })\n\n\
                 2. Security patches:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.0.2\",\n\
                        \"message\": \"Security patch 1.0.2\\n\\nSecurity:\\n- Fixed SQL injection vulnerability (CVE-2024-1234)\\n- Updated dependencies with security issues\\n\\nUpgrade immediately recommended\"\n\
                    })\n\n\
                 PRE-RELEASE VERSIONS:\n\
                 1. Alpha release (early testing):\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v2.0.0-alpha.1\",\n\
                        \"message\": \"Alpha release 2.0.0-alpha.1\\n\\nExperimental release for early testing.\\nAPI may change without notice.\\nNot for production use.\"\n\
                    })\n\n\
                 2. Beta release (feature complete):\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v2.0.0-beta.1\",\n\
                        \"message\": \"Beta release 2.0.0-beta.1\\n\\nFeature complete for 2.0.0.\\nAPI is stable but may have bugs.\\nTesting and feedback welcome.\"\n\
                    })\n\n\
                 3. Release candidate:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v2.0.0-rc.1\",\n\
                        \"message\": \"Release candidate 2.0.0-rc.1\\n\\nCandidate for final 2.0.0 release.\\nThoroughly tested, production-ready.\\nWill become v2.0.0 if no issues found.\"\n\
                    })\n\n\
                 VERSIONING PROGRESSION:\n\
                 Initial development: v0.1.0, v0.2.0, v0.3.0...\n\
                 Pre-1.0 versions indicate unstable API\n\n\
                 First stable: v1.0.0\n\
                 Bug fix: v1.0.1, v1.0.2\n\
                 New features: v1.1.0, v1.2.0\n\
                 Breaking changes: v2.0.0\n\n\
                 With pre-releases:\n\
                 v1.9.0 → v2.0.0-alpha.1 → v2.0.0-alpha.2 → \n\
                 v2.0.0-beta.1 → v2.0.0-rc.1 → v2.0.0\n\n\
                 VERSION INCREMENT RULES:\n\
                 MAJOR version (X.0.0): Incompatible API changes\n\
                 - Reset MINOR and PATCH to 0\n\
                 - Example: v1.9.5 → v2.0.0\n\n\
                 MINOR version (x.Y.0): New features, backward-compatible\n\
                 - Reset PATCH to 0\n\
                 - Example: v1.2.7 → v1.3.0\n\n\
                 PATCH version (x.y.Z): Bug fixes, backward-compatible\n\
                 - Increment only\n\
                 - Example: v1.2.7 → v1.2.8\n\n\
                 VERSION 0.x.x SPECIAL RULES:\n\
                 During initial development (0.x.x):\n\
                 - Anything may change\n\
                 - Not considered stable\n\
                 - MINOR acts like MAJOR (can break compatibility)\n\
                 - Use until ready for v1.0.0\n\n\
                 BEST PRACTICES:\n\
                 1. Always prefix with 'v' (v1.0.0, not 1.0.0)\n\
                 2. Use annotated tags for all version tags\n\
                 3. Include changelog in tag message\n\
                 4. Tag from main/master branch after merge\n\
                 5. Create tag after updating version in code\n\
                 6. Document breaking changes clearly\n\
                 7. Follow semver strictly for public APIs\n\
                 8. Use pre-release versions for testing\n\
                 9. Never reuse or modify version tags\n\
                 10. Push tags after creation: git_push({\"path\": \"/project\", \"tags\": true})\n\n\
                 CHANGELOG IN TAG MESSAGE:\n\
                 Include structured changelog:\n\
                 - BREAKING CHANGES: (if MAJOR)\n\
                 - New Features: (if MINOR)\n\
                 - Bug Fixes: (if PATCH)\n\
                 - Security: (for security patches)\n\
                 - Deprecated: (features planned for removal)\n\
                 - Dependencies: (major dependency updates)\n\n\
                 REAL-WORLD EXAMPLES:\n\
                 v1.0.0 - First stable release\n\
                 v1.0.1 - Hotfix for critical bug\n\
                 v1.1.0 - New export feature\n\
                 v1.2.0 - Performance improvements\n\
                 v2.0.0-alpha.1 - Breaking changes preview\n\
                 v2.0.0-beta.1 - Beta testing\n\
                 v2.0.0-rc.1 - Release candidate\n\
                 v2.0.0 - Major release with breaking changes\n\
                 v2.0.1 - Bug fixes for 2.0.0\n\
                 v2.1.0 - New features added to 2.x line",
            ),
        },
    ]
}

/// Complete release workflow with tagging
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What is the complete workflow for creating and managing release tags?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Release tagging is a critical part of software deployment. Here are complete workflows for various release scenarios.\n\n\
                 STANDARD RELEASE WORKFLOW:\n\
                 1. Prepare release on main branch:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\
                    git_pull({\"path\": \"/project\"})\n\
                    // Ensure working directory is clean\n\n\
                 2. Verify everything is ready:\n\
                    git_status({\"path\": \"/project\"})\n\
                    // Should show \"nothing to commit, working tree clean\"\n\n\
                 3. Create the release tag:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.0.0\",\n\
                        \"message\": \"Release 1.0.0\\n\\nNew Features:\\n- Feature A\\n- Feature B\\n\\nBug Fixes:\\n- Fix for issue #123\"\n\
                    })\n\n\
                 4. Push commits and tags:\n\
                    git_push({\"path\": \"/project\"})\n\
                    git_push({\"path\": \"/project\", \"tags\": true})\n\
                    // Or push specific tag: git_push({\"path\": \"/project\", \"tag\": \"v1.0.0\"})\n\n\
                 5. Verify tag was pushed:\n\
                    git_tag({\"path\": \"/project\", \"operation\": \"list\"})\n\n\
                 HOTFIX WORKFLOW:\n\
                 1. Create hotfix branch from tag:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"hotfix-1.0.1\", \"create\": true, \"start_point\": \"v1.0.0\"})\n\n\
                 2. Make fixes and commit:\n\
                    // Make your fixes\n\
                    git_add({\"path\": \"/project\", \"files\": [\"file.rs\"]})\n\
                    git_commit({\"path\": \"/project\", \"message\": \"Fix critical bug\"})\n\n\
                 3. Merge to main:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\
                    git_merge({\"path\": \"/project\", \"branch\": \"hotfix-1.0.1\"})\n\n\
                 4. Create patch release tag:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.0.1\",\n\
                        \"message\": \"Hotfix 1.0.1\\n\\nCritical Fixes:\\n- Fixed crash on startup\\n- Corrected data corruption issue\"\n\
                    })\n\n\
                 5. Push and cleanup:\n\
                    git_push({\"path\": \"/project\"})\n\
                    git_push({\"path\": \"/project\", \"tags\": true})\n\
                    git_branch_delete({\"path\": \"/project\", \"name\": \"hotfix-1.0.1\"})\n\n\
                 BETA RELEASE WORKFLOW:\n\
                 1. Create release branch:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"release-2.0\", \"create\": true})\n\n\
                 2. Tag first beta:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v2.0.0-beta.1\",\n\
                        \"message\": \"Beta 1 for version 2.0.0\\n\\nTesting needed for:\\n- New API endpoints\\n- Database migrations\\n- UI changes\"\n\
                    })\n\n\
                 3. Push for testing:\n\
                    git_push({\"path\": \"/project\", \"set_upstream\": true})\n\
                    git_push({\"path\": \"/project\", \"tag\": \"v2.0.0-beta.1\"})\n\n\
                 4. After fixes, tag beta 2:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v2.0.0-beta.2\",\n\
                        \"message\": \"Beta 2 for version 2.0.0\\n\\nFixed:\\n- Issues from beta.1 testing\\n- Performance improvements\"\n\
                    })\n\n\
                 5. Final release:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v2.0.0\",\n\
                        \"message\": \"Release 2.0.0\\n\\nStable release after successful beta testing.\"\n\
                    })\n\n\
                 TAGGING SPECIFIC COMMIT:\n\
                 1. Find the commit to tag:\n\
                    git_log({\"path\": \"/project\", \"limit\": 20})\n\
                    // Review history to find the right commit\n\n\
                 2. Tag that commit:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.5.0\",\n\
                        \"target\": \"abc1234\",\n\
                        \"message\": \"Backport release 1.5.0\\n\\nTagging historical commit for maintenance branch.\"\n\
                    })\n\n\
                 3. Verify tag points to correct commit:\n\
                    git_show({\"path\": \"/project\", \"object\": \"v1.5.0\"})\n\n\
                 FIXING INCORRECT TAG:\n\
                 1. If tag was just created (not pushed):\n\
                    git_tag({\"path\": \"/project\", \"operation\": \"delete\", \"name\": \"v1.0.0\"})\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.0.0\",\n\
                        \"message\": \"Corrected release 1.0.0\"\n\
                    })\n\n\
                 2. If tag was already pushed (dangerous!):\n\
                    // Coordinate with team first!\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.0.0\",\n\
                        \"message\": \"Corrected release 1.0.0\",\n\
                        \"force\": true\n\
                    })\n\
                    git_push({\"path\": \"/project\", \"tag\": \"v1.0.0\", \"force\": true})\n\
                    // Notify all team members to re-fetch\n\n\
                 MULTI-VERSION SUPPORT:\n\
                 1. Create maintenance branch:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"maint-1.x\", \"create\": true, \"start_point\": \"v1.9.0\"})\n\n\
                 2. Tag maintenance releases:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.9.1\",\n\
                        \"message\": \"Maintenance release 1.9.1\\n\\nBackported fixes for v1.x users.\"\n\
                    })\n\n\
                 3. Continue v2.x development on main:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v2.1.0\",\n\
                        \"message\": \"Release 2.1.0\\n\\nNew features for v2.x line.\"\n\
                    })\n\n\
                 VERIFICATION CHECKLIST:\n\
                 After creating any release tag:\n\n\
                 1. Verify tag created:\n\
                    git_tag({\"path\": \"/project\", \"operation\": \"list\"})\n\n\
                 2. Check tag details:\n\
                    git_show({\"path\": \"/project\", \"object\": \"v1.0.0\"})\n\
                    // Verify commit, message, and tagger info\n\n\
                 3. Verify correct commit:\n\
                    git_log({\"path\": \"/project\", \"limit\": 1})\n\
                    // Ensure tag points to intended commit\n\n\
                 4. Check working directory is clean:\n\
                    git_status({\"path\": \"/project\"})\n\n\
                 5. Confirm push succeeded:\n\
                    // Check remote repository or CI/CD pipeline\n\n\
                 BEST PRACTICES:\n\
                 1. Always work from clean working directory\n\
                 2. Pull latest changes before tagging\n\
                 3. Use annotated tags for all releases\n\
                 4. Follow semantic versioning strictly\n\
                 5. Include comprehensive changelog in message\n\
                 6. Push tags immediately after creation\n\
                 7. Coordinate with team for shared repositories\n\
                 8. Never reuse or force-update public tags\n\
                 9. Tag from stable branch (main/master)\n\
                 10. Verify tag before announcing release\n\
                 11. Document any special upgrade steps\n\
                 12. Test deployment from tag before announcing\n\n\
                 COMMON MISTAKES TO AVOID:\n\
                 - Tagging with uncommitted changes\n\
                 - Forgetting to push tags after creation\n\
                 - Using lightweight tags for releases\n\
                 - Force-updating tags that others use\n\
                 - Tagging before tests pass\n\
                 - Inconsistent version numbering\n\
                 - Missing or poor tag messages\n\
                 - Tagging wrong commit or branch",
            ),
        },
    ]
}

/// Comprehensive guide to git_tag_create
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to creating Git tags effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Git tags mark specific points in repository history, typically for releases. Understanding when and how to create tags is essential for effective version management.\n\n\
                 =============================================================================\n\
                 TAG TYPES: ANNOTATED VS LIGHTWEIGHT\n\
                 =============================================================================\n\n\
                 ANNOTATED TAGS (Recommended for releases):\n\
                 git_tag_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"v1.0.0\",\n\
                     \"message\": \"Release version 1.0.0\"\n\
                 })\n\n\
                 Properties:\n\
                 - Full Git objects with their own SHA-1\n\
                 - Stores tagger name, email, date\n\
                 - Contains message describing the tag\n\
                 - Can be GPG-signed for verification\n\
                 - Recommended for all public releases\n\
                 - Required by many deployment systems\n\n\
                 LIGHTWEIGHT TAGS (For temporary markers):\n\
                 git_tag_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"checkpoint\"\n\
                 })\n\
                 // No message = lightweight\n\n\
                 Properties:\n\
                 - Just a pointer to a commit\n\
                 - No metadata stored\n\
                 - Fast and simple\n\
                 - Good for personal bookmarks\n\
                 - Not recommended for shared tags\n\n\
                 =============================================================================\n\
                 BASIC TAG CREATION\n\
                 =============================================================================\n\n\
                 1. Tag current commit (HEAD):\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.0.0\",\n\
                        \"message\": \"First stable release\"\n\
                    })\n\n\
                 2. Tag specific commit:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v0.9.0\",\n\
                        \"target\": \"abc1234\",\n\
                        \"message\": \"Beta release\"\n\
                    })\n\n\
                 3. Overwrite existing tag (careful!):\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.0.0\",\n\
                        \"message\": \"Corrected release\",\n\
                        \"force\": true\n\
                    })\n\n\
                 PARAMETERS:\n\
                 - path (required): Repository path\n\
                 - name (required): Tag name (e.g., \"v1.0.0\")\n\
                 - message (optional): Tag message (creates annotated if provided)\n\
                 - target (optional): Commit to tag (default: HEAD)\n\
                 - force (optional): Overwrite existing tag (default: false)\n\n\
                 =============================================================================\n\
                 SEMANTIC VERSIONING WITH TAGS\n\
                 =============================================================================\n\n\
                 FORMAT: vMAJOR.MINOR.PATCH[-PRERELEASE]\n\n\
                 MAJOR VERSION (Breaking changes):\n\
                 git_tag_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"v2.0.0\",\n\
                     \"message\": \"Major release 2.0.0\\n\\nBREAKING CHANGES:\\n- Removed old API\\n- Changed config format\"\n\
                 })\n\n\
                 MINOR VERSION (New features):\n\
                 git_tag_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"v1.1.0\",\n\
                     \"message\": \"Minor release 1.1.0\\n\\nNew Features:\\n- Added CSV export\\n- New dashboard\"\n\
                 })\n\n\
                 PATCH VERSION (Bug fixes):\n\
                 git_tag_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"v1.0.1\",\n\
                     \"message\": \"Patch release 1.0.1\\n\\nBug Fixes:\\n- Fixed crash on startup\"\n\
                 })\n\n\
                 PRE-RELEASE VERSIONS:\n\
                 Alpha: v2.0.0-alpha.1 (early testing)\n\
                 Beta: v2.0.0-beta.1 (feature complete)\n\
                 RC: v2.0.0-rc.1 (release candidate)\n\n\
                 =============================================================================\n\
                 RELEASE WORKFLOWS\n\
                 =============================================================================\n\n\
                 STANDARD RELEASE:\n\
                 1. Ensure clean state:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"main\"})\n\
                    git_pull({\"path\": \"/project\"})\n\
                    git_status({\"path\": \"/project\"})\n\n\
                 2. Create release tag:\n\
                    git_tag_create({\n\
                        \"path\": \"/project\",\n\
                        \"name\": \"v1.0.0\",\n\
                        \"message\": \"Release 1.0.0\\n\\nChangelog summary here\"\n\
                    })\n\n\
                 3. Push to remote:\n\
                    git_push({\"path\": \"/project\"})\n\
                    git_push({\"path\": \"/project\", \"tags\": true})\n\n\
                 HOTFIX RELEASE:\n\
                 1. Branch from release tag:\n\
                    git_checkout({\"path\": \"/project\", \"branch\": \"hotfix\", \"create\": true, \"start_point\": \"v1.0.0\"})\n\
                 2. Fix and commit\n\
                 3. Merge back to main\n\
                 4. Create patch tag:\n\
                    git_tag_create({\"path\": \"/project\", \"name\": \"v1.0.1\", \"message\": \"Hotfix 1.0.1\"})\n\n\
                 =============================================================================\n\
                 TAG NAMING CONVENTIONS\n\
                 =============================================================================\n\n\
                 GOOD TAG NAMES:\n\
                 - v1.0.0 (semantic version)\n\
                 - v2.1.3 (patch release)\n\
                 - v3.0.0-beta.1 (pre-release)\n\
                 - release-2024-01 (date-based)\n\
                 - v1.0.0-hotfix.1 (hotfix variant)\n\n\
                 AVOID:\n\
                 - 1.0.0 (missing 'v' prefix)\n\
                 - latest (moving target)\n\
                 - final (not descriptive)\n\
                 - v1 (not specific enough)\n\
                 - my-tag (too generic)\n\n\
                 PREFIX CONVENTIONS:\n\
                 - v1.0.0: Standard version prefix\n\
                 - release/1.0.0: Some teams use this\n\
                 - prod-v1.0.0: Environment prefixes\n\
                 Choose one convention and stick to it!\n\n\
                 =============================================================================\n\
                 TAG MESSAGES\n\
                 =============================================================================\n\n\
                 GOOD TAG MESSAGE STRUCTURE:\n\
                 git_tag_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"v1.0.0\",\n\
                     \"message\": \"Release 1.0.0\\n\\nNew Features:\\n- Feature A\\n- Feature B\\n\\nBug Fixes:\\n- Fix for #123\\n- Fix for #456\\n\\nBreaking Changes:\\n- API endpoint /old removed\\n\\nUpgrade Notes:\\n- Update config file format\\n- Run migration script\"\n\
                 })\n\n\
                 MESSAGE SECTIONS:\n\
                 1. Title: Brief summary (first line)\n\
                 2. New Features: What's new\n\
                 3. Bug Fixes: What's fixed\n\
                 4. Breaking Changes: What broke (if MAJOR)\n\
                 5. Upgrade Notes: How to upgrade\n\
                 6. Dependencies: Major dependency updates\n\
                 7. Security: Security fixes (if any)\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 1. FIRST STABLE RELEASE:\n\
                 git_tag_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"v1.0.0\",\n\
                     \"message\": \"First stable release\\n\\nInitial stable API with full documentation.\"\n\
                 })\n\n\
                 2. FEATURE RELEASE:\n\
                 git_tag_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"v1.1.0\",\n\
                     \"message\": \"Feature release 1.1.0\\n\\nAdded CSV export and new dashboard.\"\n\
                 })\n\n\
                 3. SECURITY PATCH:\n\
                 git_tag_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"v1.0.1\",\n\
                     \"message\": \"Security patch 1.0.1\\n\\nFixed CVE-2024-1234. Upgrade immediately.\"\n\
                 })\n\n\
                 4. BETA TESTING:\n\
                 git_tag_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"v2.0.0-beta.1\",\n\
                     \"message\": \"Beta 1 for version 2.0.0\\n\\nFeature complete. Testing welcome.\"\n\
                 })\n\n\
                 =============================================================================\n\
                 VERIFICATION AND TROUBLESHOOTING\n\
                 =============================================================================\n\n\
                 VERIFY TAG CREATED:\n\
                 git_tag({\"path\": \"/project\", \"operation\": \"list\"})\n\n\
                 CHECK TAG DETAILS:\n\
                 git_show({\"path\": \"/project\", \"object\": \"v1.0.0\"})\n\n\
                 LIST RECENT TAGS:\n\
                 git_tag({\"path\": \"/project\", \"operation\": \"list\"})\n\n\
                 DELETE TAG (if mistake):\n\
                 git_tag({\"path\": \"/project\", \"operation\": \"delete\", \"name\": \"v1.0.0\"})\n\n\
                 FIX WRONG TAG:\n\
                 1. Delete wrong tag\n\
                 2. Create correct tag\n\
                 3. If already pushed, coordinate with team!\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. TAG CREATION:\n\
                    - Always use annotated tags for releases\n\
                    - Tag from clean working directory\n\
                    - Verify commits are correct before tagging\n\
                    - Include comprehensive changelog in message\n\n\
                 2. VERSIONING:\n\
                    - Follow semantic versioning strictly\n\
                    - Prefix with 'v' (v1.0.0)\n\
                    - Never reuse version numbers\n\
                    - Use pre-release versions for testing\n\n\
                 3. WORKFLOW:\n\
                    - Pull latest changes before tagging\n\
                    - Tag from stable branch (main/master)\n\
                    - Push tags immediately after creation\n\
                    - Verify push succeeded\n\n\
                 4. COLLABORATION:\n\
                    - Coordinate tag creation in teams\n\
                    - Don't force-update public tags\n\
                    - Document tagging process\n\
                    - Use CI/CD for automated tagging\n\n\
                 5. DOCUMENTATION:\n\
                    - Write clear tag messages\n\
                    - Include breaking changes prominently\n\
                    - Link to detailed changelog\n\
                    - Add upgrade instructions\n\n\
                 =============================================================================\n\
                 COMMON MISTAKES TO AVOID\n\
                 =============================================================================\n\n\
                 1. Using lightweight tags for releases\n\
                 2. Forgetting to push tags after creation\n\
                 3. Tagging with uncommitted changes\n\
                 4. Force-updating tags others depend on\n\
                 5. Inconsistent version numbering\n\
                 6. Empty or poor tag messages\n\
                 7. Tagging wrong branch or commit\n\
                 8. Not verifying tag before announcing\n\
                 9. Reusing version numbers\n\
                 10. Ignoring semantic versioning rules\n\n\
                 =============================================================================\n\
                 DECISION GUIDE\n\
                 =============================================================================\n\n\
                 WHEN TO TAG:\n\
                 - Releasing to production\n\
                 - Marking major milestones\n\
                 - Creating beta/RC versions\n\
                 - Deploying to environments\n\
                 - Need stable reference point\n\n\
                 ANNOTATED OR LIGHTWEIGHT:\n\
                 - Production releases → ANNOTATED\n\
                 - Public versions → ANNOTATED\n\
                 - Shared with team → ANNOTATED\n\
                 - Personal bookmarks → LIGHTWEIGHT\n\
                 - Temporary markers → LIGHTWEIGHT\n\n\
                 VERSION INCREMENT:\n\
                 - Breaking changes → MAJOR (v2.0.0)\n\
                 - New features → MINOR (v1.1.0)\n\
                 - Bug fixes → PATCH (v1.0.1)\n\
                 - Testing → PRERELEASE (v2.0.0-beta.1)\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Create annotated tag:\n\
                 git_tag_create({\"path\": \"/project\", \"name\": \"v1.0.0\", \"message\": \"Release 1.0.0\"})\n\n\
                 Create lightweight tag:\n\
                 git_tag_create({\"path\": \"/project\", \"name\": \"checkpoint\"})\n\n\
                 Tag specific commit:\n\
                 git_tag_create({\"path\": \"/project\", \"name\": \"v1.0.0\", \"target\": \"abc1234\", \"message\": \"Release\"})\n\n\
                 List all tags:\n\
                 git_tag({\"path\": \"/project\", \"operation\": \"list\"})\n\n\
                 View tag details:\n\
                 git_show({\"path\": \"/project\", \"object\": \"v1.0.0\"})\n\n\
                 Push tags:\n\
                 git_push({\"path\": \"/project\", \"tags\": true})\n\n\
                 Delete tag:\n\
                 git_tag({\"path\": \"/project\", \"operation\": \"delete\", \"name\": \"v1.0.0\"})\n\n\
                 Remember: Tags mark important points in history. Use them wisely to track releases, milestones, and deployments!",
            ),
        },
    ]
}
