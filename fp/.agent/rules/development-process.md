---
description: Spec-driven approaches and proper documentation management.
trigger: always_on
---
# Development Process

1.  **Spec-Driven**: Do NOT write code without a specification. `implementation_plan.md` MUST be written and reviewed first.
    _Rationale: Specs prevent AI from solving the wrong problem — planning errors are cheaper than code errors._
2.  **Documentation**: Keep `README.md` and codebase wikis updated alongside code changes. Document *why*, not *what*.
3.  **Project Conventions**: Every repository has idiomatic structures. Respect the existing setup (e.g., placing tests in the right directory, matching file naming conventions).
4.  **No Hallucinations**: Verify missing context via `grep` or file system reads before making assumptions about schemas or architectures.
    _Rationale: AI models confabulate plausible-sounding but incorrect code paths — always verify._
5.  **Reindex After Changes**: See `code-intelligence.md` rule #4 — run `omni index` after any file changes (create, modify, delete). If DB is down, fallback data is saved automatically; import with `omni index --import-fallback` when DB recovers.
    _Rationale: New files need graph entries; deleted files need stale records cleaned up._
6.  **Bite-Sized Plans**: Implementation plans MUST break tasks into 2-5 minute steps with exact file paths, exact code, and exact test commands. Each step should be completable by a junior engineer with zero project context.
    _Rationale: Smaller tasks reduce hallucination risk in sub-agents and enable precise progress tracking._
