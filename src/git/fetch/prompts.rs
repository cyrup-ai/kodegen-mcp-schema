//! Prompt messages for git_fetch tool

use super::prompt_args::GitFetchPromptArgs;
use crate::tool::PromptProvider;
use rmcp::model::{PromptArgument, PromptMessage, PromptMessageContent, PromptMessageRole};

/// Prompt provider for git_fetch tool
///
/// This is the ONLY way to provide prompts for git_fetch - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct FetchPrompts;

impl PromptProvider for FetchPrompts {
    type PromptArgs = GitFetchPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("prune") => prompt_prune(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some("Scenario to show (basic, prune)".to_string()),
            required: Some(false),
        }]
    }
}

/// Basic fetching operations
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I fetch changes from a remote repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_fetch to download objects and refs from remote repository. Here's how to perform basic fetching:\n\n\
                 BASIC EXAMPLES:\n\n\
                 1. Fetch from origin:\n\
                    git_fetch({\n\
                        \"path\": \"/project\"\n\
                    })\n\n\
                 2. Fetch all remotes:\n\
                    git_fetch({\n\
                        \"path\": \"/project\",\n\
                        \"all\": true\n\
                    })\n\n\
                 3. Fetch with tags:\n\
                    git_fetch({\n\
                        \"path\": \"/project\",\n\
                        \"tags\": true\n\
                    })\n\n\
                 4. Fetch specific branch:\n\
                    git_fetch({\n\
                        \"path\": \"/project\",\n\
                        \"refspec\": \"develop\"\n\
                    })\n\n\
                 5. Fetch from specific remote:\n\
                    git_fetch({\n\
                        \"path\": \"/project\",\n\
                        \"remote\": \"upstream\"\n\
                    })\n\n\
                 RESPONSE STRUCTURE:\n\
                 {\n\
                   \"success\": true,\n\
                   \"new_commits\": 5,\n\
                   \"new_branches\": [\"feature/x\"],\n\
                   \"updated_branches\": [\"origin/main\", \"origin/develop\"],\n\
                   \"remote\": \"origin\"\n\
                 }\n\n\
                 WHAT FETCH DOES:\n\
                 - Downloads commits you don't have\n\
                 - Updates origin/* refs\n\
                 - Does NOT merge or change local files\n\
                 - Safe to run anytime\n\n\
                 WHY USE FETCH:\n\
                 - Check what changed remotely before integrating\n\
                 - Review commits before merging\n\
                 - Safer than git_pull (fetch+merge in one step)\n\
                 - Required before rebasing or merging\n\n\
                 PARAMETERS:\n\
                 - path (required): Repository directory path\n\
                 - remote (optional): Which remote to fetch from (default: origin)\n\
                 - all (optional): Fetch from all remotes (default: false)\n\
                 - tags (optional): Also fetch tags (default: false)\n\
                 - prune (optional): Remove stale remote branches (default: false)\n\
                 - refspec (optional): Specific branch to fetch\n\n\
                 AFTER FETCHING:\n\
                 - Your local branches remain unchanged\n\
                 - Remote-tracking branches are updated (origin/main, origin/develop, etc.)\n\
                 - Working directory is not modified\n\
                 - Use git_status to see if you're behind\n\
                 - Use git_log or git_diff to review changes before merging\n\n\
                 SAFE OPERATION:\n\
                 - Fetch never modifies your working directory\n\
                 - Fetch never changes your local branches\n\
                 - Fetch never loses your work\n\
                 - You can fetch as often as you want\n\n\
                 COMMON WORKFLOWS:\n\
                 1. git_fetch({\"path\": \"/project\"})  // Download changes\n\
                 2. git_status({\"path\": \"/project\"})  // Check if behind\n\
                 3. git_diff({\"path\": \"/project\", \"from\": \"HEAD\", \"to\": \"origin/main\"})  // Review\n\
                 4. git_merge({\"path\": \"/project\", \"branch\": \"origin/main\"})  // Integrate if desired\n\n\
                 FETCH vs PULL:\n\
                 - git_fetch: Download only, safe, review first\n\
                 - git_pull: Fetch + merge together, less control\n\
                 - Use fetch when you want to review changes before integrating\n\
                 - Use pull when you're sure you want to merge immediately",
            ),
        },
    ]
}

/// Pruning stale remote references
fn prompt_prune() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text("How do I clean up deleted remote branches?"),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use prune to remove stale remote-tracking branches. Here's how:\n\n\
                 PRUNING STALE REFS:\n\n\
                 1. Fetch and prune:\n\
                    git_fetch({\n\
                        \"path\": \"/project\",\n\
                        \"prune\": true\n\
                    })\n\
                    // Removes refs to deleted remote branches\n\n\
                 2. See what would be pruned:\n\
                    git_fetch({\n\
                        \"path\": \"/project\",\n\
                        \"prune\": true,\n\
                        \"dry_run\": true\n\
                    })\n\n\
                 3. After colleague deletes branch:\n\
                    // Their branch deleted on server\n\
                    git_fetch({\"path\": \"/project\", \"prune\": true})\n\
                    // Now origin/their-branch is gone\n\n\
                 WHY PRUNE:\n\
                 - Remote branches get deleted\n\
                 - Local refs become stale\n\
                 - Prune cleans up references\n\
                 - Keeps branch list clean\n\n\
                 WHAT GETS PRUNED:\n\
                 - Remote-tracking branches (origin/feature-x)\n\
                 - Only if the branch was deleted on the remote\n\
                 - Does NOT affect your local branches\n\
                 - Does NOT affect your working directory\n\n\
                 WHAT DOESN'T GET PRUNED:\n\
                 - Your local branches (feature-x)\n\
                 - Your working directory files\n\
                 - Your uncommitted changes\n\
                 - Your commits\n\n\
                 PRUNING SCENARIO:\n\
                 1. Pull request merged on GitHub\n\
                 2. Feature branch deleted on GitHub\n\
                 3. Your local still shows origin/feature-x\n\
                 4. git_fetch with prune removes origin/feature-x\n\
                 5. Your local feature-x branch still exists (if you have it)\n\n\
                 REGULAR MAINTENANCE:\n\
                 git_fetch({\n\
                     \"path\": \"/project\",\n\
                     \"prune\": true\n\
                 })\n\
                 Run this regularly to keep remote-tracking branches synchronized\n\n\
                 AFTER PRUNING:\n\
                 Check your local branches:\n\
                 git_branch_list({\"path\": \"/project\"})\n\
                 \n\
                 Delete corresponding local branches if desired:\n\
                 git_branch_delete({\n\
                     \"path\": \"/project\",\n\
                     \"branch\": \"feature-x\"\n\
                 })\n\n\
                 PRUNE ALL REMOTES:\n\
                 git_fetch({\n\
                     \"path\": \"/project\",\n\
                     \"all\": true,\n\
                     \"prune\": true\n\
                 })\n\
                 Fetches from all remotes and prunes stale branches from each\n\n\
                 DRY RUN:\n\
                 See what would be pruned without actually pruning:\n\
                 git_fetch({\n\
                     \"path\": \"/project\",\n\
                     \"prune\": true,\n\
                     \"dry_run\": true\n\
                 })\n\
                 Response shows which branches would be removed:\n\
                 {\n\
                   \"would_prune\": [\n\
                     \"origin/old-feature\",\n\
                     \"origin/merged-pr\"\n\
                   ]\n\
                 }\n\n\
                 BEST PRACTICES:\n\
                 - Include prune:true in regular fetches\n\
                 - Run after merging pull requests\n\
                 - Use dry_run to preview pruning\n\
                 - Manually delete local branches afterward\n\
                 - Keeps your branch list manageable\n\n\
                 SAFETY:\n\
                 - Pruning is safe (only removes remote refs)\n\
                 - Your local work is never affected\n\
                 - Can always re-fetch if needed\n\
                 - Remote branches can be restored if they still exist on server",
            ),
        },
    ]
}
