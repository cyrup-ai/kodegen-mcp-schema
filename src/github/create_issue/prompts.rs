//! Prompt messages for github_create_issue tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::CreateIssuePromptArgs;

/// Prompt provider for github_create_issue tool
///
/// This is the ONLY way to provide prompts for github_create_issue - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct CreateIssuePrompts;

impl PromptProvider for CreateIssuePrompts {
    type PromptArgs = CreateIssuePromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("bug_report") => prompt_bug_report(),
            Some("metadata") => prompt_metadata(),
            Some("templates") => prompt_templates(),
            _ => prompt_metadata(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (bug_report, metadata, templates)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO CREATE GITHUB ISSUES
// ============================================================================

/// Bug report creation patterns and best practices
fn prompt_bug_report() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create effective bug reports using github_create_issue?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Creating effective bug reports requires clear structure and comprehensive information. Here's how to create bug reports that get fixed quickly:\n\n\
                 BASIC BUG REPORT:\n\
                 1. Simple bug with description:\n\
                    github_create_issue({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"title\": \"Login button not working on mobile Safari\",\n\
                      \"body\": \"## Description\\nThe login button doesn't respond to taps on iOS Safari.\\n\\n## Steps to Reproduce\\n1. Open site on iPhone\\n2. Tap login button\\n3. Nothing happens\\n\\n## Expected Behavior\\nLogin modal should appear\\n\\n## Actual Behavior\\nButton does not respond to touch events\\n\\n## Environment\\n- Device: iPhone 12\\n- OS: iOS 15.3\\n- Browser: Safari 15\"\n\
                    })\n\n\
                 2. Bug with code reproduction:\n\
                    github_create_issue({\n\
                      \"owner\": \"rust-lang\",\n\
                      \"repo\": \"rust\",\n\
                      \"title\": \"Compiler panic with empty slice pattern\",\n\
                      \"body\": \"## Bug Description\\nThe compiler panics when processing empty slice patterns.\\n\\n## Code to Reproduce\\n```rust\\nfn main() {\\n    let arr = [];\\n    match arr {\\n        [] => println!(\\\"empty\\\"),\\n    }\\n}\\n```\\n\\n## Error Output\\n```\\nthread 'rustc' panicked at 'index out of bounds'\\nstack backtrace:\\n  0: rust_begin_unwind\\n  1: core::panicking::panic_fmt\\n```\\n\\n## Expected\\nCode should compile without panic\\n\\n## Compiler Version\\nrustc 1.65.0-nightly\"\n\
                    })\n\n\
                 3. Performance bug with metrics:\n\
                    github_create_issue({\n\
                      \"owner\": \"facebook\",\n\
                      \"repo\": \"react\",\n\
                      \"title\": \"Memory leak in useEffect with rapid state updates\",\n\
                      \"body\": \"## Issue\\nMemory usage grows unbounded with rapid state updates.\\n\\n## Reproduction\\n```jsx\\nfunction Component() {\\n  const [count, setCount] = useState(0);\\n  useEffect(() => {\\n    const interval = setInterval(() => setCount(c => c + 1), 10);\\n    return () => clearInterval(interval);\\n  }, []);\\n  return <div>{count}</div>;\\n}\\n```\\n\\n## Observed Behavior\\n- Memory: 50MB → 500MB in 30 seconds\\n- Browser becomes unresponsive\\n\\n## Expected\\n- Memory should stabilize\\n- Cleanup functions should run\\n\\n## Profiling Data\\n[Screenshot of Chrome DevTools memory timeline attached]\"\n\
                    })\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"issue_number\": 123,\n\
                   \"html_url\": \"https://github.com/user/project/issues/123\",\n\
                   \"message\": \"Issue #123 created successfully\"\n\
                 }\n\n\
                 ESSENTIAL BUG REPORT SECTIONS:\n\
                 1. Description:\n\
                    - What is broken?\n\
                    - When did it start happening?\n\
                    - What is the impact?\n\n\
                 2. Steps to Reproduce:\n\
                    - Numbered, sequential steps\n\
                    - Include minimal code if applicable\n\
                    - Specify exact versions/environment\n\n\
                 3. Expected Behavior:\n\
                    - What should happen?\n\
                    - Reference documentation if available\n\n\
                 4. Actual Behavior:\n\
                    - What actually happens?\n\
                    - Include error messages\n\
                    - Add screenshots/logs if helpful\n\n\
                 5. Environment Information:\n\
                    - OS and version\n\
                    - Software versions (language, framework, browser)\n\
                    - Hardware details if relevant\n\n\
                 FORMATTING BEST PRACTICES:\n\
                 - Use Markdown headers (##) for sections\n\
                 - Use code blocks (```) for code, logs, errors\n\
                 - Use lists (1., 2., 3.) for steps\n\
                 - Include language hints: ```rust, ```javascript\n\
                 - Keep code minimal but complete\n\n\
                 COMMON BUG CATEGORIES:\n\
                 1. Crash/Panic bugs:\n\
                    - Title: \"Panic/Crash when...\"\n\
                    - Include full stack trace\n\
                    - Specify panic message\n\n\
                 2. Incorrect behavior:\n\
                    - Title: \"Function X returns wrong value\"\n\
                    - Show expected vs actual output\n\
                    - Include test case\n\n\
                 3. Performance issues:\n\
                    - Title: \"Slow/Memory leak in...\"\n\
                    - Include metrics (time, memory, CPU)\n\
                    - Show profiling data if available\n\n\
                 4. UI/UX bugs:\n\
                    - Title: \"Button/Feature not working on...\"\n\
                    - Include screenshots\n\
                    - Specify browser/device\n\n\
                 TIPS FOR GOOD BUG REPORTS:\n\
                 - Make reproduction steps as simple as possible\n\
                 - Remove unnecessary code/details\n\
                 - Test your reproduction steps before submitting\n\
                 - Search for existing issues first\n\
                 - Be specific in the title (not just \"It's broken\")\n\
                 - Include version numbers for all dependencies\n\
                 - Add \"Reproduction:\" label if you have minimal repro\n\n\
                 WHAT TO AVOID:\n\
                 - Vague titles like \"Doesn't work\" or \"Error\"\n\
                 - Missing reproduction steps\n\
                 - Screenshots of code instead of code blocks\n\
                 - \"Works on my machine\" without details\n\
                 - Combining multiple unrelated bugs in one issue",
            ),
        },
    ]
}

