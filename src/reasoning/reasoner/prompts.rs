//! Prompt messages for reasoner tool

use crate::tool::PromptProvider;
use rmcp::model::{PromptArgument, PromptMessage, PromptMessageContent, PromptMessageRole};
use super::prompt_args::ReasonerPromptArgs;

/// Prompt provider for reasoner tool
///
/// This is the ONLY way to provide prompts for reasoner - tools cannot implement inline.
/// The PromptProvider trait is sealed and can only be implemented in kodegen-mcp-schema.
pub struct ReasonerPrompts;

impl PromptProvider for ReasonerPrompts {
    type PromptArgs = ReasonerPromptArgs;

    fn generate_prompts(args: &Self::PromptArgs) -> Vec<PromptMessage> {
        match args.scenario.as_deref() {
            Some("beam_search") => prompt_beam_search(),
            Some("mcts") => prompt_mcts(),
            Some("branching") => prompt_branching(),
            Some("strategies") => prompt_strategies(),
            _ => prompt_basic(),
        }
    }

    fn prompt_arguments() -> Vec<PromptArgument> {
        vec![PromptArgument {
            name: "scenario".to_string(),
            title: None,
            description: Some(
                "Scenario to show (basic, beam_search, mcts, branching, strategies)".to_string(),
            ),
            required: Some(false),
        }]
    }
}

// ============================================================================
// HELPER FUNCTIONS - TEACH AI AGENTS HOW TO USE REASONER
// ============================================================================

/// Basic reasoning steps
fn prompt_basic() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use the reasoner tool for basic step-by-step reasoning?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "STRUCTURED REASONING:\n\n\
                 The reasoner tool helps you think through problems systematically by breaking them into discrete steps.\n\n\
                 1. Start reasoning chain:\n\
                    reasoner({\n\
                        \"thought\": \"Let me analyze the authentication bug. First, I'll identify where tokens are validated.\",\n\
                        \"thought_number\": 1,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": true\n\
                    })\n\n\
                 2. Continue reasoning:\n\
                    reasoner({\n\
                        \"thought\": \"The token validation happens in auth.rs. I see the expiry check but no refresh logic.\",\n\
                        \"thought_number\": 2,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": true\n\
                    })\n\n\
                 3. Intermediate steps (3-4 omitted for brevity):\n\
                    reasoner({\n\
                        \"thought\": \"Examined refresh token flow in OAuth spec. Need to implement token refresh before validation.\",\n\
                        \"thought_number\": 3,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": true\n\
                    })\n\n\
                 4. Final conclusion:\n\
                    reasoner({\n\
                        \"thought\": \"The fix requires adding refresh token handling before expiry check.\",\n\
                        \"thought_number\": 5,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": false\n\
                    })\n\n\
                 KEY PARAMETERS:\n\
                 - thought: Current reasoning step (detailed explanation of what you're thinking)\n\
                 - thought_number: Step counter (1-based, must increment sequentially)\n\
                 - total_thoughts: Expected total steps (estimate, can adjust as you go)\n\
                 - next_thought_needed: Continue (true) or finish (false)\n\n\
                 RESPONSE FIELDS:\n\
                 - node_id: Unique identifier for this thought\n\
                 - score: Quality score for this reasoning step\n\
                 - depth: How deep in the reasoning tree\n\
                 - is_complete: Whether reasoning chain is finished\n\
                 - best_score: Highest scoring path so far\n\n\
                 WHEN TO USE:\n\
                 - Debugging complex issues\n\
                 - Step-by-step problem solving\n\
                 - Breaking down large tasks\n\
                 - Methodical analysis",
            ),
        },
    ]
}

