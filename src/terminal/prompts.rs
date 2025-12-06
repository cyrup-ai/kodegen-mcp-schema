//! Prompt messages for terminal tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptMessage, PromptMessageRole, PromptMessageContent, PromptArgument};
use super::prompt_args::TerminalPromptArgs;

/// Prompt provider for terminal tool
///
/// This is the ONLY way to provide prompts for terminal - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct TerminalPrompts;

impl PromptProvider for TerminalPrompts {
    type PromptArgs = TerminalPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("basic") => prompt_basic(),
            Some("parallel") => prompt_parallel(),
            Some("background") => prompt_background(),
            Some("monitoring") => prompt_monitoring(),
            _ => prompt_comprehensive(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![
            PromptArgument {
                name: "scenario".to_string(),
                title: None,
                description: Some("Scenario to show (basic, parallel, background, monitoring)".to_string()),
                required: Some(false),
            }
        ]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE TERMINALS
// ============================================================================

/// Basic command execution and path quoting
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I execute basic shell commands using the terminal tool?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The terminal tool executes shell commands in persistent, stateful sessions. Here's how to use it for basic command execution:\n\n\
                 BASIC COMMAND EXECUTION:\n\
                 1. Simple command:\n\
                    terminal({\"action\": \"EXEC\", \"command\": \"ls -la\"})\n\n\
                 2. Command with path (ALWAYS quote paths with spaces):\n\
                    terminal({\"action\": \"EXEC\", \"command\": \"cd \\\"/path/with spaces\\\" && ls\"})\n\n\
                 3. Sequential commands (use && to chain):\n\
                    terminal({\"action\": \"EXEC\", \"command\": \"mkdir build && cd build && cmake ..\"})\n\n\
                 4. Check file before acting:\n\
                    terminal({\"action\": \"EXEC\", \"command\": \"test -f config.toml && cargo build\"})\n\n\
                 CRITICAL PATH QUOTING RULES:\n\
                 - ALWAYS use double quotes around paths with spaces\n\
                 - Escape quotes inside JSON strings: \\\"path with spaces\\\"\n\
                 - Example: {\"command\": \"cd \\\"/Users/name/My Projects\\\" && ls\"}\n\n\
                 SEQUENTIAL CHAINING:\n\
                 - Use && for commands that depend on previous success\n\
                 - Example: git add . && git commit -m \\\"msg\\\" && git push\n\
                 - First command must succeed for next to run\n\
                 - If any command fails, chain stops immediately\n\n\
                 PARAMETERS:\n\
                 - action: \"EXEC\" (execute), \"READ\" (check progress), \"LIST\" (show terminals), \"KILL\" (cleanup)\n\
                 - command: Shell command to execute (required for EXEC)\n\
                 - terminal: Terminal number 0-N (default: 0)\n\
                 - await_completion_ms: Timeout in milliseconds (default: 300000 = 5 minutes)\n\
                 - clear: Clear terminal before command (default: true)\n\
                 - tail: Lines to return from output (default: 2000)\n\n\
                 RESPONSE:\n\
                 - terminal: Terminal number used\n\
                 - exit_code: 0 = success, non-zero = error, null = still running\n\
                 - cwd: Current working directory after command\n\
                 - duration_ms: How long command took\n\
                 - completed: true if finished, false if timed out or still running\n\
                 - Output appears in display field (Content[0])\n\n\
                 COMMON PATTERNS:\n\
                 1. Build project:\n\
                    terminal({\"command\": \"cargo build --release\"})\n\
                 2. Run tests:\n\
                    terminal({\"command\": \"npm test\"})\n\
                 3. Git workflow:\n\
                    terminal({\"command\": \"git status && git add . && git commit -m \\\"Update\\\" && git push\"})\n\
                 4. Check then install:\n\
                    terminal({\"command\": \"which node || brew install node\"})\n\n\
                 DIRECTORY PERSISTENCE:\n\
                 - Terminals maintain their working directory between commands\n\
                 - Each terminal (0, 1, 2...) has independent state\n\
                 - Use absolute paths when working across different directories\n\
                 - Example: cd /project && cargo build (cwd persists in terminal:0)\n\n\
                 ERROR HANDLING:\n\
                 - Check exit_code in response: 0 = success\n\
                 - Non-zero exit codes indicate errors\n\
                 - Use && chaining to stop on first error\n\
                 - Use || for fallback commands (cmd1 || cmd2)",
            ),
        },
    ]
}

