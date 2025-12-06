//! Prompt messages for github_list_releases tool

use crate::tool::{PromptProvider, SealedPromptProvider};
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ListReleasesPromptArgs;

/// Prompt provider for list_releases tool
///
/// This is the ONLY way to provide prompts for list_releases - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ListReleasesPrompts;

// Implement the sealed trait to allow PromptProvider implementation
impl SealedPromptProvider for ListReleasesPrompts {}

impl PromptProvider for ListReleasesPrompts {
    type PromptArgs = ListReleasesPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("details") => prompt_details(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, details, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE GITHUB_LIST_RELEASES
// ============================================================================

/// Basic release listing patterns
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list releases in a GitHub repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_list_releases tool retrieves all releases from a repository, including release notes, assets, and version information.\n\n\
                 BASIC USAGE:\n\
                 1. List all releases:\n\
                    github_list_releases({\"owner\": \"rust-lang\", \"repo\": \"rust\"})\n\n\
                 2. Get first page (most recent releases):\n\
                    github_list_releases({\"owner\": \"tokio-rs\", \"repo\": \"tokio\", \"per_page\": 30})\n\n\
                 3. Paginated results:\n\
                    github_list_releases({\"owner\": \"microsoft\", \"repo\": \"vscode\", \"page\": 1, \"per_page\": 10})\n\n\
                 4. Get next page:\n\
                    github_list_releases({\"owner\": \"microsoft\", \"repo\": \"vscode\", \"page\": 2, \"per_page\": 10})\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (username or organization)\n\
                 - repo (required): Repository name\n\
                 - page (optional): Page number for pagination (default: 1)\n\
                 - per_page (optional): Results per page (max 100, default: 30)\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable:\n\
                 - Public repositories: No token required but rate limited\n\
                 - Private repositories: Token with repo scope required\n\
                 - Higher rate limits: Token provides 5,000 requests/hour vs 60 unauthenticated\n\n\
                 RESPONSE STRUCTURE:\n\
                 Returns JSON with:\n\
                 {\n\
                   \"releases\": [\n\
                     {\n\
                       \"id\": 12345678,\n\
                       \"tag_name\": \"v1.2.0\",\n\
                       \"name\": \"Release 1.2.0\",\n\
                       \"body\": \"## What's New\\n- Feature 1\\n- Bug fix 2\",\n\
                       \"draft\": false,\n\
                       \"prerelease\": false,\n\
                       \"created_at\": \"2024-01-15T10:30:00Z\",\n\
                       \"published_at\": \"2024-01-15T11:00:00Z\",\n\
                       \"author\": {\"login\": \"maintainer\"},\n\
                       \"html_url\": \"https://github.com/owner/repo/releases/tag/v1.2.0\",\n\
                       \"assets\": [\n\
                         {\n\
                           \"name\": \"binary-linux-amd64.tar.gz\",\n\
                           \"size\": 12345678,\n\
                           \"download_count\": 1523,\n\
                           \"browser_download_url\": \"https://...\"\n\
                         }\n\
                       ]\n\
                     }\n\
                   ],\n\
                   \"total_count\": 45,\n\
                   \"page\": 1,\n\
                   \"per_page\": 30\n\
                 }\n\n\
                 QUICK CHECKS:\n\
                 1. Check if project has releases:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\"})\n\
                    // Empty releases array means no releases published\n\n\
                 2. Get latest release info:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 1})\n\
                    // First item in releases array is most recent\n\n\
                 3. Check version history:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 10})\n\
                    // Last 10 releases in chronological order (newest first)\n\n\
                 SORTING:\n\
                 - Releases are returned in descending order by created_at\n\
                 - Most recent release appears first in the array\n\
                 - Draft releases included if you have push access\n\
                 - Pre-releases included by default\n\n\
                 FILTERING RESULTS:\n\
                 The API returns all types of releases:\n\
                 - Published releases: draft=false, prerelease=false\n\
                 - Draft releases: draft=true (only visible with push access)\n\
                 - Pre-releases: prerelease=true (beta, RC, alpha versions)\n\
                 - Filter in your code based on these flags\n\n\
                 COMMON PATTERNS:\n\
                 1. Latest stable release:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\"})\n\
                    // Find first release where draft=false and prerelease=false\n\n\
                 2. All versions in last year:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 100})\n\
                    // Filter by created_at or published_at timestamp\n\n\
                 3. Check for new releases:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 5})\n\
                    // Compare tag_name with current version\n\n\
                 ALTERNATIVE FOR LATEST ONLY:\n\
                 If you only need the latest release, use:\n\
                 github_get_latest_release({\"owner\": \"user\", \"repo\": \"project\"})\n\
                 This is more efficient than listing all releases.",
            ),
        },
    ]
}

