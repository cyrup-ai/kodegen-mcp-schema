//! Prompt messages for github_accept_repo_invitation tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GithubAcceptRepoInvitationPromptArgs;

/// Prompt provider for accept_repo_invitation tool
///
/// This is the ONLY way to provide prompts for accept_repo_invitation - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct GithubAcceptRepoInvitationPrompts;

impl PromptProvider for GithubAcceptRepoInvitationPrompts {
    type PromptArgs = GithubAcceptRepoInvitationPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("workflows") => prompt_workflows(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show examples for (basic, workflows)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO ACCEPT REPOSITORY INVITATIONS
// ============================================================================

/// Basic invitation acceptance
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I accept repository invitations using the github_accept_repo_invitation tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The github_accept_repo_invitation tool accepts pending repository invitations, granting you access to private repositories and collaboration opportunities.\n\n\
                 ACCEPTING REPOSITORY INVITATIONS:\n\n\
                 1. Accept invitation with invitation ID:\n\
                    github_accept_repo_invitation({\n\
                        \"invitation_id\": 12345678\n\
                    })\n\n\
                 2. Typical workflow - check pending invitations first:\n\
                    // Step 1: List pending invitations to get invitation IDs\n\
                    github_pending_invitations({})\n\
                    // Returns list with invitation_id for each pending invitation\n\
                    \n\
                    // Step 2: Accept the invitation using the ID\n\
                    github_accept_repo_invitation({\n\
                        \"invitation_id\": 12345678\n\
                    })\n\n\
                 RESPONSE FORMAT:\n\
                 {\n\
                   \"success\": true,\n\
                   \"repository\": \"org/private-repo\",\n\
                   \"invitation_id\": 12345678,\n\
                   \"permissions\": \"write\",\n\
                   \"message\": \"Successfully accepted invitation to org/private-repo\"\n\
                 }\n\n\
                 AFTER ACCEPTING:\n\
                 - Full access to repository granted immediately\n\
                 - Can clone, pull, and view private repository\n\
                 - Push/write access depends on permission level you were invited with\n\
                 - Repository appears in your GitHub repositories list\n\
                 - Permissions: read, write, admin, or maintain (depending on invitation)\n\n\
                 PARAMETERS:\n\
                 - invitation_id (required): Numeric ID of the pending invitation\n\
                   - Get this from github_pending_invitations tool\n\
                   - Must be a valid pending invitation for your account\n\
                   - Cannot accept already-accepted or expired invitations\n\n\
                 AUTHENTICATION:\n\
                 Requires GITHUB_TOKEN environment variable with scopes:\n\
                 - repo (full repository access)\n\
                 - Or user scope for accepting organization invitations\n\n\
                 COMMON USE CASES:\n\
                 1. Joining team repositories:\n\
                    - Team lead invites you to private project repository\n\
                    - Accept invitation to gain access\n\
                    - Start contributing immediately\n\
                 \n\
                 2. Collaborating on open source:\n\
                    - Maintainer grants you write access to contribute\n\
                    - Accept invitation to become a collaborator\n\
                    - Can push branches and create pull requests directly\n\
                 \n\
                 3. Accessing private forks:\n\
                    - Someone shares their private fork with you\n\
                    - Accept to view and collaborate on their work\n\
                    - Useful for code reviews and pair programming\n\n\
                 ERROR SCENARIOS:\n\
                 1. 404 Not Found:\n\
                    - Invitation ID doesn't exist or already accepted\n\
                    - Invitation may have been withdrawn by sender\n\
                    - Invitation may have expired (typically 7 days)\n\
                    Fix: Check github_pending_invitations for current valid IDs\n\
                 \n\
                 2. 403 Forbidden:\n\
                    - GITHUB_TOKEN lacks required scopes\n\
                    - Token doesn't have 'repo' or 'user' scope\n\
                    Fix: Generate new token with repo scope\n\
                 \n\
                 3. 401 Unauthorized:\n\
                    - GITHUB_TOKEN is invalid or expired\n\
                    - No token provided\n\
                    Fix: Set valid GITHUB_TOKEN environment variable\n\n\
                 RATE LIMITING:\n\
                 - Authenticated: 5,000 requests/hour\n\
                 - Unauthenticated: Not applicable (requires auth)\n\
                 - Check X-RateLimit-Remaining header in response\n\
                 - Accepting invitations counts against your API quota\n\n\
                 BEST PRACTICES:\n\
                 - Always list pending invitations first to get valid IDs\n\
                 - Accept invitations promptly (they expire after 7 days)\n\
                 - Verify repository name in response before proceeding\n\
                 - Check permission level to understand your access rights\n\
                 - Test access by attempting to clone after accepting\n\
                 - Keep track of which repositories you've gained access to\n\n\
                 VERIFICATION:\n\
                 After accepting, verify access:\n\
                 1. Try cloning the repository:\n\
                    git_clone({\n\
                        \"url\": \"https://github.com/org/private-repo.git\",\n\
                        \"path\": \"/local/path\"\n\
                    })\n\
                 \n\
                 2. List your repositories to confirm it appears:\n\
                    github_list_repositories({\n\
                        \"affiliation\": \"collaborator\"\n\
                    })",
            ),
        },
    ]
}

