---
description: "Phase 1.5: Dual-Engine Parallel — IDE + Ollama work simultaneously for maximum throughput."
---
# Phase 1.5: DUAL-ENGINE PARALLEL

> **When:** Task has ≥3 sub-tasks, and the IDE wants to optimize speed by doing part of the work in parallel.
> **Skip when:** Task is simple, or Ollama is powerful enough to handle everything alone.

## Workflow

1. **Gather Context** (no Ollama needed):
   ```bash
   omni context "task description" --project my-app
   ```
   IDE receives skills + memory + code graph + reasoning bank context directly from BM25 + SurrealDB.

2. **Split Tasks** — IDE is the Brain, assigns work:
   - **→ Ollama (Hands):** File editing tasks (coding, refactoring, test writing) → `omni run --dag`
   - **→ IDE (Brain):** Analysis, review, browser testing, git operations → native tools

3. **Parallel Execution**:
   ```
   ┌──────────────────────┐     ┌──────────────────────────┐
   │  Ollama Swarm (bg)   │     │  IDE Agent (foreground)  │
   │  omni run --dag ...  │     │  Analysis, review, git   │
   │  Coding tasks        │ ──▶ │  Browser testing         │
   │                      │     │  Architecture decisions  │
   └──────────────────────┘     └──────────────────────────┘
   ```

4. **Ollama Failure → Fallback**:
   - If `omni run` exit code ≠ 0 → IDE takes back the failed task
   - IDE uses `omni context` to get enriched context
   - IDE uses native tools (`view_file`, `replace_file_content`, `write_to_file`) to complete the task

5. **IDE Reviews ALL Output**:
   - Review output from both Ollama (read file changes) and itself
   - Merge results → Phase 2 (VERIFY)

## Decision Matrix: IDE vs Ollama

| Task Type | Engine | Why |
|-----------|--------|-----|
| Write/edit code files | Ollama | Has `write_file`, `replace_in_file` tools |
| Write tests | Ollama | Can run in sandbox |
| Architecture analysis | IDE | Stronger model, better reasoning |
| Code review | IDE | Final authority, spec compliance |
| Browser testing | IDE | Only IDE has browser tools |
| Git operations | IDE | Only IDE has git access |
| Security audit | Either | IDE for complex, Ollama for pattern scan |
