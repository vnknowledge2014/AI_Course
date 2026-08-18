# AGENTS.md — OmniUltraAgent Kit for Antigravity IDE

> Auto-detected by Antigravity IDE v1.20.3+
>
> **Scope:** IDE orchestration — architecture, protections, workflows, IDE-specific features (compaction, cost routing, sandbox).
> For detailed DAG schema, sub-agent tool reference, runtime injection behavior, and inter-task isolation rules, see the **omni-server** skill (`~/.gemini/config/skills/omni-server/SKILL.md`).

## Core Directives

1.  **Read Rules First**: Before starting any task, ALWAYS read `.agent/rules/*.md`.
2.  **Use `/bmad` Workflow**: The unified master workflow auto-routes to the right phase.
3.  **Plan → Act → Verify**: Write `implementation_plan.md` for major work. Run tests + linter to verify.
4.  **Context Budget**: Before designing DAGs, run `omni run --capabilities "_"` to learn sub-agent model limits. Use `budget_tokens` per task to prevent context overflow.

## Three-Layer Architecture

```
Layer 1: Antigravity IDE (Brain)
  → Model selection, Knowledge Items, Rules, Workflows, Browser, Sandbox
  → Handles: planning, research, DAG design, result interpretation

Layer 2: omni run CLI (Hands)
  → Enforces: skill search, memory recall, swarm execution, memory persistence
  → Cannot be skipped — runs as compiled Rust code

Layer 3: LLM Sub-Agents (Workers)
  → Powered by Ollama (local models + cloud routing via :cloud tag)
  → Ephemeral: born per-task, destroyed after
  → Have tools: read_file, write_file, replace_in_file, patch, grep_search, list_directory, file_outline, bash, diagnostics, skill_search, skill_content, skill_resource, tool_search, tool_content, web_search, web_fetch, http_request, dns_resolve, port_check, tls_inspect, sandbox_exec, web_crawl, pattern_scan, input_guard_test, prompt_test
  → Multi-turn execution with auto tool-calling
```

## Agent Roles (10 roles)

| Role | Model Config | Purpose |
|------|-------------|--------|
| `coder` | `coder_model` | Code generation, write tools |
| `analyzer` | `analyzer_model` | Analysis, architecture, code intelligence |
| `reviewer` | `analyzer_model` | Code review (🔴🟡💭✅ structured output) |
| `tester` | `coder_model` | Test writing, TDD |
| `debugger` | `coder_model` | Root cause analysis, bug diagnosis |
| `security_auditor` | `analyzer_model` | Vulnerability scanning, compliance |
| `docs_writer` | `analyzer_model` | Documentation, README, API refs |
| `refactorer` | `coder_model` | Code cleanup, pattern migration |
| `planner` | `analyzer_model` | Architecture design, planning |
| `deployer` | `coder_model` | CI/CD, infrastructure, deployment |

> **Full role reference** (tools per role, aliases, personas): see `~/.gemini/config/skills/omni-server/SKILL.md` → "Role Master Reference".
>
> Models are configured in `omni.config.yaml` via `agents.coder_model` and `agents.analyzer_model`. All sub-agents run through Ollama.

## How to Use (via `omni run`)

### Decision Tree — Pick ONE mode per task:

| Task Type | Command | When |
|-----------|---------|------|
| **File modifications** (edit code, fix bugs, replace text, refactor) | `omni run "<task>"` | Sub-agents have `write_file`, `replace_in_file`, `grep_search` tools |
| **Analysis only** (understand structure, find info, plan) | `omni run "<task>" --dry-run` | Just need skill search + memory, no file changes |
| **Non-code tasks** (screenshots, browser, git, deploy) | Native tools directly | `omni run` agents can't do browser/git/deploy |

### Rules:
- **DO NOT** run `--dry-run` then redo the same work with native tools. That defeats the purpose.
- **DO** use `omni run` (without `--dry-run`) when the task involves editing files — even simple edits. The sub-agents have file tools.
- **DO** fall back to native tools only if `omni run` fails (swarm error).
- `omni run` agents are LLM-powered via Ollama and have these tools: `read_file`, `write_file`, `replace_in_file`, `patch`, `grep_search`, `list_directory`, `file_outline`, `bash`, `diagnostics`, `skill_search`, `skill_content`, `web_search`, `web_fetch`, code intelligence tools: `query_callers`, `query_callees`, `query_hierarchy`, plus security tools: `http_request`, `dns_resolve`, `port_check`, `tls_inspect`, `sandbox_exec`, `web_crawl`, `pattern_scan`, `input_guard_test`, `prompt_test`.

