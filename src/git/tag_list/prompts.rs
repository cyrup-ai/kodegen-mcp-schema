//! Prompt messages for git_tag_list tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitTagListPromptArgs;

/// Prompt provider for git_tag_list tool
///
/// This is the ONLY way to provide prompts for git_tag_list - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GitTagListPrompts;

impl PromptProvider for GitTagListPrompts {
    type PromptArgs = GitTagListPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("filtering") => prompt_filtering(),
            Some("sorting") => prompt_sorting(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, filtering, sorting, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST GIT TAGS
// ============================================================================

/// Basic tag listing operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list tags in a Git repository using git_tag_list?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_tag_list tool lists existing tags in a repository. Here's how to use it for basic tag listing:\n\n\
                 BASIC TAG LISTING:\n\
                 1. List all tags:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\"\n\
                    })\n\n\
                 RESPONSE FORMAT:\n\
                 {\n\
                   \"tags\": [\n\
                     \"v0.1.0\",\n\
                     \"v0.2.0\",\n\
                     \"v1.0.0\",\n\
                     \"v1.0.1\",\n\
                     \"v1.1.0\"\n\
                   ]\n\
                 }\n\n\
                 2. List tags after fetching from remote:\n\
                    // First, fetch tags from remote\n\
                    git_fetch({\"path\": \"/project\", \"tags\": true})\n\
                    // Then list all tags (includes remote tags)\n\
                    git_tag_list({\"path\": \"/project\"})\n\n\
                 3. Check if repository has any tags:\n\
                    git_tag_list({\"path\": \"/project\"})\n\
                    // Empty array means no tags: {\"tags\": []}\n\n\
                 TAG LIST CHARACTERISTICS:\n\
                 - Shows all tags in the repository\n\
                 - Includes both local and fetched remote tags\n\
                 - Alphabetically sorted by default\n\
                 - Returns array of tag names as strings\n\
                 - Empty array if no tags exist\n\n\
                 PARAMETERS:\n\
                 - path (required): Path to the Git repository\n\
                 - pattern (optional): Glob pattern to filter tags\n\
                 - sort (optional): Sort order (version, -version, creatordate, -creatordate)\n\
                 - limit (optional): Maximum number of tags to return\n\n\
                 COMMON PATTERNS:\n\
                 1. Check available versions:\n\
                    git_tag_list({\"path\": \"/project\"})\n\
                 2. After cloning a repository:\n\
                    git_clone({\"url\": \"https://github.com/org/repo\", \"path\": \"/local\"})\n\
                    git_tag_list({\"path\": \"/local\"})\n\
                 3. Before creating a new tag:\n\
                    git_tag_list({\"path\": \"/project\"})\n\
                    // Check existing tags to avoid conflicts\n\n\
                 RESPONSE STRUCTURE:\n\
                 The response always contains a \"tags\" array:\n\
                 - tags: Array of tag name strings\n\
                 - Order depends on sort parameter (default: alphabetical)\n\
                 - Can be empty if no tags exist\n\n\
                 USE CASES:\n\
                 - Discover available versions\n\
                 - Check which releases exist\n\
                 - Verify tag creation succeeded\n\
                 - Browse repository version history\n\
                 - Find latest or specific versions",
            ),
        },
    ]
}

