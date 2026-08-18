---
description: "Master Workflow: One command to rule them all. Auto-routes to the right phase based on task type."
---

// turbo-all

# OmniBMAD — Unified Master Workflow

> `/bmad` is the only workflow you need. It auto-detects what to do.
>
> This workflow knows about **all** sub-workflows, **all** omni CLI commands, and **all** agent roles.
> The model reads this file once, then routes autonomously — zero prior knowledge required from the user.

---

## Step 0: DETECT & ROUTE

Read the user's prompt and match against these signal patterns. Route to the **first match**.

### Core Development Pipeline

| Signal | Route To |
|--------|----------|
| "build X", "design", "integrate", "what's the best way", "architect" | → **Phase 0** → 0.5 → 0.7 → 1 → 2 |
| "explore", "architecture", "understand codebase", "knowledge graph" | → **Phase 0** (Code Intelligence auto-activates) |
| "build X" (clear scope, has spec/plan already) | → **Phase 0.7** → 1 → 2 |
| "fix bug", "add field", "rename", "refactor" (small change) | → **Phase 1** directly |
| "review", "check code", "PR" | → **Phase 3** |
| "deploy", "ship", "release" | → **Phase 4** |

### Specialized Pipelines

| Signal | Route To |
|--------|----------|
| "pentest", "red team", "hack", "attack surface", "OSINT", "exploit", "vulnerability scan" | → **🔴 RED TEAM** |
| "security audit", "blue team", "guardrails", "compliance", "adversarial test", "detection rules" | → **🔵 BLUE TEAM** |
| "security scan" (quick, one-shot, no full pipeline) | → `omni security scan --project <name>` directly |
| "create skill", "generate skill", "skill from URL", "skill from repo", "new SKILL.md" | → **🛠️ SKILL CREATE** |
| "explore", "architecture", "knowledge graph", "impact analysis", "codebase map", "visualize code" | → **🔍 EXPLORE** |

### Dual-Engine Optimization (within any pipeline)

| Signal | Route To |
|--------|----------|
| Task has ≥3 sub-tasks AND speed matters | → Activate **⚡ DUAL-ENGINE** mode alongside current phase |

If unclear → **Phase 0.7** → Phase 1.

---

## Omni CLI Quick Reference

> Use this to select the right command. For full details, read the `omni-server` skill.

```
omni run "task"                  → File modifications via swarm (skills → memory → agents → persist)
omni run "task" --dry-run        → Analysis only, no file edits
omni run --dag file.json         → Execute custom DAG (multi-agent pipeline)
omni run "task" --sandbox        → Git worktree isolation: merge on success, discard on fail
omni run --capabilities "_"     → Query runtime config (models, context windows, tools per role)
omni context "task" --project X  → Get enriched context without Ollama (BM25 + SurrealDB)
omni index                       → Reindex code after changes (mtime-based, ~200ms)
omni index --import-fallback     → Import offline data when DB recovers
omni impact <file>               → Show downstream impact before editing a file
omni impact                      → Auto-detect changed files from git diff
omni graph --project X --serve   → Interactive D3.js codebase viewer in browser
omni graph --project X           → Export knowledge graph as JSON
omni security scan --project X   → Quick pattern-based security baseline (21 patterns, CWE mapping)
omni skill validate <path>       → Validate skill quality (grade A-F, target ≥24/30)
omni search-skills "query"       → Search 1900+ skills via BM25
omni doctor                      → Diagnose system health
```

---

## Phase 0: RESEARCH

> **When:** Building something new, integrating unknowns, or user requests research.
> **Skip:** Scope already clear, simple fix/refactor, or user says "skip research".

1. **Web Research**: `search_web`, `read_url_content` — frameworks, best practices, pitfalls.
2. **Codebase Analysis**: `omni run --dry-run` or `grep_search`, `list_dir`, `view_file`.
3. **Code Intelligence** (auto-activate on "explore", "architecture", "understand", "refactor large"):
   - `omni index --project $(basename $PWD)` — ensure code graph is fresh
   - `omni graph --project $(basename $PWD) --output .omni/knowledge-graph.json` — export graph
   - `omni impact <file>` — BFS ripple analysis for refactoring targets
   - Review `.omni/architecture-summary.md` if exists
4. **Knowledge Check**: Review Knowledge Items (KIs) for relevant past work.
5. **Omni Context**: `omni context "task" --project X` for skills + memory + code graph without Ollama.
6. **Output**: Save findings to `research_notes.md` artifact.

→ **Phase 0.5**

---

## Phase 0.5: BRAINSTORM

> **When:** After research, or "brainstorm", "design", "let's think about..."
> **Skip:** User has clear spec or says "just implement this".