```bash
# Quick task — default DAG (analyzer → coder → reviewer)
omni run "add user authentication with JWT"

# Analysis only (no file edits) — use --dry-run
omni run "analyze which components use deprecated API" --dry-run

# Query model capabilities (IDE integration)
omni run --capabilities "_"

# Custom DAG designed by Antigravity
omni run --dag plan.json

# Validate skill quality (6-dimension scoring + security scan)
omni skill validate path/to/SKILL.md
omni skill validate --all
```

## IDE Direct Context Access (No Ollama Required)

When Ollama is unavailable or IDE wants to work in parallel with swarm, use `omni context` to get enriched context directly from BM25 + SurrealDB:

```bash
omni context "implement JWT auth" --project my-app       # Plain text output
omni context "debug memory leak" --project my-app --json # JSON for programmatic use
omni context "refactor auth module" --limit 3             # Limit results per section
```

Returns 4 sections — **none require Ollama**:
1. **Skills** — BM25 in-memory index search (top matches from 1900+ skills)
2. **Memory** — SurrealDB fulltext search on project memories
3. **Reasoning Bank** — Past successful task trajectories
4. **Code Graph** — Matching symbols from `code_element` table

Use this in **Phase 1.5 (Dual-Engine Parallel)** of the `/bmad` workflow to enrich IDE prompts while Ollama handles coding tasks.

## Built-in Protections & Enhancements

| Feature | What it does | Automatic? |
|---------|-------------|------------|
| **Input Guard** | Blocks prompt injection patterns (41 detectors across 9 categories: instruction override, role switching, jailbreak, context manipulation, encoding bypass, CJK injection, social engineering, template injection, padding detection) before task processing | ✅ Always on |
| **Anti-Pattern System** | Extracts `## Anti-Patterns` from skills, injects as negative guidance to agents | ✅ If skill has section |
| **Security Scanner** | 5-category skill security scan (command injection, secrets, file ops, network exfil, priv escalation). Blocks deployment on critical findings | ✅ On `omni skill validate` |
| **RAG Enricher + Graph RAG** | Queries `code_element` by vector similarity, injects top-3 code snippets enriched with 1-hop graph context (callers, callees, extends) as agent context. Cosine dedup against skill context | ✅ If code indexed + SurrealDB available |
| **Reference Skills** | Auto-loads best-matching `reference/` subdirectory file from skills by keyword overlap with task | ✅ If skill has `reference/` dir |
| **Budget Gate** | Prevents context overflow by gating context injection: Skills (always) > Memory (< High) > RAG (< Medium) | ✅ Always on |
| **Stream-Chain** | Passes upstream task outputs to downstream tasks via `system_prompt` injection (4096 char cap) | ✅ For multi-wave DAGs |
| **ReasoningBank** | Persists task trajectories (steps, outcome, model, confidence) and recalls past successes during memory phase | ✅ Requires SurrealDB |
| **Incremental Reindex** | After swarm execution, re-indexes changed files (new, modified, deleted) with mtime-based detection, AST-first relation extraction, and batch embedding. Graph + embeddings always stay in sync. If SurrealDB is down, saves fallback data to `.agent/fallback_data/` for later import via `omni index --import-fallback`. IDE agents: run `omni index` per rule in `development-process.md` | ✅ On `omni run`; rule-driven for IDE |

## Code Intelligence & Knowledge Graph

Omni indexes code into SurrealDB (`code_element`, `calls`, `imports`, `extends` tables) and provides tools to explore and analyze the graph:

```bash
# Export knowledge graph as JSON (React Flow compatible) or DOT (GraphViz)
omni graph --project my-project --output .omni/knowledge-graph.json
omni graph --project my-project --format dot --output graph.dot

# Interactive graph viewer in browser (D3.js force-directed)
omni graph --project my-project --serve
omni graph --project my-project --serve --port 9000  # Custom port

# Analyze impact of file changes (BFS on code graph)
omni impact src/commands/run.rs          # Explicit file
omni impact                               # Auto-detect from git diff
omni impact --depth 5 --json             # Deeper traversal + JSON output
```

