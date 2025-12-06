//! Prompt messages for git_stash_list tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitStashListPromptArgs;

/// Prompt provider for git_stash_list tool
///
/// This is the ONLY way to provide prompts for git_stash_list - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct StashListPrompts;

impl PromptProvider for StashListPrompts {
    type PromptArgs = GitStashListPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("details") => prompt_details(),
            Some("management") => prompt_management(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, details, management)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST AND MANAGE STASHES
// ============================================================================

/// Basic stash listing operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I see all my stashed changes?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_stash_list to see all stashed changes in a repository. This shows you what work has been temporarily saved.\n\n\
                 LISTING STASHES:\n\n\
                 1. List all stashes:\n\
                    git_stash_list({\n\
                        \"path\": \"/project\"\n\
                    })\n\n\
                 RESPONSE FORMAT:\n\
                 {\n\
                   \"stashes\": [\n\
                     {\n\
                       \"index\": 0,\n\
                       \"ref\": \"stash@{0}\",\n\
                       \"branch\": \"main\",\n\
                       \"message\": \"WIP on main: abc1234 Previous commit\"\n\
                     },\n\
                     {\n\
                       \"index\": 1,\n\
                       \"ref\": \"stash@{1}\",\n\
                       \"branch\": \"feature/x\",\n\
                       \"message\": \"feature work in progress\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 UNDERSTANDING STASH ENTRIES:\n\
                 - index: Stash number (0 is most recent)\n\
                 - ref: Full stash reference to use with other git commands\n\
                 - branch: Which branch the stash was created on\n\
                 - message: Description of what was stashed\n\n\
                 STASH ENTRY FORMAT:\n\
                 - stash@{0}: Most recent stash (created last)\n\
                 - stash@{1}: Second most recent\n\
                 - stash@{N}: Older stashes have higher numbers\n\
                 - Branch: Shows where stash originated\n\
                 - Message: Usually \"WIP on <branch>: <commit>\" or custom message\n\n\
                 WHEN TO LIST STASHES:\n\
                 - Before creating a new stash (see what's already there)\n\
                 - When deciding which stash to apply\n\
                 - To find a specific piece of work you stashed\n\
                 - Before cleaning up old stashes\n\
                 - To verify a stash was created successfully\n\n\
                 COMMON PATTERNS:\n\
                 1. Check what's stashed:\n\
                    git_stash_list({\"path\": \"/repo\"})\n\n\
                 2. Find specific work:\n\
                    git_stash_list({\"path\": \"/repo\"})\n\
                    // Look for stash with relevant message\n\n\
                 3. Verify stash creation:\n\
                    git_stash_save({\"path\": \"/repo\", \"message\": \"New feature\"})\n\
                    git_stash_list({\"path\": \"/repo\"})\n\
                    // Should see new stash at index 0\n\n\
                 EMPTY STASH LIST:\n\
                 If the response shows an empty stashes array, it means:\n\
                 - No work has been stashed in this repository\n\
                 - All previous stashes have been applied/dropped\n\
                 - This is the normal state for most repositories",
            ),
        },
    ]
}

