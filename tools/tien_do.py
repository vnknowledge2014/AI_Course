#!/usr/bin/env python3
"""Đo tiến độ thật so với phạm vi v1.0 đã đóng băng.

Viết ra vì một lý do cụ thể: trong dự án này tôi đã báo sai con số nhiều lần —
"40/40 xanh" khi 21 file là khung rỗng, "cổng bí mật sạch" khi nó bỏ sót đúng
khoá đã sinh ra nó, và MASTERPLAN thì ghi phạm vi v1.0 là "~180 bài" trong khi
ba phần cấu thành cộng lại là 302.

Mẫu số chung: một con số được nói ra mà không ai đo lại. Script này đo.

Ba nguồn, và chúng phải khớp nhau:
  1. Phạm vi đóng băng — MASTERPLAN §0 quyết định 13
  2. Mạch đã thiết kế  — bảng trong các `MACH.md`
  3. Bài đã viết       — file `.lesson.md` trên đĩa

    python3 tools/tien_do.py
    python3 tools/tien_do.py --json
"""
from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys

GOC = pathlib.Path(__file__).resolve().parent.parent
NOI_DUNG = GOC / "content"

# Phạm vi v1.0, chép từ MASTERPLAN §0 quyết định 13 và §9.1.
# Đặt ở đây để script có một mốc CỐ ĐỊNH mà đối chiếu — nếu MASTERPLAN đổi thì
# dòng này phải đổi theo, và việc phải sửa hai chỗ là cố ý: nó buộc người sửa
# phạm vi phải nhìn thấy con số thật.
PHAM_VI = [
    ("onboarding", "R0 — Bãi đáp của Byte", 40, ["may-tinh-noi-gi", "ra-lenh-cho-byte"]),
    ("nen-tang", "R1 — Nền tảng lập trình", 150,
     ["gia-tri-bien-kieu", "re-nhanh-va-lap", "ham-vien-gach", "list-dict-set-tuple",
      "chuong-trinh-that"]),
    ("toan", "R2.T1–T3 — Toán & Toán rời rạc", 112,
     ["cam-nhan-so", "dai-so-va-ham-so", "logic-va-chung-minh"]),
]

MACH = {
    "onboarding": NOI_DUNG / "onboarding" / "MACH.md",
    "nen-tang": NOI_DUNG / "nen-tang" / "MACH.md",
    "toan": NOI_DUNG / "toan" / "MACH.md",
}


def dem_mach(t: pathlib.Path) -> int:
    """Số dòng bài trong mọi bảng mạch của một file."""
    if not t.is_file():
        return 0
    return len(re.findall(r"^\|\s*\*{0,2}\d+\*{0,2}\s*\|\s*`", t.read_text(encoding="utf-8"), re.M))


def da_viet(realm: str) -> dict[str, int]:
    """Số bài `.lesson.md` theo module, đọc từ frontmatter chứ không từ đường dẫn."""
    ra: dict[str, int] = {}
    goc = NOI_DUNG / realm
    if not goc.is_dir():
        return ra
    for t in goc.rglob("*.lesson.md"):
        m = re.search(r"^module:\s*(\S+)", t.read_text(encoding="utf-8"), re.M)
        if m:
            k = m.group(1).strip().strip('"')
            ra[k] = ra.get(k, 0) + 1
    return ra


def main() -> int:
    ap = argparse.ArgumentParser(description="Đo tiến độ so với phạm vi v1.0")
    ap.add_argument("--json", action="store_true")
    ns = ap.parse_args()

    bao = []
    for realm, ten, chi_tieu, track in PHAM_VI:
        viet = da_viet(realm)
        bao.append({
            "realm": realm,
            "ten": ten,
            "chi_tieu": chi_tieu,
            "co_mach": dem_mach(MACH[realm]),
            "da_viet": sum(viet.values()),
            "track": [{"id": k, "da_viet": viet.get(k, 0)} for k in track],
        })

    chuong = len(list((NOI_DUNG / "legacy").rglob("*.chapter.md"))) if (NOI_DUNG / "legacy").is_dir() else 0
    tong_ct = sum(b["chi_tieu"] for b in bao)
    tong_mach = sum(b["co_mach"] for b in bao)
    tong_viet = sum(b["da_viet"] for b in bao)

    if ns.json:
        print(json.dumps(
            {"tier_a": {"chi_tieu": tong_ct, "co_mach": tong_mach, "da_viet": tong_viet},
             "tier_c": {"chi_tieu": 159, "da_co": chuong},
             "realm": bao}, ensure_ascii=False, indent=2))
        return 0

    def thanh(x: int, n: int, rong: int = 24) -> str:
        day = 0 if n == 0 else round(x / n * rong)
        return "█" * day + "·" * (rong - day)

    print("TIER-A — bài tương tác\n")
    print(f"  {'realm':34s} {'viết':>5s} {'mạch':>5s} {'đích':>5s}")
    for b in bao:
        print(f"  {b['ten']:34s} {b['da_viet']:5d} {b['co_mach']:5d} {b['chi_tieu']:5d}"
              f"   {thanh(b['da_viet'], b['chi_tieu'])}")
        for tr in b["track"]:
            dau = "·" if tr["da_viet"] == 0 else " "
            print(f"     {dau} {tr['id']:30s} {tr['da_viet']:5d}")
    pc = 0 if tong_ct == 0 else tong_viet / tong_ct * 100
    print(f"\n  {'TỔNG':34s} {tong_viet:5d} {tong_mach:5d} {tong_ct:5d}   {pc:.0f}% phạm vi v1.0")

    print(f"\nTIER-C — chương đọc\n")
    print(f"  {chuong}/159   {thanh(chuong, 159)}")
    if chuong == 0:
        print("  (chạy `python3 tools/di_tru_sach.py` để sinh)")

    con = tong_ct - tong_viet
    print(f"\nCòn {con} bài chưa viết trong phạm vi v1.0.")
    chua_mach = tong_ct - tong_mach
    if chua_mach > 0:
        print(f"Trong đó {chua_mach} bài chưa cả mạch — phải thiết kế trước khi viết được.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