/// Pattern-based tag filtering
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I filter tags by pattern when listing them?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use the pattern parameter to filter tags by glob patterns. This helps narrow down large tag lists to specific versions.\n\n\
                 FILTERING BY PATTERN:\n\
                 1. Filter by major version:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"pattern\": \"v1.*\"\n\
                    })\n\
                    // Returns only v1.x.x tags: [\"v1.0.0\", \"v1.0.1\", \"v1.1.0\", \"v1.2.0\"]\n\n\
                 2. Filter by specific minor version:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"pattern\": \"v1.0.*\"\n\
                    })\n\
                    // Returns only v1.0.x tags: [\"v1.0.0\", \"v1.0.1\", \"v1.0.2\"]\n\n\
                 3. Find pre-release tags:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"pattern\": \"*-beta*\"\n\
                    })\n\
                    // Returns beta tags: [\"v2.0.0-beta.1\", \"v2.0.0-beta.2\"]\n\n\
                 4. Find release candidates:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"pattern\": \"*-rc*\"\n\
                    })\n\
                    // Returns RC tags: [\"v1.5.0-rc.1\", \"v1.5.0-rc.2\"]\n\n\
                 5. Filter by year prefix:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"pattern\": \"2024-*\"\n\
                    })\n\
                    // Returns tags starting with 2024-\n\n\
                 6. Find all v2.x releases:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"pattern\": \"v2.*\"\n\
                    })\n\n\
                 PATTERN SYNTAX:\n\
                 - * matches zero or more characters\n\
                 - ? matches exactly one character\n\
                 - [abc] matches any character in the set\n\
                 - [a-z] matches any character in the range\n\n\
                 PATTERN EXAMPLES:\n\
                 - v1.* matches v1.0.0, v1.2.3, v1.10.5\n\
                 - v*.0.0 matches v1.0.0, v2.0.0, v10.0.0\n\
                 - *-alpha* matches v1.0.0-alpha.1, v2.0.0-alpha\n\
                 - release-* matches release-2024, release-v1.0\n\
                 - v1.?.0 matches v1.0.0, v1.1.0, v1.2.0 (single digit)\n\n\
                 COMBINING WITH OTHER PARAMETERS:\n\
                 Filter and sort:\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"pattern\": \"v1.*\",\n\
                     \"sort\": \"version\"\n\
                 })\n\
                 // Returns v1.x tags sorted by semantic version\n\n\
                 Filter and limit:\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"pattern\": \"v2.*\",\n\
                     \"limit\": 5\n\
                 })\n\
                 // Returns up to 5 v2.x tags\n\n\
                 COMMON FILTERING WORKFLOWS:\n\
                 1. Find latest patch in minor version:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"pattern\": \"v1.5.*\",\n\
                        \"sort\": \"-version\",\n\
                        \"limit\": 1\n\
                    })\n\
                    // Gets newest v1.5.x tag\n\n\
                 2. List all stable releases (exclude pre-releases):\n\
                    // First get all tags\n\
                    git_tag_list({\"path\": \"/project\", \"pattern\": \"v*\"})\n\
                    // Filter out tags with - in them (pre-releases)\n\n\
                 3. Find tags for specific major version:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"pattern\": \"v3.*\",\n\
                        \"sort\": \"version\"\n\
                    })\n\n\
                 CASE SENSITIVITY:\n\
                 - Patterns are case-sensitive by default\n\
                 - v1.* will NOT match V1.0.0\n\
                 - Use appropriate casing for your tag naming convention\n\n\
                 BEST PRACTICES:\n\
                 - Use consistent tag naming (v1.0.0 vs 1.0.0)\n\
                 - Filter before sorting for better performance\n\
                 - Combine pattern with limit to get specific results\n\
                 - Use wildcards strategically to match your versioning scheme",
            ),
        },
    ]
}