Sub-agents can also use code intelligence tools:
- `query_callers <name>` — find upstream consumers of a symbol
- `query_callees <name>` — find downstream dependencies
- `query_hierarchy <name>` — find type hierarchy (extends/implements)
- `file_outline <path>` — get file structure without full content (saves context)

### DAG Patterns

| Project Type | Typical DAG |
|-------------|-------------|
| **New feature** | `planner(design)` → `coder(implement)` → `tester(tests)` → `reviewer(review)` |
| **Bug fix** | `debugger(diagnose+fix)` → `tester(regression tests)` → `reviewer(review)` |
| **Refactor** | `planner(plan)` → `refactorer(A)` ∥ `refactorer(B)` → `reviewer(review)` |
| **Security audit** | `security_auditor(scan)` → `coder(fix)` → `security_auditor(verify)` |
| **Documentation** | `analyzer(analyze)` → `docs_writer(document)` → `reviewer(review)` |
| **Deployment** | `planner(plan)` → `deployer(implement)` → `tester(smoke tests)` |

Tasks with no dependency on each other **run in parallel** (same wave).

> **Full DAG schema, JSON format, and examples**: see `~/.gemini/config/skills/omni-server/SKILL.md` → "Swarm Execution — DAG Reference".
> Key fields: `id`, `title`, `role`, `depends_on`, `system_prompt`, `tools`, `model`, `budget_tokens`.

### DAG File Lifecycle

Antigravity creates custom DAG files directly in `.agent/memory/`:
1. Write DAG JSON → `.agent/memory/dag-<name>.json`
2. Run `omni run --dag .agent/memory/dag-<name>.json`
3. Omni auto-deletes the DAG from `.agent/memory/` after completion

> **Rule**: Never create DAG files in `/tmp/`. Always use `.agent/memory/`.

### Sizing Guidelines

| Scope | Agents | Example |
|-------|--------|---------|
| Small fix | 1-2 | `coder → tester` |
| Feature | 3-5 | `architect → coders → tester → reviewer` |
| Major refactor | 5-10 | `architect → many coders (parallel) → tester → reviewer` |
| **Don't overdo it** | >10 | Split into separate `omni run` calls |

## Model Routing

> **⚠️ MANDATORY:** Before designing any DAG, run `omni run --capabilities "_"` to get the **actual** models, context windows, and tools from the live config. Never assume model names from this document.

The swarm engine auto-selects models per role from `omni.config.yaml`:

| Role | Config Key |
|------|-----------|
| `coder` | `agents.coder_model` |
| `analyzer` | `agents.analyzer_model` |
| `debugger` | `agents.coder_model` |
| `security_auditor` | `agents.analyzer_model` |
| `docs_writer` | `agents.analyzer_model` |
| `refactorer` | `agents.coder_model` |
| `planner` | `agents.analyzer_model` |
| `deployer` | `agents.coder_model` |

Default model names are defined in `omni.config.default.yaml` (provisioned by `omni setup`). To change models, edit `omni.config.yaml` — all docs, agents, and tooling automatically pick up the new values.

## Workflow

Single unified workflow: **`/bmad`** — auto-detects task type and routes to the right phase. See `.agent/workflows/bmad.md`.

**Full pipeline (for new features/integrations):**
```
Phase 0: RESEARCH → Phase 0.5: BRAINSTORM → Phase 0.7: PLAN (+ Self-Critique)
    → Phase 1: BUILD (omni run) → Phase 2: VERIFY → Phase 3: REVIEW (Two-Stage)
```

**Phase 3 Two-Stage Review** uses a Custom DAG (`dag-review.json`):
1. `analyzer` checks spec compliance against `implementation_plan.md`
2. `reviewer` checks code quality, patterns, security (depends on step 1)


## Skills & Tools

Skills are synced globally to `~/.config/_skills_/` (1900+ skills). Sub-agents can search skills autonomously via their `skill_search` tool. **Never load all skills at once — agents search on demand.**

Bundled CLI tools are synced to `~/.config/_tools_/` (e.g. `OmniTransCLI`). Sub-agents discover and use them via `tool_search` → `tool_content` → `bash`. IDE can browse with `omni tools list/search/read`.

## Security Data Zone

Blue Team and Red Team workflows write output to `.agent/security/` (not `.agent/memory/`):