/// Detailed stash information and examination
fn prompt_details() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I examine the contents of a specific stash?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_stash_list to find stashes, then examine their contents with other git tools. This helps you decide which stash to apply or drop.\n\n\
                 DETAILED STASH INFO:\n\n\
                 1. List stashes to get references:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // Returns stash references like stash@{0}, stash@{1}\n\n\
                 2. Show stash contents as diff:\n\
                    git_show({\n\
                        \"path\": \"/project\",\n\
                        \"object\": \"stash@{0}\"\n\
                    })\n\
                    // See full diff of what's in the stash\n\n\
                 3. See which files are in a stash:\n\
                    git_diff({\n\
                        \"path\": \"/project\",\n\
                        \"from\": \"stash@{0}^!\"\n\
                    })\n\
                    // Shows files modified in the stash\n\n\
                 EXAMINING STASH WORKFLOW:\n\
                 Step 1: List all stashes\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 Response:\n\
                 {\n\
                   \"stashes\": [\n\
                     {\"index\": 0, \"ref\": \"stash@{0}\", \"message\": \"WIP: authentication\"},\n\
                     {\"index\": 1, \"ref\": \"stash@{1}\", \"message\": \"WIP: database schema\"},\n\
                     {\"index\": 2, \"ref\": \"stash@{2}\", \"message\": \"bug fix attempt\"}\n\
                   ]\n\
                 }\n\n\
                 Step 2: Pick stash to examine (e.g., stash@{1})\n\
                 git_show({\"path\": \"/repo\", \"object\": \"stash@{1}\"})\n\
                 // Shows full diff of database schema changes\n\n\
                 Step 3: Decide what to do\n\
                 - If needed: git_stash_apply({\"path\": \"/repo\", \"stash\": \"stash@{1}\"})\n\
                 - If not needed: git_stash_drop({\"path\": \"/repo\", \"stash\": \"stash@{1}\"})\n\n\
                 UNDERSTANDING STASH REFERENCES:\n\
                 - stash@{0}: Most recent stash (use this reference in other commands)\n\
                 - stash@{0}^: Parent commit of the stash\n\
                 - stash@{0}^!: The stash commit itself (for diffs)\n\
                 - stash@{0}^1: First parent (base commit)\n\
                 - stash@{0}^2: Second parent (index state, if exists)\n\
                 - stash@{0}^3: Third parent (untracked files, if exists)\n\n\
                 STASH MESSAGE INTERPRETATION:\n\
                 - \"WIP on branch: commit_hash commit_message\": Default message\n\
                 - Custom message: User provided description\n\
                 - Branch shows where stash was created\n\
                 - Commit shows base state before stashing\n\n\
                 DECIDING WHICH STASH TO USE:\n\
                 1. Read messages to find relevant work\n\
                 2. Check branch to see original context\n\
                 3. Use git_show to review actual changes\n\
                 4. Apply the one you need\n\
                 5. Drop obsolete stashes\n\n\
                 STASH INSPECTION BEST PRACTICES:\n\
                 - Always list before applying/dropping\n\
                 - Review contents with git_show before applying\n\
                 - Check which branch stash was created on\n\
                 - Consider if stash is still relevant\n\
                 - Use descriptive messages when creating stashes (helps later)",
            ),
        },
    ]
}

