//! Prompt messages for claude_agent tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::ClaudeAgentPromptArgs;

/// Prompt provider for claude_agent tool
///
/// This is the ONLY way to provide prompts for claude_agent - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ClaudeAgentPrompts;

impl PromptProvider for ClaudeAgentPrompts {
    type PromptArgs = ClaudeAgentPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("specialized") => prompt_specialized_agents(),
            Some("parallel") => prompt_parallel_coordination(),
            Some("research") => prompt_research_agents(),
            Some("monitoring") => prompt_progress_monitoring(),
            _ => prompt_basic_delegation(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario: basic, specialized, parallel, research, monitoring".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACHING AI AGENTS META-COGNITION
// ============================================================================

/// Teach: Simple task delegation - when to delegate vs handle directly
fn prompt_basic_delegation() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "When should I delegate work to a sub-agent using claude_agent? How do I do basic delegation?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The claude_agent tool enables AI agents to spawn autonomous sub-agents for task delegation. Here's when and how to use it:\n\n\
                 WHEN TO DELEGATE TO A SUB-AGENT:\n\
                 ✓ Task requires 10+ minutes of focused work\n\
                 ✓ Task has clear success criteria the agent can verify independently\n\
                 ✓ You need to continue with other work while the task runs\n\
                 ✓ Task requires deep exploration of specific directories/files\n\
                 ✓ Task benefits from isolated context (focused attention)\n\n\
                 WHEN TO HANDLE DIRECTLY (DON'T SPAWN):\n\
                 ✗ Simple 1-2 step tasks (overhead not justified)\n\
                 ✗ Already have necessary context loaded\n\
                 ✗ Task requires cross-cutting coordination across multiple areas\n\
                 ✗ Need immediate result (sub-agent has 2-3s overhead)\n\
                 ✗ Task requires judgment calls only you can make\n\n\
                 BASIC DELEGATION PATTERN:\n\
                 1. SPAWN: Create agent with clear task\n\
                 2. Wait for completion (or check progress with READ)\n\
                 3. Review results\n\
                 4. KILL: Cleanup when done\n\n\
                 EXAMPLE 1: Simple Delegation\n\
                 // Delegate research task to sub-agent\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Search the codebase for all authentication patterns. Document what you find.\",\n\
                   \"max_turns\": 10,\n\
                   \"await_completion_ms\": 300000  // 5 min timeout\n\
                 })\n\
                 // → Returns when complete or after 5 minutes\n\
                 // → Check completed field to see if finished\n\n\
                 EXAMPLE 2: Fire-and-Forget Delegation\n\
                 // Start long-running task in background\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Run all tests and analyze failures. Create detailed report.\",\n\
                   \"max_turns\": 20,\n\
                   \"await_completion_ms\": 0  // Fire-and-forget\n\
                 })\n\
                 // → Returns immediately, agent works in background\n\n\
                 // Check progress later\n\
                 claude_agent({\"action\": \"READ\", \"agent\": 0})\n\
                 // → See current output and status\n\n\
                 KEY PARAMETERS:\n\
                 • action: \"SPAWN\" (create), \"SEND\" (follow-up), \"READ\" (check), \"LIST\" (overview), \"KILL\" (cleanup)\n\
                 • agent: Instance number (0, 1, 2...) - use different numbers for parallel work\n\
                 • prompt: Clear task description (required for SPAWN/SEND)\n\
                 • max_turns: Conversation limit (default: 10) - prevents runaway agents\n\
                 • await_completion_ms: Timeout in milliseconds\n\
                   - Default: 300000 (5 minutes) - blocks until complete\n\
                   - Custom: Set any timeout\n\
                   - 0: Fire-and-forget (returns immediately)\n\n\
                 BEST PRACTICES:\n\
                 1. Write clear, specific prompts with success criteria\n\
                 2. Set reasonable max_turns (10-20 for most tasks)\n\
                 3. Use await_completion_ms: 0 for truly long tasks\n\
                 4. Always KILL agents when done to free resources\n\
                 5. Check completed field before trusting results\n\
                 6. Use READ to monitor long-running agents\n\n\
                 Remember: Delegation adds 2-3s overhead. Only delegate when the task justifies it!",
            ),
        },
    ]
}

