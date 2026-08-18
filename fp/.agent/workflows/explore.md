---
description: Explore codebase architecture via knowledge graph and impact analysis
---
// turbo-all

# /explore — Redirects to /bmad Phase 0

> **This workflow has been merged into `/bmad` Phase 0 (RESEARCH → Code Intelligence).**
> Running `/explore` is equivalent to running `/bmad` with an "explore architecture" task.
>
> The Code Intelligence step auto-activates in Phase 0 when signals like
> "explore", "architecture", "understand codebase", or "knowledge graph" are detected.

## Quick Reference

These commands are now part of `/bmad` Phase 0 step 3:

```bash
# 1. Ensure code is indexed
omni index --project $(basename $PWD)

# 2. Export knowledge graph
omni graph --project $(basename $PWD) --output .omni/knowledge-graph.json

# 3. Impact analysis (for specific files)
omni impact src/commands/run.rs

# 4. Security graph (optional)
omni security scan --project $(basename $PWD)
```

> **Use `/bmad` directly** — it auto-routes to Code Intelligence when appropriate.