/// Beam search strategy
fn prompt_beam_search() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use beam search strategy to explore multiple solution paths?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "BEAM SEARCH STRATEGY:\n\n\
                 Beam search explores multiple solution paths simultaneously, keeping only the top N candidates at each step.\n\
                 This is ideal when you have multiple viable approaches and want to compare them systematically.\n\n\
                 1. Configure beam search:\n\
                    reasoner({\n\
                        \"thought\": \"Analyzing database optimization. Considering indexing vs query rewrite.\",\n\
                        \"thought_number\": 1,\n\
                        \"total_thoughts\": 6,\n\
                        \"next_thought_needed\": true,\n\
                        \"strategy_type\": \"beam_search\",\n\
                        \"beam_width\": 3\n\
                    })\n\n\
                 2. Beam tracks top paths:\n\
                    reasoner({\n\
                        \"thought\": \"Path 1: Add composite index on (user_id, created_at). Path 2: Rewrite JOIN to subquery. Path 3: Add query cache.\",\n\
                        \"thought_number\": 2,\n\
                        \"total_thoughts\": 6,\n\
                        \"next_thought_needed\": true,\n\
                        \"strategy_type\": \"beam_search\",\n\
                        \"beam_width\": 3\n\
                    })\n\n\
                 3. Evaluate paths:\n\
                    reasoner({\n\
                        \"thought\": \"Path 1 score: High (solves root cause). Path 2 score: Medium (complex). Path 3 score: Low (doesn't address N+1).\",\n\
                        \"thought_number\": 3,\n\
                        \"total_thoughts\": 6,\n\
                        \"next_thought_needed\": true,\n\
                        \"strategy_type\": \"beam_search\",\n\
                        \"beam_width\": 3\n\
                    })\n\n\
                 HOW IT WORKS:\n\
                 - Maintains beam_width top-scoring paths\n\
                 - At each step, explores all paths\n\
                 - Prunes lower-scoring alternatives\n\
                 - Keeps only top N paths for next iteration\n\
                 - Final result is highest-scoring complete path\n\n\
                 BEAM PARAMETERS:\n\
                 - strategy_type: \"beam_search\"\n\
                 - beam_width: 1-10 (default: 3)\n\
                   - beam_width: 1 = greedy search (fastest, may miss better solutions)\n\
                   - beam_width: 3-5 = balanced exploration (recommended)\n\
                   - beam_width: 8-10 = thorough exploration (slower, more comprehensive)\n\n\
                 USE CASES:\n\
                 - Multiple valid approaches to compare\n\
                 - Architecture decisions with trade-offs\n\
                 - Optimization problems with several strategies\n\
                 - Uncertain which path is best\n\
                 - Want to explore alternatives systematically\n\n\
                 ADVANTAGES:\n\
                 - Explores multiple solutions in parallel\n\
                 - Prevents getting stuck on suboptimal path\n\
                 - Balances breadth and depth of search\n\
                 - Efficient compared to exhaustive search",
            ),
        },
    ]
}

/// Monte Carlo Tree Search
fn prompt_mcts() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use Monte Carlo Tree Search for complex decision-making?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "MONTE CARLO TREE SEARCH:\n\n\
                 MCTS uses Monte Carlo simulations to explore and evaluate decision trees.\n\
                 It balances exploration (trying new paths) with exploitation (following proven paths).\n\n\
                 1. Start MCTS reasoning:\n\
                    reasoner({\n\
                        \"thought\": \"Evaluating refactoring approaches for the payment module.\",\n\
                        \"thought_number\": 1,\n\
                        \"total_thoughts\": 8,\n\
                        \"next_thought_needed\": true,\n\
                        \"strategy_type\": \"mcts\",\n\
                        \"num_simulations\": 50\n\
                    })\n\n\
                 2. MCTS evaluates branches:\n\
                    reasoner({\n\
                        \"thought\": \"Branch A: Extract PaymentProcessor interface. Branch B: Use strategy pattern. Branch C: Keep monolithic with better organization.\",\n\
                        \"thought_number\": 2,\n\
                        \"total_thoughts\": 8,\n\
                        \"next_thought_needed\": true,\n\
                        \"strategy_type\": \"mcts\",\n\
                        \"num_simulations\": 50\n\
                    })\n\n\
                 3. Deep dive on promising branch:\n\
                    reasoner({\n\
                        \"thought\": \"Branch A shows best simulation results. Exploring interface design: PaymentProcessor with process(), validate(), refund() methods.\",\n\
                        \"thought_number\": 3,\n\
                        \"total_thoughts\": 8,\n\
                        \"next_thought_needed\": true,\n\
                        \"strategy_type\": \"mcts\",\n\
                        \"num_simulations\": 50\n\
                    })\n\n\
                 HOW MCTS WORKS:\n\
                 1. Selection: Choose most promising node using UCB1 (Upper Confidence Bound)\n\
                 2. Expansion: Add new child nodes to explore\n\
                 3. Simulation: Run random playouts to evaluate\n\
                 4. Backpropagation: Update node scores based on results\n\
                 5. Repeat for num_simulations iterations\n\n\
                 MCTS VARIANTS:\n\
                 - \"mcts\": Standard MCTS with UCB1\n\
                   - Balanced exploration/exploitation\n\
                   - Good for most problems\n\
                   - Uses sqrt(2) exploration constant\n\n\
                 - \"mcts_002_alpha\": High exploration variant\n\
                   - Increased exploration coefficient\n\
                   - Tries more unproven paths\n\
                   - Better for creative/novel solutions\n\
                   - Use when best path is unclear\n\n\
                 - \"mcts_002alt_alpha\": Length-rewarding variant\n\
                   - Rewards longer reasoning chains\n\
                   - Encourages thorough analysis\n\
                   - Better for problems needing detail\n\
                   - Use for comprehensive solutions\n\n\
                 MCTS PARAMETERS:\n\
                 - num_simulations: 1-150 (default: 30)\n\
                   - 10-30: Quick decisions, faster response\n\
                   - 30-80: Balanced evaluation (recommended)\n\
                   - 80-150: Thorough analysis, slower but more rigorous\n\n\
                 USE CASES:\n\
                 - Complex architectural decisions\n\
                 - Many possible solution paths\n\
                 - Need rigorous evaluation of alternatives\n\
                 - Uncertain problem space\n\
                 - Want to discover non-obvious solutions\n\n\
                 ADVANTAGES:\n\
                 - Systematically explores decision tree\n\
                 - Balances breadth and depth automatically\n\
                 - Finds high-quality solutions reliably\n\
                 - Adapts exploration based on results\n\
                 - Handles large search spaces efficiently",
            ),
        },
    ]
}

