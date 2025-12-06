//! Prompt messages for github_pending_invitations tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GithubPendingInvitationsPromptArgs;

/// Prompt provider for pending_invitations tool
///
/// This is the ONLY way to provide prompts for pending_invitations - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GithubPendingInvitationsPrompts;

impl PromptProvider for GithubPendingInvitationsPrompts {
    type PromptArgs = GithubPendingInvitationsPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("workflows") => prompt_workflows(),
            Some("filtering") => prompt_filtering(),
            Some("integration") => prompt_integration(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show examples for (basic, workflows, filtering, integration)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO MANAGE REPOSITORY INVITATIONS
// ============================================================================

/// Basic invitation listing
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list pending repository invitations using the github_pending_invitations tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_pending_invitations tool retrieves all pending repository invitations for your GitHub account, allowing you to see which repositories you've been invited to collaborate on.\n\n\
                 LISTING PENDING INVITATIONS:\n\n\
                 1. Get all pending invitations (simplest form):\n\
                    github_pending_invitations({})\n\n\
                 2. Response format:\n\
                    {\n\
                      \"invitations\": [\n\
                        {\n\
                          \"id\": 12345678,\n\
                          \"repository\": {\n\
                            \"id\": 987654321,\n\
                            \"name\": \"private-repo\",\n\
                            \"full_name\": \"org/private-repo\",\n\
                            \"private\": true,\n\
                            \"description\": \"A private collaborative project\",\n\
                            \"html_url\": \"https://github.com/org/private-repo\"\n\
                          },\n\
                          \"inviter\": {\n\
                            \"login\": \"team-lead\",\n\
                            \"id\": 123456,\n\
                            \"avatar_url\": \"https://avatars.githubusercontent.com/u/123456\",\n\
                            \"html_url\": \"https://github.com/team-lead\"\n\
                          },\n\
                          \"invitee\": {\n\
                            \"login\": \"your-username\",\n\
                            \"id\": 789012,\n\
                            \"html_url\": \"https://github.com/your-username\"\n\
                          },\n\
                          \"permissions\": \"write\",\n\
                          \"created_at\": \"2025-12-01T10:30:00Z\",\n\
                          \"html_url\": \"https://github.com/org/private-repo/invitations\"\n\
                        },\n\
                        {\n\
                          \"id\": 87654321,\n\
                          \"repository\": {\n\
                            \"full_name\": \"user/another-repo\",\n\
                            \"private\": true\n\
                          },\n\
                          \"inviter\": {\"login\": \"maintainer\"},\n\
                          \"permissions\": \"admin\",\n\
                          \"created_at\": \"2025-12-02T14:20:00Z\"\n\
                        }\n\
                      ],\n\
                      \"total_count\": 2\n\
                    }\n\n\
                 UNDERSTANDING THE RESPONSE:\n\n\
                 Key fields in each invitation:\n\
                 - id: Unique invitation identifier (use this to accept invitation)\n\
                 - repository: Information about the repository you're invited to\n\
                   - full_name: Owner/repository format (e.g., \"org/private-repo\")\n\
                   - private: Whether it's a private repository\n\
                   - description: Repository purpose/description\n\
                 - inviter: User who sent the invitation\n\
                   - login: GitHub username of inviter\n\
                 - permissions: Access level being granted\n\
                   - \"read\": Read-only access (view code, issues, PRs)\n\
                   - \"write\": Write access (push code, create branches)\n\
                   - \"admin\": Admin access (manage settings, webhooks)\n\
                   - \"maintain\": Maintain access (manage without admin rights)\n\
                 - created_at: When invitation was sent\n\n\
                 PERMISSION LEVELS EXPLAINED:\n\n\
                 READ:\n\
                 - View repository code and history\n\
                 - Read issues and pull requests\n\
                 - Clone private repository\n\
                 - Cannot push changes or modify repository\n\n\
                 WRITE:\n\
                 - All read permissions\n\
                 - Push commits to repository\n\
                 - Create and merge pull requests\n\
                 - Manage issues and projects\n\
                 - Most common permission for collaborators\n\n\
                 MAINTAIN:\n\
                 - All write permissions\n\
                 - Manage issues and pull requests without delete access\n\
                 - Edit repository settings (limited)\n\
                 - Good for team leads and senior contributors\n\n\
                 ADMIN:\n\
                 - Full repository access\n\
                 - Manage collaborators and teams\n\
                 - Configure webhooks and integrations\n\
                 - Delete repository and change sensitive settings\n\
                 - Highest permission level\n\n\
                 EMPTY INVITATIONS LIST:\n\
                 If no pending invitations:\n\
                 {\n\
                   \"invitations\": [],\n\
                   \"total_count\": 0\n\
                 }\n\
                 This means:\n\
                 - No pending invitations at this time\n\
                 - All previous invitations have been accepted or declined\n\
                 - Or you haven't been invited to any repositories yet\n\n\
                 PARAMETERS:\n\
                 The tool accepts an empty object {} - no parameters required.\n\
                 - Automatically fetches all invitations for authenticated user\n\
                 - No pagination - returns all pending invitations in one call\n\
                 - Results are sorted by creation date (most recent first)\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo (full repository access)\n\
                 - Or user scope to view invitation details\n\
                 Without proper token, will receive authentication error\n\n\
                 WHEN TO CHECK INVITATIONS:\n\
                 1. After receiving GitHub notification email\n\
                 2. Before starting work on a new project\n\
                 3. Periodically to ensure you haven't missed invitations\n\
                 4. When you can't access a repository you expect to have access to\n\
                 5. During onboarding to new teams or organizations\n\n\
                 COMMON SCENARIOS:\n\n\
                 Scenario 1: New team member onboarding\n\
                 - Admin sends multiple repository invitations\n\
                 - New member checks: github_pending_invitations({})\n\
                 - Sees all team repositories they're being added to\n\
                 - Can review and accept all at once\n\n\
                 Scenario 2: Open source collaboration\n\
                 - Maintainer invites you to contribute with write access\n\
                 - Check invitation details to see permission level\n\
                 - Accept to gain direct push access\n\
                 - No more forking required for contributions\n\n\
                 Scenario 3: Organization migration\n\
                 - Company moves repositories to new organization\n\
                 - Mass invitations sent to all team members\n\
                 - Check invitations to see all new repository locations\n\
                 - Accept to maintain access after migration\n\n\
                 NEXT STEPS AFTER LISTING:\n\
                 After reviewing invitations, you typically want to:\n\
                 1. Accept desired invitations using github_accept_repo_invitation\n\
                 2. Note the invitation_id for acceptance\n\
                 3. Review permissions to understand your access level\n\
                 4. Check repository descriptions to understand projects\n\n\
                 TROUBLESHOOTING:\n\n\
                 No invitations showing up?\n\
                 - Check GitHub email for invitation notifications\n\
                 - Verify GITHUB_TOKEN has correct scopes\n\
                 - Invitations may have expired (30 days typically)\n\
                 - May have already accepted the invitation\n\n\
                 Authentication errors?\n\
                 - Verify GITHUB_TOKEN is set and valid\n\
                 - Token must have repo or user scope\n\
                 - Check token hasn't expired\n\n\
                 BEST PRACTICES:\n\
                 1. Check invitations regularly (weekly or after notifications)\n\
                 2. Review repository descriptions before accepting\n\
                 3. Note permission levels to understand access rights\n\
                 4. Accept promptly to avoid expiration\n\
                 5. Keep track of which organizations you're joining\n\
                 6. Verify inviter is legitimate for security",
            ),
        },
    ]
}