/// Metadata usage patterns (labels, assignees, milestones)
fn prompt_metadata() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use labels, assignees, and other metadata when creating GitHub issues?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "GitHub issue metadata helps with organization, prioritization, and workflow automation. Here's how to use labels, assignees, and milestones effectively:\n\n\
                 USING LABELS:\n\
                 1. Single label:\n\
                    github_create_issue({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"title\": \"Fix authentication bug\",\n\
                      \"body\": \"Users cannot log in...\",\n\
                      \"labels\": [\"bug\"]\n\
                    })\n\n\
                 2. Multiple labels for categorization:\n\
                    github_create_issue({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"title\": \"Critical security vulnerability in auth module\",\n\
                      \"body\": \"SQL injection possible...\",\n\
                      \"labels\": [\"bug\", \"security\", \"priority:critical\", \"area:auth\"]\n\
                    })\n\n\
                 3. Labels for workflow automation:\n\
                    github_create_issue({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"title\": \"Add TypeScript support\",\n\
                      \"body\": \"Convert codebase to TypeScript...\",\n\
                      \"labels\": [\"enhancement\", \"help-wanted\", \"good-first-issue\"]\n\
                    })\n\n\
                 USING ASSIGNEES:\n\
                 1. Assign to specific person:\n\
                    github_create_issue({\n\
                      \"owner\": \"company\",\n\
                      \"repo\": \"product\",\n\
                      \"title\": \"Implement OAuth integration\",\n\
                      \"body\": \"Add OAuth 2.0 support...\",\n\
                      \"assignees\": [\"alice-dev\"]\n\
                    })\n\n\
                 2. Assign to multiple team members:\n\
                    github_create_issue({\n\
                      \"owner\": \"company\",\n\
                      \"repo\": \"product\",\n\
                      \"title\": \"Database migration to PostgreSQL\",\n\
                      \"body\": \"Migrate from MySQL to PostgreSQL...\",\n\
                      \"assignees\": [\"bob-backend\", \"carol-devops\"],\n\
                      \"labels\": [\"database\", \"migration\"]\n\
                    })\n\n\
                 3. Assign with milestone tracking:\n\
                    github_create_issue({\n\
                      \"owner\": \"company\",\n\
                      \"repo\": \"product\",\n\
                      \"title\": \"Implement user dashboard\",\n\
                      \"body\": \"Create dashboard showing user analytics...\",\n\
                      \"assignees\": [\"david-frontend\"],\n\
                      \"labels\": [\"feature\", \"ui\"],\n\
                      \"milestone\": 5\n\
                    })\n\n\
                 COMBINING METADATA:\n\
                 1. Bug with full metadata:\n\
                    github_create_issue({\n\
                      \"owner\": \"org\",\n\
                      \"repo\": \"app\",\n\
                      \"title\": \"Memory leak in background worker\",\n\
                      \"body\": \"## Bug Description\\nWorker process memory grows indefinitely...\\n\\n## Steps\\n1. Start worker\\n2. Monitor memory\\n3. Observe leak\",\n\
                      \"labels\": [\"bug\", \"priority:high\", \"component:worker\", \"needs-investigation\"],\n\
                      \"assignees\": [\"performance-team-lead\"],\n\
                      \"milestone\": 3\n\
                    })\n\n\
                 2. Feature with team assignment:\n\
                    github_create_issue({\n\
                      \"owner\": \"org\",\n\
                      \"repo\": \"app\",\n\
                      \"title\": \"Add real-time notifications\",\n\
                      \"body\": \"## Feature\\nImplement WebSocket-based notifications...\\n\\n## Requirements\\n- Real-time updates\\n- Browser notifications\\n- Mobile push\",\n\
                      \"labels\": [\"enhancement\", \"area:notifications\", \"needs-design\"],\n\
                      \"assignees\": [\"websocket-expert\", \"mobile-dev\"]\n\
                    })\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"issue_number\": 789,\n\
                   \"html_url\": \"https://github.com/user/project/issues/789\",\n\
                   \"message\": \"Issue #789 created successfully\"\n\
                 }\n\n\
                 COMMON LABEL PATTERNS:\n\
                 1. Type labels:\n\
                    - bug: Something isn't working\n\
                    - enhancement/feature: New functionality\n\
                    - documentation: Docs improvements\n\
                    - question: Request for information\n\n\
                 2. Priority labels:\n\
                    - priority:critical: Must fix immediately\n\
                    - priority:high: Fix soon\n\
                    - priority:medium: Normal priority\n\
                    - priority:low: Nice to have\n\n\
                 3. Status labels:\n\
                    - needs-investigation: Requires analysis\n\
                    - needs-reproduction: Cannot reproduce\n\
                    - in-progress: Actively being worked on\n\
                    - blocked: Waiting on something\n\n\
                 4. Area labels:\n\
                    - area:frontend: UI/UX issues\n\
                    - area:backend: Server-side issues\n\
                    - area:database: Database issues\n\
                    - area:infrastructure: DevOps/deployment\n\n\
                 5. Community labels:\n\
                    - good-first-issue: Easy for newcomers\n\
                    - help-wanted: Need community help\n\
                    - duplicate: Already reported\n\
                    - wontfix: Will not be addressed\n\n\
                 ASSIGNEE BEST PRACTICES:\n\
                 - Only assign users with repository access\n\
                 - Verify username spelling (case-sensitive)\n\
                 - Don't over-assign (2-3 people max)\n\
                 - Assign based on expertise/ownership\n\
                 - Consider time zones and availability\n\
                 - Use team mentions in body if no specific assignee\n\n\
                 MILESTONE USAGE:\n\
                 - milestone: Integer ID of the milestone\n\
                 - Use for release planning (v1.0, v2.0)\n\
                 - Group related issues\n\
                 - Track progress toward goals\n\
                 - Get milestone ID from GitHub API or UI\n\n\
                 AUTOMATION WITH LABELS:\n\
                 1. GitHub Actions can trigger on labels:\n\
                    - \"priority:critical\" → notify team\n\
                    - \"good-first-issue\" → add to project board\n\
                    - \"security\" → run security scan\n\n\
                 2. Project boards can filter by labels:\n\
                    - Show only \"bug\" issues in bug column\n\
                    - Track \"enhancement\" in feature column\n\n\
                 3. Notifications and subscriptions:\n\
                    - Watch issues with specific labels\n\
                    - Filter email notifications\n\n\
                 METADATA VALIDATION:\n\
                 - Labels must exist in repository\n\
                 - Create labels first if they don't exist\n\
                 - Assignees must have repository access\n\
                 - Milestone must exist (use milestone ID)\n\
                 - Invalid metadata returns 422 error\n\n\
                 BEST PRACTICES:\n\
                 - Use consistent label naming conventions\n\
                 - Document label meanings in repo\n\
                 - Don't create too many labels (15-25 is good)\n\
                 - Use label colors meaningfully\n\
                 - Review and update labels periodically\n\
                 - Assign early to show ownership\n\
                 - Use labels for filtering and automation\n\n\
                 WHAT TO AVOID:\n\
                 - Misspelled labels (creates new label)\n\
                 - Too many labels on one issue\n\
                 - Assigning without permission\n\
                 - Using labels inconsistently\n\
                 - Creating duplicate labels",
            ),
        },
    ]
}

