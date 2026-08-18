---
description: Prevent terminal crashes from excessive output. Applies to IDE agent and sub-agents.
trigger: always_on
---

# Terminal Safety

> Prevent Pty Host crashes from excessive terminal output.
> **Applies to:** Antigravity IDE agent AND Ollama sub-agents.

1.  **Never dump code to terminal**: Do NOT use `echo`, `cat`, `printf`, or heredoc (`<< EOF`) to output large code blocks to stdout. Use `write_file` tool or file redirection (`> file.txt`) instead.
    _Rationale: Large stdout output (>50KB) crashes the IDE's Pty Host, causing permanent hang._
2.  **Always truncate command output**: Append `| tail -50` or `| head -100` to commands that may produce long output (e.g., `cargo test`, `npm install`, `git log`, `find`, `grep`). Use `2>&1 | tail -30` for error-heavy commands.
    _Rationale: Unbounded output floods the Pty buffer and causes terminal disconnect._
3.  **Never run unbounded commands**: Commands like `cat large_file.rs`, `curl URL` (large response), `cargo test` (verbose), or `find / -name "*.rs"` MUST always have output limits.
    _Rationale: Even 1MB of output can crash the Pty Host._
4.  **Use file tools over terminal**: Prefer `view_file` / `read_file` over `cat`. Prefer `write_file` / `replace_in_file` over `sed -i` or heredocs. Prefer `grep_search` over `grep -r`.
    _Rationale: File tools have built-in size limits and don't use the Pty._
5.  **Background long-running processes**: For commands that run >30 seconds (builds, installs, downloads), run them with proper async handling. Never leave a terminal blocking indefinitely.
    _Rationale: Blocking terminals prevent the IDE from sending subsequent commands._
6.  **Recovery**: If a terminal becomes unresponsive, terminate it and open a new one rather than waiting indefinitely.

## Anti-Patterns

```
# ❌ BAD: Dumping code to terminal
echo "fn main() { ... 200 lines ... }"
cat << 'EOF'
... hundreds of lines ...
EOF
cargo test 2>&1  # unbounded output

# ✅ GOOD: Using proper tools
write_file src/main.rs "fn main() { ... }"  # use file tool
cargo test 2>&1 | tail -30                  # truncated output
cargo check 2>&1 | tail -5                  # minimal output
git log -n 10 --oneline                     # bounded output
```