/// Detailed release information and field meanings
fn prompt_details() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What information is included in release details?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Release responses contain comprehensive metadata about each release, including version info, release notes, assets, and publication status.\n\n\
                 RELEASE FIELDS:\n\
                 1. Identification:\n\
                    - id: Unique numeric identifier for the release\n\
                    - tag_name: Git tag associated with release (e.g., \"v1.2.0\")\n\
                    - name: Human-readable release title (e.g., \"Version 1.2.0\")\n\
                    - node_id: GraphQL node identifier\n\n\
                 2. Content:\n\
                    - body: Release notes in Markdown format\n\
                    - body_html: Rendered HTML version of release notes\n\
                    - body_text: Plain text version of release notes\n\n\
                 3. Status Flags:\n\
                    - draft: true if not yet published (only visible with push access)\n\
                    - prerelease: true for beta/RC/alpha versions\n\
                    - published: Opposite of draft flag\n\n\
                 4. Timestamps:\n\
                    - created_at: When release was created (ISO 8601 format)\n\
                    - published_at: When release was published (null for drafts)\n\n\
                 5. Author Info:\n\
                    - author: Object with login, id, avatar_url, etc.\n\
                    - Indicates who created the release\n\n\
                 6. URLs:\n\
                    - html_url: Web page for the release\n\
                    - url: API endpoint for this release\n\
                    - assets_url: API endpoint for release assets\n\
                    - upload_url: Endpoint for uploading new assets\n\
                    - tarball_url: Source code as .tar.gz\n\
                    - zipball_url: Source code as .zip\n\n\
                 ASSET DETAILS:\n\
                 Each release can have multiple downloadable assets:\n\
                 {\n\
                   \"assets\": [\n\
                     {\n\
                       \"id\": 87654321,\n\
                       \"name\": \"myapp-linux-amd64.tar.gz\",\n\
                       \"label\": \"Linux x86_64 binary\",\n\
                       \"content_type\": \"application/gzip\",\n\
                       \"size\": 12345678,\n\
                       \"download_count\": 1523,\n\
                       \"created_at\": \"2024-01-15T11:00:00Z\",\n\
                       \"updated_at\": \"2024-01-15T11:00:00Z\",\n\
                       \"browser_download_url\": \"https://github.com/.../myapp-linux-amd64.tar.gz\",\n\
                       \"url\": \"https://api.github.com/repos/.../assets/87654321\",\n\
                       \"uploader\": {\"login\": \"ci-bot\"}\n\
                     },\n\
                     {\n\
                       \"name\": \"myapp-darwin-arm64.tar.gz\",\n\
                       \"size\": 11234567,\n\
                       \"download_count\": 892,\n\
                       \"browser_download_url\": \"https://...\"\n\
                     },\n\
                     {\n\
                       \"name\": \"myapp-windows-amd64.zip\",\n\
                       \"size\": 13456789,\n\
                       \"download_count\": 2341,\n\
                       \"browser_download_url\": \"https://...\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 UNDERSTANDING RELEASE TYPES:\n\
                 1. Published Release (stable):\n\
                    draft: false\n\
                    prerelease: false\n\
                    published_at: Has timestamp\n\
                    → Ready for production use\n\n\
                 2. Pre-release (beta/RC):\n\
                    draft: false\n\
                    prerelease: true\n\
                    published_at: Has timestamp\n\
                    → Testing/preview version\n\n\
                 3. Draft Release:\n\
                    draft: true\n\
                    prerelease: false or true\n\
                    published_at: null\n\
                    → Not yet public (requires push access to see)\n\n\
                 RELEASE NOTES (body field):\n\
                 - Written in Markdown\n\
                 - Often includes:\n\
                   - What's New section\n\
                   - Breaking changes\n\
                   - Bug fixes\n\
                   - Contributors acknowledgment\n\
                   - Installation instructions\n\
                   - Upgrade notes\n\
                 - Can be auto-generated from commits\n\
                 - May reference issues/PRs with #123 syntax\n\n\
                 ASSET PATTERNS:\n\
                 Common naming conventions:\n\
                 - Platform: -linux-, -darwin-, -windows-\n\
                 - Architecture: -amd64, -arm64, -x86_64\n\
                 - Format: .tar.gz, .zip, .deb, .rpm, .dmg, .exe\n\
                 - Example: \"myapp-v1.2.0-linux-amd64.tar.gz\"\n\n\
                 FINDING ASSETS:\n\
                 1. List release:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 1})\n\n\
                 2. Check assets array:\n\
                    releases[0].assets.forEach(asset => {\n\
                      console.log(asset.name, asset.browser_download_url);\n\
                    });\n\n\
                 3. Filter by platform:\n\
                    const linuxAsset = releases[0].assets.find(a => \n\
                      a.name.includes('linux') && a.name.includes('amd64')\n\
                    );\n\n\
                 DOWNLOAD COUNTS:\n\
                 - download_count shows total downloads for each asset\n\
                 - Useful for popularity metrics\n\
                 - Includes all downloads since asset was uploaded\n\
                 - Can help identify most popular platforms\n\n\
                 TIMESTAMPS:\n\
                 - created_at: When release entry was created (may be draft)\n\
                 - published_at: When release went public (null for drafts)\n\
                 - All timestamps in ISO 8601 format (UTC)\n\
                 - Example: \"2024-01-15T10:30:00Z\"\n\
                 - Parse with standard date libraries\n\n\
                 SOURCE CODE ARCHIVES:\n\
                 Every release automatically includes:\n\
                 - tarball_url: .tar.gz of source at tag\n\
                 - zipball_url: .zip of source at tag\n\
                 - These are NOT in assets array\n\
                 - Generated automatically by GitHub\n\
                 - Always available even with no explicit assets",
            ),
        },
    ]
}

