//! Prompt messages for github_dependabot_alerts tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GithubDependabotAlertsPromptArgs;

/// Prompt provider for github_dependabot_alerts tool
///
/// This is the ONLY way to provide prompts for github_dependabot_alerts - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GithubDependabotAlertsPrompts;

impl PromptProvider for GithubDependabotAlertsPrompts {
    type PromptArgs = GithubDependabotAlertsPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("filtering") => prompt_filtering(),
            Some("remediation") => prompt_remediation(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, filtering, remediation, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE DEPENDABOT ALERTS
// ============================================================================

/// Basic alert listing and understanding response structure
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list and understand Dependabot security alerts?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_dependabot_alerts tool retrieves dependency vulnerability alerts from GitHub's Dependabot. Here's how to list and understand alerts:\n\n\
                 BASIC ALERT LISTING:\n\
                 1. Get all alerts for a repository:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\"\n\
                    })\n\n\
                 2. Get alerts for an organization:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"myorg\",\n\
                        \"repo\": \"backend\"\n\
                    })\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"alerts\": [\n\
                     {\n\
                       \"number\": 1,\n\
                       \"state\": \"open\",\n\
                       \"dependency\": {\n\
                         \"package\": {\"name\": \"lodash\"},\n\
                         \"manifest_path\": \"package.json\"\n\
                       },\n\
                       \"security_advisory\": {\n\
                         \"ghsa_id\": \"GHSA-xxxx-yyyy-zzzz\",\n\
                         \"cve_id\": \"CVE-2021-12345\",\n\
                         \"severity\": \"high\",\n\
                         \"summary\": \"Prototype Pollution\",\n\
                         \"description\": \"Versions of lodash prior to 4.17.21 are vulnerable...\",\n\
                         \"published_at\": \"2021-02-15T00:00:00Z\"\n\
                       },\n\
                       \"security_vulnerability\": {\n\
                         \"vulnerable_version_range\": \"< 4.17.21\",\n\
                         \"first_patched_version\": \"4.17.21\"\n\
                       },\n\
                       \"created_at\": \"2021-03-01T10:30:00Z\",\n\
                       \"updated_at\": \"2021-03-01T10:30:00Z\",\n\
                       \"dismissed_at\": null,\n\
                       \"dismissed_by\": null,\n\
                       \"dismissed_reason\": null\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 UNDERSTANDING ALERT FIELDS:\n\
                 - number: Unique alert ID for this repository\n\
                 - state: \"open\" (needs attention), \"dismissed\" (acknowledged), \"fixed\" (resolved)\n\
                 - dependency.package.name: Name of vulnerable package\n\
                 - dependency.manifest_path: File where dependency is declared (package.json, Cargo.toml, etc.)\n\
                 - security_advisory.severity: \"critical\", \"high\", \"medium\", or \"low\"\n\
                 - security_advisory.summary: Short description of vulnerability\n\
                 - security_advisory.ghsa_id: GitHub Security Advisory ID\n\
                 - security_advisory.cve_id: Common Vulnerabilities and Exposures ID (if assigned)\n\
                 - security_vulnerability.vulnerable_version_range: Which versions are affected\n\
                 - security_vulnerability.first_patched_version: Version that fixes the vulnerability\n\n\
                 COMMON SCENARIOS:\n\
                 1. Check a project's security status:\n\
                    github_dependabot_alerts({\"owner\": \"myteam\", \"repo\": \"webapp\"})\n\
                    Review the alerts array to see all vulnerabilities\n\n\
                 2. Understand what needs fixing:\n\
                    Look for alerts with state: \"open\"\n\
                    Check security_advisory.severity for priority\n\
                    Note first_patched_version for the fix\n\n\
                 3. Track resolved issues:\n\
                    Alerts with state: \"fixed\" show previously resolved vulnerabilities\n\
                    Useful for security audit trails\n\n\
                 ALERT STATES EXPLAINED:\n\
                 - open: Vulnerability exists and needs attention\n\
                 - dismissed: Maintainer acknowledged but chose not to fix (see dismissed_reason)\n\
                 - fixed: Dependency has been updated to a patched version\n\n\
                 SEVERITY LEVELS:\n\
                 - critical: Immediate risk, actively exploited vulnerabilities\n\
                 - high: Significant risk, should be fixed urgently\n\
                 - medium: Moderate risk, plan to fix in near term\n\
                 - low: Minor risk, fix when convenient\n\n\
                 BEST PRACTICES:\n\
                 1. Check alerts regularly (weekly or as part of CI/CD)\n\
                 2. Prioritize by severity: critical and high first\n\
                 3. Read the full advisory description to understand impact\n\
                 4. Check if your code actually uses the vulnerable functionality\n\
                 5. Test after updating dependencies\n\
                 6. Document dismissal reasons if not fixing\n\n\
                 READING VULNERABILITIES:\n\
                 When you see an alert:\n\
                 1. Check the package name and manifest path\n\
                 2. Note the severity level\n\
                 3. Read the summary to understand the vulnerability type\n\
                 4. Check vulnerable_version_range to confirm you're affected\n\
                 5. Note first_patched_version for the upgrade target\n\
                 6. Review the full description for exploit details and impact\n\n\
                 EXAMPLE WORKFLOW:\n\
                 # Get alerts\n\
                 github_dependabot_alerts({\"owner\": \"acme\", \"repo\": \"api\"})\n\
                 \n\
                 # Response shows:\n\
                 # Alert #1: lodash < 4.17.21 (high severity)\n\
                 # Alert #2: axios < 0.21.1 (moderate severity)\n\
                 # Alert #3: minimist < 1.2.6 (critical severity)\n\
                 \n\
                 # Action plan:\n\
                 # 1. Fix critical first (minimist)\n\
                 # 2. Then high (lodash)\n\
                 # 3. Then moderate (axios)",
            ),
        },
    ]
}