- Ask **1 question per message**, prefer **multiple choice**, max **5 questions**
- YAGNI ruthlessly — cut features user hasn't asked for
- Propose 2-3 approaches with trade-offs, lead with recommendation
- Present design section by section: Architecture → Data flow → Error handling → Testing

→ **Phase 0.7**

---

## Phase 0.7: PLAN

> **When:** Before any BUILD phase. **Skip:** Task < 5 lines changed.

Write `implementation_plan.md` — each task MUST have:
- Exact file paths (create/modify/delete)
- Exact code or pseudo-code
- Exact test commands with expected output
- 2-5 minute completion time per step

**Self-Critique Gate** (before sending to user):
1. Can each task be completed in 2-5 min? If not → split
2. Does code reference functions that don't exist? Verify with `grep_search`
3. Does plan exceed brainstorming scope? If yes → trim
4. Max 2 self-critique iterations. Then send for user approval.

**Impact Check** (for modifications to existing code):
```bash
omni impact <file-to-modify>   # See downstream effects before planning changes
```

→ **Phase 1**

---

## Phase 1: BUILD — Swarm Does the Work

> You are Brain. Swarm agents are Hands. Delegate file modifications via `omni run`.

**File edits:**
```bash
omni run "<task description>"          # Full pipeline: skills → memory → swarm → persist
omni run --dag .agent/memory/dag.json  # Custom DAG
```

**Analysis only (no edits):**
```bash
omni run "<task>" --dry-run
```

**Non-code tasks** (browser, git, deploy): Use native tools directly.

> ⚠️ Do NOT `--dry-run` then redo with native tools. Trust the swarm first.
> 💡 Use `--sandbox` for risky changes — auto-reverts on failure.

### Dual-Engine Decision Gate

**Before executing**, evaluate the task split:

| Condition | Action |
|-----------|--------|
| Task has ≥3 sub-tasks AND you want max speed | → Activate **⚡ DUAL-ENGINE** mode (see below) |
| Task is simple or Ollama handles everything | → Standard Phase 1 execution |

### GitButler Integration (automatic when `but` CLI installed)

Lifecycle hooks handle git operations — **no manual git commands needed**:

| Hook | Action | When |
|------|--------|------|
| `pre_run` | `but branch new agent/run-<ts>` | Creates virtual branch for this run |
| `post_task` | `but commit --ai` | Auto-commits after each agent task |
| `post_run` | `but push` | Pushes all branches to remote |
| `on_error` | `but undo` | Rolls back last operation on failure |

PR creation: `bash .agent/skills/gitbutler-omni-workflow/scripts/but_pr_create.sh`

**After swarm:** ✅ Succeeded → Phase 2 | ❌ Failed → `but undo` auto-rollback → fallback to native tools → Phase 2

**Transition check:** All files accounted for, no partial implementations, document failures.

### Pty Safety
- **ALWAYS** truncate: `cargo test 2>&1 | tail -30`
- **NEVER** dump code to terminal — use `write_file`
- **If Pty crashes:** terminate dead terminal → open new → re-run with truncation

---

## ⚡ Phase 1.5: DUAL-ENGINE PARALLEL

> **When:** Task has ≥3 sub-tasks and you want maximum throughput by working alongside Ollama.
> **Skip:** Task is simple, or Ollama is powerful enough to handle everything alone.

### How It Works

1. **Gather Context** (no Ollama needed):
   ```bash
   omni context "task description" --project my-app
   ```
   IDE receives skills + memory + code graph + reasoning bank context directly from BM25 + SurrealDB.

2. **Split Tasks** — IDE is Brain, assigns work:

   | Task Type | Engine | Why |
   |-----------|--------|-----|
   | Write/edit code files | Ollama (`omni run --dag`) | Has `write_file`, `replace_in_file` tools |
   | Write tests | Ollama | Can run in sandbox |
   | Architecture analysis | IDE | Stronger model, better reasoning |
   | Code review | IDE | Final authority, spec compliance |
   | Browser testing | IDE | Only IDE has browser tools |
   | Git operations | IDE | Only IDE has git access |
   | Security audit | Either | IDE for complex, Ollama for pattern scan |

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

> Full reference: `reference/dual-engine.md`

---

## Phase 2: VERIFY + RALPH Loop

1. `cargo check` (or `npm run build`, `go build`)
2. `cargo test` (or `npm test`, `pytest`)
3. `cargo clippy -- -D warnings` (or `eslint`)
4. `omni index` — reindex changed code. If DB down → fallback auto-saved, import later with `omni index --import-fallback`.

**All pass → Done ✨** | **Any fail → RALPH Loop** (max 3 cycles):

