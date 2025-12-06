//! Prompt messages for process_kill tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ProcessKillPromptArgs;

/// Prompt provider for process_kill tool
///
/// This is the ONLY way to provide prompts for process_kill - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ProcessKillPrompts;

impl PromptProvider for ProcessKillPrompts {
    type PromptArgs = ProcessKillPromptArgs;

    fn generate_prompts(_args: &Self::PromptArgs) -> Vec<PromptMessage> {
        prompt_basic()
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO KILL PROCESSES SAFELY
// ============================================================================

/// Basic process termination
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I terminate a process using the process_kill tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The process_kill tool terminates a running process by its PID (Process ID). This is a forceful operation that sends SIGKILL.\n\n\
                 KILLING PROCESSES:\n\n\
                 1. Kill by PID:\n\
                    process_kill({\n\
                        \"pid\": 12345\n\
                    })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"pid\": 12345,\n\
                   \"killed\": true\n\
                 }\n\n\
                 WARNING:\n\
                 - Sends SIGKILL (forceful)\n\
                 - Process cannot cleanup\n\
                 - Use carefully\n\n\
                 BASIC USAGE:\n\
                 The process_kill tool requires only one parameter:\n\
                 - pid (required): The Process ID to terminate\n\n\
                 Example:\n\
                 process_kill({\"pid\": 12345})\n\n\
                 This sends SIGKILL to process 12345, which:\n\
                 - Terminates immediately\n\
                 - Cannot be caught or ignored by the process\n\
                 - Does not allow graceful shutdown\n\
                 - May leave temporary files or incomplete operations\n\n\
                 RESPONSE STRUCTURE:\n\
                 On success:\n\
                 {\n\
                   \"pid\": 12345,\n\
                   \"killed\": true\n\
                 }\n\n\
                 On failure (process not found):\n\
                 {\n\
                   \"pid\": 12345,\n\
                   \"killed\": false,\n\
                   \"error\": \"Process not found\"\n\
                 }\n\n\
                 On failure (permission denied):\n\
                 {\n\
                   \"pid\": 1,\n\
                   \"killed\": false,\n\
                   \"error\": \"Permission denied\"\n\
                 }\n\n\
                 WHAT IS SIGKILL?\n\
                 SIGKILL is signal 9 on Unix-like systems:\n\
                 - Most forceful termination method\n\
                 - Process receives no notification\n\
                 - Cannot be caught, blocked, or ignored\n\
                 - Kernel terminates the process immediately\n\
                 - All resources are released by the operating system\n\
                 - No cleanup handlers run\n\n\
                 WHEN TO USE:\n\
                 - Process is frozen or unresponsive\n\
                 - Process does not respond to graceful shutdown\n\
                 - Immediate termination required\n\n\
                 LIMITATIONS:\n\
                 - Cannot kill system processes (PID 1, kernel threads)\n\
                 - May require elevated privileges for other users' processes\n\
                 - Does not kill child processes automatically on all systems\n\
                 - Not suitable for processes needing graceful shutdown\n\n\
                 IMPORTANT NOTES:\n\
                 - Always verify the PID before killing\n\
                 - Use process_list to find the correct PID\n\
                 - SIGKILL is final and immediate - no undo\n\n\
                 ERROR MESSAGES:\n\
                 - \"Process not found\": PID doesn't exist (already terminated or invalid)\n\
                 - \"Permission denied\": Insufficient privileges (system process or other user's process)\n\
                 - \"Operation not permitted\": Cannot kill protected processes\n\n\
                 PROCESS ID NOTES:\n\
                 - PIDs are positive integers\n\
                 - PID 1 is init/systemd (NEVER kill this!)\n\
                 - Low PIDs (< 100) are typically system processes\n\
                 - PIDs can be reused after process terminates\n\
                 - Always verify PID is current before killing",
            ),
        },
    ]
}


