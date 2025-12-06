//! Prompt messages for memory_recall tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::MemoryRecallPromptArgs;

/// Prompt provider for memory_recall tool
///
/// This is the ONLY way to provide prompts for memory_recall - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct MemoryRecallPrompts;

impl PromptProvider for MemoryRecallPrompts {
    type PromptArgs = MemoryRecallPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("semantic") => prompt_semantic(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, semantic)".to_string()),
                required: Some(false),
            }
        ]
    }
}

/// Basic memory retrieval
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I retrieve memories from a library?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The memory_recall tool retrieves relevant memories using semantic search. It finds content similar in meaning to your query.\n\n\
                 BASIC RETRIEVAL:\n\n\
                 memory_recall({\n\
                     \"library\": \"project-notes\",\n\
                     \"context\": \"authentication\",\n\
                     \"limit\": 5\n\
                 })\n\n\
                 RESPONSE:\n\
                 {\n\
                   \"memories\": [\n\
                     {\n\
                       \"content\": \"The API uses JWT tokens for authentication with 1h expiry.\",\n\
                       \"similarity\": 0.92\n\
                     },\n\
                     {\n\
                       \"content\": \"Auth middleware validates tokens in src/auth/middleware.rs\",\n\
                       \"similarity\": 0.87\n\
                     }\n\
                   ],\n\
                   \"library\": \"project-notes\",\n\
                   \"count\": 2\n\
                 }\n\n\
                 PARAMETERS:\n\
                 - library (required): Name of memory library to search\n\
                 - context (required): Query text for semantic matching\n\
                 - limit (optional): Maximum results (default: 10)\n\n\
                 HOW SEMANTIC SEARCH WORKS:\n\
                 - Finds memories by meaning, not just keywords\n\
                 - \"authentication\" matches JWT, tokens, login, etc.\n\
                 - Returns results ranked by similarity (0-1)\n\
                 - Higher similarity = more relevant\n\n\
                 BASIC EXAMPLES:\n\n\
                 1. Find configuration info:\n\
                    memory_recall({\n\
                        \"library\": \"codebase\",\n\
                        \"context\": \"where is the config file\",\n\
                        \"limit\": 3\n\
                    })\n\n\
                 2. Find API endpoints:\n\
                    memory_recall({\n\
                        \"library\": \"api-docs\",\n\
                        \"context\": \"create user endpoint\",\n\
                        \"limit\": 5\n\
                    })\n\n\
                 3. Find error solutions:\n\
                    memory_recall({\n\
                        \"library\": \"debugging\",\n\
                        \"context\": \"connection pool exhausted\",\n\
                        \"limit\": 3\n\
                    })\n\n\
                 4. Find patterns:\n\
                    memory_recall({\n\
                        \"library\": \"rust-patterns\",\n\
                        \"context\": \"error handling best practices\",\n\
                        \"limit\": 5\n\
                    })\n\n\
                 RESPONSE FIELDS:\n\
                 - memories: Array of matching memories\n\
                 - content: The stored text\n\
                 - similarity: How well it matches (0.0-1.0)\n\
                 - library: Which library was searched\n\
                 - count: Number of results returned\n\n\
                 BEST PRACTICES:\n\
                 - Use natural language queries\n\
                 - Be specific about what you're looking for\n\
                 - Start with limit: 5, increase if needed\n\
                 - Check similarity scores to gauge relevance\n\
                 - Try different phrasings if results aren't helpful",
            ),
        },
    ]
}

/// Understanding semantic search
fn prompt_semantic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How does semantic search work in memory_recall?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Semantic search finds memories by meaning using vector embeddings, not just keyword matching.\n\n\
                 HOW IT WORKS:\n\n\
                 1. EMBEDDING GENERATION:\n\
                    When you store a memory, it gets converted to a vector embedding - a numerical representation of meaning.\n\n\
                 2. QUERY EMBEDDING:\n\
                    Your search context is also converted to an embedding.\n\n\
                 3. SIMILARITY MATCHING:\n\
                    The system finds stored embeddings closest to your query embedding using cosine similarity.\n\n\
                 SEMANTIC MATCHING EXAMPLES:\n\n\
                 Stored memory:\n\
                 \"JWT tokens are signed using HMAC-SHA256 with a secret key\"\n\n\
                 These queries ALL find this memory:\n\
                 - \"authentication\" → similarity: 0.85\n\
                 - \"token signing\" → similarity: 0.91\n\
                 - \"security cryptography\" → similarity: 0.78\n\
                 - \"how do we verify users\" → similarity: 0.82\n\
                 - \"JWT\" → similarity: 0.94\n\n\
                 SIMILARITY SCORES:\n\n\
                 0.90-1.00: Excellent match, highly relevant\n\
                 0.80-0.89: Good match, likely relevant\n\
                 0.70-0.79: Moderate match, possibly relevant\n\
                 0.60-0.69: Weak match, may be tangential\n\
                 Below 0.60: Poor match, likely not relevant\n\n\
                 OPTIMIZING QUERIES:\n\n\
                 VAGUE (less effective):\n\
                 memory_recall({\n\
                     \"library\": \"codebase\",\n\
                     \"context\": \"stuff\"\n\
                 })\n\n\
                 SPECIFIC (more effective):\n\
                 memory_recall({\n\
                     \"library\": \"codebase\",\n\
                     \"context\": \"database connection configuration\"\n\
                 })\n\n\
                 NATURAL LANGUAGE (most effective):\n\
                 memory_recall({\n\
                     \"library\": \"codebase\",\n\
                     \"context\": \"how do I configure the PostgreSQL connection pool\"\n\
                 })\n\n\
                 QUERY STRATEGIES:\n\n\
                 1. USE QUESTIONS:\n\
                    \"how do I handle errors in async functions\"\n\
                    Better than: \"error async\"\n\n\
                 2. INCLUDE CONTEXT:\n\
                    \"Rust pattern for spawning background tasks\"\n\
                    Better than: \"spawn\"\n\n\
                 3. BE SPECIFIC:\n\
                    \"JWT token expiration and refresh logic\"\n\
                    Better than: \"tokens\"\n\n\
                 4. TRY VARIATIONS:\n\
                    If \"auth\" doesn't work, try:\n\
                    - \"authentication flow\"\n\
                    - \"login process\"\n\
                    - \"user verification\"",
            ),
        },
    ]
}