| Cycle | Strategy | Action |
|-------|----------|--------|
| 1 | Direct fix | Address exact error |
| 2 | Alternative approach | Different implementation |
| 3 | Scope reduction | Simplify or split task |
| 4+ | **HALT** | Escalate to user with all 3 attempts |

**RALPH = Retry → Adapt → Learn → Persist → Halt**
- Log failure pattern + fix attempt + result
- `omni run` auto-persists trajectories to ReasoningBank

### Output Quality Gate
- [ ] No `TODO`/`FIXME`/`TBD` in modified files
- [ ] No hardcoded secrets/credentials
- [ ] New public APIs have doc-comments
- [ ] Error messages are user-friendly

---

## Phase 2.5: HANDOFF

> If task spans multiple sessions, create `HANDOFF.md` at project root: What was done, What's next, Key decisions, Known issues.

---

## Phase 3: REVIEW (Two-Stage)

> After Phase 2 passes, or via "review" requests.

1. Create `.agent/memory/dag-review.json` with: analyzer (spec compliance) → reviewer (code quality)
2. Run: `omni run --dag .agent/memory/dag-review.json`
3. If `[REJECT]` → fix → re-run

---

## Phase 4: DEPLOY

> Only on explicit "deploy"/"ship"/"release".

1. `cargo build --release` (or `npm run build`)
2. Deploy: Docker / Cloud Run / Vercel
3. Smoke test: `curl http://localhost:3000/health`

---

# Specialized Pipelines

> These are self-contained workflows activated by the router in Step 0.
> Each section contains enough context to execute. For full phase-by-phase instructions,
> read the referenced workflow file.

---

## 🔴 RED TEAM — Offensive Security Pipeline

> **Activated by:** "pentest", "red team", "hack", "attack surface", "OSINT", "exploit", "vulnerability scan"
> **Full instructions:** `red-team.md`

### Prerequisites
- `omni` CLI + Ollama + SurrealDB running
- Create `scope.md` with: target, Rules of Engagement, allowed techniques, time window
- ⚠️ **Legal**: Only run against targets with explicit written authorization

### Pipeline

```
Phase 1 RECON → Phase 2 VULN SCAN → Phase 3 EXPLOIT → Phase 3.5 POST-EXPLOIT → Phase 4 REPORT
```

| Phase | What | Output |
|-------|------|--------|
| 1. OSINT Recon | web_search + web_fetch + skill_search (3 parallel analyzers) | `.agent/security/red/attack-surface.md` |
| 2. Vuln Analysis | CVE research, web/infra/cloud analysis, MITRE mapping | `.agent/security/red/vuln-report.md` |
| 3. Exploit Planning | Attack chains, PoC scripts, playbooks (NON-DESTRUCTIVE) | `.agent/security/red/exploit-plan.md` + `exploits/` + `playbooks/` |
| 3.5. Post-Exploit | Lateral movement, persistence, privesc, exfil simulation (THEORETICAL) | `.agent/security/red/post-exploit-report.md` |
| 4. Final Report | Executive summary, findings table, remediation priorities | `.agent/security/red/pentest-report.md` + `executive-brief.md` |

### Execution
Each phase is a single `omni run "..."` command. Read `red-team.md` for the exact prompts.

### After completion
- Run `omni dream` to consolidate STM
- Blue Team reads `pentest-report.md` in their Phase 2 (cross-feed)

---

## 🔵 BLUE TEAM — Defensive Security Pipeline

> **Activated by:** "security audit", "blue team", "guardrails", "compliance", "adversarial", "detection rules"
> **Full instructions:** `blue-team.md`

### Prerequisites
- `omni` CLI + Ollama + SurrealDB running
- Create `.agent/security/blue/security-baseline.md` with: system description, architecture, existing controls, compliance frameworks
- Quick baseline first: `omni security scan --project <name>`

### Pipeline

```
Phase 1 GUARDRAILS → Phase 2 ADVERSARIAL → Phase 3 DETECTION → Phase 4 COMPLIANCE → Phase 5 SYNC → Phase 6 HARDEN
```

| Phase | What | Output |
|-------|------|--------|
| 1. Guardrail Validation | Input/output/tool-level/HITL guards (20+ test cases per category) | `.agent/security/blue/guardrail-rules.md` |
| 2. Adversarial Simulation | 12 OWASP categories × 60+ test cases × CVSS scoring. Cross-feeds from Red Team if available | `.agent/security/blue/adversarial-report.md` |
| 3. Detection Engineering | Gap analysis, 30+ detection rules (YAML), monitoring config | `.agent/security/blue/detection-rules.md` + `monitoring-config.md` |
| 4. Compliance & Remediation | OWASP LLM / MITRE ATLAS / NIST AI RMF / ISO 42001 / EU AI Act / GDPR mapping | `.agent/security/blue/compliance-report.md` + `remediation-plan.md` |
| 5. Documentation Sync | Update risk-register, architecture, ai-policy, AGENTS.md | `.agent/security/compliance/` |
| 6. Continuous Hardening | Weekly regression (top 20 tests), monthly compliance trend, guardrail updates | `.agent/security/history/` |

