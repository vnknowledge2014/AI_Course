#!/usr/bin/env python3
"""Extract standalone-compilable Rust snippets from the Rust_Books chapters."""

from __future__ import annotations

import json
import re
import shutil
import subprocess
import sys
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BOOKS = ROOT / "fp/Rust_Books"
CORPUS = ROOT / "crates/byte-rust-conformance/corpus"

FENCE = re.compile(
    r"^(?P<indent>[ \t]*)(?P<ticks>`{3,}|~{3,})[ \t]*(?P<lang>[A-Za-z0-9_+-]*)[^\n]*\n"
    r"(?P<body>.*?)"
    r"^(?P=indent)(?P=ticks)[ \t]*$",
    re.S | re.M,
)

RUST_LANGS = {"rust", "rs", "rust,no_run", "rust,ignore"}

EXTERNAL_CRATES = {
    "serde", "serde_json", "serde_yaml", "tokio", "rig", "axum", "sqlx", "nom",
    "proptest", "quickcheck", "rayon", "reqwest", "anyhow", "thiserror", "clap",
    "futures", "async_trait", "chrono", "uuid", "regex", "rand", "log", "tracing",
    "itertools", "im", "frunk", "diesel", "sea_orm", "actix", "actix_web", "warp",
    "hyper", "tonic", "prost", "criterion", "mockall", "wiremock", "tower",
    "sqlite", "rusqlite", "redis", "mongodb", "lazy_static", "once_cell",
    "parking_lot", "crossbeam", "dashmap", "bytes", "wgpu", "candle", "candle_core",
    "ndarray", "polars", "petgraph", "num", "num_traits", "either", "eyre",
    "color_eyre", "config", "dotenv", "envy", "validator", "jsonwebtoken",
    "argon2", "bcrypt", "sha2", "md5", "hex", "base64", "url", "mime", "tempfile",
    "walkdir", "glob", "indicatif", "console", "colored", "termcolor", "structopt",
    "pyo3", "wasm_bindgen", "js_sys", "web_sys", "leptos", "yew", "dioxus",
    "bevy", "ggez", "macroquad", "image", "plotters", "csv", "toml", "ron",
    "bincode", "rmp_serde", "postcard", "arbitrary", "insta", "rstest",
    "test_case", "pretty_assertions", "assert_cmd", "predicates", "trybuild",
    "cargo_husky", "divan", "iai", "flamegraph", "pprof", "opentelemetry",
    "metrics", "prometheus", "sentry", "openai", "async_openai", "llm", "ollama",
    "qdrant_client", "tiktoken_rs", "hf_hub", "tokenizers", "burn", "tch", "dfdx",
    "smallvec", "arrayvec", "tinyvec", "bitflags", "byteorder", "memmap2",
    "libc", "nix", "winapi", "windows", "cc", "bindgen", "cbindgen",
}

# pseudo-code / ellipsis markers
ELLIPSIS = re.compile(r"(\.\.\.|…|…)")
PSEUDO = re.compile(
    r"(?im)^\s*(//|/\*)?\s*(\.\.\.|…|<[^>]*>\s*$|TODO|FIXME|snip|elided|omitted"
    r"|rest of|etc\.|and so on|như trên|tương tự|lược bỏ|bỏ qua)",
)

ITEM_START = re.compile(
    r"(?m)^\s*(#\[|#!\[|///|//!|pub\s|fn\s|struct\s|enum\s|trait\s|impl\b|mod\s"
    r"|type\s|const\s|static\s|use\s|macro_rules!|unsafe\s|extern\s|async\s+fn\b)"
)

STMT_ONLY = re.compile(
    r"(?m)^\s*(let\s|println!|assert|match\s|if\s|for\s|while\s|loop\s|return\b"
    r"|\}|\)|\.|\w+\s*\()"
)


def rust_fences(md: Path):
    text = md.read_text(encoding="utf-8", errors="replace")
    for m in FENCE.finditer(text):
        lang = m.group("lang").lower()
        if lang in RUST_LANGS or lang.startswith("rust"):
            line = text.count("\n", 0, m.start()) + 1
            yield line, m.group("body")


def strip_hidden(code: str) -> str:
    # rustdoc hidden lines `# foo;`
    out = []
    for ln in code.splitlines():
        s = ln.lstrip()
        if s.startswith("# ") or s == "#":
            continue
        out.append(ln)
    return "\n".join(out)