/// Invitation management workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are the complete workflows for managing repository invitations?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Repository invitation management involves checking, reviewing, and accepting invitations in organized workflows. Here are the complete patterns.\n\n\
                 BASIC WORKFLOW: CHECK AND ACCEPT\n\n\
                 Step 1: Check pending invitations\n\
                 github_pending_invitations({})\n\n\
                 Response shows:\n\
                 {\n\
                   \"invitations\": [\n\
                     {\n\
                       \"id\": 12345678,\n\
                       \"repository\": {\"full_name\": \"org/backend-api\"},\n\
                       \"inviter\": {\"login\": \"tech-lead\"},\n\
                       \"permissions\": \"write\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 Step 2: Accept the invitation\n\
                 github_accept_repo_invitation({\n\
                     \"invitation_id\": 12345678\n\
                 })\n\n\
                 Step 3: Verify access by checking repository\n\
                 github_get_file_contents({\n\
                     \"owner\": \"org\",\n\
                     \"repo\": \"backend-api\",\n\
                     \"path\": \"README.md\"\n\
                 })\n\n\
                 BULK ACCEPTANCE WORKFLOW:\n\n\
                 When you have multiple invitations from same organization:\n\n\
                 1. Get all invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Filter by organization (in your code/logic):\n\
                    invitations_from_org = filter invitations where repository.full_name starts with \"myorg/\"\n\n\
                 3. Accept each one:\n\
                    For each invitation in invitations_from_org:\n\
                        github_accept_repo_invitation({\"invitation_id\": invitation.id})\n\n\
                 Example with 3 repositories:\n\
                 // Get invitations\n\
                 result = github_pending_invitations({})\n\
                 // Returns: org/repo1 (id: 111), org/repo2 (id: 222), org/repo3 (id: 333)\n\
                 \n\
                 // Accept all\n\
                 github_accept_repo_invitation({\"invitation_id\": 111})\n\
                 github_accept_repo_invitation({\"invitation_id\": 222})\n\
                 github_accept_repo_invitation({\"invitation_id\": 333})\n\n\
                 SELECTIVE ACCEPTANCE WORKFLOW:\n\n\
                 When you want to accept only specific invitations:\n\n\
                 1. List all invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Review each invitation's details:\n\
                    - Repository name and description\n\
                    - Who invited you (inviter.login)\n\
                    - Permission level\n\
                    - When invited (created_at)\n\n\
                 3. Accept only desired repositories:\n\
                    github_accept_repo_invitation({\"invitation_id\": 12345678})\n\
                    github_accept_repo_invitation({\"invitation_id\": 87654321})\n\n\
                 4. Leave others pending or let them expire\n\n\
                 PERMISSION-BASED WORKFLOW:\n\n\
                 Accept invitations based on permission level:\n\n\
                 1. Get invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Filter by permission:\n\
                    - Accept all \"write\" and \"admin\" invitations (you need these for work)\n\
                    - Review \"read\" invitations more carefully (may be optional)\n\n\
                 3. Accept based on criteria:\n\
                    // Accept high-permission invitations\n\
                    if permissions in [\"write\", \"admin\", \"maintain\"]:\n\
                        github_accept_repo_invitation({\"invitation_id\": id})\n\n\
                 TEAM ONBOARDING WORKFLOW:\n\n\
                 Complete workflow for new team members:\n\n\
                 Day 1: Initial setup\n\
                 1. Check invitations:\n\
                    github_pending_invitations({})\n\
                    // Should see all team repositories\n\n\
                 2. Verify expected invitations:\n\
                    - Check you have invitations from your organization\n\
                    - Confirm inviter is your manager/team lead\n\
                    - Verify permission levels match your role\n\n\
                 3. Accept all team invitations:\n\
                    For each invitation from your org:\n\
                        github_accept_repo_invitation({\"invitation_id\": id})\n\n\
                 4. List repositories to confirm access:\n\
                    github_search_repositories({\n\
                        \"query\": \"org:myorg\"\n\
                    })\n\
                    // Should now see all repositories including private ones\n\n\
                 PERIODIC AUDIT WORKFLOW:\n\n\
                 Regular checks to ensure you don't miss invitations:\n\n\
                 Weekly/Monthly:\n\
                 1. Check for new invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. If total_count > 0:\n\
                    - Review each invitation\n\
                    - Check creation date (old invitations may expire soon)\n\
                    - Verify inviter is legitimate\n\
                    - Accept or decline based on relevance\n\n\
                 3. If total_count = 0:\n\
                    - All clear, no action needed\n\
                    - Continue regular work\n\n\
                 SECURITY REVIEW WORKFLOW:\n\n\
                 Before accepting invitations:\n\n\
                 1. Get invitation details:\n\
                    github_pending_invitations({})\n\n\
                 2. For each invitation, verify:\n\
                    - Inviter is someone you know or expect\n\
                    - Repository name matches expected project\n\
                    - Organization is legitimate (check inviter.html_url)\n\
                    - Permission level is appropriate for your role\n\n\
                 3. Red flags to watch for:\n\
                    - Unexpected admin permissions\n\
                    - Inviter you don't recognize\n\
                    - Repository name with typos (phishing attempt)\n\
                    - Organization you've never heard of\n\n\
                 4. If suspicious, DON'T accept:\n\
                    - Let invitation expire\n\
                    - Contact inviter through other channels to verify\n\
                    - Report to security team if needed\n\n\
                 5. If legitimate, accept:\n\
                    github_accept_repo_invitation({\"invitation_id\": id})\n\n\
                 PRIORITY-BASED WORKFLOW:\n\n\
                 Handling invitations by priority:\n\n\
                 1. List all invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Categorize by priority:\n\
                    HIGH: Work-critical repositories, your team's projects\n\
                    MEDIUM: Cross-team collaboration, optional projects\n\
                    LOW: Archive repositories, read-only access\n\n\
                 3. Accept in order:\n\
                    // High priority first\n\
                    github_accept_repo_invitation({\"invitation_id\": high_priority_id})\n\
                    \n\
                    // Then medium priority\n\
                    github_accept_repo_invitation({\"invitation_id\": medium_priority_id})\n\
                    \n\
                    // Low priority when time permits\n\
                    github_accept_repo_invitation({\"invitation_id\": low_priority_id})\n\n\
                 POST-ACCEPTANCE WORKFLOW:\n\n\
                 After accepting invitations:\n\n\
                 1. Accept invitation:\n\
                    github_accept_repo_invitation({\"invitation_id\": 12345678})\n\n\
                 2. Clone repository locally:\n\
                    git_clone({\n\
                        \"url\": \"https://github.com/org/repo.git\",\n\
                        \"path\": \"/local/path/repo\"\n\
                    })\n\n\
                 3. Check repository structure:\n\
                    github_get_file_contents({\n\
                        \"owner\": \"org\",\n\
                        \"repo\": \"repo\",\n\
                        \"path\": \"README.md\"\n\
                    })\n\n\
                 4. Set up local development environment per README\n\n\
                 5. Verify you can push (if write access):\n\
                    git_diff({\"path\": \"/local/path/repo\", \"from\": \"HEAD\"})\n\n\
                 INVITATION EXPIRATION WORKFLOW:\n\n\
                 Handling time-sensitive invitations:\n\n\
                 1. Check invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. For each invitation, check created_at:\n\
                    - Parse date: \"2025-11-15T10:00:00Z\"\n\
                    - Calculate age: current_date - created_at\n\
                    - Invitations typically expire after 7-30 days\n\n\
                 3. Prioritize old invitations:\n\
                    - If age > 20 days: URGENT - accept immediately\n\
                    - If age > 10 days: HIGH - accept soon\n\
                    - If age < 10 days: NORMAL - accept when ready\n\n\
                 4. Accept oldest first:\n\
                    Sort by created_at ascending\n\
                    Accept in order to prevent expiration\n\n\
                 CROSS-TOOL INTEGRATION WORKFLOW:\n\n\
                 Combining with other GitHub tools:\n\n\
                 1. Check invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Get repository details:\n\
                    github_get_file_contents({\n\
                        \"owner\": \"org\",\n\
                        \"repo\": \"repo\",\n\
                        \"path\": \"README.md\"\n\
                    })\n\
                    // Can access public repos before accepting\n\n\
                 3. Check repository activity:\n\
                    github_list_commits({\n\
                        \"owner\": \"org\",\n\
                        \"repo\": \"repo\",\n\
                        \"limit\": 10\n\
                    })\n\
                    // See recent activity\n\n\
                 4. Review open issues:\n\
                    github_list_issues({\n\
                        \"owner\": \"org\",\n\
                        \"repo\": \"repo\",\n\
                        \"state\": \"open\"\n\
                    })\n\
                    // Understand current work\n\n\
                 5. Accept if relevant:\n\
                    github_accept_repo_invitation({\"invitation_id\": 12345678})\n\n\
                 BEST PRACTICES:\n\
                 1. Check invitations at least weekly\n\
                 2. Accept work-critical invitations immediately\n\
                 3. Review permission levels before accepting\n\
                 4. Verify inviter identity for security\n\
                 5. Keep track of organizations you join\n\
                 6. Accept invitations before they expire\n\
                 7. Set up local clones after accepting\n\
                 8. Document which repositories you've joined\n\
                 9. Regularly audit your repository access\n\
                 10. Report suspicious invitations to security team",
            ),
        },
    ]
}