/// Filtering alerts by severity, state, and other criteria
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I filter Dependabot alerts by severity, state, or other criteria?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "You can filter Dependabot alerts to focus on specific vulnerabilities using various parameters. Here's how:\n\n\
                 FILTERING BY SEVERITY:\n\
                 1. Critical vulnerabilities only:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"severity\": \"critical\"\n\
                    })\n\n\
                 2. High severity vulnerabilities:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"severity\": \"high\"\n\
                    })\n\n\
                 3. Multiple severity levels:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"severity\": \"high,critical\"\n\
                    })\n\
                    Returns both high and critical alerts\n\n\
                 FILTERING BY STATE:\n\
                 1. Open alerts only (needs attention):\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"state\": \"open\"\n\
                    })\n\n\
                 2. Dismissed alerts (to review decisions):\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"state\": \"dismissed\"\n\
                    })\n\n\
                 3. Fixed alerts (audit trail):\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"state\": \"fixed\"\n\
                    })\n\n\
                 COMBINING FILTERS:\n\
                 1. Open critical vulnerabilities:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"state\": \"open\",\n\
                        \"severity\": \"critical\"\n\
                    })\n\
                    Perfect for urgent security review\n\n\
                 2. Open high and critical:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"state\": \"open\",\n\
                        \"severity\": \"high,critical\"\n\
                    })\n\
                    Focus on most important unfixed issues\n\n\
                 3. Recently fixed high severity:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"state\": \"fixed\",\n\
                        \"severity\": \"high\"\n\
                    })\n\
                    Review what was recently patched\n\n\
                 FILTERING BY PACKAGE ECOSYSTEM:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"ecosystem\": \"npm\"\n\
                 })\n\
                 Supported ecosystems: npm, pip, composer, nuget, maven, rubygems, cargo\n\n\
                 FILTERING BY PACKAGE NAME:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"package\": \"lodash\"\n\
                 })\n\
                 See all alerts for a specific dependency\n\n\
                 FILTERING BY MANIFEST:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"manifest\": \"package.json\"\n\
                 })\n\
                 Focus on vulnerabilities in a specific dependency file\n\n\
                 SEVERITY LEVEL GUIDE:\n\
                 - critical: CVSS score 9.0-10.0\n\
                   * Immediate action required\n\
                   * Often actively exploited\n\
                   * Can lead to complete system compromise\n\
                 \n\
                 - high: CVSS score 7.0-8.9\n\
                   * Priority fix within days\n\
                   * Significant security risk\n\
                   * Can lead to data breach or service disruption\n\
                 \n\
                 - medium: CVSS score 4.0-6.9\n\
                   * Plan to fix within weeks\n\
                   * Moderate security risk\n\
                   * May require specific conditions to exploit\n\
                 \n\
                 - low: CVSS score 0.1-3.9\n\
                   * Fix when convenient\n\
                   * Minor security risk\n\
                   * Limited impact or difficult to exploit\n\n\
                 PRACTICAL FILTERING WORKFLOWS:\n\
                 1. Daily security check (focus on urgent):\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"production-app\",\n\
                        \"state\": \"open\",\n\
                        \"severity\": \"critical,high\"\n\
                    })\n\n\
                 2. Weekly security review (all open):\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"production-app\",\n\
                        \"state\": \"open\"\n\
                    })\n\n\
                 3. Audit dismissed alerts:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"production-app\",\n\
                        \"state\": \"dismissed\"\n\
                    })\n\
                    Review dismissal reasons to ensure they're still valid\n\n\
                 4. Package-specific investigation:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"production-app\",\n\
                        \"package\": \"express\"\n\
                    })\n\
                    See all historical alerts for a dependency\n\n\
                 5. Ecosystem-specific review:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"monorepo\",\n\
                        \"ecosystem\": \"npm\",\n\
                        \"state\": \"open\"\n\
                    })\n\
                    Focus on one language's dependencies\n\n\
                 PAGINATION:\n\
                 For repositories with many alerts:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"per_page\": 50,\n\
                     \"page\": 1\n\
                 })\n\
                 Default: 30 alerts per page\n\
                 Maximum: 100 alerts per page\n\n\
                 SORTING RESULTS:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"sort\": \"created\",\n\
                     \"direction\": \"desc\"\n\
                 })\n\
                 Sort options: created, updated\n\
                 Direction: asc (oldest first), desc (newest first)\n\n\
                 BEST PRACTICES:\n\
                 1. Start with critical and high severity for triage\n\
                 2. Filter by state: \"open\" for actionable items\n\
                 3. Use ecosystem filter in polyglot projects\n\
                 4. Review dismissed alerts periodically\n\
                 5. Filter by package to track repeat offenders\n\
                 6. Sort by updated to see recently changed alerts",
            ),
        },
    ]
}

