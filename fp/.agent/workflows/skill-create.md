---
description: "Create new AI Skills from provided sources (URLs, docs, code). Adapts skill-generator's 8-phase pipeline."
---

// turbo-all

# /skill-create — Skill Generator Workflow

> Create production-quality SKILL.md files from any source material.

## Usage

Provide sources in your request:
- URLs (GitHub repos, docs, articles)
- Local files (code, READMEs, configs)
- Ideas/descriptions in natural language

Example: `/skill-create from https://github.com/example/project — create a skill for deploying with Docker`

---

## Phase 1: EXTRACT — Understand the Source

1. **Classify Source Type**: Identify if the provided sources are:
   - **General/Code sources** (URLs, repository READMEs, small codebase files, text instructions).
   - **Long-form documents or Books** (PDF, EPUB, DOCX, HTML, RTF, MOBI, AZW, AZW3).

2. **Branching Process**:
   - **For General/Code sources (Standard Mode)**:
     - Read URLs using `read_url_content` or `browser_subagent`.
     - Read local files using `view_file`.
     - Identify the core capability, step-by-step instructions, anti-patterns, and errors.
     - Document findings in a scratch note at `/tmp/skill-extract.md`.
   - **For Long-form documents or Books (Book Mode)**:
     - Propose a skill name slug: `{author-lastname}-{core-concept}` or title-based.
     - Detect if this is a new skill creation or an **Update/Fold-in** operation (target skill directory already contains `SKILL.md` and a `chapters/` sub-folder). If it's an Update, skip structure extraction and use the **Update / Fold-in Workflow**.
     - Ask the user to identify the book type:
       > "What kind of content do these sources have? This helps me choose the best extraction method.
       > 1. **Technical** — has code blocks, tables, formulas, diagrams (e.g. programming books, academic papers, architecture guides)
       > 2. **Text-heavy** — mostly prose, few or no tables/code (e.g. management, productivity, narrative non-fiction)
       > 3. **Not sure** — I'll use the fast method and warn you if quality seems limited"
     - Run the Python extractor script:
       ```bash
       python3 scripts/book_to_skill/extract.py <INPUT_PATHS> --mode <technical|text> --install-missing ask
       ```
     - Read `<tempdir>/book_skill_work/metadata.json` to inspect the results.
     - Present the user with a pre-flight cost estimate:
       - Input tokens ≈ `estimated_tokens` from metadata × 1.3
       - Output tokens ≈ chapters × per-chapter budget + 10,000 (SKILL.md + supporting files)
       - Wait for user confirmation to proceed (Full Conversion / Update vs Analyze Only).
     - For books over ~50k tokens, use RLM (Recursive Language Model) probe methods (wc, grep, sed offsets) instead of reading the entire extracted text at once:
       ```bash
       # Find chapter offsets without loading the whole file
       grep -n -E "^\s*(Chapter|CHAPTER)\s+[0-9]+" "$FULL_TEXT_PATH" | head -40
       # Pull only the chapter you need
       sed -n '<start>,<end>p' "$FULL_TEXT_PATH"
       # Verify a framework is mentioned before claiming it
       grep -c -i "<framework>" "$FULL_TEXT_PATH"
       ```

---

## Phase 2: DETECT — Classify Complexity & Depth

- **For Standard Mode**:
  Score the skill on these dimensions before generating:

  | Dimension | Question | Score |
  |-----------|----------|-------|
  | Scope | How many distinct steps? | 1-5 |
  | Dependencies | External tools/APIs required? | 1-5 |
  | Error Surface | How many things can go wrong? | 1-5 |
  | Domain Knowledge | Prior expertise needed? | 1-5 |

  - **Sum ≤ 8** → Simple skill (800-1500 chars)
  - **Sum 9-15** → Medium skill (1500-3500 chars)
  - **Sum 16-20** → Complex skill (3500-5000 chars)