/// Filtering and organizing invitations
fn prompt_filtering() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I filter and organize repository invitations effectively?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "While the github_pending_invitations tool returns all invitations, you can filter and organize them based on various criteria to manage them effectively.\n\n\
                 FILTERING BY ORGANIZATION:\n\n\
                 Get invitations from specific organization:\n\n\
                 1. Get all invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Filter by organization (in your logic):\n\
                    For each invitation in response.invitations:\n\
                        if invitation.repository.full_name starts with \"myorg/\":\n\
                            // This is from your organization\n\
                            process this invitation\n\n\
                 Example response:\n\
                 {\n\
                   \"invitations\": [\n\
                     {\"id\": 111, \"repository\": {\"full_name\": \"myorg/backend\"}},\n\
                     {\"id\": 222, \"repository\": {\"full_name\": \"otherorg/frontend\"}},\n\
                     {\"id\": 333, \"repository\": {\"full_name\": \"myorg/database\"}}\n\
                   ]\n\
                 }\n\
                 \n\
                 Filtered result for \"myorg\":\n\
                 - invitation 111: myorg/backend\n\
                 - invitation 333: myorg/database\n\n\
                 FILTERING BY PERMISSION LEVEL:\n\n\
                 Focus on invitations with specific access levels:\n\n\
                 1. Get all invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Filter by permission:\n\
                    // High-privilege invitations (write, admin, maintain)\n\
                    high_privilege = filter where permissions in [\"write\", \"admin\", \"maintain\"]\n\
                    \n\
                    // Read-only invitations\n\
                    read_only = filter where permissions == \"read\"\n\n\
                 Example filtering:\n\
                 {\n\
                   \"invitations\": [\n\
                     {\"id\": 111, \"permissions\": \"admin\"},   // High privilege\n\
                     {\"id\": 222, \"permissions\": \"read\"},    // Read only\n\
                     {\"id\": 333, \"permissions\": \"write\"}    // High privilege\n\
                   ]\n\
                 }\n\
                 \n\
                 High privilege invitations: 111, 333\n\
                 Read-only invitations: 222\n\n\
                 FILTERING BY INVITER:\n\n\
                 See invitations from specific people:\n\n\
                 1. Get all invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Filter by inviter:\n\
                    // From your manager\n\
                    from_manager = filter where inviter.login == \"manager-username\"\n\
                    \n\
                    // From team members\n\
                    team_members = [\"alice\", \"bob\", \"charlie\"]\n\
                    from_team = filter where inviter.login in team_members\n\n\
                 Example:\n\
                 {\n\
                   \"invitations\": [\n\
                     {\"id\": 111, \"inviter\": {\"login\": \"alice\"}},\n\
                     {\"id\": 222, \"inviter\": {\"login\": \"stranger\"}},\n\
                     {\"id\": 333, \"inviter\": {\"login\": \"bob\"}}\n\
                   ]\n\
                 }\n\
                 \n\
                 From team: 111 (alice), 333 (bob)\n\
                 Unknown: 222 (stranger)\n\n\
                 FILTERING BY AGE:\n\n\
                 Find old or recent invitations:\n\n\
                 1. Get all invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Parse creation dates:\n\
                    For each invitation:\n\
                        parse invitation.created_at to date object\n\
                        calculate age = current_date - created_at\n\n\
                 3. Filter by age:\n\
                    // Urgent (older than 20 days)\n\
                    urgent = filter where age > 20 days\n\
                    \n\
                    // Recent (less than 7 days)\n\
                    recent = filter where age < 7 days\n\
                    \n\
                    // Moderate age (7-20 days)\n\
                    moderate = filter where 7 <= age <= 20 days\n\n\
                 Example:\n\
                 {\n\
                   \"invitations\": [\n\
                     {\"id\": 111, \"created_at\": \"2025-11-10T10:00:00Z\"},  // 25 days old\n\
                     {\"id\": 222, \"created_at\": \"2025-12-01T10:00:00Z\"},  // 4 days old\n\
                     {\"id\": 333, \"created_at\": \"2025-11-20T10:00:00Z\"}   // 15 days old\n\
                   ]\n\
                 }\n\
                 \n\
                 Urgent: 111 (25 days)\n\
                 Recent: 222 (4 days)\n\
                 Moderate: 333 (15 days)\n\n\
                 FILTERING BY REPOSITORY TYPE:\n\n\
                 Distinguish private vs public invitations:\n\n\
                 1. Get all invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Filter by privacy:\n\
                    // Private repositories\n\
                    private_repos = filter where repository.private == true\n\
                    \n\
                    // Public repositories\n\
                    public_repos = filter where repository.private == false\n\n\
                 Note: Most invitations are for private repositories since public repos\n\
                 don't typically require formal invitations.\n\n\
                 ORGANIZING BY PRIORITY:\n\n\
                 Create priority tiers:\n\n\
                 1. Get all invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Assign priorities:\n\
                    CRITICAL:\n\
                    - From your organization\n\
                    - Admin or maintain permissions\n\
                    - Age > 20 days (expiring soon)\n\
                    - From your direct manager\n\
                    \n\
                    HIGH:\n\
                    - From your organization\n\
                    - Write permissions\n\
                    - Age > 10 days\n\
                    - From known team members\n\
                    \n\
                    MEDIUM:\n\
                    - From partner organizations\n\
                    - Write permissions\n\
                    - Age < 10 days\n\
                    \n\
                    LOW:\n\
                    - Read-only access\n\
                    - From unknown users\n\
                    - Archive repositories\n\n\
                 3. Process by priority:\n\
                    // Accept critical first\n\
                    For each critical_invitation:\n\
                        github_accept_repo_invitation({\"invitation_id\": id})\n\
                    \n\
                    // Then high priority\n\
                    For each high_invitation:\n\
                        github_accept_repo_invitation({\"invitation_id\": id})\n\n\
                 GROUPING BY ORGANIZATION:\n\n\
                 Organize invitations by source organization:\n\n\
                 1. Get all invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Group by organization:\n\
                    organizations = {}\n\
                    For each invitation:\n\
                        org_name = invitation.repository.full_name.split(\"/\")[0]\n\
                        if org_name not in organizations:\n\
                            organizations[org_name] = []\n\
                        organizations[org_name].append(invitation)\n\n\
                 3. Result structure:\n\
                    {\n\
                      \"myorg\": [\n\
                        {\"id\": 111, \"repository\": {\"full_name\": \"myorg/backend\"}},\n\
                        {\"id\": 222, \"repository\": {\"full_name\": \"myorg/frontend\"}}\n\
                      ],\n\
                      \"partner\": [\n\
                        {\"id\": 333, \"repository\": {\"full_name\": \"partner/shared-lib\"}}\n\
                      ]\n\
                    }\n\n\
                 4. Accept by organization:\n\
                    // Accept all from your organization\n\
                    For each invitation in organizations[\"myorg\"]:\n\
                        github_accept_repo_invitation({\"invitation_id\": invitation.id})\n\n\
                 SORTING STRATEGIES:\n\n\
                 Different ways to sort invitations:\n\n\
                 1. By date (oldest first - expiring soon):\n\
                    Sort by created_at ascending\n\
                    Accept oldest invitations first to prevent expiration\n\n\
                 2. By permission level (highest first):\n\
                    Sort order: admin > maintain > write > read\n\
                    Accept high-permission invitations first\n\n\
                 3. By repository name (alphabetical):\n\
                    Sort by repository.full_name alphabetically\n\
                    Easier to track and organize\n\n\
                 4. By inviter (grouped by person):\n\
                    Group by inviter.login\n\
                    See all invitations from each person\n\n\
                 FILTERING WORKFLOW EXAMPLE:\n\n\
                 Complete filtering workflow:\n\n\
                 1. Get invitations:\n\
                    result = github_pending_invitations({})\n\
                    all_invitations = result.invitations\n\n\
                 2. Apply filters:\n\
                    // Filter: Only from my organization with write+ permissions\n\
                    filtered = []\n\
                    For each inv in all_invitations:\n\
                        org = inv.repository.full_name.split(\"/\")[0]\n\
                        perms = inv.permissions\n\
                        if org == \"myorg\" and perms in [\"write\", \"admin\", \"maintain\"]:\n\
                            filtered.append(inv)\n\n\
                 3. Sort by age (oldest first):\n\
                    sorted_invitations = sort filtered by created_at ascending\n\n\
                 4. Accept in order:\n\
                    For each inv in sorted_invitations:\n\
                        github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 COMPLEX FILTERING EXAMPLE:\n\n\
                 Multi-criteria filtering:\n\n\
                 1. Get all invitations:\n\
                    result = github_pending_invitations({})\n\n\
                 2. Define criteria:\n\
                    accept_criteria = {\n\
                        \"organizations\": [\"myorg\", \"partnerorg\"],\n\
                        \"permissions\": [\"write\", \"admin\", \"maintain\"],\n\
                        \"max_age_days\": 25,\n\
                        \"trusted_inviters\": [\"manager\", \"tech-lead\", \"alice\"]\n\
                    }\n\n\
                 3. Apply all criteria:\n\
                    filtered = []\n\
                    For each inv in result.invitations:\n\
                        org = inv.repository.full_name.split(\"/\")[0]\n\
                        age = calculate_days_since(inv.created_at)\n\
                        \n\
                        if (org in accept_criteria.organizations AND\n\
                            inv.permissions in accept_criteria.permissions AND\n\
                            age < accept_criteria.max_age_days AND\n\
                            inv.inviter.login in accept_criteria.trusted_inviters):\n\
                            filtered.append(inv)\n\n\
                 4. Review and accept:\n\
                    For each inv in filtered:\n\
                        print(f\"Accepting: {inv.repository.full_name}\")\n\
                        github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 EXCLUSION FILTERING:\n\n\
                 Filter out unwanted invitations:\n\n\
                 1. Get all invitations:\n\
                    github_pending_invitations({})\n\n\
                 2. Define exclusions:\n\
                    exclude = {\n\
                        \"organizations\": [\"spam-org\", \"unknown-org\"],\n\
                        \"permissions\": [\"admin\"],  // Don't auto-accept admin\n\
                        \"repository_patterns\": [\"test-\", \"archive-\"]\n\
                    }\n\n\
                 3. Filter out excluded:\n\
                    safe_invitations = []\n\
                    For each inv in all_invitations:\n\
                        org = inv.repository.full_name.split(\"/\")[0]\n\
                        repo_name = inv.repository.full_name\n\
                        \n\
                        skip = False\n\
                        if org in exclude.organizations: skip = True\n\
                        if inv.permissions in exclude.permissions: skip = True\n\
                        for pattern in exclude.repository_patterns:\n\
                            if pattern in repo_name: skip = True\n\
                        \n\
                        if not skip:\n\
                            safe_invitations.append(inv)\n\n\
                 4. Process safe invitations only\n\n\
                 BEST PRACTICES:\n\
                 1. Always check organization before accepting\n\
                 2. Prioritize high-permission invitations (write+)\n\
                 3. Accept oldest invitations first to prevent expiration\n\
                 4. Group by organization for bulk operations\n\
                 5. Filter out suspicious inviters\n\
                 6. Keep track of accepted invitations\n\
                 7. Set up exclusion rules for known spam\n\
                 8. Review read-only invitations carefully\n\
                 9. Verify inviter identity before accepting admin access\n\
                 10. Document your filtering criteria for consistency",
            ),
        },
    ]
}

