//! Config category module

// Re-export all config tool name constants from kodegen_config
pub use kodegen_config::{CONFIG_GET, CONFIG_SET};

pub mod config_get;
pub mod config_set;

// Re-export config_get tool
pub use config_get::{
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
    SetConfigValueArgs,
    ConfigSetOutput,
    SetConfigValuePromptArgs,
    SetConfigValuePrompts,
    ConfigValue,
};