/// Issue template patterns and formats
fn prompt_templates() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use issue templates when creating GitHub issues?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Issue templates provide consistent structure for bug reports, feature requests, and other issue types. Here's how to follow and create template-based issues:\n\n\
                 USING BUG REPORT TEMPLATE:\n\
                 1. Standard bug template:\n\
                    github_create_issue({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"title\": \"[BUG] Application crashes on startup\",\n\
                      \"body\": \"## Bug Description\\nApplication crashes immediately after launch\\n\\n## Environment\\n- OS: macOS 14.2\\n- Version: 2.3.1\\n- Installation: Homebrew\\n\\n## Steps to Reproduce\\n1. Install via `brew install app`\\n2. Run `app start`\\n3. Observe crash\\n\\n## Expected Behavior\\nApplication should start successfully\\n\\n## Actual Behavior\\nApplication crashes with segmentation fault\\n\\n## Logs\\n```\\nSegmentation fault: 11\\nStack trace:\\n  0: app::main\\n  1: std::rt::lang_start\\n```\\n\\n## Additional Context\\nWorks fine on Linux Ubuntu 22.04\"\n\
                    })\n\n\
                 2. Performance bug template:\n\
                    github_create_issue({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"title\": \"[PERF] Slow query performance with large datasets\",\n\
                      \"body\": \"## Performance Issue\\nDatabase queries extremely slow with >10k records\\n\\n## Metrics\\n- Query time: 15s (expected <1s)\\n- CPU usage: 100%\\n- Memory: 2GB\\n\\n## Query\\n```sql\\nSELECT * FROM users\\nWHERE created_at > '2024-01-01'\\nORDER BY email;\\n```\\n\\n## Database\\n- Engine: PostgreSQL 14\\n- Records: 50,000 users\\n- Indexes: Primary key only\\n\\n## Profiling Results\\n[Include EXPLAIN ANALYZE output]\\n\\n## Proposed Solution\\nAdd index on (created_at, email)\"\n\
                    })\n\n\
                 USING FEATURE REQUEST TEMPLATE:\n\
                 1. Standard feature template:\n\
                    github_create_issue({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"title\": \"[FEATURE] Add export to CSV functionality\",\n\
                      \"body\": \"## Feature Summary\\nAllow users to export data to CSV format\\n\\n## Motivation\\nUsers need to analyze data in Excel/spreadsheet tools. Currently must copy-paste manually.\\n\\n## Detailed Design\\n### UI\\n- Add \\\"Export CSV\\\" button in data table toolbar\\n- Show progress spinner during export\\n- Auto-download file when ready\\n\\n### API\\n```typescript\\nexportToCSV(options: {\\n  columns: string[];\\n  filters?: Filter[];\\n  filename?: string;\\n}): Promise<Blob>\\n```\\n\\n### Format\\n- UTF-8 encoding\\n- Comma-separated\\n- Quote fields with commas\\n- Header row with column names\\n\\n## Alternatives Considered\\n1. Export to Excel (.xlsx) - more complex\\n2. Export to JSON - less user-friendly\\n3. Print to PDF - not analyzable\\n\\n## Implementation Notes\\n- Use papaparse library for CSV generation\\n- Implement server-side for large datasets\\n- Add rate limiting (max 5 exports/minute)\"\n\
                    })\n\n\
                 2. Integration feature template:\n\
                    github_create_issue({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"title\": \"[INTEGRATION] Add Slack notifications\",\n\
                      \"body\": \"## Integration Request\\nSend notifications to Slack channels\\n\\n## Use Cases\\n1. Alert on system errors\\n2. Notify on deployment completion\\n3. Report daily metrics\\n\\n## Configuration\\n```yaml\\nslack:\\n  webhook_url: https://hooks.slack.com/...\\n  channels:\\n    errors: '#alerts'\\n    deploys: '#deployments'\\n    metrics: '#analytics'\\n```\\n\\n## Message Format\\n```json\\n{\\n  \\\"text\\\": \\\"Deployment completed\\\",\\n  \\\"attachments\\\": [...]\\n}\\n```\\n\\n## Security\\n- Store webhook URL in secrets\\n- Validate message content\\n- Rate limit messages\\n\\n## Similar Integrations\\n- Discord (already implemented)\\n- Microsoft Teams (planned)\"\n\
                    })\n\n\
                 USING DOCUMENTATION TEMPLATE:\n\
                 1. Documentation improvement:\n\
                    github_create_issue({\n\
                      \"owner\": \"user\",\n\
                      \"repo\": \"project\",\n\
                      \"title\": \"[DOCS] Add authentication guide\",\n\
                      \"body\": \"## Documentation Request\\nAdd comprehensive authentication setup guide\\n\\n## Current State\\nAuthentication is mentioned briefly in README but lacks detail\\n\\n## Needed Content\\n1. Overview of auth system\\n2. Setup instructions\\n3. Configuration options\\n4. Code examples\\n5. Troubleshooting guide\\n\\n## Target Audience\\nNew users setting up for first time\\n\\n## Proposed Structure\\n```\\ndocs/authentication/\\n  - overview.md\\n  - quickstart.md\\n  - configuration.md\\n  - examples.md\\n  - troubleshooting.md\\n```\\n\\n## Examples to Include\\n- Basic username/password\\n- OAuth integration\\n- JWT tokens\\n- API key authentication\\n\\n## Related Issues\\n#45, #67 (users confused about auth)\"\n\
                    })\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"owner\": \"user\",\n\
                   \"repo\": \"project\",\n\
                   \"issue_number\": 234,\n\
                   \"html_url\": \"https://github.com/user/project/issues/234\",\n\
                   \"message\": \"Issue #234 created successfully\"\n\
                 }\n\n\
                 COMMON TEMPLATE TYPES:\n\
                 1. Bug Report Template:\n\
                    - Bug description\n\
                    - Steps to reproduce\n\
                    - Expected vs actual behavior\n\
                    - Environment details\n\
                    - Logs/screenshots\n\n\
                 2. Feature Request Template:\n\
                    - Feature summary\n\
                    - Motivation/use case\n\
                    - Detailed design\n\
                    - Alternatives considered\n\
                    - Implementation notes\n\n\
                 3. Performance Issue Template:\n\
                    - Performance problem\n\
                    - Metrics (time, memory, CPU)\n\
                    - Profiling data\n\
                    - Expected performance\n\
                    - Proposed optimization\n\n\
                 4. Security Issue Template:\n\
                    - Vulnerability description\n\
                    - Affected versions\n\
                    - Attack vector\n\
                    - Impact assessment\n\
                    - Proposed fix\n\n\
                 5. Documentation Template:\n\
                    - What needs documenting\n\
                    - Current state\n\
                    - Proposed content\n\
                    - Target audience\n\
                    - Related docs\n\n\
                 TEMPLATE BENEFITS:\n\
                 - Ensures consistency across issues\n\
                 - Captures all necessary information\n\
                 - Easier to triage and prioritize\n\
                 - Faster response from maintainers\n\
                 - Better searchability\n\
                 - Reduces back-and-forth questions\n\n\
                 TITLE PREFIXES:\n\
                 Use prefixes to categorize quickly:\n\
                 - [BUG] - Bug reports\n\
                 - [FEATURE] - Feature requests\n\
                 - [DOCS] - Documentation\n\
                 - [PERF] - Performance issues\n\
                 - [SECURITY] - Security vulnerabilities\n\
                 - [QUESTION] - Questions\n\n\
                 MARKDOWN FORMATTING IN TEMPLATES:\n\
                 - Use ## for section headers\n\
                 - Use ``` for code blocks\n\
                 - Use - or * for lists\n\
                 - Use **bold** for emphasis\n\
                 - Use [links](url) for references\n\
                 - Use > for quotes\n\
                 - Use tables for structured data\n\n\
                 BEST PRACTICES:\n\
                 - Follow the template structure completely\n\
                 - Don't delete template sections (write \"N/A\" if not applicable)\n\
                 - Use code blocks for code, not screenshots\n\
                 - Include all requested information\n\
                 - Be specific and detailed\n\
                 - Link related issues with #number\n\
                 - Update issue if more info becomes available\n\n\
                 CREATING REUSABLE TEMPLATES:\n\
                 Store templates in `.github/ISSUE_TEMPLATE/` directory:\n\
                 - bug_report.md\n\
                 - feature_request.md\n\
                 - documentation.md\n\
                 Each template has YAML frontmatter with metadata\n\n\
                 WHAT TO AVOID:\n\
                 - Ignoring template structure\n\
                 - Deleting placeholder text without replacing\n\
                 - Incomplete information\n\
                 - Generic descriptions\n\
                 - Missing code examples\n\
                 - No reproduction steps",
            ),
        },
    ]
}