/// Sorting tags by different criteria
fn prompt_sorting() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I sort tags when listing them?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use the sort parameter to control tag ordering. This is crucial for finding latest versions and understanding version progression.\n\n\
                 SORTING OPTIONS:\n\
                 1. Sort by semantic version (ascending):\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"sort\": \"version\"\n\
                    })\n\
                    // Returns: [\"v0.1.0\", \"v0.2.0\", \"v1.0.0\", \"v1.0.1\", \"v1.1.0\"]\n\
                    // Correct semantic version order: 0.1.0 < 0.2.0 < 1.0.0\n\n\
                 2. Sort by version (descending - latest first):\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"sort\": \"-version\"\n\
                    })\n\
                    // Returns: [\"v1.1.0\", \"v1.0.1\", \"v1.0.0\", \"v0.2.0\", \"v0.1.0\"]\n\
                    // Most recent version first\n\n\
                 3. Sort by creation date (oldest first):\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"sort\": \"creatordate\"\n\
                    })\n\
                    // Returns tags in chronological order\n\n\
                 4. Sort by creation date (newest first):\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"sort\": \"-creatordate\"\n\
                    })\n\
                    // Returns most recently created tags first\n\n\
                 5. Default (no sort parameter):\n\
                    git_tag_list({\"path\": \"/project\"})\n\
                    // Alphabetical order by tag name\n\n\
                 SORT VALUES:\n\
                 - \"version\": Semantic version order (v0.1.0 < v0.2.0 < v1.0.0)\n\
                 - \"-version\": Reverse version order (newest version first)\n\
                 - \"creatordate\": By tag creation date (oldest first)\n\
                 - \"-creatordate\": By creation date (newest first)\n\
                 - null/omitted: Alphabetical by tag name\n\n\
                 VERSION SORTING BEHAVIOR:\n\
                 Semantic version sort correctly handles:\n\
                 - Major version: v2.0.0 > v1.9.9\n\
                 - Minor version: v1.10.0 > v1.9.0 (not alphabetical!)\n\
                 - Patch version: v1.0.10 > v1.0.9\n\
                 - Pre-releases: v1.0.0 > v1.0.0-rc.1 > v1.0.0-beta.1\n\n\
                 Example of version vs alphabetical sorting:\n\
                 Alphabetical: [\"v1.1.0\", \"v1.10.0\", \"v1.2.0\"]  ❌ Wrong!\n\
                 Version sort: [\"v1.1.0\", \"v1.2.0\", \"v1.10.0\"]   ✓ Correct!\n\n\
                 COMBINING SORT WITH OTHER PARAMETERS:\n\
                 1. Get latest version with limit:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"sort\": \"-version\",\n\
                        \"limit\": 1\n\
                    })\n\
                    // Returns: [\"v1.5.2\"] (most recent version)\n\n\
                 2. Get latest 5 versions:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"sort\": \"-version\",\n\
                        \"limit\": 5\n\
                    })\n\n\
                 3. Get oldest tags by date:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"sort\": \"creatordate\",\n\
                        \"limit\": 3\n\
                    })\n\
                    // First 3 tags created historically\n\n\
                 4. Latest beta releases:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"pattern\": \"*-beta*\",\n\
                        \"sort\": \"-version\",\n\
                        \"limit\": 3\n\
                    })\n\n\
                 SORT USE CASES:\n\
                 version sort:\n\
                 - Finding latest/oldest version\n\
                 - Understanding version progression\n\
                 - Checking upgrade paths\n\
                 - Validating semantic versioning\n\n\
                 creatordate sort:\n\
                 - Finding recently created tags\n\
                 - Understanding tagging timeline\n\
                 - Identifying release frequency\n\
                 - Debugging tag creation issues\n\n\
                 PRACTICAL EXAMPLES:\n\
                 1. Find latest stable release:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"pattern\": \"v*\",\n\
                        \"sort\": \"-version\",\n\
                        \"limit\": 1\n\
                    })\n\
                    // Skip pre-releases by filtering pattern\n\n\
                 2. Get version history in order:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"sort\": \"version\"\n\
                    })\n\
                    // See version evolution: v0.1.0 → v1.0.0 → v2.0.0\n\n\
                 3. Check recent tagging activity:\n\
                    git_tag_list({\n\
                        \"path\": \"/project\",\n\
                        \"sort\": \"-creatordate\",\n\
                        \"limit\": 10\n\
                    })\n\
                    // Last 10 tags created\n\n\
                 BEST PRACTICES:\n\
                 - Use -version sort to find latest releases\n\
                 - Use version sort to validate version sequence\n\
                 - Use creatordate sort to understand tagging timeline\n\
                 - Combine with limit to get specific number of tags\n\
                 - Use version sort for semantic version tags (v1.2.3)\n\
                 - Use creatordate sort for date-based tags (2024-01-15)",
            ),
        },
    ]
}

