//! Prompt argument types for memory_check_memorize_status tool

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CheckMemorizeStatusPromptArgs {}