/// Integration with other GitHub tools
fn prompt_integration() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I integrate github_pending_invitations with other GitHub tools?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_pending_invitations tool works seamlessly with other GitHub tools to create powerful workflows for repository management and collaboration.\n\n\
                 INTEGRATION WITH ACCEPT_REPO_INVITATION:\n\n\
                 The most common integration - checking and accepting invitations:\n\n\
                 1. List pending invitations:\n\
                    invitations = github_pending_invitations({})\n\n\
                 2. Extract invitation IDs:\n\
                    For each invitation in invitations.invitations:\n\
                        invitation_id = invitation.id\n\n\
                 3. Accept desired invitations:\n\
                    github_accept_repo_invitation({\n\
                        \"invitation_id\": invitation_id\n\
                    })\n\n\
                 Complete workflow:\n\
                 // Step 1: Check invitations\n\
                 result = github_pending_invitations({})\n\
                 \n\
                 // Step 2: Review and accept\n\
                 if result.total_count > 0:\n\
                     for inv in result.invitations:\n\
                         print(f\"Found invitation to {inv.repository.full_name}\")\n\
                         print(f\"Inviter: {inv.inviter.login}\")\n\
                         print(f\"Permissions: {inv.permissions}\")\n\
                         \n\
                         // Accept if criteria met\n\
                         if inv.permissions in [\"write\", \"admin\"]:\n\
                             github_accept_repo_invitation({\n\
                                 \"invitation_id\": inv.id\n\
                             })\n\
                             print(f\"Accepted: {inv.repository.full_name}\")\n\n\
                 INTEGRATION WITH GET_FILE_CONTENTS:\n\n\
                 Preview repository before accepting invitation:\n\n\
                 1. Get pending invitations:\n\
                    invitations = github_pending_invitations({})\n\n\
                 2. For each invitation, preview public files:\n\
                    For each inv in invitations.invitations:\n\
                        owner, repo = inv.repository.full_name.split(\"/\")\n\
                        \n\
                        // Read README to understand project\n\
                        readme = github_get_file_contents({\n\
                            \"owner\": owner,\n\
                            \"repo\": repo,\n\
                            \"path\": \"README.md\"\n\
                        })\n\
                        \n\
                        // Review project description\n\
                        if \"relevant to my work\" in readme.content:\n\
                            github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 Note: This only works for public repositories or repositories where\n\
                 you already have some access. Private repos require accepting first.\n\n\
                 INTEGRATION WITH SEARCH_REPOSITORIES:\n\n\
                 Verify access after accepting invitations:\n\n\
                 1. Get and accept invitations:\n\
                    invitations = github_pending_invitations({})\n\
                    for inv in invitations.invitations:\n\
                        github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 2. Verify new repositories are accessible:\n\
                    all_repos = github_search_repositories({\n\
                        \"query\": \"org:myorg\"\n\
                    })\n\n\
                 3. Check that invited repositories appear in results:\n\
                    for repo in all_repos.items:\n\
                        print(f\"Can now access: {repo.full_name}\")\n\n\
                 INTEGRATION WITH LIST_ISSUES:\n\n\
                 Check repository activity before accepting:\n\n\
                 1. Get invitations:\n\
                    invitations = github_pending_invitations({})\n\n\
                 2. Check repository activity:\n\
                    For each inv in invitations.invitations:\n\
                        owner, repo = inv.repository.full_name.split(\"/\")\n\
                        \n\
                        // Check open issues to see if repo is active\n\
                        issues = github_list_issues({\n\
                            \"owner\": owner,\n\
                            \"repo\": repo,\n\
                            \"state\": \"open\",\n\
                            \"limit\": 10\n\
                        })\n\
                        \n\
                        issue_count = len(issues.items)\n\
                        print(f\"{inv.repository.full_name}: {issue_count} open issues\")\n\
                        \n\
                        // Accept if active repository\n\
                        if issue_count > 0:\n\
                            github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 INTEGRATION WITH LIST_COMMITS:\n\n\
                 Check repository recency:\n\n\
                 1. Get invitations:\n\
                    invitations = github_pending_invitations({})\n\n\
                 2. Check recent activity:\n\
                    For each inv in invitations.invitations:\n\
                        owner, repo = inv.repository.full_name.split(\"/\")\n\
                        \n\
                        // Get recent commits\n\
                        commits = github_list_commits({\n\
                            \"owner\": owner,\n\
                            \"repo\": repo,\n\
                            \"limit\": 5\n\
                        })\n\
                        \n\
                        if commits.items:\n\
                            last_commit_date = commits.items[0].commit.author.date\n\
                            print(f\"Last activity: {last_commit_date}\")\n\
                            \n\
                            // Accept if recently active\n\
                            age = calculate_days_since(last_commit_date)\n\
                            if age < 30:  // Active in last 30 days\n\
                                github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 INTEGRATION WITH GIT_CLONE:\n\n\
                 Accept and immediately clone repositories:\n\n\
                 1. Get invitations:\n\
                    invitations = github_pending_invitations({})\n\n\
                 2. Accept invitations:\n\
                    For each inv in invitations.invitations:\n\
                        // Accept invitation\n\
                        result = github_accept_repo_invitation({\n\
                            \"invitation_id\": inv.id\n\
                        })\n\
                        \n\
                        if result.success:\n\
                            // Clone immediately\n\
                            repo_name = inv.repository.full_name.split(\"/\")[1]\n\
                            git_clone({\n\
                                \"url\": f\"https://github.com/{inv.repository.full_name}.git\",\n\
                                \"path\": f\"/local/workspace/{repo_name}\"\n\
                            })\n\
                            print(f\"Cloned {inv.repository.full_name} to local workspace\")\n\n\
                 INTEGRATION WITH CREATE_BRANCH:\n\n\
                 Accept invitation and start working immediately:\n\n\
                 1. Accept invitation:\n\
                    invitations = github_pending_invitations({})\n\
                    inv = invitations.invitations[0]  // First invitation\n\
                    \n\
                    github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 2. Create feature branch to start work:\n\
                    owner, repo = inv.repository.full_name.split(\"/\")\n\
                    \n\
                    github_create_branch({\n\
                        \"owner\": owner,\n\
                        \"repo\": repo,\n\
                        \"branch_name\": \"feature/my-first-contribution\",\n\
                        \"from_branch\": \"main\"\n\
                    })\n\n\
                 3. Clone and checkout:\n\
                    git_clone({\n\
                        \"url\": f\"https://github.com/{inv.repository.full_name}.git\",\n\
                        \"path\": f\"/workspace/{repo}\",\n\
                        \"branch\": \"feature/my-first-contribution\"\n\
                    })\n\n\
                 INTEGRATION WITH LIST_BRANCHES:\n\n\
                 Check repository structure before accepting:\n\n\
                 1. Get invitations:\n\
                    invitations = github_pending_invitations({})\n\n\
                 2. Check branch structure:\n\
                    For each inv in invitations.invitations:\n\
                        owner, repo = inv.repository.full_name.split(\"/\")\n\
                        \n\
                        // See what branches exist\n\
                        branches = github_list_branches({\n\
                            \"owner\": owner,\n\
                            \"repo\": repo\n\
                        })\n\
                        \n\
                        print(f\"Repository {repo} has {len(branches.items)} branches\")\n\
                        \n\
                        // Accept if well-structured (multiple branches = active development)\n\
                        if len(branches.items) > 3:\n\
                            github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 INTEGRATION WITH GET_ME:\n\n\
                 Verify invitation is for correct account:\n\n\
                 1. Get current user info:\n\
                    me = github_get_me({})\n\
                    my_username = me.login\n\n\
                 2. Check invitations are for you:\n\
                    invitations = github_pending_invitations({})\n\
                    \n\
                    for inv in invitations.invitations:\n\
                        // Verify invitee matches current user\n\
                        if inv.invitee.login == my_username:\n\
                            print(f\"Invitation for {my_username}: {inv.repository.full_name}\")\n\
                            github_accept_repo_invitation({\"invitation_id\": inv.id})\n\
                        else:\n\
                            print(f\"WARNING: Invitation for different user: {inv.invitee.login}\")\n\n\
                 INTEGRATION WITH CREATE_ISSUE:\n\n\
                 Accept invitation and immediately create onboarding issue:\n\n\
                 1. Accept invitation:\n\
                    invitations = github_pending_invitations({})\n\
                    for inv in invitations.invitations:\n\
                        github_accept_repo_invitation({\"invitation_id\": inv.id})\n\
                        \n\
                        owner, repo = inv.repository.full_name.split(\"/\")\n\n\
                 2. Create onboarding issue:\n\
                    github_create_issue({\n\
                        \"owner\": owner,\n\
                        \"repo\": repo,\n\
                        \"title\": \"New team member onboarding\",\n\
                        \"body\": \"Hello! I just joined the repository. What should I work on first?\",\n\
                        \"labels\": [\"onboarding\", \"question\"]\n\
                    })\n\n\
                 INTEGRATION WITH SEARCH_ISSUES:\n\n\
                 Find onboarding resources after accepting:\n\n\
                 1. Accept invitations:\n\
                    invitations = github_pending_invitations({})\n\
                    for inv in invitations.invitations:\n\
                        github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 2. Search for onboarding issues:\n\
                    onboarding = github_search_issues({\n\
                        \"query\": \"repo:org/repo label:onboarding is:issue\"\n\
                    })\n\
                    \n\
                    for issue in onboarding.items:\n\
                        print(f\"Onboarding resource: {issue.title}\")\n\
                        print(f\"URL: {issue.html_url}\")\n\n\
                 MULTI-TOOL WORKFLOW EXAMPLE:\n\n\
                 Complete onboarding workflow:\n\n\
                 1. Check pending invitations:\n\
                    invitations = github_pending_invitations({})\n\
                    print(f\"Found {invitations.total_count} pending invitations\")\n\n\
                 2. Review and accept:\n\
                    accepted_repos = []\n\
                    for inv in invitations.invitations:\n\
                        // Filter: only accept write+ from known org\n\
                        if (inv.repository.full_name.startswith(\"myorg/\") and\n\
                            inv.permissions in [\"write\", \"admin\", \"maintain\"]):\n\
                            \n\
                            result = github_accept_repo_invitation({\"invitation_id\": inv.id})\n\
                            accepted_repos.append(inv.repository.full_name)\n\n\
                 3. Clone all accepted repositories:\n\
                    for repo_full_name in accepted_repos:\n\
                        repo_name = repo_full_name.split(\"/\")[1]\n\
                        git_clone({\n\
                            \"url\": f\"https://github.com/{repo_full_name}.git\",\n\
                            \"path\": f\"/workspace/{repo_name}\"\n\
                        })\n\n\
                 4. Get README for each:\n\
                    for repo_full_name in accepted_repos:\n\
                        owner, repo = repo_full_name.split(\"/\")\n\
                        readme = github_get_file_contents({\n\
                            \"owner\": owner,\n\
                            \"repo\": repo,\n\
                            \"path\": \"README.md\"\n\
                        })\n\
                        print(f\"\\n=== {repo_full_name} ===\")\n\
                        print(readme.content)\n\n\
                 5. Check for open issues:\n\
                    for repo_full_name in accepted_repos:\n\
                        owner, repo = repo_full_name.split(\"/\")\n\
                        issues = github_list_issues({\n\
                            \"owner\": owner,\n\
                            \"repo\": repo,\n\
                            \"state\": \"open\",\n\
                            \"limit\": 5\n\
                        })\n\
                        print(f\"\\n{repo_full_name} has {len(issues.items)} open issues\")\n\
                        for issue in issues.items:\n\
                            print(f\"  - {issue.title}\")\n\n\
                 BEST PRACTICES:\n\
                 1. Always verify repository details before accepting\n\
                 2. Clone repositories immediately after accepting\n\
                 3. Read README to understand project structure\n\
                 4. Check recent activity (commits, issues) to gauge activity\n\
                 5. Create feature branch to start contributing\n\
                 6. Search for onboarding issues or documentation\n\
                 7. Verify current user matches invitation recipient\n\
                 8. Use search tools to understand codebase\n\
                 9. Check branch structure for development workflow\n\
                 10. Integrate with CI/CD tools after setup",
            ),
        },
    ]
}