/// Common tag listing workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows for using git_tag_list?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Here are practical workflows that combine git_tag_list with other operations for real-world use cases.\n\n\
                 WORKFLOW 1: FIND LATEST RELEASE\n\
                 Get the most recent version tag:\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"sort\": \"-version\",\n\
                     \"limit\": 1\n\
                 })\n\
                 // Returns: [\"v1.5.2\"]\n\
                 // Use this to determine latest version for changelog\n\n\
                 WORKFLOW 2: CHECK BEFORE CREATING TAG\n\
                 Verify tag doesn't exist before creating:\n\
                 // Step 1: List existing tags in that version range\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"pattern\": \"v1.0.*\"\n\
                 })\n\
                 // Returns: [\"v1.0.0\", \"v1.0.1\", \"v1.0.2\"]\n\
                 \n\
                 // Step 2: Create next patch version\n\
                 git_tag_create({\n\
                     \"path\": \"/project\",\n\
                     \"name\": \"v1.0.3\",\n\
                     \"message\": \"Patch release\"\n\
                 })\n\
                 \n\
                 // Step 3: Verify tag was created\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"pattern\": \"v1.0.*\"\n\
                 })\n\
                 // Should now include v1.0.3\n\n\
                 WORKFLOW 3: VIEW RELEASE HISTORY\n\
                 Get complete version history:\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"sort\": \"version\"\n\
                 })\n\
                 // Returns all tags in semantic version order\n\
                 // Use this for:\n\
                 // - Generating CHANGELOG.md\n\
                 // - Understanding version progression\n\
                 // - Documenting release timeline\n\n\
                 WORKFLOW 4: CHECK REMOTE TAGS\n\
                 Fetch and list remote tags:\n\
                 // Step 1: Fetch tags from remote\n\
                 git_fetch({\"path\": \"/project\", \"tags\": true})\n\
                 \n\
                 // Step 2: List all tags (includes remote)\n\
                 git_tag_list({\"path\": \"/project\"})\n\
                 // Now see tags from remote that weren't in local\n\n\
                 WORKFLOW 5: FIND TAGS FOR SPECIFIC BRANCH\n\
                 List tags reachable from a branch:\n\
                 // Step 1: Checkout the branch\n\
                 git_checkout({\"path\": \"/project\", \"branch\": \"release-1.x\"})\n\
                 \n\
                 // Step 2: List tags\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"pattern\": \"v1.*\"\n\
                 })\n\n\
                 WORKFLOW 6: DETERMINE NEXT VERSION\n\
                 Calculate what the next version should be:\n\
                 // Get current latest version\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"sort\": \"-version\",\n\
                     \"limit\": 1\n\
                 })\n\
                 // Returns: [\"v1.5.2\"]\n\
                 \n\
                 // Decide based on changes:\n\
                 // - Breaking changes → v2.0.0 (major)\n\
                 // - New features → v1.6.0 (minor)\n\
                 // - Bug fixes → v1.5.3 (patch)\n\n\
                 WORKFLOW 7: LIST PRE-RELEASE TAGS\n\
                 Find all pre-release versions:\n\
                 // Beta releases\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"pattern\": \"*-beta*\",\n\
                     \"sort\": \"-version\"\n\
                 })\n\
                 \n\
                 // Release candidates\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"pattern\": \"*-rc*\",\n\
                     \"sort\": \"-version\"\n\
                 })\n\n\
                 WORKFLOW 8: COMPARE LOCAL AND REMOTE TAGS\n\
                 // List local tags\n\
                 git_tag_list({\"path\": \"/project\"})\n\
                 // Note the tags\n\
                 \n\
                 // Fetch from remote\n\
                 git_fetch({\"path\": \"/project\", \"tags\": true})\n\
                 \n\
                 // List again to see new tags\n\
                 git_tag_list({\"path\": \"/project\"})\n\
                 // Compare to see which tags were fetched\n\n\
                 WORKFLOW 9: VALIDATE SEMVER COMPLIANCE\n\
                 Check if tags follow semantic versioning:\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"sort\": \"version\"\n\
                 })\n\
                 // Review for proper progression:\n\
                 // ✓ v1.0.0 → v1.0.1 → v1.1.0 → v2.0.0\n\
                 // ✗ v1.0.0 → v1.0.2 → v1.0.1 (out of order)\n\n\
                 WORKFLOW 10: FIND TAGS IN DATE RANGE\n\
                 Get tags created in specific time period:\n\
                 // Get all tags sorted by date\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"sort\": \"-creatordate\"\n\
                 })\n\
                 // Then filter by examining tag info with git_show\n\n\
                 INTEGRATION WITH OTHER GIT TOOLS:\n\
                 1. After cloning:\n\
                    git_clone({\"url\": \"...\", \"path\": \"/local\"})\n\
                    git_tag_list({\"path\": \"/local\"})\n\
                    // See available versions\n\n\
                 2. Before checking out:\n\
                    git_tag_list({\"path\": \"/project\"})\n\
                    git_checkout({\"path\": \"/project\", \"ref\": \"v1.5.0\"})\n\
                    // Checkout specific version\n\n\
                 3. With git_show:\n\
                    git_tag_list({\"path\": \"/project\", \"limit\": 1, \"sort\": \"-version\"})\n\
                    git_show({\"path\": \"/project\", \"ref\": \"v1.5.2\"})\n\
                    // Get details about latest tag\n\n\
                 4. Before pushing:\n\
                    git_tag_list({\"path\": \"/project\", \"pattern\": \"v2.0.0\"})\n\
                    git_push({\"path\": \"/project\", \"tags\": true})\n\
                    // Verify tag exists before pushing\n\n\
                 BEST PRACTICES:\n\
                 - Always list tags before creating to avoid duplicates\n\
                 - Use sort + limit to get specific results efficiently\n\
                 - Combine with pattern to narrow down searches\n\
                 - Fetch tags before listing if checking remote state\n\
                 - Use -version sort to find latest releases\n\
                 - Verify tag creation by listing again after create",
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
                "Give me a complete guide to using git_tag_list effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_tag_list tool lists tags in a Git repository with powerful filtering, sorting, and limiting capabilities.\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 Simple list all tags:\n\
                 git_tag_list({\"path\": \"/project\"})\n\n\
                 Returns:\n\
                 {\n\
                   \"tags\": [\"v0.1.0\", \"v0.2.0\", \"v1.0.0\", \"v1.0.1\"]\n\
                 }\n\n\
                 PARAMETERS:\n\
                 - path (required): Path to Git repository\n\
                 - pattern (optional): Glob pattern to filter tags\n\
                 - sort (optional): Sort order (version, -version, creatordate, -creatordate)\n\
                 - limit (optional): Maximum number of tags to return\n\n\
                 =============================================================================\n\
                 FILTERING WITH PATTERNS\n\
                 =============================================================================\n\n\
                 Filter by major version:\n\
                 git_tag_list({\"path\": \"/project\", \"pattern\": \"v1.*\"})\n\
                 // Returns only v1.x.x tags\n\n\
                 Filter by minor version:\n\
                 git_tag_list({\"path\": \"/project\", \"pattern\": \"v1.0.*\"})\n\
                 // Returns only v1.0.x tags\n\n\
                 Find pre-release tags:\n\
                 git_tag_list({\"path\": \"/project\", \"pattern\": \"*-beta*\"})\n\
                 // Returns all beta tags\n\n\
                 Find release candidates:\n\
                 git_tag_list({\"path\": \"/project\", \"pattern\": \"*-rc*\"})\n\
                 // Returns all RC tags\n\n\
                 PATTERN SYNTAX:\n\
                 - * matches any characters: v1.* matches v1.0.0, v1.2.3\n\
                 - ? matches single character: v1.?.0 matches v1.0.0, v1.1.0\n\
                 - [abc] matches any in set\n\
                 - [a-z] matches range\n\n\
                 =============================================================================\n\
                 SORTING OPTIONS\n\
                 =============================================================================\n\n\
                 Sort by semantic version (ascending):\n\
                 git_tag_list({\"path\": \"/project\", \"sort\": \"version\"})\n\
                 // Returns: [\"v0.1.0\", \"v0.2.0\", \"v1.0.0\", \"v1.1.0\"]\n\n\
                 Sort by version (descending - latest first):\n\
                 git_tag_list({\"path\": \"/project\", \"sort\": \"-version\"})\n\
                 // Returns: [\"v1.1.0\", \"v1.0.0\", \"v0.2.0\", \"v0.1.0\"]\n\n\
                 Sort by creation date (oldest first):\n\
                 git_tag_list({\"path\": \"/project\", \"sort\": \"creatordate\"})\n\
                 // Chronological order\n\n\
                 Sort by creation date (newest first):\n\
                 git_tag_list({\"path\": \"/project\", \"sort\": \"-creatordate\"})\n\
                 // Most recent tags first\n\n\
                 SORT VALUES:\n\
                 - version: Semantic version order (v0.1.0 < v1.0.0 < v1.1.0)\n\
                 - -version: Reverse version (v1.1.0 > v1.0.0 > v0.1.0)\n\
                 - creatordate: By creation time (oldest first)\n\
                 - -creatordate: By creation time (newest first)\n\
                 - null: Alphabetical (default)\n\n\
                 VERSION SORT HANDLES:\n\
                 - Major: v2.0.0 > v1.9.9 ✓\n\
                 - Minor: v1.10.0 > v1.9.0 ✓ (not alphabetical!)\n\
                 - Patch: v1.0.10 > v1.0.9 ✓\n\
                 - Pre-release: v1.0.0 > v1.0.0-rc.1 ✓\n\n\
                 =============================================================================\n\
                 LIMITING RESULTS\n\
                 =============================================================================\n\n\
                 Get latest version:\n\
                 git_tag_list({\"path\": \"/project\", \"sort\": \"-version\", \"limit\": 1})\n\
                 // Returns: [\"v1.5.2\"]\n\n\
                 Get latest 5 versions:\n\
                 git_tag_list({\"path\": \"/project\", \"sort\": \"-version\", \"limit\": 5})\n\n\
                 Get oldest 3 tags:\n\
                 git_tag_list({\"path\": \"/project\", \"sort\": \"creatordate\", \"limit\": 3})\n\n\
                 =============================================================================\n\
                 COMBINING PARAMETERS\n\
                 =============================================================================\n\n\
                 Latest v1.x release:\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"pattern\": \"v1.*\",\n\
                     \"sort\": \"-version\",\n\
                     \"limit\": 1\n\
                 })\n\n\
                 Latest 3 beta releases:\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"pattern\": \"*-beta*\",\n\
                     \"sort\": \"-version\",\n\
                     \"limit\": 3\n\
                 })\n\n\
                 Recent v2.x tags:\n\
                 git_tag_list({\n\
                     \"path\": \"/project\",\n\
                     \"pattern\": \"v2.*\",\n\
                     \"sort\": \"-creatordate\",\n\
                     \"limit\": 10\n\
                 })\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 WORKFLOW 1: Find latest release\n\
                 git_tag_list({\"path\": \"/project\", \"sort\": \"-version\", \"limit\": 1})\n\n\
                 WORKFLOW 2: Check before creating tag\n\
                 // List existing tags in range\n\
                 git_tag_list({\"path\": \"/project\", \"pattern\": \"v1.0.*\"})\n\
                 // Create next version\n\
                 git_tag_create({\"path\": \"/project\", \"name\": \"v1.0.3\", \"message\": \"Patch\"})\n\
                 // Verify creation\n\
                 git_tag_list({\"path\": \"/project\", \"pattern\": \"v1.0.*\"})\n\n\
                 WORKFLOW 3: View release history\n\
                 git_tag_list({\"path\": \"/project\", \"sort\": \"version\"})\n\
                 // Shows progression: v0.1.0 → v1.0.0 → v2.0.0\n\n\
                 WORKFLOW 4: Check remote tags\n\
                 git_fetch({\"path\": \"/project\", \"tags\": true})\n\
                 git_tag_list({\"path\": \"/project\"})\n\
                 // Now includes fetched remote tags\n\n\
                 WORKFLOW 5: Find next version number\n\
                 git_tag_list({\"path\": \"/project\", \"sort\": \"-version\", \"limit\": 1})\n\
                 // Returns [\"v1.5.2\"]\n\
                 // Next: v1.5.3 (patch), v1.6.0 (minor), or v2.0.0 (major)\n\n\
                 WORKFLOW 6: List pre-releases\n\
                 git_tag_list({\"path\": \"/project\", \"pattern\": \"*-beta*\", \"sort\": \"-version\"})\n\
                 git_tag_list({\"path\": \"/project\", \"pattern\": \"*-rc*\", \"sort\": \"-version\"})\n\n\
                 =============================================================================\n\
                 RESPONSE STRUCTURE\n\
                 =============================================================================\n\n\
                 All responses contain a tags array:\n\
                 {\n\
                   \"tags\": [\"v0.1.0\", \"v0.2.0\", \"v1.0.0\"]\n\
                 }\n\n\
                 - tags: Array of tag name strings\n\
                 - Order determined by sort parameter\n\
                 - Empty array if no matching tags: {\"tags\": []}\n\
                 - Array truncated if limit specified\n\n\
                 =============================================================================\n\
                 INTEGRATION WITH OTHER TOOLS\n\
                 =============================================================================\n\n\
                 With git_fetch:\n\
                 git_fetch({\"path\": \"/project\", \"tags\": true})\n\
                 git_tag_list({\"path\": \"/project\"})\n\n\
                 With git_checkout:\n\
                 git_tag_list({\"path\": \"/project\"})\n\
                 git_checkout({\"path\": \"/project\", \"ref\": \"v1.0.0\"})\n\n\
                 With git_tag_create:\n\
                 git_tag_list({\"path\": \"/project\", \"pattern\": \"v1.*\"})\n\
                 git_tag_create({\"path\": \"/project\", \"name\": \"v1.5.0\"})\n\n\
                 With git_push:\n\
                 git_tag_list({\"path\": \"/project\", \"pattern\": \"v2.0.0\"})\n\
                 git_push({\"path\": \"/project\", \"tags\": true})\n\n\
                 With git_show:\n\
                 git_tag_list({\"path\": \"/project\", \"sort\": \"-version\", \"limit\": 1})\n\
                 git_show({\"path\": \"/project\", \"ref\": \"v1.5.2\"})\n\n\
                 =============================================================================\n\
                 USE CASES\n\
                 =============================================================================\n\n\
                 - Discover available versions in repository\n\
                 - Find latest release for installation\n\
                 - Check which releases exist before tagging\n\
                 - Generate changelog from version history\n\
                 - Validate semantic versioning compliance\n\
                 - Find pre-release versions for testing\n\
                 - Determine next version number\n\
                 - Compare local and remote tags\n\
                 - Browse release timeline\n\
                 - Identify version gaps or issues\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. LIST BEFORE CREATING:\n\
                    Always check existing tags to avoid duplicates\n\
                    git_tag_list({\"path\": \"/project\", \"pattern\": \"v1.0.*\"})\n\n\
                 2. USE VERSION SORT:\n\
                    For semantic versioned tags, always use sort: \"version\"\n\
                    Alphabetical sort gives wrong order for v1.10.0 > v1.9.0\n\n\
                 3. FETCH FIRST:\n\
                    When checking remote state, fetch tags first\n\
                    git_fetch({\"tags\": true}) → git_tag_list()\n\n\
                 4. COMBINE PARAMETERS:\n\
                    Use pattern + sort + limit together for precise results\n\
                    git_tag_list({\"pattern\": \"v1.*\", \"sort\": \"-version\", \"limit\": 1})\n\n\
                 5. FILTER EFFICIENTLY:\n\
                    Use pattern to reduce results before sorting\n\
                    Better: pattern: \"v1.*\" then sort\n\
                    Worse: sort all tags then filter\n\n\
                 6. VERIFY OPERATIONS:\n\
                    List tags after create/delete to confirm success\n\n\
                 =============================================================================\n\
                 DECISION TREE\n\
                 =============================================================================\n\n\
                 Need to find latest version?\n\
                 → git_tag_list({\"sort\": \"-version\", \"limit\": 1})\n\n\
                 Need to see all v1.x versions?\n\
                 → git_tag_list({\"pattern\": \"v1.*\", \"sort\": \"version\"})\n\n\
                 Need to check if tag exists?\n\
                 → git_tag_list({\"pattern\": \"v1.0.0\"})\n\n\
                 Need to see version history?\n\
                 → git_tag_list({\"sort\": \"version\"})\n\n\
                 Need to see recent tags?\n\
                 → git_tag_list({\"sort\": \"-creatordate\", \"limit\": 10})\n\n\
                 Need pre-release versions?\n\
                 → git_tag_list({\"pattern\": \"*-beta*\", \"sort\": \"-version\"})\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 List all: git_tag_list({\"path\": \"/project\"})\n\
                 Latest: git_tag_list({\"path\": \"/project\", \"sort\": \"-version\", \"limit\": 1})\n\
                 Filter: git_tag_list({\"path\": \"/project\", \"pattern\": \"v1.*\"})\n\
                 History: git_tag_list({\"path\": \"/project\", \"sort\": \"version\"})\n\
                 Recent: git_tag_list({\"path\": \"/project\", \"sort\": \"-creatordate\", \"limit\": 5})\n\
                 Betas: git_tag_list({\"path\": \"/project\", \"pattern\": \"*-beta*\"})\n\n\
                 Remember: Use pattern to filter, sort to order, limit to cap results!",
            ),
        },
    ]
}