def uses_external_crate(code: str) -> str | None:
    for m in re.finditer(r"(?m)^\s*(?:pub\s+)?use\s+([A-Za-z_][A-Za-z0-9_]*)", code):
        root = m.group(1)
        if root in EXTERNAL_CRATES:
            return root
    for m in re.finditer(r"(?m)^\s*extern\s+crate\s+([A-Za-z_][A-Za-z0-9_]*)", code):
        if m.group(1) in EXTERNAL_CRATES:
            return m.group(1)
    for m in re.finditer(r"\b([a-z_][a-z0-9_]*)::", code):
        if m.group(1) in EXTERNAL_CRATES:
            return m.group(1)
    if re.search(r"(?m)^\s*#\[(tokio::|serde|derive\([^)]*(Serialize|Deserialize))",
                 code):
        return "serde/tokio-attr"
    return None


def classify(code: str) -> tuple[str | None, str]:
    """Return (reject_reason | None, normalized_code)."""
    body = strip_hidden(code).strip("\n")
    if not body.strip():
        return "empty", body
    if len(body.strip()) < 20:
        return "too_short", body

    # toml/shell/output blocks mislabeled as rust
    if re.search(r"(?m)^\s*(\$|\[dependencies\]|cargo |error\[E)", body):
        return "not_code", body

    if ELLIPSIS.search(body) or PSEUDO.search(body):
        return "ellipsis_or_pseudo", body

    ext = uses_external_crate(body)
    if ext:
        return f"external_crate", body

    has_main = re.search(r"(?m)^\s*(pub\s+)?(async\s+)?fn\s+main\s*\(", body)
    if has_main:
        if re.search(r"(?m)^\s*async\s+fn\s+main", body):
            return "external_crate", body  # async main needs a runtime
        return None, body

    if ITEM_START.search(body):
        # make sure the very first non-comment/non-blank line is item-like
        for ln in body.splitlines():
            s = ln.strip()
            if not s or s.startswith("//"):
                continue
            if not ITEM_START.match(ln):
                return "not_standalone_item", body
            break
        return None, body + "\n\nfn main() {}\n"

    return "no_main_no_item", body


def main() -> int:
    if CORPUS.exists():
        shutil.rmtree(CORPUS)
    CORPUS.mkdir(parents=True, exist_ok=True)

    files = sorted(BOOKS.rglob("chapter_*.md"))
    total = 0
    rejects = Counter()
    reject_detail = Counter()
    kept = []

    for md in files:
        rel = md.relative_to(BOOKS)
        stem = re.sub(r"[^A-Za-z0-9]+", "_", str(rel.with_suffix(""))).strip("_")
        idx = 0
        for line, code in rust_fences(md):
            total += 1
            idx += 1
            reason, norm = classify(code)
            if reason:
                rejects[reason] += 1
                if reason == "external_crate":
                    ext = uses_external_crate(strip_hidden(code)) or "async-main"
                    reject_detail[ext] += 1
                continue
            name = f"{stem}__{idx:03d}_L{line}.rs"
            path = CORPUS / name
            path.write_text(norm.rstrip() + "\n", encoding="utf-8")
            kept.append((path, str(rel), line))

    # compile check
    outdir = CORPUS.parent / ".metadata_out"
    if outdir.exists():
        shutil.rmtree(outdir)
    outdir.mkdir(parents=True)

    def check(item):
        path = item[0]
        r = subprocess.run(
            ["rustc", "--edition", "2021", "--emit", "metadata",
             "--crate-type", "bin", "-A", "warnings",
             "-o", str(outdir / (path.stem + ".rmeta")), str(path)],
            capture_output=True, text=True, timeout=90,
        )
        return path, r.returncode, r.stderr

    ok, fail = [], []
    with ThreadPoolExecutor(max_workers=8) as ex:
        for path, rc, err in ex.map(check, kept):
            if rc == 0:
                ok.append(path)
            else:
                codes = re.findall(r"error\[(E\d+)\]", err)
                first = codes[0] if codes else (
                    "E-syntax" if "expected" in err else "E-other")
                fail.append((path, first))

    # remove failures from the corpus: only positives stay
    fail_reasons = Counter()
    for path, code in fail:
        fail_reasons[code] += 1
        path.unlink()
    shutil.rmtree(outdir, ignore_errors=True)

    report = {
        "files_scanned": len(files),
        "total_rust_snippets": total,
        "kept_for_compile": len(kept),
        "compiled_ok": len(ok),
        "compile_failed": len(fail),
        "prefilter_rejects": dict(rejects.most_common()),
        "external_crate_breakdown": dict(reject_detail.most_common(12)),
        "compile_failure_by_error": dict(fail_reasons.most_common(12)),
        "corpus_dir": str(CORPUS),
    }
    (CORPUS.parent / "extraction_report.json").write_text(
        json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
