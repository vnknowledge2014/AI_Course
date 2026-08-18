---
description: "Git workflow powered by GitButler CLI — Conventional Commits, virtual/stacked branches, AI commits, full undo. Installed and initialized automatically by omni setup."
trigger: always_on
---
# Git Workflow (GitButler CLI + Conventional Commits)

> All version control goes through GitButler CLI (`but`). It layers on top of Git —
> all standard `git` commands still work underneath. `omni setup` auto-installs and
> initializes GitButler.
>
> **Global skills override**: When this rule is active, do NOT use global skills
> `git-pushing` or `git-pr-workflows-git-workflow`. Use `but` equivalents instead.

## 0. Prerequisites

```bash
but setup    # One-time: initialize GitButler management on this repo
but teardown # Revert to vanilla Git (safe, preserves metadata)
```

After `but setup`, GitButler manages a `gitbutler/workspace` branch and virtual branches.
All standard `git` commands still work — GitButler layers on top.

## 1. Atomic Commits

Commits MUST represent a single, logical change.

```bash
# Commit specific files/hunks only (atomic)
but commit -p h0,i0 -o -m "feat(auth): add JWT rotation"

# Commit all unassigned changes
but commit -m "fix: resolve race condition"

# AI-generated commit message (validate format after)
but commit --ai
```

_Rationale: Enables git bisect, meaningful review, and safe individual reverts._

## 2. Conventional Commits (v1.0.0)

> Spec: https://www.conventionalcommits.org/en/v1.0.0/

### Format

```
<type>[(scope)][!]: <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | SemVer | When to use |
|------|--------|-------------|
| `feat` | MINOR | New feature for the user/codebase |
| `fix` | PATCH | Bug fix |
| `docs` | — | Documentation only |
| `style` | — | Formatting, whitespace (no logic change) |
| `refactor` | — | Code restructure (no feature/fix) |
| `perf` | — | Performance improvement |
| `test` | — | Adding or correcting tests |
| `build` | — | Build system or external dependencies |
| `ci` | — | CI/CD configuration |
| `chore` | — | Maintenance (no src/test change) |
| `revert` | — | Reverting a previous commit |

### Breaking Changes → MAJOR

Two ways to signal a breaking change:

```bash
# Option 1: ! after type/scope
but commit -m "feat(api)!: remove deprecated auth endpoint"

# Option 2: Multi-line with BREAKING CHANGE footer
but commit -m "feat(api): migrate to OAuth2

BREAKING CHANGE: apiKey auth method removed, use bearer token instead."
```

### Rules (from spec)

1. Commits MUST be prefixed with a type, followed by optional scope, optional `!`, and REQUIRED colon + space.
2. `feat` MUST be used when a commit adds a new feature.
3. `fix` MUST be used when a commit represents a bug fix.
4. A scope MAY be provided after the type — a noun in parentheses describing a section of the codebase (e.g., `fix(parser):`).
5. A description MUST immediately follow the colon and space after the type/scope prefix.
6. A body MAY be provided after a blank line following the description.
7. A `BREAKING CHANGE:` footer MUST be uppercase. `BREAKING-CHANGE` is synonymous.
8. If `!` is used in the type/scope prefix, `BREAKING CHANGE:` footer MAY be omitted.

### Examples

```bash
# Feature with scope
but commit -m "feat(auth): add JWT refresh token rotation"

# Bug fix (no scope)
but commit -m "fix: prevent race condition in request queue"

# Breaking change with ! and footer
but commit -m "feat(api)!: switch to v2 response format

BREAKING CHANGE: response data field is now an array instead of object."

# Docs change
but commit -m "docs: update API reference for /users endpoint"

# Multi-paragraph body with footers
but commit -m "fix(db): resolve connection pool exhaustion under load

Introduce connection recycling with 30s max idle timeout.
Remove manual retry logic that caused thundering herd.

Reviewed-by: senior-dev
Refs: #456"
```

### Using `--ai` with Conventional Commits

When using `but commit --ai`, the AI-generated message may not follow Conventional Commits format.
Always validate and reword if necessary:

```bash
but commit --ai                    # AI generates message
but reword <commit-id> -m "feat(scope): proper message"  # Fix if needed
```

Or use the `but_smart_commit.sh` wrapper script which validates automatically.

## 3. Branching Strategy (Virtual Branches)

GitButler supports **parallel** and **stacked** branches simultaneously.

### Parallel Branches (independent work)

```bash
but branch new feature/<name>    # e.g., feature/jwt-auth
but branch new bugfix/<name>     # e.g., bugfix/pool-leak
but branch new chore/<name>      # e.g., chore/deps-update
```

### Stacked Branches (dependent chain)

```bash
but branch new step-1-api         # Base branch
but branch new step-2-frontend -a step-1-api   # Stacked on step-1
but branch new step-3-tests -a step-2-frontend  # Stacked on step-2
```

Stacked branches auto-create stacked PRs when pushed.

### Naming Convention

| Type | Pattern | Example |
|------|---------|------------|
| Feature | `feature/<name>` | `feature/jwt-auth` |
| Bug fix | `bugfix/<name>` | `bugfix/pool-leak` |
| Chore | `chore/<name>` | `chore/deps-update` |
| Agent task | `agent/<task-id>` | `agent/implement-auth` |

**Never** commit unassigned changes directly — always create or target a branch.
**Never** commit directly to `main` or `dev`.

## 4. Staging & Assigning

```bash
# Assign files to a branch (like git add, but per-branch)
but stage <file-id> <branch>      # e.g., but stage h0 feature/auth
but stage app/ feature/auth       # Glob assign