### Execution
Each phase is a single `omni run "..."` command. Read `blue-team.md` for the exact prompts.

### After completion
- Run `omni dream` to consolidate STM
- Check HITL pending count, remind user if > 0

---

## 🛠️ SKILL CREATE — Generate AI Skills

> **Activated by:** "create skill", "generate skill", "skill from URL/repo", "new SKILL.md"
> **Full instructions:** `skill-create.md`

### Usage
Provide sources in the request: URLs (GitHub repos, docs, articles), local files, or natural language descriptions.

### Pipeline

```
[1. EXTRACT] → [2. DETECT] → [3. GENERATE] → [4. VALIDATE] → [5. TEST] → [6. ITERATE] → [7. FINALIZE]
```

| Phase | What |
|-------|------|
| 1. Extract | Read all sources, identify core capability, extract structured components |
| 2. Detect | Score complexity (scope + dependencies + error surface + domain knowledge). Sum ≤8 = simple, 9-15 = medium, 16-20 = complex |
| 3. Generate | Write `~/.config/_skills_/<skill-name>/SKILL.md` using template (frontmatter mandatory, anti-patterns mandatory, real code examples) |
| 4. Validate | `omni skill validate <path>` — target score ≥ 24/30 (Grade A). If < 24 → fix and loop |
| 5. Test | Dry run: can an agent follow without additional context? Are examples copy-paste ready? |
| 6. Iterate | Compare with existing similar skills. New skill must score higher and add unique value |
| 7. Finalize | Confirm Grade A, report to user |

### Quality Gate
**Never finalize a skill with Grade B or lower.** Loop Phase 3-4 until Grade A (≥ 24/30).

---

## 🔍 EXPLORE — Codebase Architecture Exploration

> **Activated by:** "explore", "architecture", "knowledge graph", "impact analysis", "codebase map", "visualize code"
> **Full instructions:** `explore.md`

### Pipeline

```bash
# Step 1: Ensure code is indexed
omni index --project $(basename $PWD)

# Step 2: Export knowledge graph
omni graph --project $(basename $PWD) --output .omni/knowledge-graph.json

# Step 3: Architecture analysis (analyzer agent summarizes the graph)
omni run "Read .omni/knowledge-graph.json and produce an architecture summary: layers, entry points, dependencies, circular dependency warnings. Write to .omni/architecture-summary.md" --dry-run

# Step 4: Impact analysis (optional, for specific files)
omni impact src/commands/run.rs

# Step 5: Interactive visualization (optional)
omni graph --project $(basename $PWD) --serve
```

### Use cases
- Plan refactoring safely
- Onboard new team members
- Identify high-risk files (many callers) and dead code (no callers)
- Security graph: `omni security scan` + `omni security graph`

### SurrealDB Required
Steps 2-5 require SurrealDB. If DB is down, `omni index` saves fallback data automatically.
When DB recovers: `omni index --import-fallback` → `omni index` → proceed normally.

---

# Flow Summary

### Core Development Pipeline

```
User Input → [DETECT & ROUTE]
  ├─ build/design → Phase 0 → 0.5 → 0.7 → 1 (± 1.5 Dual-Engine) → 2 → Done
  ├─ explore/arch → Phase 0 (Code Intelligence) → Done or → 0.7 → 1
  ├─ fix/refactor → Phase 1 (± 1.5) → 2 → Done
  ├─ review       → Phase 3
  └─ deploy       → Phase 4
Phase 2 fail → RALPH (max 3) → HALT if unresolved
```

### Specialized Pipelines

```
User Input → [DETECT & ROUTE]
  ├─ pentest/red team    → 🔴 RECON → VULN → EXPLOIT → POST-EXPLOIT → REPORT
  ├─ security/blue team  → 🔵 GUARDRAILS → ADVERSARIAL → DETECTION → COMPLIANCE → SYNC → HARDEN
  ├─ create skill        → 🛠️ EXTRACT → DETECT → GENERATE → VALIDATE → TEST → ITERATE → FINALIZE
  ├─ explore/architecture→ 🔍 INDEX → GRAPH → ANALYZE → IMPACT → VISUALIZE
  └─ security scan (quick)→ omni security scan --project <name>
```

> `omni run` auto-handles memory persistence and re-indexing.
> After any specialized pipeline: run `omni dream` to consolidate STM, then `omni index` to sync code graph.
