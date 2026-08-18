---
name: rusty-context
description: Skill for interacting with the Rusty-Context Agentic Memory Ecosystem.
---

# Rusty-Context Skill

This skill allows agents (like OmniUltraAgent) to manage their own context (STM & LTM) using the `rusty-context` CLI.

## How it works

The agent should invoke the CLI commands to manage its context.

### 1. Indexing (Incremental Sync)
When a new document is added or a source file is modified, the agent should run:
```bash
cargo run --bin rusty-context -- index --path <file_or_dir>
```
The system will automatically compute the AST or Markdown delta ($\Delta$) and update the Context Tree losslessly.

### 2. Searching
To retrieve context from the Long-Term Memory (The Palace):
```bash
cargo run --bin rusty-context -- search --query "What is the architecture of..."
```
This performs a reasoning-based tree retrieval over the hierarchical context structure (Wings -> Rooms -> Drawers).

### 3. Dreaming (Self-Evolving Loop)
At the end of a major task, or when the STM reaches capacity, the agent **MUST** run the dream sequence:
```bash
cargo run --bin rusty-context -- dream
```
This triggers a self-evaluation loop where the LLM reviews its recent traces. High-confidence insights are promoted to LTM (Drawers), while contradictions are flagged for HITL (Human-in-the-loop).