/// Managing multiple stashes and cleanup workflows
fn prompt_management() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I manage multiple stashes and keep them organized?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_stash_list as the foundation for managing stashed work. List stashes frequently to keep track of what's saved and clean up old entries.\n\n\
                 STASH MANAGEMENT:\n\n\
                 1. Check existing stashes before saving new work:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // See what's already stashed\n\
                    git_stash_save({\n\
                        \"path\": \"/project\",\n\
                        \"message\": \"New feature work\"\n\
                    })\n\
                    // Save new stash with descriptive message\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // Verify it appears at stash@{0}\n\n\
                 2. Find and apply specific stash:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // Review list:\n\
                    // stash@{0}: WIP: new feature\n\
                    // stash@{1}: WIP: refactoring\n\
                    // stash@{2}: bug fix attempt\n\
                    \n\
                    // Found it! Apply stash@{2}\n\
                    git_stash_apply({\n\
                        \"path\": \"/project\",\n\
                        \"stash\": \"stash@{2}\"\n\
                    })\n\n\
                 3. Clean up obsolete stashes:\n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // Review which are obsolete\n\
                    \n\
                    git_stash_drop({\n\
                        \"path\": \"/project\",\n\
                        \"stash\": \"stash@{3}\"\n\
                    })\n\
                    // Remove old stash\n\
                    \n\
                    git_stash_list({\"path\": \"/project\"})\n\
                    // Verify removal (numbers shift down)\n\n\
                 4. Complete stash cleanup workflow:\n\
                    // Step 1: List all stashes\n\
                    git_stash_list({\"path\": \"/repo\"})\n\
                    \n\
                    // Step 2: Review each one\n\
                    git_show({\"path\": \"/repo\", \"object\": \"stash@{0}\"})\n\
                    git_show({\"path\": \"/repo\", \"object\": \"stash@{1}\"})\n\
                    git_show({\"path\": \"/repo\", \"object\": \"stash@{2}\"})\n\
                    \n\
                    // Step 3: Apply useful ones\n\
                    git_stash_apply({\"path\": \"/repo\", \"stash\": \"stash@{1}\"})\n\
                    \n\
                    // Step 4: Drop obsolete ones\n\
                    git_stash_drop({\"path\": \"/repo\", \"stash\": \"stash@{0}\"})\n\
                    git_stash_drop({\"path\": \"/repo\", \"stash\": \"stash@{2}\"})\n\
                    \n\
                    // Step 5: Verify cleanup\n\
                    git_stash_list({\"path\": \"/repo\"})\n\n\
                 STASH NUMBERING AFTER OPERATIONS:\n\
                 - Numbers shift when you drop a stash\n\
                 - New stashes always go to index 0\n\
                 - Old stashes move to higher numbers\n\
                 \n\
                 Example:\n\
                 Before: stash@{0}, stash@{1}, stash@{2}\n\
                 Drop stash@{1}\n\
                 After: stash@{0}, stash@{1} (was stash@{2})\n\
                 \n\
                 Before: stash@{0}, stash@{1}\n\
                 Create new stash\n\
                 After: stash@{0} (new), stash@{1} (was stash@{0}), stash@{2} (was stash@{1})\n\n\
                 STASH MANAGEMENT WORKFLOW:\n\
                 1. List to see what's stashed\n\
                 2. Review each if needed (git_show)\n\
                 3. Apply useful ones (git_stash_apply)\n\
                 4. Drop obsolete ones (git_stash_drop)\n\
                 5. List again to verify\n\n\
                 STASH ORGANIZATION BEST PRACTICES:\n\
                 - Use descriptive messages when creating stashes\n\
                 - List stashes regularly to stay aware\n\
                 - Don't accumulate too many stashes (>5 is a code smell)\n\
                 - Drop stashes after applying successfully\n\
                 - Consider committing instead of long-term stashing\n\
                 - Review and clean up stashes weekly\n\n\
                 COMMON STASH MANAGEMENT SCENARIOS:\n\n\
                 Scenario 1: Too many stashes\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Shows 10+ stashes - time to clean up!\n\
                 // Review each, keep only relevant ones\n\n\
                 Scenario 2: Can't find stashed work\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Read all messages to find the right one\n\
                 // Use git_show to confirm contents\n\n\
                 Scenario 3: Stash conflicts after drops\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Note current indices\n\
                 git_stash_drop({\"path\": \"/repo\", \"stash\": \"stash@{1}\"})\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Verify indices shifted correctly\n\n\
                 Scenario 4: Organizing work in progress\n\
                 // Before starting new task\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Clean up any old stashes first\n\
                 git_stash_save({\"path\": \"/repo\", \"message\": \"Task A: WIP\"})\n\
                 \n\
                 // Later, switch to Task B\n\
                 git_stash_save({\"path\": \"/repo\", \"message\": \"Task A: partial\"})\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // See both tasks listed\n\
                 \n\
                 // Resume Task A\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Find Task A stash\n\
                 git_stash_apply({\"path\": \"/repo\", \"stash\": \"stash@{0}\"})\n\n\
                 PREVENTING STASH BUILDUP:\n\
                 - Commit work instead of stashing when possible\n\
                 - Use branches for parallel work (not multiple stashes)\n\
                 - Apply and commit stashed work within a few days\n\
                 - Drop stashes after successful application\n\
                 - Set a personal limit (e.g., max 3 active stashes)\n\n\
                 STASH LIST AS AUDIT TRAIL:\n\
                 - Shows history of interrupted work\n\
                 - Reveals work patterns and context switches\n\
                 - Helps find lost or forgotten work\n\
                 - Indicates areas needing better organization\n\
                 - Empty list = clean workflow (ideal state)",
            ),
        },
    ]
}