- **For Book Mode**:
  Ask the user what the skill should help them do, and determine the `DEPTH` level:
  - Answer includes "Apply the author's frameworks" or "Think with the author's mental models" → **DEPTH=study**
  - Answer is only "Reference specific chapters and concepts" → **DEPTH=reference**

---

## Phase 3: GENERATE — Write Skill Components

### Pipeline A: Standard Mode (Single-File Skill)

Create the file at: `~/.config/_skills_/<skill-name>/SKILL.md`

Use this template:

```markdown
---
name: <skill-name>
description: <1-sentence description of what this skill teaches and when to use it>
---

# <Title>

<2-3 sentence overview of the capability>

## Prerequisites

- <tool/dependency 1>
- <tool/dependency 2>

## Usage

### Step 1: <Action>
<Clear instruction with code example>

```<language>
<code>
```

### Step 2: <Action>
...

## Error Handling

| Error | Cause | Fix |
|-------|-------|-----|
| <error msg> | <why> | <solution> |

## Anti-Patterns

- Do NOT <common mistake 1>
- Do NOT <common mistake 2>
- Do NOT <common mistake 3>

## Examples

### Basic Example
<minimal working example>

### Advanced Example
<production-ready example with error handling>
```

### Pipeline B: Book Mode (Multi-File Skill)

Create the directory layout:
```bash
mkdir -p "$SKILLS_HOME/<skill_name>/chapters"
```

1. **Chapter Summaries**: Create `$SKILLS_HOME/<skill_name>/chapters/ch<NN>-<slug>.md` for each chapter.
   - **Adaptive Sizing Matrix (Target per chapter)**:
     - `technical/study` → 2,000–3,000 tokens (reproduce 1 worked example, detailed criteria, why-it-works notes)
     - `technical/reference` → 1,200–1,800 tokens
     - `text/study` → 1,000–1,800 tokens (reproduce 1 worked example)
     - `text/reference` → 800–1,200 tokens
   - **Structure**:
     - `# Chapter N: <Title>`
     - `## Core Idea` (1-2 sentences)
     - `## Frameworks Introduced` (exact formulations, when to use, how)
     - `## Key Concepts` (5-10 terms with 1-sentence definition)
     - `## Mental Models` (2-4 thinking tools)
     - `## Anti-patterns` (what to avoid and why)
     - `## Code Examples / Reference Tables` (if technical)
     - `## Worked Example` (if DEPTH=study)
     - `## Key Takeaways` (3-7 actionable insights)
     - `## Connects To` (references to other chapters/concepts)

2. **glossary.md**: Create `$SKILLS_HOME/<skill_name>/glossary.md`
   - Every significant term from the book, alphabetically sorted (e.g. `**Term** — definition (Ch N)`). Max 1,500 tokens.

3. **patterns.md**: Create `$SKILLS_HOME/<skill_name>/patterns.md`
   - All concrete techniques, design patterns, or algorithms (e.g. `## Pattern Name\n**When to use**: ...\n**How**: ...\n**Trade-offs**: ...`). Max 2,000 tokens.

4. **cheatsheet.md**: Create `$SKILLS_HOME/<skill_name>/cheatsheet.md`
   - Capture author's judgment: decision rules ("When X, do Y, because Z"), decision trees/flowcharts, trade-off matrices, default thresholds/rules of thumb, tells/smells. Max 1,200 tokens.