/// Complete invitation workflows
fn prompt_workflows() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are common workflows for managing and accepting GitHub repository invitations?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Here are comprehensive workflows for managing GitHub repository invitations effectively:\n\n\
                 WORKFLOW 1: CHECK AND ACCEPT ALL PENDING INVITATIONS\n\
                 Perfect for processing multiple invitations at once:\n\
                 \n\
                 // Step 1: List all pending invitations\n\
                 github_pending_invitations({})\n\
                 // Returns: [\n\
                 //   {\"invitation_id\": 111, \"repository\": \"team/project-a\", \"permissions\": \"write\"},\n\
                 //   {\"invitation_id\": 222, \"repository\": \"org/project-b\", \"permissions\": \"admin\"},\n\
                 //   {\"invitation_id\": 333, \"repository\": \"user/project-c\", \"permissions\": \"read\"}\n\
                 // ]\n\
                 \n\
                 // Step 2: Review and accept each invitation\n\
                 github_accept_repo_invitation({\"invitation_id\": 111})\n\
                 github_accept_repo_invitation({\"invitation_id\": 222})\n\
                 github_accept_repo_invitation({\"invitation_id\": 333})\n\
                 \n\
                 // Step 3: Verify all repositories are accessible\n\
                 github_list_repositories({\"affiliation\": \"collaborator\"})\n\n\
                 WORKFLOW 2: SELECTIVE ACCEPTANCE WITH VERIFICATION\n\
                 When you want to carefully review before accepting:\n\
                 \n\
                 // Step 1: Get pending invitations\n\
                 github_pending_invitations({})\n\
                 \n\
                 // Step 2: For each invitation, check repository details\n\
                 github_get_repository({\n\
                     \"owner\": \"team\",\n\
                     \"repo\": \"project-a\"\n\
                 })\n\
                 // Review: description, language, stars, topics, etc.\n\
                 \n\
                 // Step 3: Accept only if it matches your criteria\n\
                 github_accept_repo_invitation({\"invitation_id\": 111})\n\
                 \n\
                 // Step 4: Verify access by listing repository contents\n\
                 github_get_file_contents({\n\
                     \"owner\": \"team\",\n\
                     \"repo\": \"project-a\",\n\
                     \"path\": \"README.md\"\n\
                 })\n\n\
                 WORKFLOW 3: ACCEPT AND IMMEDIATELY CLONE\n\
                 When you need to start working right away:\n\
                 \n\
                 // Step 1: Accept the invitation\n\
                 github_accept_repo_invitation({\"invitation_id\": 12345})\n\
                 // Response includes repository name: \"org/private-repo\"\n\
                 \n\
                 // Step 2: Clone repository to local machine\n\
                 git_clone({\n\
                     \"url\": \"https://github.com/org/private-repo.git\",\n\
                     \"path\": \"/projects/private-repo\"\n\
                 })\n\
                 \n\
                 // Step 3: Verify clone and check branches\n\
                 terminal({\n\
                     \"command\": \"cd /projects/private-repo && git branch -a\"\n\
                 })\n\
                 \n\
                 // Step 4: Start working on a new branch\n\
                 github_create_branch({\n\
                     \"owner\": \"org\",\n\
                     \"repo\": \"private-repo\",\n\
                     \"branch\": \"feature/my-contribution\",\n\
                     \"from_branch\": \"main\"\n\
                 })\n\n\
                 WORKFLOW 4: ORGANIZATION ONBOARDING\n\
                 When joining a new organization with multiple repositories:\n\
                 \n\
                 // Step 1: Accept organization invitation (if applicable)\n\
                 github_accept_org_invitation({\"org\": \"my-company\"})\n\
                 \n\
                 // Step 2: List all pending repository invitations\n\
                 github_pending_invitations({})\n\
                 // You may have invitations to multiple org repositories\n\
                 \n\
                 // Step 3: Accept all organization repository invitations\n\
                 github_accept_repo_invitation({\"invitation_id\": 101})  // my-company/frontend\n\
                 github_accept_repo_invitation({\"invitation_id\": 102})  // my-company/backend\n\
                 github_accept_repo_invitation({\"invitation_id\": 103})  // my-company/shared-lib\n\
                 \n\
                 // Step 4: Clone all required repositories for local development\n\
                 git_clone({\n\
                     \"url\": \"https://github.com/my-company/frontend.git\",\n\
                     \"path\": \"/workspace/frontend\"\n\
                 })\n\
                 git_clone({\n\
                     \"url\": \"https://github.com/my-company/backend.git\",\n\
                     \"path\": \"/workspace/backend\"\n\
                 })\n\
                 git_clone({\n\
                     \"url\": \"https://github.com/my-company/shared-lib.git\",\n\
                     \"path\": \"/workspace/shared-lib\"\n\
                 })\n\n\
                 WORKFLOW 5: AUTOMATED INVITATION MONITORING\n\
                 For regularly checking and processing new invitations:\n\
                 \n\
                 // Step 1: Check for new invitations (run periodically)\n\
                 github_pending_invitations({})\n\
                 \n\
                 // Step 2: Filter invitations by criteria\n\
                 // Example: Only accept invitations with 'write' or 'admin' permissions\n\
                 // Example: Only accept from specific organizations\n\
                 // Example: Only accept repositories matching naming patterns\n\
                 \n\
                 // Step 3: Auto-accept based on rules\n\
                 // If invitation matches criteria:\n\
                 github_accept_repo_invitation({\"invitation_id\": 456})\n\
                 \n\
                 // Step 4: Log accepted invitations for tracking\n\
                 // Store in database or append to log file\n\n\
                 USE CASES BY SCENARIO:\n\
                 \n\
                 1. JOINING TEAM REPOSITORIES:\n\
                    - New team member onboarding\n\
                    - Contractor gaining access to client projects\n\
                    - Cross-team collaboration on shared services\n\
                    Pattern: Accept all, then clone required repos\n\
                 \n\
                 2. ACCESSING PRIVATE PROJECTS:\n\
                    - Beta testing access to private repositories\n\
                    - Early access to unreleased projects\n\
                    - Academic collaboration on research code\n\
                    Pattern: Selective acceptance with verification\n\
                 \n\
                 3. COLLABORATING ON OPEN SOURCE:\n\
                    - Becoming a maintainer of open source project\n\
                    - Joining organization as trusted contributor\n\
                    - Getting write access to submit fixes directly\n\
                    Pattern: Accept, clone, create feature branch\n\
                 \n\
                 4. CODE REVIEW ACCESS:\n\
                    - External reviewer invited to private repository\n\
                    - Security audit access to source code\n\
                    - Technical due diligence for acquisitions\n\
                    Pattern: Accept read-only, review without cloning\n\n\
                 INTEGRATION WITH OTHER TOOLS:\n\
                 \n\
                 After accepting invitations, you can use:\n\
                 - github_list_branches: See available branches\n\
                 - github_list_issues: View open issues to work on\n\
                 - github_list_pull_requests: Check ongoing PRs\n\
                 - github_get_file_contents: Browse repository files\n\
                 - git_clone: Clone repository locally\n\
                 - github_create_branch: Start new work\n\
                 - github_push_files: Contribute changes\n\
                 - github_create_pull_request: Submit contributions\n\n\
                 PERMISSION LEVELS EXPLAINED:\n\
                 - READ: View code, issues, PRs (clone only)\n\
                 - WRITE: Push commits, create branches, manage issues/PRs\n\
                 - ADMIN: Full access including settings, collaborators, webhooks\n\
                 - MAINTAIN: Between write and admin, can manage some settings\n\n\
                 TROUBLESHOOTING:\n\
                 \n\
                 Problem: Invitation not appearing in pending list\n\
                 - Check email for invitation link\n\
                 - Invitation may have been withdrawn\n\
                 - May have been sent to different account\n\
                 - Invitation may have expired (7 day limit)\n\
                 \n\
                 Problem: Cannot clone after accepting\n\
                 - Wait a few seconds for GitHub to propagate permissions\n\
                 - Check GITHUB_TOKEN has correct scopes\n\
                 - Verify repository name is correct\n\
                 - Ensure using HTTPS URL with token authentication\n\
                 \n\
                 Problem: Accepted wrong invitation\n\
                 - Cannot undo acceptance via API\n\
                 - Must manually remove yourself from repository\n\
                 - Go to repository Settings > Collaborators\n\
                 - Or ask repository owner to remove you",
            ),
        },
    ]
}

