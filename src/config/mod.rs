//! Config category module

pub mod config_get;
pub mod config_set;

// Re-export config_get tool
pub use config_get::{
    CONFIG_GET,
    GetConfigArgs,
    ConfigGetOutput,
    ConfigGetPromptArgs,
    ConfigGetPrompts,
    SystemInfo,
    MemoryInfo,
    ClientRecord,
    ClientInfo,
};

// Re-export config_set tool
pub use config_set::{
    CONFIG_SET,
    SetConfigValueArgs,
    ConfigSetOutput,
    SetConfigValuePromptArgs,
    SetConfigValuePrompts,
    ConfigValue,
};