/// Comprehensive guide covering all stash listing scenarios
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using git_stash_list effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_stash_list tool shows all stashed changes in a repository. Use it to track temporarily saved work, decide what to apply, and manage stash cleanup.\n\n\
                 =============================================================================\n\
                 BASIC USAGE\n\
                 =============================================================================\n\n\
                 List all stashes in a repository:\n\
                 git_stash_list({\n\
                     \"path\": \"/path/to/repository\"\n\
                 })\n\n\
                 PARAMETERS:\n\
                 - path (required): Path to git repository\n\n\
                 RESPONSE FORMAT:\n\
                 {\n\
                   \"stashes\": [\n\
                     {\n\
                       \"index\": 0,\n\
                       \"ref\": \"stash@{0}\",\n\
                       \"branch\": \"main\",\n\
                       \"message\": \"WIP on main: abc1234 Latest work\"\n\
                     },\n\
                     {\n\
                       \"index\": 1,\n\
                       \"ref\": \"stash@{1}\",\n\
                       \"branch\": \"feature/auth\",\n\
                       \"message\": \"authentication in progress\"\n\
                     }\n\
                   ]\n\
                 }\n\n\
                 RESPONSE FIELDS:\n\
                 - index: Stash number (0 = most recent)\n\
                 - ref: Full reference for use in other git commands\n\
                 - branch: Which branch stash was created on\n\
                 - message: Description of stashed changes\n\n\
                 =============================================================================\n\
                 UNDERSTANDING STASH ORDER\n\
                 =============================================================================\n\n\
                 STASH STACK BEHAVIOR:\n\
                 - Stashes work like a stack (LIFO - Last In, First Out)\n\
                 - Most recent stash is always stash@{0}\n\
                 - Older stashes have higher numbers\n\
                 - Creating new stash pushes others down\n\
                 - Dropping a stash shifts numbers down\n\n\
                 NUMBERING EXAMPLES:\n\n\
                 Initial state:\n\
                 stash@{0}: Feature A\n\
                 stash@{1}: Feature B\n\
                 stash@{2}: Feature C\n\n\
                 After creating new stash:\n\
                 stash@{0}: Feature D (NEW)\n\
                 stash@{1}: Feature A (was stash@{0})\n\
                 stash@{2}: Feature B (was stash@{1})\n\
                 stash@{3}: Feature C (was stash@{2})\n\n\
                 After dropping stash@{1}:\n\
                 stash@{0}: Feature D\n\
                 stash@{1}: Feature B (was stash@{2})\n\
                 stash@{2}: Feature C (was stash@{3})\n\n\
                 =============================================================================\n\
                 WHEN TO USE GIT_STASH_LIST\n\
                 =============================================================================\n\n\
                 1. BEFORE CREATING NEW STASH:\n\
                    Check what's already there to avoid confusion\n\
                    git_stash_list({\"path\": \"/repo\"})\n\
                    // See existing stashes\n\
                    git_stash_save({\"path\": \"/repo\", \"message\": \"New work\"})\n\
                    // Add new stash\n\n\
                 2. WHEN DECIDING WHAT TO APPLY:\n\
                    Review all stashes to find the right one\n\
                    git_stash_list({\"path\": \"/repo\"})\n\
                    // Find the stash you need\n\
                    git_stash_apply({\"path\": \"/repo\", \"stash\": \"stash@{2}\"})\n\
                    // Apply specific stash\n\n\
                 3. DURING STASH CLEANUP:\n\
                    See which stashes are obsolete\n\
                    git_stash_list({\"path\": \"/repo\"})\n\
                    // Review all stashes\n\
                    git_stash_drop({\"path\": \"/repo\", \"stash\": \"stash@{3}\"})\n\
                    // Remove old ones\n\n\
                 4. AFTER STASH OPERATIONS:\n\
                    Verify that operations worked correctly\n\
                    git_stash_save({\"path\": \"/repo\", \"message\": \"Test\"})\n\
                    git_stash_list({\"path\": \"/repo\"})\n\
                    // Confirm new stash at index 0\n\n\
                 5. WHEN LOOKING FOR LOST WORK:\n\
                    Find work you stashed and forgot about\n\
                    git_stash_list({\"path\": \"/repo\"})\n\
                    // Review messages to find your work\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 WORKFLOW 1: APPLY SPECIFIC STASH\n\
                 Step 1: List all stashes\n\
                 git_stash_list({\"path\": \"/project\"})\n\
                 \n\
                 Step 2: Review output\n\
                 {\n\
                   \"stashes\": [\n\
                     {\"index\": 0, \"ref\": \"stash@{0}\", \"message\": \"WIP: feature X\"},\n\
                     {\"index\": 1, \"ref\": \"stash@{1}\", \"message\": \"WIP: bug fix\"},\n\
                     {\"index\": 2, \"ref\": \"stash@{2}\", \"message\": \"authentication work\"}\n\
                   ]\n\
                 }\n\
                 \n\
                 Step 3: Apply the one you need\n\
                 git_stash_apply({\"path\": \"/project\", \"stash\": \"stash@{2}\"})\n\
                 // Apply authentication work\n\n\
                 WORKFLOW 2: CLEAN UP OLD STASHES\n\
                 Step 1: List all stashes\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 \n\
                 Step 2: Examine each stash\n\
                 git_show({\"path\": \"/repo\", \"object\": \"stash@{0}\"})\n\
                 git_show({\"path\": \"/repo\", \"object\": \"stash@{1}\"})\n\
                 git_show({\"path\": \"/repo\", \"object\": \"stash@{2}\"})\n\
                 \n\
                 Step 3: Drop obsolete ones\n\
                 git_stash_drop({\"path\": \"/repo\", \"stash\": \"stash@{0}\"})\n\
                 git_stash_drop({\"path\": \"/repo\", \"stash\": \"stash@{1}\"})\n\
                 \n\
                 Step 4: Verify cleanup\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Should show fewer stashes\n\n\
                 WORKFLOW 3: MANAGE MULTIPLE WORK STREAMS\n\
                 Save work on Feature A:\n\
                 git_stash_save({\"path\": \"/repo\", \"message\": \"Feature A: partial\"})\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Confirm saved\n\
                 \n\
                 Work on Feature B:\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"feature-b\"})\n\
                 // Make changes...\n\
                 git_stash_save({\"path\": \"/repo\", \"message\": \"Feature B: WIP\"})\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // See both stashes\n\
                 \n\
                 Resume Feature A:\n\
                 git_checkout({\"path\": \"/repo\", \"branch\": \"feature-a\"})\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Find Feature A stash\n\
                 git_stash_apply({\"path\": \"/repo\", \"stash\": \"stash@{1}\"})\n\n\
                 WORKFLOW 4: VERIFY STASH OPERATIONS\n\
                 Before operation:\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Note current state\n\
                 \n\
                 Perform operation:\n\
                 git_stash_save({\"path\": \"/repo\", \"message\": \"New stash\"})\n\
                 \n\
                 After operation:\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Verify new stash appears at index 0\n\
                 // Old stashes shifted to higher indices\n\n\
                 =============================================================================\n\
                 INTERPRETING STASH MESSAGES\n\
                 =============================================================================\n\n\
                 DEFAULT MESSAGE FORMAT:\n\
                 \"WIP on branch: commit_hash commit_message\"\n\
                 Example: \"WIP on main: abc1234 Add authentication\"\n\
                 \n\
                 This tells you:\n\
                 - \"WIP\" = Work In Progress (default prefix)\n\
                 - \"main\" = Branch where stash was created\n\
                 - \"abc1234\" = Commit hash of base state\n\
                 - \"Add authentication\" = Base commit message\n\n\
                 CUSTOM MESSAGE:\n\
                 When user provides message to git_stash_save\n\
                 Example: \"authentication feature half done\"\n\
                 \n\
                 Custom messages are more helpful:\n\
                 - Easier to find specific work\n\
                 - More descriptive of actual changes\n\
                 - Better for team communication\n\n\
                 =============================================================================\n\
                 EMPTY STASH LIST\n\
                 =============================================================================\n\n\
                 If git_stash_list returns empty array:\n\
                 {\n\
                   \"stashes\": []\n\
                 }\n\
                 \n\
                 This means:\n\
                 - No work is currently stashed\n\
                 - All previous stashes have been applied or dropped\n\
                 - Clean state (ideal for most workflows)\n\
                 - Repository has no saved work in progress\n\n\
                 =============================================================================\n\
                 INTEGRATION WITH OTHER GIT TOOLS\n\
                 =============================================================================\n\n\
                 EXAMINE STASH CONTENTS:\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 git_show({\"path\": \"/repo\", \"object\": \"stash@{0}\"})\n\
                 // See full diff of stashed changes\n\n\
                 APPLY STASH:\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 git_stash_apply({\"path\": \"/repo\", \"stash\": \"stash@{1}\"})\n\
                 // Restore changes from specific stash\n\n\
                 DROP STASH:\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 git_stash_drop({\"path\": \"/repo\", \"stash\": \"stash@{2}\"})\n\
                 // Remove specific stash\n\n\
                 CREATE STASH:\n\
                 git_stash_save({\"path\": \"/repo\", \"message\": \"Work\"})\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Verify stash created\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. LIST FREQUENTLY:\n\
                    Check stashes regularly to stay aware of saved work\n\
                    git_stash_list({\"path\": \"/repo\"})\n\n\
                 2. USE DESCRIPTIVE MESSAGES:\n\
                    When creating stashes, use clear descriptions\n\
                    git_stash_save({\"path\": \"/repo\", \"message\": \"OAuth2 implementation WIP\"})\n\
                    // Easier to find later\n\n\
                 3. KEEP STASH COUNT LOW:\n\
                    More than 5 stashes indicates poor organization\n\
                    Clean up regularly\n\n\
                 4. VERIFY OPERATIONS:\n\
                    Always list after stash operations to confirm\n\
                    git_stash_save(...)\n\
                    git_stash_list({\"path\": \"/repo\"}) // Confirm saved\n\n\
                 5. EXAMINE BEFORE APPLYING:\n\
                    Review stash contents before applying\n\
                    git_stash_list({\"path\": \"/repo\"})\n\
                    git_show({\"path\": \"/repo\", \"object\": \"stash@{0}\"})\n\
                    git_stash_apply({\"path\": \"/repo\", \"stash\": \"stash@{0}\"})\n\n\
                 6. CLEAN UP AFTER USE:\n\
                    Drop stashes after successful application\n\
                    git_stash_apply({\"path\": \"/repo\", \"stash\": \"stash@{0}\"})\n\
                    // Test that it works...\n\
                    git_stash_drop({\"path\": \"/repo\", \"stash\": \"stash@{0}\"})\n\n\
                 7. PREFER COMMITS OVER LONG-TERM STASHES:\n\
                    Stashes are for temporary storage\n\
                    Commit real work instead of keeping it stashed\n\n\
                 8. CHECK BRANCH CONTEXT:\n\
                    Note which branch stash was created on\n\
                    May affect whether it applies cleanly\n\n\
                 =============================================================================\n\
                 TROUBLESHOOTING\n\
                 =============================================================================\n\n\
                 PROBLEM: Can't find stashed work\n\
                 SOLUTION: List all stashes and read messages carefully\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Review each message\n\
                 git_show({\"path\": \"/repo\", \"object\": \"stash@{N}\"})\n\
                 // Examine contents of each\n\n\
                 PROBLEM: Too many stashes to manage\n\
                 SOLUTION: Review and clean up systematically\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 // Examine each one\n\
                 // Drop obsolete ones\n\
                 // Apply and commit useful ones\n\n\
                 PROBLEM: Stash numbers keep changing\n\
                 SOLUTION: This is normal stash behavior\n\
                 - New stashes go to index 0\n\
                 - Old ones shift to higher numbers\n\
                 - Dropped stashes cause numbers to shift down\n\
                 - Always list immediately before operations\n\n\
                 PROBLEM: Empty stash list but sure I stashed something\n\
                 SOLUTION: Check if you're in the right repository\n\
                 - Stashes are per-repository\n\
                 - Verify path parameter is correct\n\
                 - Check if someone else dropped the stash\n\
                 - Look in reflog for stash history\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 List stashes:\n\
                 git_stash_list({\"path\": \"/repo\"})\n\n\
                 List + examine specific stash:\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 git_show({\"path\": \"/repo\", \"object\": \"stash@{1}\"})\n\n\
                 List + apply specific stash:\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 git_stash_apply({\"path\": \"/repo\", \"stash\": \"stash@{0}\"})\n\n\
                 List + drop specific stash:\n\
                 git_stash_list({\"path\": \"/repo\"})\n\
                 git_stash_drop({\"path\": \"/repo\", \"stash\": \"stash@{2}\"})\n\n\
                 Verify stash creation:\n\
                 git_stash_save({\"path\": \"/repo\", \"message\": \"Work\"})\n\
                 git_stash_list({\"path\": \"/repo\"})\n\n\
                 Remember: git_stash_list is your window into stashed work. Use it frequently to maintain awareness of saved changes and keep your stash organized!",
            ),
        },
    ]
}