/// Parallel execution with multiple terminals
fn prompt_parallel() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "When should I use multiple terminals for parallel execution?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use multiple terminals when you need to run independent commands concurrently. Each terminal is a separate stateful session.\n\n\
                 WHEN TO USE PARALLEL TERMINALS:\n\
                 - Running independent build tasks simultaneously\n\
                 - Checking multiple conditions at once\n\
                 - Long-running servers while doing other work\n\
                 - Testing in different directories concurrently\n\n\
                 PARALLEL EXECUTION EXAMPLES:\n\
                 1. Run multiple builds concurrently:\n\
                    // Terminal 0: Build Rust project\n\
                    terminal({\"terminal\": 0, \"command\": \"cargo build --release\"})\n\
                    // Terminal 1: Build Node project (runs at same time)\n\
                    terminal({\"terminal\": 1, \"command\": \"npm run build\"})\n\n\
                 2. Check multiple repos:\n\
                    terminal({\"terminal\": 0, \"command\": \"cd /repo1 && git status\"})\n\
                    terminal({\"terminal\": 1, \"command\": \"cd /repo2 && git status\"})\n\
                    terminal({\"terminal\": 2, \"command\": \"cd /repo3 && git status\"})\n\n\
                 3. Run tests in parallel:\n\
                    terminal({\"terminal\": 0, \"command\": \"cargo test --package core\"})\n\
                    terminal({\"terminal\": 1, \"command\": \"cargo test --package utils\"})\n\n\
                 4. Start server and run commands:\n\
                    // Terminal 0: Start dev server (long-running)\n\
                    terminal({\"terminal\": 0, \"command\": \"npm run dev\", \"await_completion_ms\": 0})\n\
                    // Terminal 1: Run other commands while server runs\n\
                    terminal({\"terminal\": 1, \"command\": \"npm run test\"})\n\n\
                 WHEN TO USE SEQUENTIAL (&&) INSTEAD:\n\
                 - Commands depend on each other\n\
                 - Order matters\n\
                 - Later commands need results from earlier ones\n\
                 - Example: git add && git commit && git push (MUST be sequential)\n\n\
                 TERMINAL NUMBERING:\n\
                 - Terminals are numbered 0, 1, 2, 3...\n\
                 - Each terminal maintains independent state:\n\
                   - Working directory (cwd)\n\
                   - Environment variables\n\
                   - Shell history\n\
                 - Use different numbers for truly parallel work\n\
                 - Reuse same terminal for related sequential commands\n\n\
                 CHECKING PARALLEL WORK STATUS:\n\
                 Use LIST action to see all active terminals:\n\
                 terminal({\"action\": \"LIST\"})\n\
                 Returns array of terminal snapshots showing:\n\
                 - terminal: Terminal number\n\
                 - cwd: Current directory\n\
                 - exit_code: Last command result\n\
                 - completed: Whether last command finished\n\n\
                 BEST PRACTICES:\n\
                 1. Use terminal:0 for primary workflow\n\
                 2. Use terminal:1, 2, 3... for parallel tasks\n\
                 3. Clean up unused terminals with KILL action\n\
                 4. Check LIST to see which terminals are active\n\
                 5. Don't create too many terminals (cleanup when done)\n\n\
                 EXAMPLE WORKFLOW:\n\
                 // Check what terminals are active\n\
                 terminal({\"action\": \"LIST\"})\n\
                 // Run parallel linters\n\
                 terminal({\"terminal\": 0, \"command\": \"cargo clippy\"})\n\
                 terminal({\"terminal\": 1, \"command\": \"npm run lint\"})\n\
                 terminal({\"terminal\": 2, \"command\": \"shellcheck **.sh\"})\n\
                 // Check all completed\n\
                 terminal({\"action\": \"LIST\"})",
            ),
        },
    ]
}