/// Release management workflows and patterns
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows for working with releases?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Release management involves tracking versions, analyzing changes, downloading assets, and planning updates.\n\n\
                 WORKFLOW 1: CHECK FOR UPDATES\n\
                 Use case: Determine if new version available\n\n\
                 1. Get latest release:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 1})\n\n\
                 2. Extract version from response:\n\
                    const latestVersion = releases[0].tag_name;  // e.g., \"v1.2.0\"\n\
                    const currentVersion = \"v1.1.5\";\n\n\
                 3. Compare versions:\n\
                    if (compareVersions(latestVersion, currentVersion) > 0) {\n\
                      console.log(\"Update available:\", latestVersion);\n\
                      console.log(\"Release notes:\", releases[0].body);\n\
                    }\n\n\
                 WORKFLOW 2: VERSION HISTORY ANALYSIS\n\
                 Use case: Review all releases and changes\n\n\
                 1. Get all recent releases:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 50})\n\n\
                 2. Filter by date range:\n\
                    const recentReleases = releases.filter(r => {\n\
                      const releaseDate = new Date(r.published_at);\n\
                      const sixMonthsAgo = new Date();\n\
                      sixMonthsAgo.setMonth(sixMonthsAgo.getMonth() - 6);\n\
                      return releaseDate > sixMonthsAgo;\n\
                    });\n\n\
                 3. Analyze release frequency:\n\
                    console.log(`${recentReleases.length} releases in last 6 months`);\n\
                    recentReleases.forEach(r => {\n\
                      console.log(`${r.tag_name}: ${r.published_at}`);\n\
                    });\n\n\
                 WORKFLOW 3: DOWNLOAD PLATFORM-SPECIFIC BINARY\n\
                 Use case: Get correct binary for user's platform\n\n\
                 1. List latest release:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"cli-tool\", \"per_page\": 1})\n\n\
                 2. Detect platform:\n\
                    const platform = process.platform;  // 'linux', 'darwin', 'win32'\n\
                    const arch = process.arch;          // 'x64', 'arm64'\n\n\
                 3. Find matching asset:\n\
                    const asset = releases[0].assets.find(a => {\n\
                      const name = a.name.toLowerCase();\n\
                      return name.includes(platform) && name.includes(arch);\n\
                    });\n\n\
                 4. Download asset:\n\
                    if (asset) {\n\
                      console.log(\"Download from:\", asset.browser_download_url);\n\
                      console.log(\"Size:\", (asset.size / 1024 / 1024).toFixed(2), \"MB\");\n\
                    }\n\n\
                 WORKFLOW 4: RELEASE PLANNING\n\
                 Use case: Decide next version number\n\n\
                 1. Get recent releases:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 10})\n\n\
                 2. Analyze version pattern:\n\
                    releases.forEach(r => {\n\
                      console.log(r.tag_name, r.prerelease ? '(pre-release)' : '(stable)');\n\
                    });\n\
                    // Latest stable: v1.4.2\n\
                    // Latest pre-release: v1.5.0-beta.1\n\n\
                 3. Plan next version:\n\
                    // For bug fix: v1.4.3\n\
                    // For feature: v1.5.0\n\
                    // For breaking: v2.0.0\n\n\
                 4. Check for conflicts:\n\
                    const nextVersion = \"v1.5.0\";\n\
                    const exists = releases.some(r => r.tag_name === nextVersion);\n\
                    if (exists) {\n\
                      console.error(\"Version already exists!\");\n\
                    }\n\n\
                 WORKFLOW 5: MIGRATION NOTES EXTRACTION\n\
                 Use case: Understand changes between versions\n\n\
                 1. Get releases in range:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 100})\n\n\
                 2. Find version range:\n\
                    const fromVersion = \"v1.0.0\";\n\
                    const toVersion = \"v2.0.0\";\n\
                    \n\
                    const startIdx = releases.findIndex(r => r.tag_name === toVersion);\n\
                    const endIdx = releases.findIndex(r => r.tag_name === fromVersion);\n\
                    const upgradeReleases = releases.slice(startIdx, endIdx + 1);\n\n\
                 3. Extract breaking changes:\n\
                    upgradeReleases.forEach(r => {\n\
                      const body = r.body.toLowerCase();\n\
                      if (body.includes('breaking') || body.includes('migration')) {\n\
                        console.log(`\\n${r.tag_name}:`);\n\
                        console.log(r.body);\n\
                      }\n\
                    });\n\n\
                 WORKFLOW 6: RELEASE METRICS\n\
                 Use case: Track release popularity and adoption\n\n\
                 1. Get all releases:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 100})\n\n\
                 2. Calculate total downloads:\n\
                    const totalDownloads = releases.reduce((sum, release) => {\n\
                      const releaseDownloads = release.assets.reduce((s, a) => s + a.download_count, 0);\n\
                      return sum + releaseDownloads;\n\
                    }, 0);\n\
                    console.log(\"Total downloads across all releases:\", totalDownloads);\n\n\
                 3. Find most popular version:\n\
                    const releasesByDownloads = releases.map(r => ({\n\
                      version: r.tag_name,\n\
                      downloads: r.assets.reduce((s, a) => s + a.download_count, 0)\n\
                    })).sort((a, b) => b.downloads - a.downloads);\n\
                    console.log(\"Most downloaded:\", releasesByDownloads[0]);\n\n\
                 4. Analyze platform distribution:\n\
                    const platformStats = {};\n\
                    releases[0].assets.forEach(asset => {\n\
                      const name = asset.name;\n\
                      let platform = 'other';\n\
                      if (name.includes('linux')) platform = 'linux';\n\
                      else if (name.includes('darwin') || name.includes('macos')) platform = 'macos';\n\
                      else if (name.includes('windows')) platform = 'windows';\n\
                      platformStats[platform] = (platformStats[platform] || 0) + asset.download_count;\n\
                    });\n\
                    console.log(\"Downloads by platform:\", platformStats);\n\n\
                 WORKFLOW 7: CHANGELOG GENERATION\n\
                 Use case: Build comprehensive changelog from releases\n\n\
                 1. Get all releases:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\"})\n\n\
                 2. Generate markdown:\n\
                    const changelog = releases.map(r => `\n\
                      ## ${r.name || r.tag_name} - ${new Date(r.published_at).toDateString()}\n\
                      ${r.body}\n\
                    `).join('\\n\\n');\n\n\
                 3. Save to file:\n\
                    fs.writeFileSync('CHANGELOG.md', changelog);\n\n\
                 WORKFLOW 8: STABILITY CHECK\n\
                 Use case: Verify release type before upgrading\n\n\
                 1. Get latest release:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 5})\n\n\
                 2. Find latest stable:\n\
                    const latestStable = releases.find(r => !r.prerelease && !r.draft);\n\
                    console.log(\"Latest stable:\", latestStable.tag_name);\n\n\
                 3. Check if pre-release exists:\n\
                    const latestPrerelease = releases.find(r => r.prerelease);\n\
                    if (latestPrerelease) {\n\
                      console.log(\"Beta available:\", latestPrerelease.tag_name);\n\
                      console.log(\"For early adopters only\");\n\
                    }\n\n\
                 PAGINATION STRATEGY:\n\
                 For repositories with many releases:\n\n\
                 1. Start with first page:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 100, \"page\": 1})\n\n\
                 2. Check if more pages needed:\n\
                    if (releases.length === 100) {\n\
                      // Fetch next page\n\
                      github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 100, \"page\": 2})\n\
                    }\n\n\
                 3. Aggregate results:\n\
                    let allReleases = [];\n\
                    let page = 1;\n\
                    while (true) {\n\
                      const result = github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 100, \"page\": page});\n\
                      allReleases.push(...result.releases);\n\
                      if (result.releases.length < 100) break;\n\
                      page++;\n\
                    }\n\n\
                 BEST PRACTICES:\n\
                 1. Cache release data to reduce API calls\n\
                 2. Use per_page: 1 when only need latest\n\
                 3. Check prerelease flag before recommending updates\n\
                 4. Parse tag_name for semantic version comparison\n\
                 5. Display release notes (body) to users\n\
                 6. Verify asset checksums if provided\n\
                 7. Handle missing assets gracefully\n\
                 8. Monitor rate limits for high-frequency checks\n\
                 9. Filter drafts in UI (only visible to maintainers)\n\
                 10. Sort by published_at for chronological order",
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
                "Give me a complete guide to using github_list_releases effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_list_releases tool provides comprehensive access to repository releases, including version information, release notes, downloadable assets, and metadata.\n\n\
                 =============================================================================\n\
                 OVERVIEW\n\
                 =============================================================================\n\n\
                 PURPOSE:\n\
                 - List all releases in a repository\n\
                 - Access release notes and changelogs\n\
                 - Find downloadable assets (binaries, packages)\n\
                 - Track version history\n\
                 - Identify stable vs pre-release versions\n\
                 - Monitor download statistics\n\n\
                 WHEN TO USE:\n\
                 - Checking for software updates\n\
                 - Downloading platform-specific binaries\n\
                 - Analyzing release history\n\
                 - Building dependency version matrices\n\
                 - Generating changelogs\n\
                 - Tracking release frequency\n\
                 - Migration planning between versions\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 SIMPLE LIST:\n\
                 github_list_releases({\"owner\": \"rust-lang\", \"repo\": \"rust\"})\n\n\
                 WITH PAGINATION:\n\
                 github_list_releases({\n\
                   \"owner\": \"microsoft\",\n\
                   \"repo\": \"vscode\",\n\
                   \"per_page\": 30,\n\
                   \"page\": 1\n\
                 })\n\n\
                 PARAMETERS:\n\
                 - owner (required): Repository owner (user or organization)\n\
                 - repo (required): Repository name\n\
                 - page (optional): Page number for pagination (default: 1)\n\
                 - per_page (optional): Results per page (1-100, default: 30)\n\n\
                 =============================================================================\n\
                 RESPONSE STRUCTURE\n\
                 =============================================================================\n\n\
                 TOP LEVEL:\n\
                 {\n\
                   \"releases\": [...],      // Array of release objects\n\
                   \"total_count\": 45,      // Total releases available\n\
                   \"page\": 1,              // Current page number\n\
                   \"per_page\": 30          // Results per page\n\
                 }\n\n\
                 RELEASE OBJECT:\n\
                 {\n\
                   \"id\": 12345678,\n\
                   \"tag_name\": \"v1.2.0\",\n\
                   \"name\": \"Release 1.2.0\",\n\
                   \"body\": \"## What's New\\n- Feature 1\\n- Bug fix 2\",\n\
                   \"draft\": false,\n\
                   \"prerelease\": false,\n\
                   \"created_at\": \"2024-01-15T10:30:00Z\",\n\
                   \"published_at\": \"2024-01-15T11:00:00Z\",\n\
                   \"author\": {\n\
                     \"login\": \"maintainer\",\n\
                     \"id\": 123456,\n\
                     \"avatar_url\": \"https://...\"\n\
                   },\n\
                   \"html_url\": \"https://github.com/owner/repo/releases/tag/v1.2.0\",\n\
                   \"assets\": [...]         // Downloadable files (see below)\n\
                 }\n\n\
                 ASSET OBJECT:\n\
                 {\n\
                   \"id\": 87654321,\n\
                   \"name\": \"myapp-linux-amd64.tar.gz\",\n\
                   \"label\": \"Linux x86_64 binary\",\n\
                   \"content_type\": \"application/gzip\",\n\
                   \"size\": 12345678,\n\
                   \"download_count\": 1523,\n\
                   \"created_at\": \"2024-01-15T11:00:00Z\",\n\
                   \"browser_download_url\": \"https://github.com/.../myapp-linux-amd64.tar.gz\"\n\
                 }\n\n\
                 =============================================================================\n\
                 RELEASE TYPES\n\
                 =============================================================================\n\n\
                 1. PUBLISHED RELEASE (Stable):\n\
                    draft: false\n\
                    prerelease: false\n\
                    → Production-ready version\n\
                    → Recommended for general use\n\
                    → Appears in \"Latest\" section on GitHub\n\n\
                 2. PRE-RELEASE (Beta/RC/Alpha):\n\
                    draft: false\n\
                    prerelease: true\n\
                    → Testing or preview version\n\
                    → May have bugs or breaking changes\n\
                    → For early adopters\n\
                    → Examples: v2.0.0-beta.1, v1.5.0-rc.2\n\n\
                 3. DRAFT RELEASE:\n\
                    draft: true\n\
                    → Not yet published\n\
                    → Only visible to users with push access\n\
                    → Work in progress\n\
                    → Will not appear for most users\n\n\
                 FILTERING BY TYPE:\n\
                 // Latest stable only\n\
                 const stable = releases.find(r => !r.prerelease && !r.draft);\n\n\
                 // All stable releases\n\
                 const stableReleases = releases.filter(r => !r.prerelease && !r.draft);\n\n\
                 // Pre-releases only\n\
                 const prereleases = releases.filter(r => r.prerelease);\n\n\
                 =============================================================================\n\
                 WORKING WITH ASSETS\n\
                 =============================================================================\n\n\
                 COMMON ASSET PATTERNS:\n\
                 Platform identifiers:\n\
                 - Linux: linux, x86_64-unknown-linux\n\
                 - macOS: darwin, macos, x86_64-apple-darwin, aarch64-apple-darwin\n\
                 - Windows: windows, win32, x86_64-pc-windows\n\n\
                 Architecture identifiers:\n\
                 - x86_64, amd64 (Intel/AMD 64-bit)\n\
                 - aarch64, arm64 (ARM 64-bit)\n\
                 - i686, x86 (32-bit)\n\n\
                 File extensions:\n\
                 - .tar.gz, .tgz (compressed archive)\n\
                 - .zip (Windows archive)\n\
                 - .deb (Debian/Ubuntu package)\n\
                 - .rpm (RedHat/Fedora package)\n\
                 - .dmg (macOS disk image)\n\
                 - .exe, .msi (Windows installer)\n\n\
                 FINDING PLATFORM-SPECIFIC ASSETS:\n\
                 1. Get release:\n\
                    const release = releases[0];\n\n\
                 2. Search assets:\n\
                    const linuxBinary = release.assets.find(a => \n\
                      a.name.includes('linux') && a.name.includes('x86_64') && a.name.endsWith('.tar.gz')\n\
                    );\n\n\
                 3. Download URL:\n\
                    console.log(linuxBinary.browser_download_url);\n\
                    console.log(\"Size:\", (linuxBinary.size / 1024 / 1024).toFixed(2), \"MB\");\n\
                    console.log(\"Downloads:\", linuxBinary.download_count);\n\n\
                 SOURCE CODE ARCHIVES:\n\
                 Every release includes automatic source archives:\n\
                 - Source code (zip): Automatically generated\n\
                 - Source code (tar.gz): Automatically generated\n\
                 - These link to the tagged commit\n\
                 - Not included in assets array\n\n\
                 =============================================================================\n\
                 PAGINATION\n\
                 =============================================================================\n\n\
                 STRATEGY:\n\
                 1. Start with reasonable page size:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 30, \"page\": 1})\n\n\
                 2. Check if more pages exist:\n\
                    if (releases.length === per_page) {\n\
                      // Likely more pages available\n\
                    }\n\n\
                 3. Fetch subsequent pages:\n\
                    github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 30, \"page\": 2})\n\n\
                 OPTIMIZATION:\n\
                 - Use per_page: 1 if only need latest\n\
                 - Use per_page: 100 to minimize API calls\n\
                 - Cache results to avoid repeated calls\n\
                 - Check total_count to know how many pages\n\n\
                 =============================================================================\n\
                 AUTHENTICATION\n\
                 =============================================================================\n\n\
                 GITHUB_TOKEN:\n\
                 Required for:\n\
                 - Private repositories\n\
                 - Higher rate limits (5,000/hour vs 60/hour)\n\
                 - Seeing draft releases\n\
                 - Organization repositories with access control\n\n\
                 Scopes needed:\n\
                 - repo (for private repositories)\n\
                 - public_repo (for public repositories)\n\n\
                 Without token:\n\
                 - Public repositories accessible\n\
                 - Rate limited to 60 requests/hour\n\
                 - No draft releases visible\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 CHECK FOR UPDATES:\n\
                 1. github_list_releases({\"owner\": \"user\", \"repo\": \"app\", \"per_page\": 1})\n\
                 2. Compare releases[0].tag_name with current version\n\
                 3. Show update notification if newer\n\n\
                 DOWNLOAD LATEST BINARY:\n\
                 1. github_list_releases({\"owner\": \"user\", \"repo\": \"cli\", \"per_page\": 1})\n\
                 2. Find asset matching user's platform\n\
                 3. Download from browser_download_url\n\n\
                 VERSION HISTORY:\n\
                 1. github_list_releases({\"owner\": \"user\", \"repo\": \"lib\", \"per_page\": 50})\n\
                 2. Display list of versions with dates\n\
                 3. Show release notes for each\n\n\
                 MIGRATION PLANNING:\n\
                 1. github_list_releases({\"owner\": \"user\", \"repo\": \"framework\"})\n\
                 2. Find releases between current and target version\n\
                 3. Extract breaking changes from release notes\n\
                 4. Build migration checklist\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. RATE LIMITING:\n\
                    - Cache release data\n\
                    - Use conditional requests (ETags)\n\
                    - Implement exponential backoff\n\
                    - Monitor X-RateLimit-Remaining header\n\n\
                 2. VERSION COMPARISON:\n\
                    - Parse semantic versions properly\n\
                    - Handle pre-release identifiers\n\
                    - Consider build metadata\n\
                    - Use version comparison libraries\n\n\
                 3. ASSET SELECTION:\n\
                    - Detect user's platform/architecture\n\
                    - Provide fallback options\n\
                    - Verify checksums if available\n\
                    - Handle missing assets gracefully\n\n\
                 4. ERROR HANDLING:\n\
                    - Check for empty releases array\n\
                    - Handle network failures\n\
                    - Deal with missing permissions\n\
                    - Validate response structure\n\n\
                 5. USER EXPERIENCE:\n\
                    - Show release dates\n\
                    - Format file sizes\n\
                    - Highlight breaking changes\n\
                    - Link to full release notes\n\
                    - Indicate pre-release status clearly\n\n\
                 =============================================================================\n\
                 ALTERNATIVE TOOLS\n\
                 =============================================================================\n\n\
                 FOR LATEST RELEASE ONLY:\n\
                 github_get_latest_release({\"owner\": \"user\", \"repo\": \"project\"})\n\
                 - More efficient than listing all\n\
                 - Gets latest non-prerelease\n\
                 - Recommended when only need current version\n\n\
                 FOR CREATING RELEASES:\n\
                 github_create_release({\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"tag_name\": \"v1.3.0\",\n\
                   \"name\": \"Version 1.3.0\",\n\
                   \"body\": \"Release notes...\"\n\
                 })\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 List all: github_list_releases({\"owner\": \"user\", \"repo\": \"project\"})\n\
                 Latest only: github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"per_page\": 1})\n\
                 Paginated: github_list_releases({\"owner\": \"user\", \"repo\": \"project\", \"page\": 2, \"per_page\": 50})\n\
                 Find stable: releases.find(r => !r.prerelease && !r.draft)\n\
                 Find assets: release.assets.filter(a => a.name.includes('linux'))\n\
                 Check drafts: releases.filter(r => r.draft) // Only visible with push access",
            ),
        },
    ]
}