/// Teach: Tool allowlist/blocklist for constrained agents
fn prompt_specialized_agents() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create specialized sub-agents that can only use specific tools? When would I want to constrain an agent's capabilities?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "You can create specialized sub-agents with constrained tool access using allowed_tools (allowlist) or disallowed_tools (blocklist). This is critical for security, focus, and reliability.\n\n\
                 WHY CONSTRAIN AGENTS:\n\
                 🔒 Security: Prevent agents from executing code or modifying files\n\
                 🎯 Focus: Force agent to use only relevant tools for the task\n\
                 ⚡ Performance: Reduce decision paralysis from too many tool options\n\
                 ✅ Reliability: Ensure agent can't accidentally break things\n\n\
                 TWO APPROACHES:\n\n\
                 1. ALLOWLIST (allowed_tools): Agent can ONLY use these tools\n\
                 2. BLOCKLIST (disallowed_tools): Agent can use anything EXCEPT these tools\n\n\
                 EXAMPLE 1: Read-Only Research Agent\n\
                 // Create agent that can ONLY read files and search\n\
                 // Cannot write, execute, or modify anything\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Research authentication patterns in src/auth. Document all findings.\",\n\
                   \"allowed_tools\": [\"fs_read_file\", \"fs_search\", \"grep\"],\n\
                   \"max_turns\": 15\n\
                 })\n\
                 // → Agent can only read and search, cannot modify anything\n\
                 // → Perfect for safe code analysis\n\n\
                 EXAMPLE 2: Code Analysis Agent (No Execution)\n\
                 // Block execution tools while allowing file operations\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Analyze test coverage and identify gaps\",\n\
                   \"disallowed_tools\": [\"terminal\", \"bash\", \"process_kill\"],\n\
                   \"max_turns\": 20\n\
                 })\n\
                 // → Agent can read/write files but cannot execute commands\n\
                 // → Safe for file analysis tasks\n\n\
                 EXAMPLE 3: Database-Focused Agent\n\
                 // Constrain to database tools only\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Analyze database schema and suggest optimizations\",\n\
                   \"allowed_tools\": [\n\
                     \"database_query\",\n\
                     \"database_schema\",\n\
                     \"fs_read_file\"  // Can read schema files\n\
                   ],\n\
                   \"max_turns\": 15\n\
                 })\n\
                 // → Focused on database work only\n\n\
                 SECURITY PATTERNS:\n\n\
                 1. Read-Only Agent (Research/Analysis):\n\
                 allowed_tools: [\"fs_read_file\", \"fs_search\", \"grep\"]\n\n\
                 2. Safe Writer (Documentation/Logs):\n\
                 allowed_tools: [\"fs_read_file\", \"fs_write_file\", \"fs_search\"]\n\
                 disallowed_tools: [\"terminal\", \"bash\", \"fs_delete_file\"]\n\n\
                 3. Test Runner (Execute But Don't Modify):\n\
                 allowed_tools: [\"terminal\", \"fs_read_file\"]\n\n\
                 4. Database Analyst:\n\
                 allowed_tools: [\"database_query\", \"database_schema\", \"fs_read_file\"]\n\n\
                 BEST PRACTICES:\n\
                 1. Use allowed_tools when you know exactly what the agent needs (more restrictive)\n\
                 2. Use disallowed_tools to block specific dangerous tools\n\
                 3. Include fs_read_file in most allowlists (agents need to read)\n\
                 4. Always block terminal/bash for read-only agents\n\n\
                 Remember: Constrained agents are safer, faster, and more focused!",
            ),
        },
    ]
}

