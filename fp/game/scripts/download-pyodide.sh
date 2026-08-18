#!/bin/bash
# Download Pyodide for offline use
PYODIDE_VERSION="0.26.2"
PYODIDE_BASE="https://cdn.jsdelivr.net/pyodide/v${PYODIDE_VERSION}/full"
OUT_DIR="./public/pyodide"

mkdir -p "$OUT_DIR"

echo "Downloading Pyodide v${PYODIDE_VERSION}..."

files=(
  "pyodide.js"
  "pyodide.asm.js"
  "pyodide.asm.wasm"
  "python_stdlib.zip"
  "package.json"
)

for file in "${files[@]}"; do
  echo "  Downloading $file..."
  curl -sL "${PYODIDE_BASE}/${file}" -o "${OUT_DIR}/${file}"
done

echo "✅ Pyodide downloaded to ${OUT_DIR}/"
echo "Total size: $(du -sh ${OUT_DIR} | cut -f1)"