/// Fixing vulnerabilities and remediation workflows
fn prompt_remediation() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I fix vulnerabilities reported by Dependabot alerts?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Fixing Dependabot vulnerabilities involves identifying the issue, updating dependencies, and managing alerts. Here's the complete remediation workflow:\n\n\
                 STEP 1: IDENTIFY VULNERABILITIES\n\
                 Get open alerts:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"state\": \"open\"\n\
                 })\n\n\
                 STEP 2: REVIEW SPECIFIC ALERT\n\
                 Get details for a specific alert:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"alert_number\": 1\n\
                 })\n\
                 \n\
                 Response shows:\n\
                 - Vulnerable package and version\n\
                 - First patched version\n\
                 - Manifest file location\n\
                 - Vulnerability description\n\n\
                 STEP 3: UPDATE DEPENDENCY\n\
                 Based on the ecosystem, update the dependency file:\n\n\
                 FOR NPM (package.json):\n\
                 1. Read current version:\n\
                    fs_read_file({\"path\": \"/project/package.json\"})\n\
                 \n\
                 2. Update to patched version:\n\
                    fs_edit_block({\n\
                        \"path\": \"/project/package.json\",\n\
                        \"old_string\": \"\\\"lodash\\\": \\\"^4.17.10\\\"\",\n\
                        \"new_string\": \"\\\"lodash\\\": \\\"^4.17.21\\\"\"\n\
                    })\n\
                 \n\
                 3. Update lock file:\n\
                    terminal({\"command\": \"cd /project && npm install\"})\n\n\
                 FOR PYTHON (requirements.txt):\n\
                 1. Update dependency:\n\
                    fs_edit_block({\n\
                        \"path\": \"/project/requirements.txt\",\n\
                        \"old_string\": \"django==3.2.0\",\n\
                        \"new_string\": \"django==3.2.13\"\n\
                    })\n\
                 \n\
                 2. Install updated version:\n\
                    terminal({\"command\": \"cd /project && pip install -r requirements.txt\"})\n\n\
                 FOR RUST (Cargo.toml):\n\
                 1. Update dependency:\n\
                    fs_edit_block({\n\
                        \"path\": \"/project/Cargo.toml\",\n\
                        \"old_string\": \"tokio = \\\"1.0.0\\\"\",\n\
                        \"new_string\": \"tokio = \\\"1.18.0\\\"\"\n\
                    })\n\
                 \n\
                 2. Update lock file:\n\
                    terminal({\"command\": \"cd /project && cargo update -p tokio\"})\n\n\
                 FOR RUBY (Gemfile):\n\
                 1. Update gem version:\n\
                    fs_edit_block({\n\
                        \"path\": \"/project/Gemfile\",\n\
                        \"old_string\": \"gem 'rails', '~> 6.0.0'\",\n\
                        \"new_string\": \"gem 'rails', '~> 6.1.6'\"\n\
                    })\n\
                 \n\
                 2. Update dependencies:\n\
                    terminal({\"command\": \"cd /project && bundle update rails\"})\n\n\
                 STEP 4: TEST THE FIX\n\
                 1. Run tests:\n\
                    terminal({\"command\": \"cd /project && npm test\"})\n\
                 \n\
                 2. Run build:\n\
                    terminal({\"command\": \"cd /project && npm run build\"})\n\
                 \n\
                 3. Check for breaking changes:\n\
                    Review changelog for the updated dependency\n\n\
                 STEP 5: COMMIT AND PUSH\n\
                 terminal({\"command\": \"cd /project && git add . && git commit -m \\\"fix: update lodash to 4.17.21 (security)\\\" && git push\"})\n\n\
                 STEP 6: VERIFY ALERT CLOSES\n\
                 After GitHub detects the fix:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"alert_number\": 1\n\
                 })\n\
                 Alert state should change to \"fixed\"\n\n\
                 ALTERNATIVE: DISMISS IF NOT APPLICABLE\n\
                 If the vulnerability doesn't apply to your use case:\n\
                 \n\
                 github_update_dependabot_alert({\n\
                     \"owner\": \"user\",\n\
                     \"repo\": \"project\",\n\
                     \"alert_number\": 1,\n\
                     \"state\": \"dismissed\",\n\
                     \"dismissed_reason\": \"tolerable_risk\",\n\
                     \"dismissed_comment\": \"We don't use the vulnerable functionality\"\n\
                 })\n\
                 \n\
                 Valid dismissal reasons:\n\
                 - no_bandwidth: Will fix later\n\
                 - tolerable_risk: Acceptable risk level\n\
                 - inaccurate: False positive\n\
                 - not_used: Dependency not actually used in production\n\n\
                 HANDLING BREAKING CHANGES:\n\
                 If patched version has breaking changes:\n\
                 \n\
                 1. Check if minor update available:\n\
                    Sometimes a patch exists in your current major version\n\
                 \n\
                 2. Review migration guide:\n\
                    Read the package's changelog and migration documentation\n\
                 \n\
                 3. Create feature branch:\n\
                    terminal({\"command\": \"cd /project && git checkout -b security/update-lodash\"})\n\
                 \n\
                 4. Update and fix breaking changes:\n\
                    Update dependency, fix code, test thoroughly\n\
                 \n\
                 5. Create pull request:\n\
                    github_create_pull_request({\n\
                        \"owner\": \"user\",\n\
                        \"repo\": \"project\",\n\
                        \"title\": \"Security: Update lodash to fix vulnerability\",\n\
                        \"body\": \"Fixes Dependabot alert #1\",\n\
                        \"head\": \"security/update-lodash\",\n\
                        \"base\": \"main\"\n\
                    })\n\n\
                 COMPLETE REMEDIATION WORKFLOW:\n\
                 1. Get critical/high alerts:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\", \"repo\": \"app\",\n\
                        \"state\": \"open\", \"severity\": \"critical,high\"\n\
                    })\n\
                 \n\
                 2. For each alert:\n\
                    a. Review vulnerability details\n\
                    b. Check if code uses vulnerable functionality\n\
                    c. Identify patched version\n\
                    d. Update dependency file\n\
                    e. Run package manager to update lock file\n\
                    f. Run tests\n\
                    g. Commit and push\n\
                 \n\
                 3. Verify fixes:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\", \"repo\": \"app\",\n\
                        \"state\": \"open\"\n\
                    })\n\
                    Should show fewer alerts\n\n\
                 BULK UPDATES:\n\
                 For multiple alerts in same manifest:\n\
                 \n\
                 1. Update all vulnerable packages:\n\
                    fs_edit_block({\"path\": \"/project/package.json\", \"old_string\": \"...\", \"new_string\": \"...\"})\n\
                    fs_edit_block({\"path\": \"/project/package.json\", \"old_string\": \"...\", \"new_string\": \"...\"})\n\
                 \n\
                 2. Single install:\n\
                    terminal({\"command\": \"cd /project && npm install\"})\n\
                 \n\
                 3. Test once:\n\
                    terminal({\"command\": \"cd /project && npm test\"})\n\
                 \n\
                 4. Commit all fixes:\n\
                    terminal({\"command\": \"cd /project && git add . && git commit -m \\\"fix: update multiple dependencies for security\\\"\"})\n\n\
                 AUTOMATED REMEDIATION:\n\
                 Enable Dependabot security updates:\n\
                 - GitHub can auto-create PRs for security patches\n\
                 - Review and merge these PRs after CI passes\n\
                 - Reduces manual remediation work\n\n\
                 BEST PRACTICES:\n\
                 1. Fix critical and high severity first\n\
                 2. Always test after updating\n\
                 3. Update to the first patched version or later\n\
                 4. Document dismissal reasons clearly\n\
                 5. Create PRs for major version updates\n\
                 6. Enable automated security updates when possible\n\
                 7. Review changelogs for breaking changes\n\
                 8. Keep dependencies reasonably up to date to minimize breaking changes",
            ),
        },
    ]
}