/// Teach: Multiple agents working concurrently, synthesizing results
fn prompt_parallel_coordination() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I spawn multiple sub-agents to work on different tasks in parallel? How do I coordinate them and synthesize results?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "You can spawn multiple sub-agents using different agent numbers (0, 1, 2...) to work on independent tasks concurrently. This is powerful for parallel processing and complex workflows.\n\n\
                 PARALLEL AGENT PATTERN:\n\
                 1. SPAWN multiple agents with different numbers\n\
                 2. Each agent works independently and concurrently\n\
                 3. Use LIST to check overview\n\
                 4. Use READ to get individual results\n\
                 5. Synthesize results yourself (main agent)\n\
                 6. KILL all agents when done\n\n\
                 EXAMPLE 1: Parallel Code Analysis\n\
                 // Spawn 3 agents to analyze different parts of codebase\n\
                 \n\
                 // Agent 0: Backend analysis\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Analyze backend code in src/api. Check for security issues, performance problems, and code smells. Create detailed report.\",\n\
                   \"add_dirs\": [\"./src/api\", \"./src/middleware\"],\n\
                   \"allowed_tools\": [\"fs_read_file\", \"fs_search\", \"grep\"],\n\
                   \"await_completion_ms\": 0  // Fire-and-forget\n\
                 })\n\n\
                 // Agent 1: Frontend analysis (runs in parallel)\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 1,\n\
                   \"prompt\": \"Analyze frontend code in src/components. Check for accessibility issues, performance, unused code. Create detailed report.\",\n\
                   \"add_dirs\": [\"./src/components\", \"./src/utils\"],\n\
                   \"allowed_tools\": [\"fs_read_file\", \"fs_search\", \"grep\"],\n\
                   \"await_completion_ms\": 0\n\
                 })\n\n\
                 // Agent 2: Test coverage analysis (runs in parallel)\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 2,\n\
                   \"prompt\": \"Analyze test coverage. Run tests and identify gaps. Create report with recommendations.\",\n\
                   \"add_dirs\": [\"./tests\"],\n\
                   \"allowed_tools\": [\"fs_read_file\", \"fs_search\", \"terminal\"],\n\
                   \"await_completion_ms\": 0\n\
                 })\n\n\
                 // Check status of all agents\n\
                 claude_agent({\"action\": \"LIST\"})\n\
                 // → Shows all 3 agents with working status\n\n\
                 // Wait a bit, then read results\n\
                 claude_agent({\"action\": \"READ\", \"agent\": 0})  // Backend results\n\
                 claude_agent({\"action\": \"READ\", \"agent\": 1})  // Frontend results\n\
                 claude_agent({\"action\": \"READ\", \"agent\": 2})  // Test results\n\n\
                 // Synthesize findings yourself\n\
                 // Combine insights from all 3 agents\n\
                 // Create master report\n\n\
                 // Cleanup all agents\n\
                 claude_agent({\"action\": \"KILL\", \"agent\": 0})\n\
                 claude_agent({\"action\": \"KILL\", \"agent\": 1})\n\
                 claude_agent({\"action\": \"KILL\", \"agent\": 2})\n\n\
                 EXAMPLE 2: Divide-and-Conquer File Processing\n\
                 // Split large task across multiple agents\n\
                 \n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Process all .rs files in src/api and extract function signatures\",\n\
                   \"add_dirs\": [\"./src/api\"],\n\
                   \"await_completion_ms\": 0\n\
                 })\n\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 1,\n\
                   \"prompt\": \"Process all .rs files in src/db and extract function signatures\",\n\
                   \"add_dirs\": [\"./src/db\"],\n\
                   \"await_completion_ms\": 0\n\
                 })\n\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 2,\n\
                   \"prompt\": \"Process all .rs files in src/utils and extract function signatures\",\n\
                   \"add_dirs\": [\"./src/utils\"],\n\
                   \"await_completion_ms\": 0\n\
                 })\n\n\
                 // Collect and merge results\n\
                 claude_agent({\"action\": \"READ\", \"agent\": 0})\n\
                 claude_agent({\"action\": \"READ\", \"agent\": 1})\n\
                 claude_agent({\"action\": \"READ\", \"agent\": 2})\n\
                 // → Merge all function signatures into master list\n\n\
                 MONITORING PARALLEL AGENTS:\n\n\
                 // See all agents at once\n\
                 claude_agent({\"action\": \"LIST\"})\n\
                 // Returns:\n\
                 // [\n\
                 //   {agent: 0, message_count: 5, working: true, completed: false},\n\
                 //   {agent: 1, message_count: 3, working: true, completed: false},\n\
                 //   {agent: 2, message_count: 7, working: false, completed: true}\n\
                 // ]\n\n\
                 // Check specific agent\n\
                 claude_agent({\"action\": \"READ\", \"agent\": 2})\n\
                 // → Get detailed output from agent 2\n\n\
                 COORDINATION PATTERNS:\n\
                 1. Fire-and-Forget: SPAWN all with await_completion_ms: 0, use LIST to monitor, READ when completed\n\
                 2. Staged Parallel: SPAWN first batch, wait, SPAWN second batch based on results\n\
                 3. Rolling Window: SPAWN agents 0-2, reuse numbers after KILL\n\n\
                 BEST PRACTICES:\n\
                 1. Keep parallel agents focused on independent tasks\n\
                 2. Use fire-and-forget (await_completion_ms: 0) for true parallelism\n\
                 3. Use LIST for overview, READ for detailed results\n\
                 4. Always KILL agents when done to free resources\n\
                 5. YOU synthesize results - don't spawn another agent for that\n\n\
                 Remember: Spawn multiple focused agents, let them work independently, synthesize results yourself!",
            ),
        },
    ]
}

