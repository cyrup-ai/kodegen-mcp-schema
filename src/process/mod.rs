//! Process category module

pub mod process_kill;
pub mod process_list;

// Explicit re-exports to avoid ambiguous globs
pub use process_kill::{
    PROCESS_KILL,
    ProcessKillArgs,
    ProcessKillOutput,
    ProcessKillPromptArgs,
    ProcessKillPrompts,
};

pub use process_list::{
    PROCESS_LIST,
    ProcessListArgs,
    ProcessListOutput,
    ProcessListPromptArgs,
    ProcessListPrompts,
    ProcessInfo,  // Re-export nested type
};