```
.agent/security/
├── blue/                    ← Blue Team pipeline output (defensive)
│   ├── guardrail-rules.md
│   ├── adversarial-report.md
│   ├── detection-rules.md
│   ├── monitoring-config.md
│   ├── compliance-report.md
│   └── remediation-plan.md
├── red/                     ← Red Team pipeline output (offensive)
│   ├── attack-surface.md
│   ├── pentest-report.md
│   └── exploits/
├── compliance/              ← Shared compliance docs
│   ├── ai-policy.md
│   ├── risk-register.md
│   └── architecture.md
└── history/                 ← Regression snapshots
    └── regression-YYYYMMDD.md
```

> `.agent/memory/` is reserved for **DAG temp files only** (auto-cleaned after `omni run`).

## Important Notes

- **No API Server Required**: `omni run` connects to SurrealDB and Ollama directly.
- **Graceful Degradation**: If SurrealDB or Ollama are unreachable, `omni run` degrades gracefully.
- **Security**: All file operations have path traversal protection + .bak backups.
- **Security Tools**: 9 native security tools for red/blue team workflows (http_request, dns_resolve, port_check, tls_inspect, sandbox_exec, web_crawl, pattern_scan, input_guard_test, prompt_test).

## Security Sandbox Config

The `sandbox_exec` tool runs commands inside a Docker container. **Disabled by default.**

```yaml
# omni.config.yaml
security:
  sandbox:
    enabled: true                    # Must be explicitly enabled
    image: "omni-sandbox:latest"     # Docker image name
    max_timeout_secs: 120            # Max command timeout
    max_memory_mb: 512               # Container memory limit
    allowed_tools:                   # Whitelist (default: 35+ tools)
      - nmap
      - nuclei
      - sqlmap
    dangerous_tools:                 # Require HITL approval
      - hydra
      - hashcat
```

5-layer safety: Docker isolation → Tool whitelist → Scope check → HITL gate → Resource limits.

## Platform Format

Antigravity uses whatever model the user selects from the IDE UI for high-level planning. Sub-agents run through Ollama for swarm execution via `omni run`.

## IDE-Driven Proactive Compaction

Before designing DAGs, the IDE MUST:

1. **Query capabilities**: `omni run --capabilities "_"` → JSON with model context windows, tools per role
2. **Estimate context**: Calculate total tokens needed per task (task description + files + skills + memory)
3. **Compare vs budget**: Each model has a `context_window` from capabilities output
4. **Compact if needed**: IDE (powerful model) summarizes large files, splits tasks, or uses `file_outline` instead of full content
5. **Set per-task budget**: Use `model` and `budget_tokens` in DAG JSON to enforce limits

## Cost-Aware Routing

When designing DAGs, the IDE can optimize cost by:
1. Reading `model_pricing` from `omni.config.yaml` directly (not in `--capabilities` output)
2. Assigning cheaper models (analyzer) to simple sub-tasks
3. Only using expensive models (coder) for complex code generation

If `model_pricing` is not configured, use default pricing from `core/defaults.rs`.

## Additional CLI Features

> **Full documentation** for all features below: see `~/.gemini/config/skills/omni-server/SKILL.md`.

| Feature | Quick Reference | Details in SKILL.md |
|---------|----------------|--------------------|
| **Lifecycle Hooks** | `hooks:` in `omni.config.yaml` — 7 events (`pre_run`, `post_task`, `on_error`...) with env var injection | → "Lifecycle Hooks" |
| **Git Sandbox** | `omni run "task" --sandbox` — auto-merge on success, discard on fail | → "Sandbox Mode" |
| **Agent Evaluation** | `omni eval run suite.json` — test suites for agent quality | → "Agent Evaluation" |
| **Security Scanning** | `omni security scan --project app` — 21 patterns, CWE mapping, attack graph | → "Security Scanning" |
| **CLI Adapters** | `omni adapter add lint "eslint"` — CLI-first, no MCP | → "CLI Adapter Registry" |
| **IDE Context** | `omni context "task" --project app` — skills + memory + code graph, no Ollama needed | → "IDE Context" |
| **Bundled Tools** | `omni tools list` / `omni tools search "..."` / `omni tools read <Name>` — browse `~/.config/_tools_/` | → omni-server SKILL.md "Bundled Tools" |
| **Sync Tools** | `omni sync-tools` — install/update bundled tools + auto pip install CLI | → omni-server SKILL.md "Bundled Tools" |
| **Tools Index** | `omni build-tools-index` — rebuild BM25 search index for `~/.config/_tools_/` | → omni-server SKILL.md "Bundled Tools" |