/// Teach: add_dirs for deep context, research-focused patterns
fn prompt_research_agents() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I create research-focused sub-agents that have deep context from specific directories? What is add_dirs and when should I use it?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Research agents benefit from loading deep context using add_dirs parameter. This pre-loads directory contents into the agent's context, enabling thorough analysis without repeated file reads.\n\n\
                 WHAT IS add_dirs:\n\
                 • Loads directory structure and file contents into agent context\n\
                 • Agent can reference files without explicit fs_read_file calls\n\
                 • Enables comprehensive understanding of codebase sections\n\
                 • Critical for research tasks requiring broad context\n\n\
                 WHEN TO USE add_dirs:\n\
                 ✓ Agent needs to analyze entire directory structure\n\
                 ✓ Task requires understanding relationships between many files\n\
                 ✓ Research across multiple related directories\n\
                 ✓ Code review of specific modules\n\
                 ✓ Documentation generation from source\n\n\
                 WHEN NOT TO USE add_dirs:\n\
                 ✗ Agent only needs 1-2 specific files (use fs_read_file instead)\n\
                 ✗ Directories contain large binary files\n\
                 ✗ Very large directories (100+ files) - may hit context limits\n\
                 ✗ Agent doesn't need file contents (just structure)\n\n\
                 EXAMPLE 1: Deep Authentication Research\n\
                 // Load all auth-related code into agent context\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Analyze our authentication implementation. Check for security vulnerabilities, code quality issues, and best practices. Document the entire authentication flow.\",\n\
                   \"add_dirs\": [\n\
                     \"./src/auth\",\n\
                     \"./src/middleware/auth\",\n\
                     \"./tests/auth\"\n\
                   ],\n\
                   \"allowed_tools\": [\"fs_read_file\", \"fs_search\", \"grep\"],\n\
                   \"max_turns\": 20\n\
                 })\n\
                 // → Agent has full context of all auth code\n\
                 // → Can reference any file without re-reading\n\
                 // → Understands relationships between files\n\n\
                 EXAMPLE 2: Database Schema Analysis\n\
                 // Research database implementation\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Analyze database schema, migrations, and queries. Check for:\n\
                     - Missing indexes on foreign keys\n\
                     - N+1 query patterns\n\
                     - Inefficient queries\n\
                     - Schema consistency issues\n\
                   Create detailed report with recommendations.\",\n\
                   \"add_dirs\": [\n\
                     \"./src/db/schema\",\n\
                     \"./src/db/migrations\",\n\
                     \"./src/db/queries\"\n\
                   ],\n\
                   \"allowed_tools\": [\n\
                     \"fs_read_file\",\n\
                     \"fs_search\",\n\
                     \"database_schema\",\n\
                     \"database_query\"\n\
                   ],\n\
                   \"max_turns\": 25\n\
                 })\n\n\
                 add_dirs SYNTAX:\n\n\
                 // Single directory (string)\n\
                 \"add_dirs\": \"./src/auth\"\n\n\
                 // Multiple directories (array)\n\
                 \"add_dirs\": [\"./src/auth\", \"./src/middleware\", \"./tests/auth\"]\n\n\
                 // Relative or absolute paths\n\
                 \"add_dirs\": [\"./src\", \"/absolute/path/to/dir\"]\n\n\
                 RESEARCH WORKFLOW PATTERN:\n\n\
                 1. Initial broad research:\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Survey the authentication system. What patterns are used?\",\n\
                   \"add_dirs\": [\"./src/auth\"]\n\
                 })\n\n\
                 2. Follow-up with specific questions:\n\
                 claude_agent({\n\
                   \"action\": \"SEND\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"I see JWT tokens are used. How is token refresh implemented?\"\n\
                 })\n\n\
                 3. Deep dive:\n\
                 claude_agent({\n\
                   \"action\": \"SEND\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Check for security vulnerabilities in the refresh token flow\"\n\
                 })\n\n\
                 4. Get final report:\n\
                 claude_agent({\n\
                   \"action\": \"SEND\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Summarize all findings with specific recommendations\"\n\
                 })\n\n\
                 BEST PRACTICES:\n\
                 1. Load related directories together (e.g., src and tests)\n\
                 2. Use specific directories, not root (avoid loading too much)\n\
                 3. Combine with allowed_tools for read-only research agents\n\
                 4. Set higher max_turns (20-30) and timeouts for research\n\
                 5. Use SEND for iterative refinement after initial research\n\n\
                 Remember: add_dirs loads deep context for comprehensive research!",
            ),
        },
    ]
}

