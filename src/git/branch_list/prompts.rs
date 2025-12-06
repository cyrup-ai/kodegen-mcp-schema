//! Prompt messages for git_branch_list tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::GitBranchListPromptArgs;

/// Prompt provider for git_branch_list tool
///
/// This is the ONLY way to provide prompts for git_branch_list - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct BranchListPrompts;

impl PromptProvider for BranchListPrompts {
    type PromptArgs = GitBranchListPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("remote") => prompt_remote(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, remote)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO LIST GIT BRANCHES
// ============================================================================

/// Basic branch listing scenarios
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list branches in a Git repository?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The git_branch_list tool lists all branches in a repository with comprehensive filtering and sorting options. Here's how to use it:\n\n\
                 LISTING BRANCHES:\n\n\
                 1. Local branches:\n\
                    git_branch_list({\"path\": \"/project\"})\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"branches\": [\n\
                     {\"name\": \"main\", \"current\": true, \"head\": \"abc1234\"},\n\
                     {\"name\": \"develop\", \"current\": false, \"head\": \"def5678\"},\n\
                     {\"name\": \"feature/auth\", \"current\": false, \"head\": \"ghi9012\"}\n\
                   ],\n\
                   \"current\": \"main\"\n\
                 }\n\n\
                 2. Show current branch:\n\
                    git_branch_list({\"path\": \"/project\"})\n\
                    // current: true indicates checked out branch\n\n\
                 3. List with details:\n\
                    git_branch_list({\n\
                        \"path\": \"/project\",\n\
                        \"verbose\": true\n\
                    })\n\
                    // Shows last commit info for each branch\n\n\
                 INTERPRETING OUTPUT:\n\
                 - branches: Array of branch objects with metadata\n\
                 - name: Branch name (e.g., \"main\", \"feature/login\")\n\
                 - current: true if this branch is currently checked out\n\
                 - head: Commit hash at branch tip\n\
                 - With verbose: Shows commit message, author, date\n\n\
                 PARAMETERS:\n\
                 - path (required): Repository path\n\
                 - verbose (optional): Show detailed commit info (default: false)\n\
                 - all (optional): Include remote branches (default: false)\n\
                 - remote (optional): Show only remote branches (default: false)\n\n\
                 BRANCH NAMING CONVENTIONS:\n\
                 - main/master: Primary development branch\n\
                 - develop: Integration branch for features\n\
                 - feature/*: Feature development branches\n\
                 - fix/* or bugfix/*: Bug fix branches\n\
                 - hotfix/*: Emergency production fixes\n\
                 - release/*: Release preparation branches\n\
                 - test/*: Experimental/testing branches\n\n\
                 USE CASES:\n\
                 - See what branches exist before creating new one\n\
                 - Find branch names for checkout operations\n\
                 - Identify current working branch",
            ),
        },
    ]
}

/// Remote branch scenarios
fn prompt_remote() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I list remote branches and track branch relationships?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use git_branch_list to view remote branches and understand branch tracking relationships.\n\n\
                 REMOTE BRANCHES:\n\n\
                 1. List remote branches:\n\
                    git_branch_list({\n\
                        \"path\": \"/project\",\n\
                        \"remote\": true\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"branches\": [\n\
                     {\"name\": \"origin/main\", \"remote\": true, \"head\": \"abc1234\"},\n\
                     {\"name\": \"origin/develop\", \"remote\": true, \"head\": \"def5678\"},\n\
                     {\"name\": \"origin/feature/x\", \"remote\": true, \"head\": \"ghi9012\"}\n\
                   ]\n\
                 }\n\n\
                 2. All branches (local + remote):\n\
                    git_branch_list({\n\
                        \"path\": \"/project\",\n\
                        \"all\": true\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"branches\": [\n\
                     {\"name\": \"main\", \"current\": true, \"head\": \"abc1234\", \"upstream\": \"origin/main\"},\n\
                     {\"name\": \"feature/auth\", \"current\": false, \"head\": \"def5678\"},\n\
                     {\"name\": \"origin/main\", \"remote\": true, \"head\": \"abc1234\"},\n\
                     {\"name\": \"origin/develop\", \"remote\": true, \"head\": \"xyz7890\"}\n\
                   ]\n\
                 }\n\n\
                 3. After fetch operation:\n\
                    git_fetch({\"path\": \"/project\"})\n\
                    git_branch_list({\n\
                        \"path\": \"/project\",\n\
                        \"remote\": true\n\
                    })\n\
                    // See all remote branches including newly fetched\n\n\
                 4. Check upstream tracking:\n\
                    git_branch_list({\n\
                        \"path\": \"/project\",\n\
                        \"verbose\": true,\n\
                        \"all\": true\n\
                    })\n\
                    // upstream field shows tracking relationship\n\n\
                 TRACKING RELATIONSHIPS:\n\
                 Local branch \"main\" tracking \"origin/main\":\n\
                 {\n\
                   \"name\": \"main\",\n\
                   \"current\": true,\n\
                   \"upstream\": \"origin/main\",\n\
                   \"ahead\": 2,\n\
                   \"behind\": 1\n\
                 }\n\
                 - ahead: Local commits not pushed to remote\n\
                 - behind: Remote commits not pulled locally\n\n\
                 WORKFLOW EXAMPLES:\n\n\
                 1. Check remote branches before pulling:\n\
                    git_branch_list({\"path\": \"/repo\", \"remote\": true})\n\
                    // See what's available on remote\n\
                    git_pull({\"path\": \"/repo\"})\n\n\
                 2. Find branches to checkout:\n\
                    git_branch_list({\"path\": \"/repo\", \"all\": true})\n\
                    // See both local and remote branches\n\
                    git_checkout({\"path\": \"/repo\", \"target\": \"origin/feature/x\"})\n\
                    // Creates local tracking branch\n\n\
                 INTERPRETING REMOTE OUTPUT:\n\
                 - remote: true indicates remote tracking branch\n\
                 - Remote branches show as \"remoteName/branchName\"\n\
                 - upstream field shows which remote branch a local branch tracks\n\
                 - ahead/behind counts show sync status with upstream\n\n\
                 BEST PRACTICES:\n\
                 - Use remote: true to see remote-only branches\n\
                 - Use all: true to see complete picture\n\
                 - Check upstream field for tracking relationships\n\
                 - Look at ahead/behind for sync status",
            ),
        },
    ]
}