/// Comprehensive guide to pending invitations
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to managing GitHub repository invitations.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_pending_invitations tool is your gateway to managing repository access and collaboration opportunities. Here's everything you need to know.\n\n\
                 =============================================================================\n\
                 OVERVIEW: WHAT ARE REPOSITORY INVITATIONS?\n\
                 =============================================================================\n\n\
                 Repository invitations are formal requests to grant you access to private\n\
                 GitHub repositories. When someone invites you:\n\
                 - You receive email notification from GitHub\n\
                 - Invitation appears in your GitHub notifications\n\
                 - Invitation appears in github_pending_invitations tool\n\
                 - You must accept to gain access\n\
                 - Invitations expire (typically 7-30 days)\n\n\
                 WHY INVITATIONS MATTER:\n\
                 - Access to private repositories\n\
                 - Collaboration with teams\n\
                 - Contributing to closed-source projects\n\
                 - Organization membership\n\
                 - Work on confidential code\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 LISTING INVITATIONS:\n\
                 github_pending_invitations({})\n\n\
                 No parameters required - automatically fetches all pending invitations\n\
                 for the authenticated user (GITHUB_TOKEN).\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"invitations\": [\n\
                     {\n\
                       \"id\": 12345678,                    // Invitation ID (use to accept)\n\
                       \"repository\": {                    // Repository details\n\
                         \"id\": 987654321,\n\
                         \"name\": \"backend-api\",\n\
                       \"full_name\": \"myorg/backend-api\",\n\
                         \"private\": true,\n\
                         \"description\": \"Company backend API\",\n\
                         \"html_url\": \"https://github.com/myorg/backend-api\"\n\
                       },\n\
                       \"inviter\": {                      // Who invited you\n\
                         \"login\": \"tech-lead\",\n\
                         \"id\": 123456,\n\
                         \"avatar_url\": \"https://avatars.githubusercontent.com/u/123456\",\n\
                         \"html_url\": \"https://github.com/tech-lead\"\n\
                       },\n\
                       \"invitee\": {                      // You (recipient)\n\
                         \"login\": \"your-username\",\n\
                         \"id\": 789012\n\
                       },\n\
                       \"permissions\": \"write\",          // Access level\n\
                       \"created_at\": \"2025-12-01T10:30:00Z\",  // When invited\n\
                       \"html_url\": \"https://github.com/myorg/backend-api/invitations\"\n\
                     }\n\
                   ],\n\
                   \"total_count\": 1                       // Total invitations\n\
                 }\n\n\
                 =============================================================================\n\
                 PERMISSION LEVELS\n\
                 =============================================================================\n\n\
                 READ:\n\
                 - View repository code and history\n\
                 - Clone private repository\n\
                 - Read issues and pull requests\n\
                 - View discussions and wiki\n\
                 - Cannot make changes\n\
                 - Good for: Auditors, stakeholders, read-only access\n\n\
                 WRITE:\n\
                 - All read permissions PLUS:\n\
                 - Push commits directly\n\
                 - Create and merge pull requests\n\
                 - Manage issues and projects\n\
                 - Create releases\n\
                 - Most common for active contributors\n\
                 - Good for: Developers, contributors, team members\n\n\
                 MAINTAIN:\n\
                 - All write permissions PLUS:\n\
                 - Manage issues and PRs (close, label, milestone)\n\
                 - Some repository settings\n\
                 - Cannot delete or transfer repository\n\
                 - Good for: Team leads, project managers\n\n\
                 ADMIN:\n\
                 - Full repository control\n\
                 - All maintain permissions PLUS:\n\
                 - Manage collaborators\n\
                 - Configure webhooks and integrations\n\
                 - Change repository settings\n\
                 - Delete repository\n\
                 - Good for: Repository owners, senior maintainers\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 WORKFLOW 1: CHECK AND ACCEPT ALL\n\
                 Use when: You trust all inviters and want to accept everything\n\n\
                 1. Get invitations:\n\
                    invitations = github_pending_invitations({})\n\n\
                 2. Accept all:\n\
                    for inv in invitations.invitations:\n\
                        github_accept_repo_invitation({\"invitation_id\": inv.id})\n\
                        print(f\"Accepted: {inv.repository.full_name}\")\n\n\
                 WORKFLOW 2: SELECTIVE ACCEPTANCE\n\
                 Use when: You want to review before accepting\n\n\
                 1. Get invitations:\n\
                    invitations = github_pending_invitations({})\n\n\
                 2. Review each:\n\
                    for inv in invitations.invitations:\n\
                        print(f\"Repository: {inv.repository.full_name}\")\n\
                        print(f\"Inviter: {inv.inviter.login}\")\n\
                        print(f\"Permissions: {inv.permissions}\")\n\
                        print(f\"Description: {inv.repository.description}\")\n\
                        print(\"---\")\n\n\
                 3. Accept specific ones:\n\
                    github_accept_repo_invitation({\"invitation_id\": 12345678})\n\n\
                 WORKFLOW 3: ORGANIZATION-BASED\n\
                 Use when: Accepting all invitations from your company/team\n\n\
                 1. Get invitations:\n\
                    invitations = github_pending_invitations({})\n\n\
                 2. Filter by organization:\n\
                    for inv in invitations.invitations:\n\
                        org = inv.repository.full_name.split(\"/\")[0]\n\
                        if org == \"myorg\":\n\
                            github_accept_repo_invitation({\"invitation_id\": inv.id})\n\
                            print(f\"Accepted {org} repository: {inv.repository.full_name}\")\n\n\
                 WORKFLOW 4: PERMISSION-BASED\n\
                 Use when: Only accepting invitations with sufficient permissions\n\n\
                 1. Get invitations:\n\
                    invitations = github_pending_invitations({})\n\n\
                 2. Accept write+ only:\n\
                    for inv in invitations.invitations:\n\
                        if inv.permissions in [\"write\", \"admin\", \"maintain\"]:\n\
                            github_accept_repo_invitation({\"invitation_id\": inv.id})\n\
                            print(f\"Accepted with {inv.permissions}: {inv.repository.full_name}\")\n\n\
                 WORKFLOW 5: URGENT EXPIRATION HANDLING\n\
                 Use when: Need to accept old invitations before they expire\n\n\
                 1. Get invitations:\n\
                    invitations = github_pending_invitations({})\n\n\
                 2. Check age and accept old ones first:\n\
                    from datetime import datetime\n\
                    now = datetime.now()\n\
                    \n\
                    for inv in invitations.invitations:\n\
                        created = datetime.fromisoformat(inv.created_at.replace('Z', '+00:00'))\n\
                        age_days = (now - created).days\n\
                        \n\
                        if age_days > 20:  // Urgent - expiring soon\n\
                            github_accept_repo_invitation({\"invitation_id\": inv.id})\n\
                            print(f\"URGENT ({age_days} days old): {inv.repository.full_name}\")\n\n\
                 =============================================================================\n\
                 INTEGRATION WITH OTHER TOOLS\n\
                 =============================================================================\n\n\
                 ACCEPT INVITATION:\n\
                 invitations = github_pending_invitations({})\n\
                 github_accept_repo_invitation({\"invitation_id\": invitations.invitations[0].id})\n\n\
                 CLONE AFTER ACCEPTING:\n\
                 invitations = github_pending_invitations({})\n\
                 inv = invitations.invitations[0]\n\
                 github_accept_repo_invitation({\"invitation_id\": inv.id})\n\
                 git_clone({\n\
                     \"url\": f\"https://github.com/{inv.repository.full_name}.git\",\n\
                     \"path\": f\"/workspace/{inv.repository.name}\"\n\
                 })\n\n\
                 READ README BEFORE ACCEPTING:\n\
                 invitations = github_pending_invitations({})\n\
                 inv = invitations.invitations[0]\n\
                 owner, repo = inv.repository.full_name.split(\"/\")\n\
                 readme = github_get_file_contents({\"owner\": owner, \"repo\": repo, \"path\": \"README.md\"})\n\
                 // Review readme.content, then accept if relevant\n\
                 github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 CHECK REPOSITORY ACTIVITY:\n\
                 invitations = github_pending_invitations({})\n\
                 inv = invitations.invitations[0]\n\
                 owner, repo = inv.repository.full_name.split(\"/\")\n\
                 commits = github_list_commits({\"owner\": owner, \"repo\": repo, \"limit\": 5})\n\
                 // If active (recent commits), accept\n\
                 github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 =============================================================================\n\
                 AUTHENTICATION\n\
                 =============================================================================\n\n\
                 REQUIRED:\n\
                 - GITHUB_TOKEN environment variable\n\
                 - Token must have these scopes:\n\
                   - repo (full repository access)\n\
                   - OR user scope\n\n\
                 GETTING A TOKEN:\n\
                 1. Go to GitHub Settings > Developer settings > Personal access tokens\n\
                 2. Generate new token (classic)\n\
                 3. Select scopes: repo, user\n\
                 4. Generate and copy token\n\
                 5. Set environment variable: export GITHUB_TOKEN=ghp_your_token_here\n\n\
                 TESTING TOKEN:\n\
                 github_get_me({})\n\
                 // If successful, token is valid\n\n\
                 =============================================================================\n\
                 ERROR HANDLING\n\
                 =============================================================================\n\n\
                 NO INVITATIONS:\n\
                 Response: {\"invitations\": [], \"total_count\": 0}\n\
                 Meaning: No pending invitations\n\
                 Action: Nothing to do, check back later\n\n\
                 AUTHENTICATION ERROR:\n\
                 Error: 401 Unauthorized\n\
                 Meaning: GITHUB_TOKEN is missing or invalid\n\
                 Action: Check token is set and has correct scopes\n\n\
                 RATE LIMIT:\n\
                 Error: 403 Rate limit exceeded\n\
                 Meaning: Too many API calls\n\
                 Action: Wait 1 hour or use authenticated requests\n\n\
                 INVITATION EXPIRED:\n\
                 Error when accepting: 404 Not Found\n\
                 Meaning: Invitation no longer valid\n\
                 Action: Contact inviter to send new invitation\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. CHECK REGULARLY:\n\
                    Run weekly: github_pending_invitations({})\n\
                    Don't let invitations expire\n\n\
                 2. VERIFY INVITER:\n\
                    Always check inviter.login\n\
                    Ensure you recognize the person\n\
                    Watch for phishing attempts\n\n\
                 3. REVIEW PERMISSIONS:\n\
                    Understand what access you're granting\n\
                    Admin access should be rare\n\
                    Most work needs only write access\n\n\
                 4. CHECK ORGANIZATION:\n\
                    Verify repository.full_name starts with expected org\n\
                    Be cautious of unfamiliar organizations\n\n\
                 5. ACCEPT PROMPTLY:\n\
                    Don't delay on work-critical invitations\n\
                    Invitations expire (typically 7-30 days)\n\
                    Old invitations may need to be re-sent\n\n\
                 6. DOCUMENT ACCESS:\n\
                    Keep track of repositories you've joined\n\
                    Note why you have access\n\
                    Helpful for security audits\n\n\
                 7. CLONE AFTER ACCEPTING:\n\
                    Set up local development immediately\n\
                    Read README and contributing guidelines\n\
                    Understand project structure\n\n\
                 8. COORDINATE WITH TEAM:\n\
                    Confirm you should have access\n\
                    Understand your role in project\n\
                    Ask for onboarding if needed\n\n\
                 =============================================================================\n\
                 SECURITY CONSIDERATIONS\n\
                 =============================================================================\n\n\
                 RED FLAGS:\n\
                 - Invitation from unknown user\n\
                 - Repository name with typos (phishing)\n\
                 - Unexpected admin permissions\n\
                 - Organization you've never heard of\n\
                 - Suspicious repository descriptions\n\n\
                 SAFE PRACTICES:\n\
                 - Verify inviter through other channels\n\
                 - Check organization is legitimate\n\
                 - Review repository before accepting\n\
                 - Don't accept admin access unless necessary\n\
                 - Report suspicious invitations to security team\n\n\
                 IF SUSPICIOUS:\n\
                 - Don't accept the invitation\n\
                 - Contact inviter via work email/chat\n\
                 - Report to security team\n\
                 - Let invitation expire\n\n\
                 =============================================================================\n\
                 TROUBLESHOOTING\n\
                 =============================================================================\n\n\
                 PROBLEM: No invitations showing\n\
                 SOLUTIONS:\n\
                 - Check GitHub email for invitation notifications\n\
                 - Verify GITHUB_TOKEN has repo/user scopes\n\
                 - Check you haven't already accepted\n\
                 - Invitations may have expired\n\
                 - Ask inviter to resend\n\n\
                 PROBLEM: Can't accept invitation\n\
                 SOLUTIONS:\n\
                 - Verify invitation ID is correct\n\
                 - Check invitation hasn't expired\n\
                 - Ensure GITHUB_TOKEN has write permissions\n\
                 - Try refreshing invitation list\n\
                 - Contact repository admin\n\n\
                 PROBLEM: Accepted but can't access repository\n\
                 SOLUTIONS:\n\
                 - Wait a few minutes for GitHub to propagate\n\
                 - Refresh your GitHub session\n\
                 - Clone with authentication: https://token@github.com/org/repo.git\n\
                 - Check repository still exists\n\
                 - Verify invitation was actually accepted\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 List invitations:\n\
                 github_pending_invitations({})\n\n\
                 Accept invitation:\n\
                 github_accept_repo_invitation({\"invitation_id\": 12345678})\n\n\
                 Accept all from organization:\n\
                 for inv in github_pending_invitations({}).invitations:\n\
                     if inv.repository.full_name.startswith(\"myorg/\"):\n\
                         github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 Accept with write+ permissions:\n\
                 for inv in github_pending_invitations({}).invitations:\n\
                     if inv.permissions in [\"write\", \"admin\", \"maintain\"]:\n\
                         github_accept_repo_invitation({\"invitation_id\": inv.id})\n\n\
                 Clone after accepting:\n\
                 inv = github_pending_invitations({}).invitations[0]\n\
                 github_accept_repo_invitation({\"invitation_id\": inv.id})\n\
                 git_clone({\"url\": f\"https://github.com/{inv.repository.full_name}.git\", \"path\": \"/workspace\"})\n\n\
                 Remember: Check invitations regularly, verify inviters, and accept promptly!",
            ),
        },
    ]
}
