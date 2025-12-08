//! Process category module

// Re-export all process tool name constants from kodegen_config
pub use kodegen_config::{PROCESS_KILL, PROCESS_LIST};

pub mod process_kill;
pub mod process_list;

// Explicit re-exports to avoid ambiguous globs
pub use process_kill::{
    ProcessKillArgs,
    ProcessKillOutput,
    ProcessKillPromptArgs,
    ProcessKillPrompts,
};

pub use process_list::{
    ProcessListArgs,
    ProcessListOutput,
    ProcessListPromptArgs,
    ProcessListPrompts,
    ProcessInfo,  // Re-export nested type
};
