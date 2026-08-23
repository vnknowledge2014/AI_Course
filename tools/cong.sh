#!/usr/bin/env bash
# Chạy MỌI cổng của dự án, theo thứ tự rẻ-trước-đắt-sau.
#
# Bảy cổng nằm ở ba runtime khác nhau (node, python, cargo) và mỗi cái đo một
# thứ mà những cái kia mù. Không có lệnh gộp thì người ta chạy vài cái rồi
# tưởng đã kiểm hết — và trong dự án này, "tưởng đã kiểm hết" là chế độ hỏng
# tốn kém nhất: khung `TODO —` biên dịch được, `::::byte` trả `null`,
# `requireAst` thành mảng rỗng, WASM cũ hơn mã nguồn. Lần nào cũng có màu
# xanh, chỉ là màu xanh của một cổng không nhìn tới chỗ đó.
#
#   ./tools/cong.sh          chạy hết
#   ./tools/cong.sh --nhanh  bỏ qua cổng chậm (Pyodide, cargo)
set -uo pipefail
cd "$(dirname "$0")/.."

NHANH=0
[[ "${1:-}" == "--nhanh" ]] && NHANH=1

XANH=0
DO=0
BO=0

chay() {
  local ten="$1"; shift
  printf '\n\033[1m▸ %s\033[0m\n' "$ten"
  if "$@"; then
    XANH=$((XANH + 1))
  else
    DO=$((DO + 1))
    printf '\033[31m  ✗ %s ĐỎ\033[0m\n' "$ten"
  fi
}

bo_qua() {
  BO=$((BO + 1))
  printf '\n\033[2m▸ %s — bỏ qua (--nhanh)\033[0m\n' "$1"
}

# ── 1. Nội dung ───────────────────────────────────────────────────────────
chay "Biên dịch bài học"      node packages/content-compiler/dist/cli.js build content dist/content
chay "Hiến chương sư phạm"    python3 tools/kiem_bai_hoc.py
chay "Đồ thị tiền đề"         python3 tools/kiem_do_thi.py
chay "Số học trong văn xuôi"  python3 tools/kiem_so_hoc.py
chay "Bí mật lọt vào git"     python3 tools/kiem_bi_mat.py
chay "Schema khớp MASTERPLAN" python3 tools/trich_schema.py --check

# ── 2. Mã ─────────────────────────────────────────────────────────────────
chay "Test JS"                pnpm -r --silent test

if [[ $NHANH -eq 1 ]]; then
  bo_qua "Lời giải chạy thật (Pyodide)"
  bo_qua "Test Rust"
  bo_qua "Cổng đối chiếu rustc"
else
  chay "Lời giải chạy thật"   node --max-old-space-size=4096 tools/kiem_ma_bai_hoc.mjs
  chay "Test Rust"            cargo test --workspace -q
  chay "Đối chiếu rustc"      bash -c 'cd crates/byte-rust-conformance && cargo run --release -q -- --am'
fi

# ── 3. Tiến độ ────────────────────────────────────────────────────────────
printf '\n\033[1m▸ Tiến độ\033[0m\n'
python3 tools/tien_do.py | tail -12

printf '\n────────────────────────────────────────\n'
if [[ $DO -eq 0 ]]; then
  printf '\033[32m✅ %d cổng xanh' "$XANH"
  [[ $BO -gt 0 ]] && printf ', %d bỏ qua' "$BO"
  printf '\033[0m\n'
  exit 0
fi
printf '\033[31m❌ %d cổng ĐỎ / %d xanh' "$DO" "$XANH"
[[ $BO -gt 0 ]] && printf ', %d bỏ qua' "$BO"
printf '\033[0m\n'
exit 1