5. **Master SKILL.md**: Create `$SKILLS_HOME/<skill_name>/SKILL.md` (Max 4,000 tokens)
   - Layout:
     - Frontmatter with name, description, allowed-tools (Read, Grep), argument-hint.
     - `# <Title>` (with metadata: Author, Pages, Chapters, Generated date).
     - `## How to Use This Skill` (help commands like ch05 or topic search).
     - `## Core Frameworks & Mental Models` (the author's most important concepts, ~2,000 tokens).
     - `## Chapter Index` (table linking to chapter files and listing key frameworks).
     - `## Topic Index` (alphabetical topic mappings to chapters).
     - `## Supporting Files` (links to cheatsheet, glossary, patterns).

---

## Update / Fold-in Workflow (Book Mode Only)

When merging new content into an existing skill at `$SKILLS_HOME/<skill_name>/`:
1. **Read Existing Structure**: Parse `SKILL.md` (indices, frameworks, metadata), list existing chapters to find highest chapter number, read glossary, patterns, and cheatsheet.
2. **Identify Revisions vs Additions**: Read extracted text in `<tempdir>/book_skill_work/full_text.txt`.
   - Revisions: merge new details into existing chapter files and rewrite them.
   - Additions: create new chapter files starting from the next available chapter number.
3. **Merge Supporting Files**:
   - `glossary.md`: combine and alphabetize terms, append references (e.g., `(Ch 4, Ch 13)`).
   - `patterns.md` / `cheatsheet.md`: cleanly integrate new patterns and decision rules, keeping files under sizing budgets.
4. **Re-generate Master SKILL.md**: Update metadata, core frameworks, chapter index table, and topic index alphabetically.

---

## Phase 4: VALIDATE — Score Quality

Run the built-in validator:
```bash
omni skill validate ~/.config/_skills_/<skill-name>/SKILL.md
```

**Target score: ≥ 24/30 (Grade A)**

If score < 24, review the dimension breakdown and fix weak areas:

| Dimension | Fix if Low |
|-----------|-----------|
| Structure | Add/fix YAML frontmatter |
| Completeness | Add examples, error table, step-by-step |
| Clarity | Shorten lines, add headings |
| Safety | Remove dangerous commands (sudo, rm -rf) |
| Anti-Patterns | Add `## Anti-Patterns` with 3+ bullets |
| Size | Expand if < 500, trim if > 5000 |

*Note: For multi-file book skills, also manually verify that chapter links and supporting files are not broken.*

---

## Phase 5: TEST — Dry Run Simulation

Simulate the skill by mentally walking through it:

1. **Can an agent follow this without additional context?**
   - Every step must be self-contained
   - No assumed knowledge not listed in Prerequisites
2. **Are the code examples copy-paste ready?**
   - No `<placeholder>` that agents can't resolve
   - Real file paths, real command syntax
3. **Do the Anti-Patterns prevent real mistakes?**
   - Each anti-pattern should describe a specific failure mode

If any check fails → go back to Phase 3 and fix.

---

## Phase 6: ITERATE — Compare and Improve

If creating a skill that overlaps with existing skills:

```bash
omni skill validate --all | grep "<related-keyword>"
```

Compare scores. The new skill should:
- Score higher than similar existing skills
- Not duplicate content already covered
- Add unique value (different approach, better examples, more error handling)

---

## Phase 7: FINALIZE — Register and Sync

1. Verify files are properly registered at `~/.config/_skills_/<skill-name>/` (or `$SKILLS_HOME/<skill_name>/`).
2. Run final validation and clean up work directories.
3. Report final score/grade, path, and capability summary to the user.

---

## Flow Summary

```
Sources (URLs, files, books, ideas)
    │
    ▼
[1. EXTRACT] ──(Source type?)──► Book Mode ──► python3 extract.py ──► Estimate Cost
    │                                                                       │
    │ (General)                                                             ▼
    ▼                                                                   RLM Probing
[2. DETECT] ──► Simple/Medium/Complex (Standard)                        (wc/grep/sed)
    │           Study/Reference (Book)                                      │
    ▼                                                                       ▼
[3. GENERATE] ──(Pipeline A)──► Single-File SKILL.md                    (Pipeline B)
    │                                                                       │
    │◄──────────────────────────────────────────────────────────────────────┘
    ▼
[4. VALIDATE] ──► omni skill validate ──► score ≥ 24?
    │                                         │
    ▼                                     (no) ──► fix
[5. TEST] ──► Dry run simulation
    │
    ▼
[6. ITERATE] ──► Compare and deduplicate
    │
    ▼
[7. FINALIZE] ──► Register, clean up, report ✨
```

> **Quality Gate**: Never finalize a skill with Grade B or lower. Loop Phase 3-4 until Grade A.