/// Background execution and fire-and-forget patterns
fn prompt_background() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I run long-running commands in the background?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use await_completion_ms parameter to control timeout behavior. Set to 0 for fire-and-forget background execution.\n\n\
                 TIMEOUT CONTROL:\n\
                 - await_completion_ms: Maximum time to wait (milliseconds)\n\
                 - Default: 300000 (5 minutes)\n\
                 - Set to 0: Fire-and-forget (returns immediately)\n\
                 - On timeout: Returns current output, command continues in background\n\n\
                 BACKGROUND EXECUTION PATTERNS:\n\
                 1. Fire-and-forget (immediate return):\n\
                    terminal({\"command\": \"npm run dev\", \"await_completion_ms\": 0})\n\
                    // Returns immediately, server runs in background\n\n\
                 2. Custom timeout:\n\
                    terminal({\"command\": \"cargo build\", \"await_completion_ms\": 60000})\n\
                    // Wait up to 1 minute, then timeout\n\n\
                 3. Long build with progress checking:\n\
                    terminal({\"terminal\": 0, \"command\": \"cargo build --release\", \"await_completion_ms\": 10000})\n\
                    // If times out, check progress later with READ\n\n\
                 WHEN TO USE BACKGROUND EXECUTION:\n\
                 - Starting development servers\n\
                 - Running watch/file monitoring processes\n\
                 - Long compilation/build tasks\n\
                 - Database migrations\n\
                 - Any process that takes longer than 5 minutes\n\n\
                 TIMEOUT BEHAVIOR:\n\
                 When command times out:\n\
                 - Response shows completed: false\n\
                 - exit_code: null (still running)\n\
                 - Command continues executing in background\n\
                 - Use READ action to check progress later\n\
                 - Terminal maintains state and output buffer\n\n\
                 CHECKING BACKGROUND COMMANDS:\n\
                 Use READ action to see current output:\n\
                 1. Start long command:\n\
                    terminal({\"terminal\": 0, \"command\": \"cargo build --release\", \"await_completion_ms\": 0})\n\
                 2. Check progress later:\n\
                    terminal({\"terminal\": 0, \"action\": \"READ\"})\n\
                 3. Keep checking until completed: true\n\n\
                 FIRE-AND-FORGET EXAMPLES:\n\
                 1. Start dev server:\n\
                    terminal({\"terminal\": 1, \"command\": \"npm run dev\", \"await_completion_ms\": 0})\n\
                 2. Start multiple services:\n\
                    terminal({\"terminal\": 1, \"command\": \"docker-compose up\", \"await_completion_ms\": 0})\n\
                    terminal({\"terminal\": 2, \"command\": \"redis-server\", \"await_completion_ms\": 0})\n\
                 3. Long-running build:\n\
                    terminal({\"terminal\": 0, \"command\": \"cargo build --release --all-features\", \"await_completion_ms\": 0})\n\n\
                 CUSTOM TIMEOUT EXAMPLES:\n\
                 1. Quick check (10 seconds):\n\
                    terminal({\"command\": \"cargo check\", \"await_completion_ms\": 10000})\n\
                 2. Medium build (2 minutes):\n\
                    terminal({\"command\": \"npm run build\", \"await_completion_ms\": 120000})\n\
                 3. Long test suite (10 minutes):\n\
                    terminal({\"command\": \"cargo test --all\", \"await_completion_ms\": 600000})\n\n\
                 CLEANUP:\n\
                 Kill background processes when done:\n\
                 terminal({\"terminal\": 1, \"action\": \"KILL\"})\n\
                 This gracefully shuts down the terminal and any running processes.\n\n\
                 BEST PRACTICES:\n\
                 1. Use await_completion_ms: 0 for servers and watchers\n\
                 2. Use custom timeouts for known-slow operations\n\
                 3. Check progress with READ for long-running tasks\n\
                 4. Use separate terminals for background processes\n\
                 5. KILL terminals when background processes no longer needed\n\
                 6. Monitor with LIST to see all active terminals",
            ),
        },
    ]
}