# Commit only staged files
but commit -o -m "feat(auth): add middleware" feature/auth
```

## 5. Editing Commits

```bash
but reword <commit>              # Edit commit message
but amend <commit>               # Amend changes into existing commit
but squash <c1> <c2>             # Squash two commits
but squash <c1> <c2> --ai        # Squash with AI-combined message
but absorb                       # Auto-amend changes into right commits
but uncommit <commit>            # Reverse a commit (keep changes)
but move <commit> <branch>       # Move commit between branches
but pick <commit> <branch>       # Cherry-pick from unapplied branch
```

## 6. Rollback & Safety

```bash
but oplog                        # View operation history
but undo                         # Undo last operation (full snapshot restore)
but discard <file-or-hunk>       # Discard specific changes
```

> **Always prefer `but undo` over `git reset --hard`**.
> GitButler's operations log provides full snapshot-based undo of ANY operation.

_Rationale: Operations log is more comprehensive than git reflog — captures branch assignments, staging, commits, and merges._

## 7. Push & Pull Requests

```bash
but push <branch>                # Push single branch
but push --all                   # Push all branches
but pull                         # Pull upstream changes

but pr create <branch>           # Create PR on forge (GitHub/GitLab)
but pr list                      # List open PRs
```

For stacked branches, `but push` automatically sets up stacked PRs.

## 8. No Force Pushes

Do NOT force push to shared branches. GitButler handles rebasing internally.
_Rationale: Force pushes destroy remote history that other agents/developers depend on._

## 9. Conflict Resolution

```bash
but resolve <file>               # Mark file as resolved
but resolve --all                # Resolve all conflicts
but merge <branch>               # Merge a branch locally
```

When conflicts arise during `but pull` or `but merge`, GitButler pauses and shows the conflicting files. Resolve conflicts in your editor, then run `but resolve`.

## 10. Marks & Auto-Assignment Rules

```bash
but mark <path-pattern> <branch>  # Auto-assign matching files to branch
but unmark                        # Remove all marks
```

Examples:
```bash
but mark "src/auth/**" feature/auth      # All auth files → auth branch
but mark "*.test.ts" feature/tests       # All test files → tests branch
but mark "docs/**" chore/docs            # All docs → docs branch
```

_Rationale: Marks eliminate manual staging — files are auto-assigned to the right branch as they change._

## 11. Inspection (JSON mode for agents)

All commands support `--json` (`-j`) for machine-parseable output:

```bash
but status -f -j                 # Full status + files as JSON
but diff -j                      # Diff as JSON
but show <commit> -j             # Commit details as JSON
```

## 12. Commit Message Length

- **Subject line**: Max 72 characters (type + scope + description).
- **Body**: Wrap at 72 characters per line.
- **Footer**: No length limit, but keep concise.
_Rationale: Git tooling (log, shortlog, blame) truncates at 72 chars._

## 13. Lifecycle Hooks (Omni Integration)

Omni runs these automatically via `omni.config.yaml`:

| Hook | GitButler Action | Purpose |
|------|-----------------|----------|
| `pre_run` | `but branch new agent/run-<timestamp>` | Create branch per agent run |
| `post_task` | `but commit --ai` | Auto-commit after each task |
| `post_run` | `but push` | Push to remote after run |
| `on_error` | `but undo` | Rollback on agent error |

These are configured in `omni.config.yaml` and fire automatically. No manual action needed.

## Quick Reference

| Git Command | but Equivalent |
|-------------|---------------|
| `git status` | `but status` (`but status -f -j` for full JSON) |
| `git add <file>` | `but stage <file> <branch>` |
| `git commit -m "msg"` | `but commit -m "msg" [branch]` |
| `git push` | `but push [branch]` |
| `git checkout -b name` | `but branch new name` |
| `git log` | `but show <branch>` / `but oplog` |
| `git reset --hard` | `but undo` |
| `git cherry-pick` | `but pick <commit> <branch>` |
| `git rebase -i` | `but squash` / `but move` / `but absorb` |
| `gh pr create` | `but pr create <branch>` |