/// Complete invitation acceptance guide
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to accepting GitHub repository invitations including all scenarios, workflows, and best practices.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "COMPLETE GUIDE TO GITHUB REPOSITORY INVITATION ACCEPTANCE\n\n\
                 ===========================================================\n\n\
                 OVERVIEW:\n\
                 The github_accept_repo_invitation tool accepts pending repository invitations,\n\
                 granting you access to private repositories and enabling collaboration.\n\n\
                 BASIC USAGE:\n\
                 ============\n\n\
                 1. SIMPLE ACCEPTANCE:\n\
                    github_accept_repo_invitation({\n\
                        \"invitation_id\": 12345678\n\
                    })\n\
                    \n\
                    Response:\n\
                    {\n\
                      \"success\": true,\n\
                      \"repository\": \"org/private-repo\",\n\
                      \"invitation_id\": 12345678,\n\
                      \"permissions\": \"write\",\n\
                      \"message\": \"Successfully accepted invitation to org/private-repo\"\n\
                    }\n\n\
                 2. FINDING INVITATION IDS:\n\
                    Before accepting, you need the invitation ID:\n\
                    \n\
                    github_pending_invitations({})\n\
                    \n\
                    Returns list of pending invitations:\n\
                    [\n\
                      {\n\
                        \"invitation_id\": 12345678,\n\
                        \"repository\": \"org/private-repo\",\n\
                        \"inviter\": \"username\",\n\
                        \"permissions\": \"write\",\n\
                        \"created_at\": \"2024-01-15T10:30:00Z\"\n\
                      }\n\
                    ]\n\n\
                 PARAMETERS:\n\
                 ===========\n\n\
                 invitation_id (required):\n\
                 - Type: Integer\n\
                 - Description: Unique identifier for the pending invitation\n\
                 - Source: Obtained from github_pending_invitations tool\n\
                 - Constraints: Must be valid, pending, and belong to your account\n\
                 - Example: 12345678\n\n\
                 RESPONSE FIELDS:\n\
                 ===============\n\n\
                 - success: Boolean indicating if acceptance succeeded\n\
                 - repository: Full repository name in \"owner/repo\" format\n\
                 - invitation_id: The ID that was accepted (echoed back)\n\
                 - permissions: Your permission level (read, write, admin, maintain)\n\
                 - message: Human-readable status message\n\
                 - html_url: Direct link to the repository (if successful)\n\n\
                 PERMISSION LEVELS:\n\
                 ==================\n\n\
                 READ (Pull):\n\
                 - Clone repository\n\
                 - View code, issues, pull requests\n\
                 - Download releases\n\
                 - Cannot push commits or create branches\n\
                 \n\
                 WRITE (Push):\n\
                 - All read permissions\n\
                 - Push commits to repository\n\
                 - Create, edit, delete branches\n\
                 - Create, close, comment on issues\n\
                 - Create, merge, close pull requests\n\
                 - Apply labels and milestones\n\
                 \n\
                 MAINTAIN:\n\
                 - All write permissions\n\
                 - Manage repository settings (some)\n\
                 - Manage webhooks and deploy keys\n\
                 - Cannot delete repository or change ownership\n\
                 \n\
                 ADMIN:\n\
                 - All maintain permissions\n\
                 - Full repository settings access\n\
                 - Add/remove collaborators\n\
                 - Delete repository (if owner)\n\
                 - Change repository visibility\n\
                 - Manage teams (for organizations)\n\n\
                 AUTHENTICATION:\n\
                 ===============\n\n\
                 Required: GITHUB_TOKEN environment variable\n\
                 \n\
                 Token Scopes Needed:\n\
                 - repo: Full control of private repositories\n\
                 - public_repo: Access to public repositories (if token limited)\n\
                 - user: User email address access (for org invitations)\n\
                 \n\
                 Token Generation:\n\
                 1. Go to GitHub Settings > Developer settings > Personal access tokens\n\
                 2. Click \"Generate new token (classic)\"\n\
                 3. Select 'repo' scope for full access\n\
                 4. Generate and copy token\n\
                 5. Set as environment variable: export GITHUB_TOKEN=ghp_...\n\n\
                 COMPLETE WORKFLOWS:\n\
                 ===================\n\n\
                 WORKFLOW 1: Daily Invitation Check\n\
                 -----------------------------------\n\
                 // Morning routine: Check and accept new invitations\n\
                 github_pending_invitations({})\n\
                 // Review each invitation\n\
                 github_accept_repo_invitation({\"invitation_id\": 111})\n\
                 github_accept_repo_invitation({\"invitation_id\": 222})\n\
                 \n\
                 WORKFLOW 2: New Team Member Onboarding\n\
                 ---------------------------------------\n\
                 // Accept organization membership\n\
                 github_accept_org_invitation({\"org\": \"company\"})\n\
                 // Accept all repository invitations\n\
                 github_pending_invitations({})\n\
                 github_accept_repo_invitation({\"invitation_id\": 101})\n\
                 github_accept_repo_invitation({\"invitation_id\": 102})\n\
                 github_accept_repo_invitation({\"invitation_id\": 103})\n\
                 // Clone repositories for local development\n\
                 git_clone({\"url\": \"https://github.com/company/repo1.git\", \"path\": \"/work/repo1\"})\n\
                 git_clone({\"url\": \"https://github.com/company/repo2.git\", \"path\": \"/work/repo2\"})\n\
                 \n\
                 WORKFLOW 3: Selective Acceptance with Validation\n\
                 -------------------------------------------------\n\
                 // Get pending invitations\n\
                 github_pending_invitations({})\n\
                 // For each, check repository details\n\
                 github_get_repository({\"owner\": \"org\", \"repo\": \"project\"})\n\
                 // Accept if meets criteria\n\
                 github_accept_repo_invitation({\"invitation_id\": 456})\n\
                 // Verify access\n\
                 github_get_file_contents({\"owner\": \"org\", \"repo\": \"project\", \"path\": \"README.md\"})\n\
                 \n\
                 WORKFLOW 4: Accept and Start Contributing\n\
                 ------------------------------------------\n\
                 // Accept invitation\n\
                 github_accept_repo_invitation({\"invitation_id\": 789})\n\
                 // Clone repository\n\
                 git_clone({\"url\": \"https://github.com/org/repo.git\", \"path\": \"/projects/repo\"})\n\
                 // Create feature branch\n\
                 github_create_branch({\"owner\": \"org\", \"repo\": \"repo\", \"branch\": \"feature/my-work\"})\n\
                 // Make changes and push\n\
                 github_push_files({\"owner\": \"org\", \"repo\": \"repo\", \"branch\": \"feature/my-work\", \"files\": [...]})\n\
                 // Create pull request\n\
                 github_create_pull_request({\"owner\": \"org\", \"repo\": \"repo\", \"title\": \"My contribution\"})\n\n\
                 ERROR HANDLING:\n\
                 ===============\n\n\
                 404 Not Found:\n\
                 - Cause: Invitation doesn't exist, expired, or already accepted\n\
                 - Solution: Run github_pending_invitations to get current valid IDs\n\
                 - Prevention: Accept invitations within 7 days of receipt\n\
                 \n\
                 403 Forbidden:\n\
                 - Cause: GITHUB_TOKEN lacks required scopes\n\
                 - Solution: Generate new token with 'repo' scope\n\
                 - Check: Verify token has not been revoked or restricted\n\
                 \n\
                 401 Unauthorized:\n\
                 - Cause: Missing or invalid GITHUB_TOKEN\n\
                 - Solution: Set valid token: export GITHUB_TOKEN=ghp_...\n\
                 - Check: Ensure token hasn't expired (tokens can have expiration)\n\
                 \n\
                 422 Unprocessable Entity:\n\
                 - Cause: Invalid invitation_id format or already processed\n\
                 - Solution: Use integer ID from github_pending_invitations\n\
                 - Check: Ensure invitation_id is a number, not a string\n\n\
                 RATE LIMITING:\n\
                 ==============\n\n\
                 Limits:\n\
                 - Authenticated requests: 5,000 per hour\n\
                 - Unauthenticated: Not applicable (authentication required)\n\
                 - Secondary rate limit: 100 invitations accepted per hour\n\
                 \n\
                 Headers to Monitor:\n\
                 - X-RateLimit-Limit: Maximum requests allowed\n\
                 - X-RateLimit-Remaining: Requests remaining in window\n\
                 - X-RateLimit-Reset: Unix timestamp when limit resets\n\
                 \n\
                 Best Practices:\n\
                 - Batch accept invitations rather than one-by-one over time\n\
                 - Cache invitation list to avoid repeated pending_invitations calls\n\
                 - Monitor remaining quota before bulk operations\n\n\
                 BEST PRACTICES:\n\
                 ===============\n\n\
                 1. LIST BEFORE ACCEPTING:\n\
                    Always run github_pending_invitations first to see all options\n\
                    and get valid invitation IDs.\n\
                 \n\
                 2. VERIFY REPOSITORY DETAILS:\n\
                    Check repository name, owner, and permissions before accepting.\n\
                    Use github_get_repository to see description and metadata.\n\
                 \n\
                 3. ACCEPT PROMPTLY:\n\
                    Invitations expire after 7 days. Accept as soon as you're ready\n\
                    to avoid having to request re-invitation.\n\
                 \n\
                 4. CONFIRM ACCESS:\n\
                    After accepting, verify you can access the repository by attempting\n\
                    to clone it or view its contents.\n\
                 \n\
                 5. TRACK PERMISSIONS:\n\
                    Note what permission level you received. This determines what\n\
                    operations you can perform on the repository.\n\
                 \n\
                 6. SECURE YOUR TOKEN:\n\
                    Never commit GITHUB_TOKEN to repositories or share it.\n\
                    Use environment variables and secret management.\n\
                 \n\
                 7. ORGANIZATION INVITATIONS:\n\
                    When joining organizations, accept org invitation before\n\
                    accepting repository invitations.\n\n\
                 SECURITY CONSIDERATIONS:\n\
                 ========================\n\n\
                 - Review repository source before accepting from unknown entities\n\
                 - Be cautious of invitations from unfamiliar users or organizations\n\
                 - Check repository activity and contributor history\n\
                 - Understand that accepting grants the inviter visibility of your acceptance\n\
                 - Your association with the repository becomes public (for public repos)\n\
                 - Private repository access doesn't expose your commits publicly\n\
                 - Admin access should only be accepted from highly trusted sources\n\n\
                 INTEGRATION EXAMPLES:\n\
                 =====================\n\n\
                 Example 1: Accept and immediately start work\n\
                 github_accept_repo_invitation({\"invitation_id\": 123})\n\
                 git_clone({\"url\": \"https://github.com/org/repo.git\", \"path\": \"/work/repo\"})\n\
                 terminal({\"command\": \"cd /work/repo && git checkout -b feature/new-work\"})\n\
                 \n\
                 Example 2: Accept multiple and setup workspace\n\
                 github_pending_invitations({})  // Get all pending\n\
                 github_accept_repo_invitation({\"invitation_id\": 111})\n\
                 github_accept_repo_invitation({\"invitation_id\": 222})\n\
                 terminal({\"command\": \"mkdir -p /workspace && cd /workspace\"})\n\
                 git_clone({\"url\": \"https://github.com/team/frontend.git\", \"path\": \"/workspace/frontend\"})\n\
                 git_clone({\"url\": \"https://github.com/team/backend.git\", \"path\": \"/workspace/backend\"})\n\
                 \n\
                 Example 3: Automated acceptance with filtering\n\
                 pending = github_pending_invitations({})\n\
                 // Filter for specific organization\n\
                 for invitation in pending where repository.startsWith(\"myorg/\"):\n\
                     github_accept_repo_invitation({\"invitation_id\": invitation.id})\n\n\
                 COMMON QUESTIONS:\n\
                 =================\n\n\
                 Q: How do I know if I have pending invitations?\n\
                 A: Run github_pending_invitations({}). Returns empty array if none.\n\
                 \n\
                 Q: Can I decline an invitation?\n\
                 A: Yes, use github_decline_repo_invitation tool, or simply ignore it\n\
                    and it will expire after 7 days.\n\
                 \n\
                 Q: What happens if I accept the same invitation twice?\n\
                 A: Second attempt returns 404 - invitation no longer exists after\n\
                    first acceptance.\n\
                 \n\
                 Q: Can I remove myself after accepting?\n\
                 A: Not via API. Must manually remove yourself from repository settings\n\
                    or ask repository owner to remove you.\n\
                 \n\
                 Q: How long do invitations last?\n\
                 A: 7 days from creation. After expiration, inviter must send new invitation.\n\
                 \n\
                 Q: Do I need organization membership to accept repository invitations?\n\
                 A: Not always. Individual collaborator invitations work without org membership.\n\
                    Organization repository invitations may require accepting org membership first.",
            ),
        },
    ]
}