/// Monitoring command progress with READ action
fn prompt_monitoring() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I check the progress of a running or timed-out command?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "Use the READ action to check current terminal output and command status without executing a new command.\n\n\
                 READ ACTION:\n\
                 - Gets current terminal buffer snapshot\n\
                 - Shows output accumulated so far\n\
                 - Indicates if command still running or completed\n\
                 - Does NOT execute any new commands\n\
                 - Safe to call repeatedly for monitoring\n\n\
                 BASIC READ USAGE:\n\
                 terminal({\"terminal\": 0, \"action\": \"READ\"})\n\
                 Returns:\n\
                 - terminal: Which terminal (0)\n\
                 - exit_code: null = still running, 0 = success, non-zero = error\n\
                 - cwd: Current working directory\n\
                 - duration_ms: Time elapsed since command started\n\
                 - completed: true/false\n\
                 - Output in display field (Content[0])\n\n\
                 WHEN TO USE READ:\n\
                 1. After timeout:\n\
                    terminal({\"command\": \"cargo build\", \"await_completion_ms\": 10000})\n\
                    // If times out (completed: false), check progress:\n\
                    terminal({\"action\": \"READ\"})\n\n\
                 2. Monitoring background tasks:\n\
                    terminal({\"terminal\": 1, \"command\": \"npm run dev\", \"await_completion_ms\": 0})\n\
                    // Later, check if server started:\n\
                    terminal({\"terminal\": 1, \"action\": \"READ\"})\n\n\
                 3. Long-running operations:\n\
                    terminal({\"terminal\": 2, \"command\": \"cargo test --all\", \"await_completion_ms\": 0})\n\
                    // Check periodically:\n\
                    terminal({\"terminal\": 2, \"action\": \"READ\"})\n\n\
                 4. Checking parallel work status:\n\
                    // Start parallel builds\n\
                    terminal({\"terminal\": 0, \"command\": \"cargo build\", \"await_completion_ms\": 0})\n\
                    terminal({\"terminal\": 1, \"command\": \"npm build\", \"await_completion_ms\": 0})\n\
                    // Check each one\n\
                    terminal({\"terminal\": 0, \"action\": \"READ\"})\n\
                    terminal({\"terminal\": 1, \"action\": \"READ\"})\n\n\
                 MONITORING WORKFLOW:\n\
                 1. Start long command:\n\
                    terminal({\"terminal\": 0, \"command\": \"cargo build --release\", \"await_completion_ms\": 0})\n\
                 2. Wait a bit, then check:\n\
                    terminal({\"terminal\": 0, \"action\": \"READ\"})\n\
                 3. If completed: false, check again later:\n\
                    terminal({\"terminal\": 0, \"action\": \"READ\"})\n\
                 4. When completed: true, check exit_code:\n\
                    - exit_code: 0 = success\n\
                    - exit_code: non-zero = error (check output)\n\n\
                 READING SPECIFIC TERMINALS:\n\
                 - Specify terminal number to read specific session:\n\
                   terminal({\"terminal\": 2, \"action\": \"READ\"})\n\
                 - Default terminal: 0 if not specified\n\
                 - Each terminal has independent buffer\n\n\
                 COMBINED WITH LIST:\n\
                 1. See all terminals:\n\
                    terminal({\"action\": \"LIST\"})\n\
                 2. Pick one to inspect:\n\
                    terminal({\"terminal\": 3, \"action\": \"READ\"})\n\n\
                 OUTPUT CONTROL:\n\
                 Use tail parameter to limit output:\n\
                 terminal({\"terminal\": 0, \"action\": \"READ\", \"tail\": 100})\n\
                 - Shows last 100 lines only\n\
                 - Default: 2000 lines\n\
                 - Useful for long-running processes with lots of output\n\n\
                 INTERPRETING RESULTS:\n\
                 1. Still running:\n\
                    {\"exit_code\": null, \"completed\": false}\n\
                    Keep checking with READ\n\n\
                 2. Success:\n\
                    {\"exit_code\": 0, \"completed\": true}\n\
                    Command finished successfully\n\n\
                 3. Error:\n\
                    {\"exit_code\": 1, \"completed\": true}\n\
                    Command failed, check output for errors\n\n\
                 4. Still running but has output:\n\
                    {\"exit_code\": null, \"completed\": false}\n\
                    Review output to see progress (build logs, test results, etc.)\n\n\
                 BEST PRACTICES:\n\
                 - Use READ for status checks, not for executing commands\n\
                 - Check completed field first\n\
                 - If completed: true, check exit_code for success/failure\n\
                 - If completed: false, review output for progress indicators\n\
                 - Use tail parameter for long outputs\n\
                 - Don't spam READ - give commands time to run\n\
                 - Combine LIST and READ to monitor multiple terminals",
            ),
        },
    ]
}

