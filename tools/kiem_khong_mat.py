#!/usr/bin/env python3
"""Mọi khối mã trong nguồn phải có mặt trong bài đã biên dịch.

Một agent viết bài phát hiện: khối ```python title=readonly đặt trong bước
`predict` biến mất khỏi JSON. Quét ra 136 khối trong 77 trên 208 bài — mỗi
bước hỏi "trước khi bấm chạy, bạn đoán màn hình in ra gì?" đều đã vứt mất
chính đoạn mã cần đoán. Chín cổng xanh suốt.

Cổng ấy được vá ở gốc rồi. Cổng NÀY canh cả lớp, không canh riêng ca đó:
đếm khối mã ở hai đầu và bắt bên nào hụt. Nó không cần biết `title=` là gì,
không cần biết loại bước nào có ô chứa mã — nên một cách mất mã hoàn toàn
mới cũng bị bắt.

Cách đối chiếu là so văn bản NGUỒN của từng khối, không so số lượng: hai
khối có thể cùng biến mất và một khối khác nhân đôi, mà tổng vẫn khớp.

    python3 tools/kiem_khong_mat.py
"""
from __future__ import annotations

import argparse
import json
import os
import pathlib
import re
import sys

GOC = pathlib.Path(__file__).resolve().parent.parent
DICH = GOC / "dist" / "content"

MO = re.compile(r"^([ \t]*)```([a-zA-Z0-9_+-]*)(?:[ \t]+title=(\S+))?[ \t]*$")
DONG = re.compile(r"^[ \t]*```[ \t]*$")


def khoi_nguon(t: pathlib.Path) -> list[tuple[int, str, str]]:
    """(dòng, title, mã) cho từng khối trong file .lesson.md."""
    dong = t.read_text(encoding="utf-8").splitlines()
    ra: list[tuple[int, str, str]] = []
    i = 0
    while i < len(dong):
        m = MO.match(dong[i])
        if not m:
            i += 1
            continue
        so = i + 1
        i += 1
        than: list[str] = []
        while i < len(dong) and not DONG.match(dong[i]):
            than.append(dong[i])
            i += 1
        i += 1
        ra.append((so, m.group(3) or "", "\n".join(than)))
    return ra


def van_ban(x: object, gom: list[str]) -> None:
    """Gom mọi chuỗi trong cây JSON, không cần biết hình dạng.

    Cố tình không đi theo schema: cổng này phải sống sót qua mọi lần schema
    đổi, vì nó tồn tại để bắt đúng những chỗ schema và thực tế lệch nhau.
    """
    if isinstance(x, str):
        gom.append(x)
    elif isinstance(x, dict):
        for v in x.values():
            van_ban(v, gom)
    elif isinstance(x, list):
        for v in x:
            van_ban(v, gom)


def main() -> int:
    ap = argparse.ArgumentParser(description="Kiểm khối mã bị mất khi biên dịch")
    ap.add_argument("--json", action="store_true")
    ns = ap.parse_args()

    if not DICH.exists():
        print(f"❌ chưa có {DICH.relative_to(GOC)} — chạy `content build` trước.")
        return 1

    # Bản đồ id → JSON. Nguồn không mang id theo đường dẫn nên phải đọc
    # frontmatter; rẻ hơn là đọc lại toàn bộ dist một lần rồi tra theo id.
    theo_id: dict[str, str] = {}
    for f in DICH.glob("*.json"):
        if f.name == "index.json":
            continue
        theo_id[f.stem] = f.read_text(encoding="utf-8")

    thieu = []
    n_khoi = 0
    n_bai = 0
    for goc, thu_muc, tep in os.walk(GOC / "content"):
        thu_muc[:] = [d for d in thu_muc if d != "node_modules" and not d.startswith(".")]
        for ten in tep:
            if not ten.endswith(".lesson.md"):
                continue
            p = pathlib.Path(goc) / ten
            m_id = re.search(r"^id:\s*(\S+)", p.read_text(encoding="utf-8"), re.M)
            if not m_id:
                continue
            raw = theo_id.get(m_id.group(1))
            if raw is None:
                continue
            n_bai += 1
            gom: list[str] = []
            van_ban(json.loads(raw), gom)
            ca = "\n\u0000\n".join(gom)
            for so, title, ma in khoi_nguon(p):
                n_khoi += 1
                if not ma.strip():
                    continue
                if ma not in ca:
                    thieu.append({
                        "tep": str(p.relative_to(GOC)), "dong": so,
                        "title": title or "(không có)",
                        "dau": ma.strip().splitlines()[0][:70],
                    })

    if ns.json:
        print(json.dumps({"so_bai": n_bai, "so_khoi": n_khoi, "thieu": thieu},
                         ensure_ascii=False, indent=2))
        return 1 if thieu else 0

    print(f"Đã đối chiếu {n_khoi} khối mã trong {n_bai} bài.")
    if not thieu:
        print("✅ Không khối mã nào biến mất khi biên dịch.")
        return 0
    print(f"❌ {len(thieu)} khối mã CÓ trong nguồn mà KHÔNG có trong bài:\n")
    for x in thieu:
        print(f"  ✗ {x['tep']}:{x['dong']}  title={x['title']}")
        print(f"     {x['dau']}")
    print("\n  Khối mã người viết đặt ra mà người học không thấy là một câu hỏi")
    print("  thiếu đề. Vá ở trình biên dịch, đừng xoá khối đi cho hết đỏ.")
    return 1


if __name__ == "__main__":
    sys.exit(main())
