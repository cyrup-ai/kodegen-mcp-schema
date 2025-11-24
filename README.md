<div align="center">
  <img src="assets/img/banner.png" alt="Kodegen AI Banner" width="100%" />
</div>

# kodegen-mcp-schema

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org)

Lightweight MCP (Model Context Protocol) tool schema definitions for the kodegen ecosystem.

## Overview

`kodegen_mcp_schema` provides type-safe schema definitions for AI agent tools and interactions. This library serves as the single source of truth for all tool schemas, with minimal dependencies and maximum reusability.

**Design Philosophy**: Schema-only library with zero heavy dependencies. Contains only Args and PromptArgs type definitions using `serde`, `schemars`, and `serde_json`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
kodegen_mcp_schema = "0.1.0"
```

Or install directly from the repository:

```toml
[dependencies]
kodegen_mcp_schema = { git = "https://github.com/cyrup-ai/kodegen-mcp-schema" }
```

## Features

### Comprehensive Tool Schemas

- **Filesystem** - File operations, search, directory management
- **Terminal** - Command execution and process control
- **Git** - Version control operations (init, commit, branch, worktree)
- **GitHub** - Issues, PRs, code search, security scanning
- **Browser** - Web automation, research sessions, agent-based browsing
- **Database** - Schema inspection, SQL execution, connection pooling
- **Claude Agent** - Multi-agent spawning with memory and reasoning
- **Web Scraping** - Crawling, content extraction, search indexing
- **Reasoning** - Sequential thinking, MCTS, beam search strategies
- **Prompts** - Template management and rendering
- **Introspection** - Usage stats and tool call history

### Type-Safe Schemas

All schemas use Rust's type system with:
- JSON Schema generation via `schemars`
- Serde serialization/deserialization
- Validation attributes for constraints
- Comprehensive documentation

## Usage

```rust
use kodegen_mcp_schema::{ReadFileArgs, StartSearchArgs, SearchType};

// File reading with optional pagination
let read_args = ReadFileArgs {
    path: "/path/to/file.txt".to_string(),
    offset: 0,
    length: Some(100),
    is_url: false,
};

// Advanced file search
let search_args = StartSearchArgs {
    path: "/project".to_string(),
    pattern: "TODO".to_string(),
    search_type: SearchType::Content,
    file_pattern: Some("*.rs".to_string()),
    max_results: Some(100),
    ..Default::default()
};

// GitHub operations
use kodegen_mcp_schema::{CreateIssueArgs, SearchCodeArgs};

let issue = CreateIssueArgs {
    owner: "cyrup-ai".to_string(),
    repo: "kodegen-mcp-schema".to_string(),
    title: "Bug report".to_string(),
    body: Some("Description here".to_string()),
    labels: Some(vec!["bug".to_string()]),
    assignees: None,
};
```

## Schema Pattern

Each tool follows a consistent pattern with two primary types:

- **`XxxArgs`** - Runtime arguments for tool execution
- **`XxxPromptArgs`** - Arguments for generating contextual prompts

Example:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReadFileArgs {
    pub path: String,
    #[serde(default)]
    pub offset: i64,
    #[serde(default)]
    pub length: Option<usize>,
}
```

## Development

### Prerequisites

- Rust nightly toolchain
- Components: rustfmt, clippy

### Build and Test

```bash
# Build the library
cargo build

# Run tests
cargo test

# Run clippy linter
cargo clippy

# Format code
cargo fmt

# Check without building
cargo check
```

## Architecture

### Module Organization

The library is organized into domain-specific modules:

```
src/
├── lib.rs              # Re-exports all types
├── filesystem.rs       # File operations and search
├── terminal.rs         # Process and command execution
├── git.rs             # Git operations
├── github.rs          # GitHub API interactions
├── browser.rs         # Browser automation
├── citescrape.rs      # Web crawling
├── database.rs        # Database operations
├── claude_agent.rs    # Agent spawning and memory
├── reasoning.rs       # AI reasoning tools
├── prompt.rs          # Template management
├── config.rs          # Configuration
├── process.rs         # System processes
└── introspection.rs   # Usage statistics
```

### Session-Based Patterns

Several tools use async session patterns for long-running operations:

- **Browser Research**: `start_browser_research` → `get_research_status` → `get_research_result`
- **File Search**: `start_search` → `get_search_results` → `stop_search`
- **Web Crawling**: `scrape_url` → `scrape_check_results` → `scrape_search_results`
- **Terminal Commands**: `start_terminal_command` → `read_terminal_output` → `send_terminal_input`
- **Claude Agents**: `spawn_claude_agent` → `read_claude_agent_output` → `send_claude_agent_prompt`

## Advanced Features

### Powerful Search Capabilities

The filesystem search tool supports:
- Multiple search modes (files, content)
- Regex engines (Rust, PCRE2)
- Case modes (sensitive, insensitive, smart)
- Boundary matching (word, line)
- Output formats (full, files-only, count-per-file)
- Sorting, preprocessing, and compression handling

### Claude Agent Memory

Memory tools for persistent agent knowledge:
- **Memorize** - Store information in named libraries
- **Recall** - Semantic search across memories
- **List Libraries** - View available memory stores
- **Check Status** - Monitor async memorization

## Contributing

Contributions are welcome! Please ensure:

1. Code follows existing patterns (Args/PromptArgs structure)
2. All types derive required traits (Debug, Clone, Serialize, Deserialize, JsonSchema)
3. Use `#[serde(default)]` for optional fields with defaults
4. Add comprehensive documentation
5. Run `cargo fmt` and `cargo clippy` before submitting

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Links

- **Repository**: [github.com/cyrup-ai/kodegen-mcp-schema](https://github.com/cyrup-ai/kodegen-mcp-schema)
- **Issues**: [github.com/cyrup-ai/kodegen-mcp-schema/issues](https://github.com/cyrup-ai/kodegen-mcp-schema/issues)
- **Model Context Protocol**: [modelcontextprotocol.io](https://modelcontextprotocol.io)

## Keywords

`ai`, `ml`, `agents`, `candle`, `inference`, `mcp`, `schemas`, `tools`, `automation`
