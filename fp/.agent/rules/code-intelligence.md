---
description: Rules for using the code knowledge graph, impact analysis, and Graph RAG.
trigger: always_on
---

# Code Intelligence

1.  **Impact Before Edit**: Before modifying a core file, run `omni impact <file>` to see ripple effects across the codebase. This uses the `calls`/`imports`/`extends` graph to identify affected files.
    _Rationale: Prevents unintended breakage in downstream consumers._
2.  **Outline Before Read**: Use `file_outline` (sub-agent tool) before `read_file` to understand file structure without loading full content. Saves ~80% context tokens on large files.
    _Rationale: Sub-agents have limited context windows — outline-first is a force multiplier._
3.  **Graph-Aware Refactoring**: When refactoring, use `query_callers` to find all upstream consumers, and `query_callees` to map downstream dependencies. Never rename a symbol without checking callers first.
    _Rationale: Graph queries replace manual grep and catch non-obvious transitive dependencies._
4.  **Keep Index Fresh**: After any file changes (create, modify, or delete), run `omni index` to refresh the code graph and embeddings. The indexer uses mtime-based incremental checks — only changed/new files are processed (~200ms). If SurrealDB is down, data is auto-saved to `.agent/fallback_data/`; import later with `omni index --import-fallback`. If a user confirms DB/Ollama is back up, auto-run `omni index --import-fallback` then `omni index` to fully sync.
    _Rationale: Stale graphs produce incorrect impact analysis; stale embeddings degrade RAG search quality._
5.  **Interactive Visualization**: Use `omni graph --project <name> --serve` to launch an interactive D3.js graph viewer in the browser. For static exports, use `--format dot` (GraphViz) or default JSON.
    _Rationale: Visual exploration reveals architectural patterns and hidden coupling that text queries miss._
6.  **Graph RAG is Automatic**: The RAG Enricher automatically injects 1-hop graph context (callers, callees, extends) into agent prompts alongside vector search results. No manual action required.
    _Rationale: Graph context gives agents structural awareness beyond simple code similarity._