/// Branching and revision
fn prompt_branching() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "How do I use branching and revision to explore alternative approaches?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "BRANCHING AND REVISION:\n\n\
                 Branching lets you explore alternative reasoning paths from any point in your thought chain.\n\
                 This creates a tree structure where you can compare different approaches.\n\n\
                 1. Main reasoning path:\n\
                    reasoner({\n\
                        \"thought\": \"Implementing caching strategy. Starting with Redis.\",\n\
                        \"thought_number\": 1,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": true\n\
                    })\n\
                    // Returns thought_id: \"t1\"\n\n\
                 2. Continue main path:\n\
                    reasoner({\n\
                        \"thought\": \"Redis setup: need client library, connection pool, serialization strategy.\",\n\
                        \"thought_number\": 2,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": true\n\
                    })\n\
                    // Returns thought_id: \"t2\"\n\n\
                 3. Create branch from step 1 to explore alternative:\n\
                    reasoner({\n\
                        \"thought\": \"Alternative: Consider in-memory cache first for simplicity.\",\n\
                        \"thought_number\": 2,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": true,\n\
                        \"parent_id\": \"t1\"  // Branch from thought 1\n\
                    })\n\
                    // Creates new branch, returns thought_id: \"t3\"\n\n\
                 4. Develop the alternative branch:\n\
                    reasoner({\n\
                        \"thought\": \"In-memory cache: Use HashMap with LRU eviction. Simple, no external deps.\",\n\
                        \"thought_number\": 3,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": true,\n\
                        \"parent_id\": \"t3\"  // Continue alternative branch\n\
                    })\n\n\
                 5. Revise/correct earlier thought:\n\
                    reasoner({\n\
                        \"thought\": \"Correction to t2: Redis requires connection pooling for performance, not just 'nice to have'.\",\n\
                        \"thought_number\": 4,\n\
                        \"total_thoughts\": 5,\n\
                        \"next_thought_needed\": true,\n\
                        \"parent_id\": \"t2\"  // Revise the Redis path\n\
                    })\n\n\
                 HOW BRANCHING WORKS:\n\
                 - Use parent_id to specify which thought to branch from\n\
                 - Creates tree structure of reasoning paths\n\
                 - Each branch is scored independently\n\
                 - Can branch from any previous thought\n\
                 - Can have multiple branches from same parent\n\n\
                 BRANCHING USES:\n\
                 - Explore alternative solutions\n\
                 - Compare different approaches side-by-side\n\
                 - Correct earlier assumptions without losing original path\n\
                 - Try multiple strategies in parallel\n\
                 - Backtrack when you hit dead ends\n\n\
                 BEST PATHS:\n\
                 - Reasoner tracks scores for all branches\n\
                 - Returns highest-scoring complete path\n\
                 - Supports backtracking to try different branches\n\
                 - Can retrieve best path at any time\n\n\
                 TREE STRUCTURE EXAMPLE:\n\
                      t1 (Redis)\n\
                     /  \\\n\
                    t2  t3 (In-memory)\n\
                   /     \\\n\
                  t4     t5\n\
                 Each path is evaluated independently",
            ),
        },
    ]
}

/// Strategy comparison
fn prompt_strategies() -> Vec<PromptMessage> {
    vec![
        PromptMessage {
            role: PromptMessageRole::User,
            content: PromptMessageContent::text(
                "What are the different reasoning strategies and when should I use each one?",
            ),
        },
        PromptMessage {
            role: PromptMessageRole::Assistant,
            content: PromptMessageContent::text(
                "STRATEGY COMPARISON:\n\n\
                 ═══════════════════════════════════════════════════════════════\n\
                 BEAM SEARCH\n\
                 ═══════════════════════════════════════════════════════════════\n\
                 Best for: Comparing multiple approaches\n\
                 How: Maintains top N paths by score\n\
                 Config: beam_width (1-10)\n\n\
                 Example:\n\
                 reasoner({\n\
                     \"thought\": \"...\",\n\
                     \"strategy_type\": \"beam_search\",\n\
                     \"beam_width\": 3\n\
                 })\n\n\
                 ═══════════════════════════════════════════════════════════════\n\
                 MCTS (Standard)\n\
                 ═══════════════════════════════════════════════════════════════\n\
                 Best for: Complex decisions with many branches\n\
                 How: Monte Carlo simulations with UCB1 selection\n\
                 Config: num_simulations (1-150)\n\n\
                 Example:\n\
                 reasoner({\n\
                     \"thought\": \"...\",\n\
                     \"strategy_type\": \"mcts\",\n\
                     \"num_simulations\": 50\n\
                 })",
            ),
        },
    ]
}