/// Comprehensive guide covering all actions and patterns
fn prompt_comprehensive() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "Give me a complete guide to using the terminal tool effectively.",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "The terminal tool provides persistent, stateful shell sessions with support for parallel execution, background tasks, and progress monitoring.\n\n\
                 FOUR ACTIONS:\n\
                 1. EXEC (default): Execute a shell command\n\
                 2. READ: Check current terminal output/status\n\
                 3. LIST: Show all active terminals\n\
                 4. KILL: Gracefully shutdown terminal\n\n\
                 =============================================================================\n\
                 ACTION 1: EXEC - EXECUTE COMMANDS\n\
                 =============================================================================\n\n\
                 BASIC EXECUTION:\n\
                 terminal({\"action\": \"EXEC\", \"command\": \"ls -la\"})\n\
                 // action: \"EXEC\" is default, can be omitted:\n\
                 terminal({\"command\": \"ls -la\"})\n\n\
                 PATH QUOTING (CRITICAL!):\n\
                 ALWAYS quote paths with spaces:\n\
                 terminal({\"command\": \"cd \\\"/Users/name/My Projects\\\" && ls\"})\n\
                 WRONG: cd /Users/name/My Projects (will fail)\n\
                 RIGHT: cd \\\"/Users/name/My Projects\\\"\n\n\
                 SEQUENTIAL CHAINING:\n\
                 Use && to run commands in sequence:\n\
                 terminal({\"command\": \"git add . && git commit -m \\\"Update\\\" && git push\"})\n\
                 - Stops on first error\n\
                 - Each command depends on previous success\n\
                 - Use for: git workflows, build pipelines, setup scripts\n\n\
                 PARALLEL EXECUTION:\n\
                 Use different terminal numbers for concurrent work:\n\
                 terminal({\"terminal\": 0, \"command\": \"cargo build\"})\n\
                 terminal({\"terminal\": 1, \"command\": \"npm run build\"})\n\
                 terminal({\"terminal\": 2, \"command\": \"go build\"})\n\
                 All run simultaneously!\n\n\
                 TIMEOUT CONTROL:\n\
                 - Default: 300000ms (5 minutes)\n\
                 - Custom timeout:\n\
                   terminal({\"command\": \"cargo test\", \"await_completion_ms\": 120000})\n\
                 - Fire-and-forget (immediate return):\n\
                   terminal({\"command\": \"npm run dev\", \"await_completion_ms\": 0})\n\
                 - On timeout: returns current output, command continues in background\n\n\
                 PARAMETERS:\n\
                 - command (required): Shell command to execute\n\
                 - terminal (optional): Terminal number 0-N (default: 0)\n\
                 - await_completion_ms (optional): Timeout in ms (default: 300000)\n\
                 - clear (optional): Clear terminal first (default: true)\n\
                 - tail (optional): Lines to return (default: 2000)\n\n\
                 =============================================================================\n\
                 ACTION 2: READ - CHECK PROGRESS\n\
                 =============================================================================\n\n\
                 Check terminal status without executing:\n\
                 terminal({\"terminal\": 0, \"action\": \"READ\"})\n\n\
                 Returns:\n\
                 - exit_code: null (running), 0 (success), non-zero (error)\n\
                 - completed: true/false\n\
                 - cwd: Current directory\n\
                 - duration_ms: Time elapsed\n\
                 - Output in display field\n\n\
                 WHEN TO USE READ:\n\
                 1. After timeout:\n\
                    terminal({\"command\": \"cargo build\", \"await_completion_ms\": 10000})\n\
                    // Times out, check progress:\n\
                    terminal({\"action\": \"READ\"})\n\n\
                 2. Monitoring background tasks:\n\
                    terminal({\"terminal\": 1, \"command\": \"npm run dev\", \"await_completion_ms\": 0})\n\
                    // Later:\n\
                    terminal({\"terminal\": 1, \"action\": \"READ\"})\n\n\
                 3. Checking long operations:\n\
                    terminal({\"terminal\": 2, \"command\": \"cargo test --all\", \"await_completion_ms\": 0})\n\
                    // Periodically:\n\
                    terminal({\"terminal\": 2, \"action\": \"READ\"})\n\n\
                 =============================================================================\n\
                 ACTION 3: LIST - SHOW ALL TERMINALS\n\
                 =============================================================================\n\n\
                 See all active terminals:\n\
                 terminal({\"action\": \"LIST\"})\n\n\
                 Returns array of terminal snapshots:\n\
                 - terminal: Terminal number\n\
                 - cwd: Working directory\n\
                 - exit_code: Last command result\n\
                 - completed: Whether last command finished\n\n\
                 USE CASES:\n\
                 - See which terminals are active\n\
                 - Check status of parallel work\n\
                 - Find available terminal numbers\n\
                 - Monitor multiple background tasks\n\n\
                 WORKFLOW:\n\
                 1. List terminals:\n\
                    terminal({\"action\": \"LIST\"})\n\
                 2. Pick one to inspect:\n\
                    terminal({\"terminal\": 3, \"action\": \"READ\"})\n\n\
                 =============================================================================\n\
                 ACTION 4: KILL - CLEANUP TERMINAL\n\
                 =============================================================================\n\n\
                 Gracefully shutdown terminal:\n\
                 terminal({\"terminal\": 1, \"action\": \"KILL\"})\n\n\
                 - Stops any running process\n\
                 - Cleans up resources\n\
                 - Removes terminal from active list\n\
                 - Cannot be undone\n\n\
                 WHEN TO KILL:\n\
                 - Finished with background server\n\
                 - Done with parallel task\n\
                 - Need to free resources\n\
                 - Stopping long-running process\n\n\
                 =============================================================================\n\
                 DECISION TREE: WHICH ACTION TO USE?\n\
                 =============================================================================\n\n\
                 Need to RUN a command? → EXEC\n\
                 - Simple: terminal({\"command\": \"ls\"})\n\
                 - Sequential: terminal({\"command\": \"cmd1 && cmd2 && cmd3\"})\n\
                 - Parallel: Use multiple terminals (0, 1, 2...)\n\
                 - Background: terminal({\"command\": \"server\", \"await_completion_ms\": 0})\n\n\
                 Need to CHECK progress? → READ\n\
                 - After timeout: terminal({\"action\": \"READ\"})\n\
                 - Background task: terminal({\"terminal\": 1, \"action\": \"READ\"})\n\n\
                 Need to SEE all terminals? → LIST\n\
                 - terminal({\"action\": \"LIST\"})\n\n\
                 Need to STOP terminal? → KILL\n\
                 - terminal({\"terminal\": 1, \"action\": \"KILL\"})\n\n\
                 =============================================================================\n\
                 COMMON WORKFLOWS\n\
                 =============================================================================\n\n\
                 1. GIT WORKFLOW (sequential):\n\
                 terminal({\"command\": \"git status && git add . && git commit -m \\\"Update\\\" && git push\"})\n\n\
                 2. BUILD WORKFLOW (parallel):\n\
                 terminal({\"terminal\": 0, \"command\": \"cargo build --release\"})\n\
                 terminal({\"terminal\": 1, \"command\": \"npm run build\"})\n\
                 terminal({\"terminal\": 2, \"command\": \"go build ./...\"})\n\n\
                 3. TEST WORKFLOW (sequential with checks):\n\
                 terminal({\"command\": \"cargo fmt --check && cargo clippy && cargo test\"})\n\n\
                 4. DEV SERVER (background):\n\
                 terminal({\"terminal\": 1, \"command\": \"npm run dev\", \"await_completion_ms\": 0})\n\
                 // Do other work in terminal 0\n\
                 terminal({\"terminal\": 0, \"command\": \"npm run test\"})\n\
                 // Kill server when done\n\
                 terminal({\"terminal\": 1, \"action\": \"KILL\"})\n\n\
                 5. MONITORING LONG BUILD:\n\
                 terminal({\"terminal\": 0, \"command\": \"cargo build --release --all-features\", \"await_completion_ms\": 0})\n\
                 // Check progress\n\
                 terminal({\"terminal\": 0, \"action\": \"READ\"})\n\
                 // Keep checking until completed: true\n\n\
                 6. PARALLEL TESTS:\n\
                 terminal({\"terminal\": 0, \"command\": \"cargo test --package core\"})\n\
                 terminal({\"terminal\": 1, \"command\": \"cargo test --package utils\"})\n\
                 terminal({\"terminal\": 2, \"command\": \"cargo test --package api\"})\n\
                 // Check all\n\
                 terminal({\"action\": \"LIST\"})\n\n\
                 =============================================================================\n\
                 TERMINAL STATE & PERSISTENCE\n\
                 =============================================================================\n\n\
                 Each terminal maintains:\n\
                 - Working directory (cwd)\n\
                 - Environment variables\n\
                 - Shell history\n\
                 - Independent output buffer\n\n\
                 Example:\n\
                 terminal({\"terminal\": 0, \"command\": \"cd /project\"})\n\
                 terminal({\"terminal\": 0, \"command\": \"ls\"})  // Lists /project\n\
                 terminal({\"terminal\": 1, \"command\": \"ls\"})  // Lists original cwd\n\n\
                 =============================================================================\n\
                 BEST PRACTICES\n\
                 =============================================================================\n\n\
                 1. PATH QUOTING:\n\
                    ALWAYS quote paths with spaces\n\
                    terminal({\"command\": \"cd \\\"/path with spaces\\\"\"})\n\n\
                 2. SEQUENTIAL vs PARALLEL:\n\
                    Sequential (&&): When order matters or commands depend on each other\n\
                    Parallel (multiple terminals): When commands are independent\n\n\
                 3. TIMEOUT MANAGEMENT:\n\
                    Quick commands: Default (5 min) is fine\n\
                    Known-slow: Set custom timeout\n\
                    Servers/watchers: Use await_completion_ms: 0\n\n\
                 4. TERMINAL ORGANIZATION:\n\
                    terminal:0 for primary workflow\n\
                    terminal:1,2,3... for parallel tasks or background services\n\
                    KILL terminals when done to free resources\n\n\
                 5. MONITORING:\n\
                    Use READ to check progress\n\
                    Use LIST to see all active terminals\n\
                    Check completed and exit_code fields\n\n\
                 6. ERROR HANDLING:\n\
                    Check exit_code: 0 = success, non-zero = error\n\
                    Use && chaining to stop on first error\n\
                 7. DIRECTORY NAVIGATION:\n\
                    Use absolute paths for clarity\n\
                    Or use cd && command pattern\n\
                    Remember: cwd persists in each terminal\n\n\
                 =============================================================================\n\
                 RESPONSE STRUCTURE\n\
                 =============================================================================\n\n\
                 All actions return TerminalOutput:\n\
                 {\n\
                   \"terminal\": 0,              // Terminal number (null for LIST)\n\
                   \"exit_code\": 0,             // null=running, 0=success, other=error\n\
                   \"cwd\": \"/current/path\",     // Working directory\n\
                   \"duration_ms\": 1234,        // Time elapsed\n\
                   \"completed\": true,          // true=done, false=running/timeout\n\
                   \"terminals\": []             // Array for LIST action\n\
                 }\n\
                 Output text appears in display field (Content[0])\n\n\
                 =============================================================================\n\
                 QUICK REFERENCE\n\
                 =============================================================================\n\n\
                 Execute: terminal({\"command\": \"ls\"})\n\
                 Sequential: terminal({\"command\": \"cmd1 && cmd2\"})\n\
                 Parallel: terminal({\"terminal\": 0, \"command\": \"build1\"})\n\
                           terminal({\"terminal\": 1, \"command\": \"build2\"})\n\
                 Background: terminal({\"command\": \"server\", \"await_completion_ms\": 0})\n\
                 Check progress: terminal({\"action\": \"READ\"})\n\
                 List all: terminal({\"action\": \"LIST\"})\n\
                 Cleanup: terminal({\"terminal\": 1, \"action\": \"KILL\"})\n\n\
                 Remember: You are executing commands in a real shell. Quote paths with spaces, chain with &&, use multiple terminals for parallel work, and monitor with READ/LIST!",
            ),
        },
    ]
}