/// Teach: READ action, timeout handling, background execution
fn prompt_progress_monitoring() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I monitor the progress of long-running sub-agents? How do I handle timeouts and background execution?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "You can monitor sub-agent progress using READ action and control execution timing with await_completion_ms. This enables fire-and-forget patterns and long-running background agents.\n\n\
                 THREE EXECUTION MODES:\n\n\
                 1. BLOCKING (Default):\n\
                    await_completion_ms: 300000  // 5 minutes\n\
                    → Waits up to 5 minutes for completion\n\
                    → Returns results when done or current state at timeout\n\
                    → Agent continues in background if times out\n\n\
                 2. FIRE-AND-FORGET:\n\
                    await_completion_ms: 0\n\
                    → Returns immediately\n\
                    → Agent runs entirely in background\n\
                    → Use READ to check progress\n\n\
                 3. CUSTOM TIMEOUT:\n\
                    await_completion_ms: 120000  // 2 minutes\n\
                    → Wait exactly 2 minutes\n\
                    → Useful for known-duration tasks\n\n\
                 MONITORING WITH READ ACTION:\n\n\
                 READ checks agent status without sending new prompts.\n\
                 \n\
                 claude_agent({\"action\": \"READ\", \"agent\": 0})\n\
                 \n\
                 Returns:\n\
                 • output: Current agent work and findings\n\
                 • message_count: Conversation turns used\n\
                 • working: Is agent currently active (true/false)\n\
                 • completed: Has agent finished (true/false)\n\
                 • exit_code: 0 = success, non-zero = error (when completed)\n\n\
                 EXAMPLE 1: Fire-and-Forget with Periodic Monitoring\n\
                 // Start long-running research agent\n\
                 claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Perform comprehensive security audit of entire codebase. Check for vulnerabilities, insecure patterns, exposed secrets, and create detailed report.\",\n\
                   \"add_dirs\": [\"./src\", \"./tests\"],\n\
                   \"max_turns\": 30,\n\
                   \"await_completion_ms\": 0  // Fire-and-forget\n\
                 })\n\
                 // → Returns immediately, agent works in background\n\n\
                 // Continue with other work...\n\
                 // ...\n\n\
                 // Check progress later (1st check)\n\
                 claude_agent({\"action\": \"READ\", \"agent\": 0})\n\
                 // → {working: true, completed: false, message_count: 8, output: \"...current findings...\"}\n\n\
                 // Continue other work...\n\
                 // ...\n\n\
                 // Check again later (2nd check)\n\
                 claude_agent({\"action\": \"READ\", \"agent\": 0})\n\
                 // → {working: true, completed: false, message_count: 15, output: \"...more findings...\"}\n\n\
                 // Finally check completion\n\
                 claude_agent({\"action\": \"READ\", \"agent\": 0})\n\
                 // → {working: false, completed: true, exit_code: 0, output: \"...final report...\"}\n\n\
                 EXAMPLE 2: Timeout Handling\n\
                 // Start with reasonable timeout\n\
                 let result = claude_agent({\n\
                   \"action\": \"SPAWN\",\n\
                   \"agent\": 0,\n\
                   \"prompt\": \"Analyze database performance issues\",\n\
                   \"await_completion_ms\": 120000  // 2 min timeout\n\
                 })\n\n\
                 // Check if completed\n\
                 if (result.completed) {\n\
                   // Agent finished within 2 minutes\n\
                   console.log(\"Analysis complete:\", result.output)\n\
                 } else {\n\
                   // Timed out, agent still working in background\n\
                   console.log(\"Still working, current progress:\", result.output)\n\
                   \n\
                   // Check again later\n\
                   let final = claude_agent({\"action\": \"READ\", \"agent\": 0})\n\
                   if (final.completed) {\n\
                     console.log(\"Now complete:\", final.output)\n\
                   }\n\
                 }\n\n\
                 UNDERSTANDING RESPONSE FIELDS:\n\n\
                 1. completed: true/false\n\
                    • true: Agent finished, exit_code is set\n\
                    • false: Agent still working or timed out\n\n\
                 2. working: true/false\n\
                    • true: Agent actively processing\n\
                    • false: Agent idle or finished\n\n\
                 3. exit_code: number or null\n\
                    • 0: Success (when completed: true)\n\
                    • non-zero: Error (when completed: true)\n\
                    • null: Still running (when completed: false)\n\n\
                 4. message_count: number\n\
                    • How many turns used so far\n\
                    • Approaches max_turns limit\n\n\
                 5. output: string\n\
                    • Agent's current work/findings\n\
                    • Updates as agent progresses\n\
                    • Final results when completed\n\n\
                 TIMEOUT RECOMMENDATIONS:\n\n\
                 Quick tasks (1-2 min):\n\
                 await_completion_ms: 120000\n\n\
                 Medium tasks (5 min):\n\
                 await_completion_ms: 300000  // Default\n\n\
                 Long tasks (10+ min):\n\
                 await_completion_ms: 600000  // Or use 0 for fire-and-forget\n\n\
                 Very long tasks:\n\
                 await_completion_ms: 0  // Fire-and-forget, use READ\n\n\
                 BEST PRACTICES:\n\
                 1. Use await_completion_ms: 0 for truly long tasks\n\
                 2. Always check completed field before trusting results\n\
                 3. Use LIST for multi-agent overview, READ for detailed status\n\
                 4. Don't spam READ - wait 30s-1min between checks\n\n\
                 Remember: Fire-and-forget + READ enables background processing!",
            ),
        },
    ]
}