/// Security workflows and best practices
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are the best workflows for managing Dependabot security alerts?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Effective security workflows help teams stay on top of vulnerabilities. Here are proven patterns for managing Dependabot alerts:\n\n\
                 WORKFLOW 1: DAILY CRITICAL ALERT CHECK\n\
                 Run every morning or as part of daily standup:\n\
                 \n\
                 1. Check for critical vulnerabilities:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"production-app\",\n\
                        \"state\": \"open\",\n\
                        \"severity\": \"critical\"\n\
                    })\n\
                 \n\
                 2. If any found:\n\
                    - Create immediate action plan\n\
                    - Assign to developer\n\
                    - Fix within 24 hours\n\
                    - Deploy hotfix if necessary\n\
                 \n\
                 3. Check high severity:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"production-app\",\n\
                        \"state\": \"open\",\n\
                        \"severity\": \"high\"\n\
                    })\n\
                    Plan fixes for current sprint\n\n\
                 WORKFLOW 2: WEEKLY SECURITY REVIEW\n\
                 Comprehensive review every week:\n\
                 \n\
                 1. Get all open alerts:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"production-app\",\n\
                        \"state\": \"open\"\n\
                    })\n\
                 \n\
                 2. Triage by severity:\n\
                    - Critical: Immediate fix\n\
                    - High: Fix this sprint\n\
                    - Medium: Fix next sprint\n\
                    - Low: Fix when convenient\n\
                 \n\
                 3. Review dismissed alerts:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"production-app\",\n\
                        \"state\": \"dismissed\"\n\
                    })\n\
                    Verify dismissal reasons still valid\n\
                 \n\
                 4. Document decisions in team meeting notes\n\n\
                 WORKFLOW 3: PRIORITY TRIAGE PROCESS\n\
                 When multiple alerts appear:\n\
                 \n\
                 1. Critical alerts first:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\", \"repo\": \"app\",\n\
                        \"state\": \"open\", \"severity\": \"critical\"\n\
                    })\n\
                    Fix immediately, deploy ASAP\n\
                 \n\
                 2. High severity next:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\", \"repo\": \"app\",\n\
                        \"state\": \"open\", \"severity\": \"high\"\n\
                    })\n\
                    Schedule for this week\n\
                 \n\
                 3. Medium and low:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\", \"repo\": \"app\",\n\
                        \"state\": \"open\", \"severity\": \"medium,low\"\n\
                    })\n\
                    Add to backlog\n\n\
                 WORKFLOW 4: CONTINUOUS INTEGRATION CHECK\n\
                 Add to CI/CD pipeline:\n\
                 \n\
                 1. In CI script, check for critical alerts:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\", \"repo\": \"app\",\n\
                        \"state\": \"open\", \"severity\": \"critical\"\n\
                    })\n\
                 \n\
                 2. Fail build if critical alerts found:\n\
                    - Prevents deploying known critical vulnerabilities\n\
                    - Forces team to address immediately\n\
                 \n\
                 3. Warn on high severity:\n\
                    - Allow build but notify team\n\
                    - Track in metrics\n\n\
                 WORKFLOW 5: SECURITY AUDIT\n\
                 Quarterly comprehensive review:\n\
                 \n\
                 1. All repositories in organization:\n\
                    For each repo:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"org\",\n\
                        \"repo\": \"repo-name\",\n\
                        \"state\": \"open\"\n\
                    })\n\
                 \n\
                 2. Generate security report:\n\
                    - Total open alerts by severity\n\
                    - Oldest unfixed alerts\n\
                    - Repositories with most alerts\n\
                    - Trends over time\n\
                 \n\
                 3. Review and update security policies:\n\
                    - SLA for fixing vulnerabilities\n\
                    - Dismissal criteria\n\
                    - Automated update settings\n\n\
                 WORKFLOW 6: NEW ALERT RESPONSE\n\
                 When GitHub notifies of new alert:\n\
                 \n\
                 1. Review alert details:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"app\",\n\
                        \"sort\": \"created\",\n\
                        \"direction\": \"desc\"\n\
                    })\n\
                 \n\
                 2. Assess impact:\n\
                    - Is the vulnerable code path used?\n\
                    - What's the blast radius?\n\
                    - Are we exposed to the internet?\n\
                 \n\
                 3. Categorize urgency:\n\
                    - Critical + production: Hotfix\n\
                    - Critical + staging: This sprint\n\
                    - High: This sprint\n\
                    - Medium/Low: Backlog\n\
                 \n\
                 4. Take action:\n\
                    - Fix immediately or\n\
                    - Schedule fix or\n\
                    - Dismiss with reason\n\n\
                 WORKFLOW 7: DEPENDENCY UPDATE STRATEGY\n\
                 Proactive approach:\n\
                 \n\
                 1. Regular dependency updates:\n\
                    - Weekly or bi-weekly update PRs\n\
                    - Reduces severity of security updates\n\
                    - Smaller, easier changes\n\
                 \n\
                 2. Enable Dependabot automatic PRs:\n\
                    - Auto-merge for patch versions (if tests pass)\n\
                    - Review minor versions\n\
                    - Manually handle major versions\n\
                 \n\
                 3. Monitor for new alerts:\n\
                    github_dependabot_alerts({\n\
                        \"owner\": \"team\",\n\
                        \"repo\": \"app\",\n\
                        \"state\": \"open\"\n\
                    })\n\
                    Regular checks keep count low\n\n\
                 WORKFLOW 8: MULTI-REPOSITORY MANAGEMENT\n\
                 For organizations with many repos:\n\
                 \n\
                 1. List all repos (using GitHub API)\n\
                 2. For each repo, check alerts:\n\
                    github_dependabot_alerts({\"owner\": \"org\", \"repo\": repo})\n\
                 3. Aggregate results:\n\
                    - Repos with critical alerts\n\
                    - Total alert count by severity\n\
                    - Oldest alerts\n\
                 4. Prioritize:\n\
                    - Production repos first\n\
                    - Customer-facing apps\n\
                    - High-traffic services\n\n\
                 ALERT LIFECYCLE:\n\
                 1. Alert created (Dependabot detects vulnerability)\n\
                 2. Team notified (email/Slack/webhook)\n\
                 3. Triage (assess severity and impact)\n\
                 4. Action decision:\n\
                    - Fix: Update dependency\n\
                    - Dismiss: Document reason\n\
                    - Defer: Schedule for later\n\
                 5. Verification (alert auto-closes when fixed)\n\
                 6. Audit (periodic review of dismissed alerts)\n\n\
                 TEAM RESPONSIBILITIES:\n\
                 - Security Lead: Monitor critical alerts daily\n\
                 - Tech Lead: Weekly triage and planning\n\
                 - Developers: Fix assigned alerts\n\
                 - DevOps: Ensure CI checks alerts\n\
                 - Product: Prioritize security work\n\n\
                 METRICS TO TRACK:\n\
                 1. Mean time to resolution by severity\n\
                 2. Number of open alerts over time\n\
                 3. Alert backlog age\n\
                 4. Percentage of auto-fixed alerts\n\
                 5. Dismissed alert review frequency\n\n\
                 BEST PRACTICES:\n\
                 1. Never ignore critical alerts\n\
                 2. Set SLAs: Critical (24h), High (1 week), Medium (1 month)\n\
                 3. Document dismissal reasons thoroughly\n\
                 4. Enable Dependabot security updates\n\
                 5. Regular dependency updates reduce security debt\n\
                 6. Automate where possible (CI checks, auto-merge patches)\n\
                 7. Review dismissed alerts quarterly\n\
                 8. Keep dependencies reasonably current\n\
                 9. Test thoroughly after security updates\n\
                 10. Communicate security fixes to stakeholders\n\n\
                 TOOLS INTEGRATION:\n\
                 - Slack/Teams: Alert notifications\n\
                 - Jira/Linear: Track remediation tasks\n\
                 - PagerDuty: Critical alert escalation\n\
                 - Dashboards: Security metrics visualization\n\
                 - CI/CD: Automated checks and gates",
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
                "Give me a complete guide to using github_dependabot_alerts effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_dependabot_alerts tool helps you manage dependency vulnerabilities. Here's your complete guide:\n\n\
                 =============================================================================\n\
                 WHAT ARE DEPENDABOT ALERTS?\n\
                 =============================================================================\n\n\
                 Dependabot alerts notify you when:\n\
                 - A dependency has a known security vulnerability\n\
                 - A patched version is available\n\
                 - Your code may be at risk\n\n\
                 Alerts include:\n\
                 - Vulnerable package name and version\n\
                 - Severity level (critical, high, medium, low)\n\
                 - CVE/GHSA identifier\n\
                 - Description of vulnerability\n\
                 - Patched version number\n\
                 - Exploitation details\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 GET ALL ALERTS:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"repository\"\n\
                 })\n\
                 Returns all alerts for the repository\n\n\
                 GET SPECIFIC ALERT:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"username\",\n\
                     \"repo\": \"repository\",\n\
                     \"alert_number\": 1\n\
                 })\n\
                 Returns detailed information for alert #1\n\n\
                 =============================================================================\n\
                 FILTERING ALERTS\n\
                 =============================================================================\n\n\
                 BY SEVERITY:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\", \"repo\": \"project\",\n\
                     \"severity\": \"critical\"  // or \"high\", \"medium\", \"low\"\n\
                 })\n\
                 \n\
                 MULTIPLE SEVERITIES:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\", \"repo\": \"project\",\n\
                     \"severity\": \"critical,high\"  // comma-separated\n\
                 })\n\n\
                 BY STATE:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\", \"repo\": \"project\",\n\
                     \"state\": \"open\"  // or \"dismissed\", \"fixed\"\n\
                 })\n\
                 - open: Needs attention\n\
                 - dismissed: Acknowledged but not fixing\n\
                 - fixed: Resolved\n\n\
                 BY ECOSYSTEM:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\", \"repo\": \"project\",\n\
                     \"ecosystem\": \"npm\"  // or \"pip\", \"cargo\", \"maven\", etc.\n\
                 })\n\n\
                 BY PACKAGE:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\", \"repo\": \"project\",\n\
                     \"package\": \"lodash\"\n\
                 })\n\n\
                 BY MANIFEST:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\", \"repo\": \"project\",\n\
                     \"manifest\": \"package.json\"\n\
                 })\n\n\
                 COMBINING FILTERS:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"user\", \"repo\": \"project\",\n\
                     \"state\": \"open\",\n\
                     \"severity\": \"critical,high\",\n\
                     \"ecosystem\": \"npm\"\n\
                 })\n\
                 Powerful for focused review\n\n\
                 =============================================================================\n\
                 UNDERSTANDING ALERT RESPONSES\n\
                 =============================================================================\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"alerts\": [\n\
                     {\n\
                       \"number\": 1,\n\
                       \"state\": \"open\",\n\
                       \"dependency\": {\n\
                         \"package\": {\"name\": \"lodash\", \"ecosystem\": \"npm\"},\n\
                         \"manifest_path\": \"package.json\",\n\
                         \"scope\": \"runtime\"  // or \"development\"\n\
                       },\n\
                       \"security_advisory\": {\n\
                         \"ghsa_id\": \"GHSA-xxxx-yyyy-zzzz\",\n\
                         \"cve_id\": \"CVE-2021-12345\",\n\
                         \"severity\": \"high\",\n\
                         \"cvss\": {\"score\": 7.5, \"vector_string\": \"CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N\"},\n\
                         \"summary\": \"Prototype Pollution in lodash\",\n\
                         \"description\": \"Versions of lodash...\",\n\
                         \"published_at\": \"2021-02-15T00:00:00Z\",\n\
                         \"updated_at\": \"2021-02-15T00:00:00Z\"\n\
                       },\n\
                       \"security_vulnerability\": {\n\
                         \"package\": {\"name\": \"lodash\", \"ecosystem\": \"npm\"},\n\
                         \"vulnerable_version_range\": \"< 4.17.21\",\n\
                         \"first_patched_version\": \"4.17.21\"\n\
                       },\n\
                       \"url\": \"https://api.github.com/repos/...\",\n\
                       \"html_url\": \"https://github.com/...\",\n\
                       \"created_at\": \"2021-03-01T10:30:00Z\",\n\
                       \"updated_at\": \"2021-03-01T10:30:00Z\",\n\
                       \"dismissed_at\": null,\n\
                       \"dismissed_by\": null,\n\
                       \"dismissed_reason\": null,\n\
                       \"dismissed_comment\": null,\n\
                       \"fixed_at\": null\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 KEY FIELDS EXPLAINED:\n\
                 - number: Unique ID for this alert in this repo\n\
                 - state: Current status (open/dismissed/fixed)\n\
                 - dependency.package.name: Vulnerable package\n\
                 - dependency.manifest_path: Where it's declared\n\
                 - dependency.scope: runtime (production) or development\n\
                 - security_advisory.severity: Critical/High/Medium/Low\n\
                 - security_advisory.cvss.score: 0-10 severity score\n\
                 - security_advisory.summary: Quick vulnerability description\n\
                 - security_advisory.ghsa_id: GitHub Security Advisory ID\n\
                 - security_advisory.cve_id: CVE identifier (if assigned)\n\
                 - security_vulnerability.vulnerable_version_range: Affected versions\n\
                 - security_vulnerability.first_patched_version: Version to upgrade to\n\
                 - dismissed_reason: Why alert was dismissed (if applicable)\n\
                 - fixed_at: When vulnerability was resolved\n\n\
                 =============================================================================\n\
                 SEVERITY LEVELS\n\
                 =============================================================================\n\n\
                 CRITICAL (CVSS 9.0-10.0):\n\
                 - Immediate action required\n\
                 - Often actively exploited in the wild\n\
                 - Can lead to complete system compromise\n\
                 - Remote code execution, authentication bypass\n\
                 - Fix within 24 hours\n\n\
                 HIGH (CVSS 7.0-8.9):\n\
                 - Priority fix required\n\
                 - Significant security risk\n\
                 - Data breach or service disruption possible\n\
                 - SQL injection, XSS, privilege escalation\n\
                 - Fix within 1 week\n\n\
                 MEDIUM (CVSS 4.0-6.9):\n\
                 - Should fix soon\n\
                 - Moderate security risk\n\
                 - May require specific conditions to exploit\n\
                 - Information disclosure, DoS\n\
                 - Fix within 1 month\n\n\
                 LOW (CVSS 0.1-3.9):\n\
                 - Fix when convenient\n\
                 - Minor security risk\n\
                 - Limited impact or difficult to exploit\n\
                 - Edge cases, rare scenarios\n\
                 - Fix in next maintenance cycle\n\n\
                 =============================================================================\n\
                 REMEDIATION WORKFLOW\n\
                 =============================================================================\n\n\
                 STEP 1 - IDENTIFY:\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"team\", \"repo\": \"app\",\n\
                     \"state\": \"open\", \"severity\": \"critical,high\"\n\
                 })\n\n\
                 STEP 2 - REVIEW:\n\
                 For each alert:\n\
                 - Read vulnerability description\n\
                 - Check if your code uses vulnerable functionality\n\
                 - Note the patched version\n\
                 - Assess breaking changes in upgrade\n\n\
                 STEP 3 - UPDATE:\n\
                 NPM:\n\
                 fs_edit_block({\n\
                     \"path\": \"/project/package.json\",\n\
                     \"old_string\": \"\\\"lodash\\\": \\\"^4.17.10\\\"\",\n\
                     \"new_string\": \"\\\"lodash\\\": \\\"^4.17.21\\\"\"\n\
                 })\n\
                 terminal({\"command\": \"cd /project && npm install\"})\n\
                 \n\
                 PYTHON:\n\
                 fs_edit_block({\n\
                     \"path\": \"/project/requirements.txt\",\n\
                     \"old_string\": \"django==3.2.0\",\n\
                     \"new_string\": \"django==3.2.13\"\n\
                 })\n\
                 terminal({\"command\": \"cd /project && pip install -r requirements.txt\"})\n\
                 \n\
                 RUST:\n\
                 fs_edit_block({\n\
                     \"path\": \"/project/Cargo.toml\",\n\
                     \"old_string\": \"tokio = \\\"1.0.0\\\"\",\n\
                     \"new_string\": \"tokio = \\\"1.18.0\\\"\"\n\
                 })\n\
                 terminal({\"command\": \"cd /project && cargo update -p tokio\"})\n\n\
                 STEP 4 - TEST:\n\
                 terminal({\"command\": \"cd /project && npm test && npm run build\"})\n\
                 Ensure update didn't break anything\n\n\
                 STEP 5 - COMMIT:\n\
                 terminal({\"command\": \"cd /project && git add . && git commit -m \\\"fix: update lodash to 4.17.21 (security)\\\" && git push\"})\n\n\
                 STEP 6 - VERIFY:\n\
                 Alert automatically closes when GitHub detects the fix\n\
                 Check with:\n\
                 github_dependabot_alerts({\"owner\": \"team\", \"repo\": \"app\", \"alert_number\": 1})\n\n\
                 =============================================================================\n\
                 DISMISSING ALERTS\n\
                 =============================================================================\n\n\
                 When vulnerability doesn't apply:\n\
                 github_update_dependabot_alert({\n\
                     \"owner\": \"user\", \"repo\": \"project\",\n\
                     \"alert_number\": 1,\n\
                     \"state\": \"dismissed\",\n\
                     \"dismissed_reason\": \"not_used\",\n\
                     \"dismissed_comment\": \"We don't call the vulnerable function\"\n\
                 })\n\n\
                 VALID DISMISSAL REASONS:\n\
                 - no_bandwidth: Will fix later, limited resources\n\
                 - tolerable_risk: Risk is acceptable given context\n\
                 - inaccurate: False positive, not actually vulnerable\n\
                 - not_used: Dependency present but vulnerable code not executed\n\n\
                 IMPORTANT: Always provide dismissed_comment explaining reasoning\n\n\
                 =============================================================================\n\
                 RECOMMENDED WORKFLOWS\n\
                 =============================================================================\n\n\
                 DAILY (5 minutes):\n\
                 github_dependabot_alerts({\n\
                     \"owner\": \"team\", \"repo\": \"app\",\n\
                     \"state\": \"open\", \"severity\": \"critical\"\n\
                 })\n\
                 If any critical alerts, fix immediately\n\n\
                 WEEKLY (30 minutes):\n\
                 1. Review all open alerts:\n\
                    github_dependabot_alerts({\"owner\": \"team\", \"repo\": \"app\", \"state\": \"open\"})\n\
                 2. Triage by severity\n\
                 3. Plan fixes for current sprint\n\
                 4. Update dismissal reasons if needed\n\n\
                 MONTHLY (2 hours):\n\
                 1. Review dismissed alerts:\n\
                    github_dependabot_alerts({\"owner\": \"team\", \"repo\": \"app\", \"state\": \"dismissed\"})\n\
                 2. Check if dismissal reasons still valid\n\
                 3. Fix what can now be fixed\n\n\
                 QUARTERLY (1 day):\n\
                 1. Full security audit\n\
                 2. Review all repositories\n\
                 3. Update security policies\n\
                 4. Train team on new vulnerabilities\n\n\
                 =============================================================================\n\
                 DECISION TREE\n\
                 =============================================================================\n\n\
                 New alert appears:\n\
                 ├─ Is severity CRITICAL?\n\
                 │  ├─ YES → Fix within 24 hours, deploy hotfix\n\
                 │  └─ NO → Continue\n\
                 ├─ Is severity HIGH?\n\
                 │  ├─ YES → Fix within 1 week\n\
                 │  └─ NO → Continue\n\
                 ├─ Is severity MEDIUM?\n\
                 │  ├─ YES → Schedule for current sprint\n\
                 │  └─ NO → Continue\n\
                 └─ Is severity LOW?\n\
                    └─ YES → Add to backlog, fix when convenient\n\n\
                 Can't fix immediately:\n\
                 ├─ Breaking changes in update?\n\
                 │  ├─ YES → Create feature branch, fix breaking changes, PR\n\
                 │  └─ NO → Continue\n\
                 ├─ Vulnerability not applicable?\n\
                 │  ├─ YES → Dismiss with reason \"not_used\" or \"inaccurate\"\n\
                 │  └─ NO → Continue\n\
                 └─ Limited resources?\n\
                    └─ YES → Dismiss with reason \"no_bandwidth\", schedule for later\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. RESPOND QUICKLY: Critical alerts need same-day action\n\
                 2. PRIORITIZE: Use severity levels to triage\n\
                 3. DOCUMENT: Always explain dismissal reasons\n\
                 4. TEST: Always test after updating dependencies\n\
                 5. AUTOMATE: Enable Dependabot security update PRs\n\
                 6. REVIEW: Regularly review dismissed alerts\n\
                 7. UPDATE: Keep dependencies reasonably current\n\
                 8. MONITOR: Regular checks, don't wait for alerts to pile up\n\
                 9. COMMUNICATE: Notify team about security fixes\n\
                 10. LEARN: Understand vulnerabilities, not just fix them\n\n\
                 =============================================================================\n\
                 COMMON PATTERNS\n\
                 =============================================================================\n\n\
                 SECURITY AUDIT:\n\
                 github_dependabot_alerts({\"owner\": \"org\", \"repo\": \"app\", \"state\": \"open\"})\n\
                 Review all, create action plan\n\n\
                 URGENT TRIAGE:\n\
                 github_dependabot_alerts({\"owner\": \"org\", \"repo\": \"app\", \"state\": \"open\", \"severity\": \"critical,high\"})\n\
                 Focus on highest priority\n\n\
                 PACKAGE INVESTIGATION:\n\
                 github_dependabot_alerts({\"owner\": \"org\", \"repo\": \"app\", \"package\": \"lodash\"})\n\
                 See all historical issues with a dependency\n\n\
                 ECOSYSTEM REVIEW:\n\
                 github_dependabot_alerts({\"owner\": \"org\", \"repo\": \"app\", \"ecosystem\": \"npm\", \"state\": \"open\"})\n\
                 Focus on one language's dependencies\n\n\
                 POST-UPDATE VERIFICATION:\n\
                 github_dependabot_alerts({\"owner\": \"org\", \"repo\": \"app\", \"state\": \"fixed\"})\n\
                 Confirm alerts closed after fix\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 All alerts: {\"owner\": \"user\", \"repo\": \"project\"}\n\
                 Critical only: {\"owner\": \"user\", \"repo\": \"project\", \"severity\": \"critical\"}\n\
                 Open alerts: {\"owner\": \"user\", \"repo\": \"project\", \"state\": \"open\"}\n\
                 By package: {\"owner\": \"user\", \"repo\": \"project\", \"package\": \"lodash\"}\n\
                 Multiple filters: {\"owner\": \"user\", \"repo\": \"project\", \"state\": \"open\", \"severity\": \"critical,high\"}\n\n\
                 Remember: Security is not a one-time task. Regular monitoring and prompt action keep your code secure!",
            ),
        },
    ]
}
