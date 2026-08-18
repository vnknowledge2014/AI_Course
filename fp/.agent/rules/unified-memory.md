---
description: Unified memory architecture bridging IDE and Omni memory systems. Covers STM/LTM tiers, PARA classification, consolidation lifecycle, and workflow integration.
trigger: always_on
---
# Unified Memory Architecture

## Two Memory Systems — IDE + Omni

- **IDE memory:** KIs, conversation logs, artifacts, implicit preferences (`.pb`)
- **Omni memory:** STM (48h), LTM (PARA-classified), ReasoningBank, CodeGraph, HITL Queue (SurrealDB)
- **Bridge:** `omni context` (read Omni→IDE), `omni run` (write IDE→Omni), `omni index` (code sync), filesystem (shared)
- **Gap:** IDE artifacts/KIs do NOT auto-sync to Omni. Use `omni run` with documentation tasks to persist critical insights.
- **DB naming:** Project DB names are always **lowercased** (e.g., `OmniUltraAgent_Kit` → `omniultraagent_kit`). All Memory V2 schemas auto-apply during `omni start` or `omni run` startup. No manual `omni run` is needed before using memory commands.

## Memory Tiers & Priority

| Tier | Table | TTL | Priority |
|------|-------|-----|----------|
| LTM | `ltm_events` | Until invalidated | 1st (validated knowledge) |
| STM | `stm_traces` | 48h | 2nd (recent traces) |
| Global | `ltm_events` (cross-project) | Until invalidated | 3rd |
| Legacy | `memory` | Permanent | 4th (backward compat, read-only) |

## PARA Classification (for LTM promotion)

- **projects** = active time-bound work (e.g., "implementing JWT auth")
- **areas** = ongoing responsibilities (e.g., "code review standards")
- **resources** = reference material, patterns (DEFAULT — most knowledge)
- **archive** = completed/superseded work

## Lifecycle

```
omni run → auto STM flush → omni dream → consolidation:
  ≥0.75 confidence → LTM auto-promote (PARA-classified)
  0.50-0.75         → LTM flagged for review
  <0.50             → HITL queue
  contradicts existing → HITL queue
```

## Workflow Integration

- **Before `omni run`:** use `omni context` to check what Omni remembers
- **After direct file edits:** run `omni index` to sync code graph
- **After large red/blue-team runs:** run `omni dream` to consolidate STM
- **After `omni run`:** check HITL pending count in output, remind user if > 0
- **/bmad:** Phases 0-0.7 = IDE-only memory. Phase 1+ = `omni run` = auto memory pipeline. After verify: `omni index`.
- **/red-team:** Each phase = `omni run`. Cross-phase context via files + memory recall. After pipeline: `omni dream`.
- **/blue-team:** Same as red-team. Phase 2 cross-reads red-team via files AND memory. Phase 6: periodic `omni dream`.

## Thresholds

| Constant | Value |
|----------|-------|
| STM TTL | 48 hours |
| LTM auto-promote | confidence ≥ 0.75 |
| HITL queue | confidence < 0.50 |
| LTM token budget | 3000 tokens |
| STM token budget | 2000 tokens |

## Anti-Patterns

- ❌ Don't rely solely on IDE KIs — check `omni context` for Omni-side knowledge
- ❌ Don't skip `omni index` after direct file edits — code graph goes stale
- ❌ Don't approve-all HITL without review — contradictions slip through
- ❌ Don't modify legacy `memory` table — read-only backward compat
- ❌ Don't skip `omni dream` after large red/blue-team runs — STM accumulates unbounded
