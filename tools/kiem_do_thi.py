#!/usr/bin/env python3
"""Kiểm đồ thị tiền đề: mọi `requires` phải có một bài ĐỨNG TRƯỚC `teaches`.

Trường `teaches` và `requires` trong frontmatter không phải trang trí. Theo
MASTERPLAN §5, Director dùng chúng để lọc thuật ngữ chưa dạy khi Byte nói, và
để quyết định bài nào đã mở khoá. Một `requires` không nguồn nào cấp sẽ tạo
một nút treo vĩnh viễn: nếu đồ thị dùng để mở khoá thì bài đó không bao giờ
mở được; nếu dùng để lọc thuật ngữ thì Director gác nhầm.

Trình biên dịch không kiểm được chuyện này — nó chỉ nhìn một file mỗi lần, mà
đây là tính chất của cả tập.

    python3 tools/kiem_do_thi.py
    python3 tools/kiem_do_thi.py --json
"""
from __future__ import annotations

import argparse
import collections
import json
import pathlib
import sys

GOC = pathlib.Path(__file__).resolve().parent.parent
DICH = GOC / "dist" / "content"
THU_TU = GOC / "content" / "curriculum" / "thu-tu.yaml"


def doc_thu_tu() -> list[tuple[str, str]]:
    """[(realm, track)] theo đúng thứ tự học.

    Đọc bằng tay thay vì bằng PyYAML: file này cố tình giữ hình dạng đơn giản,
    và tools/ không nên kéo thêm phụ thuộc chỉ để đọc hai tầng danh sách.
    """
    ra: list[tuple[str, str]] = []
    realm = None
    trong_track = False
    for dong in THU_TU.read_text(encoding="utf-8").splitlines():
        s = dong.strip()
        if s.startswith("#") or not s:
            continue
        if s.startswith("- id:"):
            realm = s.split(":", 1)[1].strip()
            trong_track = False
        elif s == "track:":
            trong_track = True
        elif trong_track and s.startswith("- ") and realm:
            ra.append((realm, s[2:].strip()))
    return ra


def main() -> int:
    ap = argparse.ArgumentParser(description="Kiểm đồ thị tiền đề của học liệu")
    ap.add_argument("--json", action="store_true")
    ns = ap.parse_args()

    if not DICH.is_dir():
        print(f"chưa có {DICH} — chạy `content-compiler build` trước", file=sys.stderr)
        return 2

    thu_tu = doc_thu_tu()
    hang = {(r, t): i for i, (r, t) in enumerate(thu_tu)}

    bai = []
    la = []
    for f in sorted(DICH.glob("*.json")):
        if f.name == "index.json":
            continue
        d = json.loads(f.read_text(encoding="utf-8"))
        khoa = (d["track"], d["module"])
        if khoa not in hang:
            la.append((d["id"], khoa))
            continue
        bai.append((hang[khoa], d["order"], d))
    bai.sort(key=lambda x: (x[0], x[1]))

    day: dict[str, tuple[int, str]] = {}
    can: dict[str, list[tuple[int, str]]] = collections.defaultdict(list)
    for i, (_, _, d) in enumerate(bai):
        for s in d.get("teaches") or []:
            day.setdefault(s, (i, d["id"]))
        for s in d.get("requires") or []:
            can[s].append((i, d["id"]))

    treo = sorted(s for s in can if s not in day)
    som = [
        (s, r_id, r_i, day[s][1], day[s][0])
        for s, rs in can.items()
        if s in day
        for (r_i, r_id) in rs
        if r_i < day[s][0]
    ]
    som.sort(key=lambda x: x[2])

    if ns.json:
        print(json.dumps(
            {"so_bai": len(bai), "treo": treo,
             "dung_som": [{"skill": s, "bai": b, "day_o": d} for s, b, _, d, _ in som],
             "module_la": [{"bai": b, "module": list(k)} for b, k in la]},
            ensure_ascii=False, indent=2))
        return 1 if (treo or som or la) else 0

    print(f"Đã kiểm {len(bai)} bài theo thứ tự ở {THU_TU.relative_to(GOC)}.")
    loi = 0

    if la:
        loi += len(la)
        print(f"\n❌ {len(la)} bài nằm ở module KHÔNG có trong bảng thứ tự:")
        for b, k in la:
            print(f"   {b}  ({k[0]}/{k[1]})")
        print("   → thêm module vào thu-tu.yaml, hoặc sửa frontmatter cho khớp.")

    if treo:
        loi += len(treo)
        print(f"\n❌ {len(treo)} skill được YÊU CẦU mà không bài nào DẠY:")
        for s in treo:
            print(f"   {s}")
            for _, b in can[s][:3]:
                print(f"      cần ở {b}")
        print("   → hoặc thêm `teaches` vào bài dạy nó, hoặc bỏ khỏi `requires`.")

    if som:
        loi += len(som)
        print(f"\n❌ {len(som)} chỗ yêu cầu một skill chỉ được dạy VỀ SAU:")
        for s, b, _, d, _ in som[:20]:
            print(f"   {s}\n      cần ở  {b}\n      dạy ở  {d}")
        if len(som) > 20:
            print(f"   … và {len(som) - 20} chỗ nữa")
        print("   → hoặc đổi thứ tự bài, hoặc bài trước phải dạy skill ấy.")

    if loi == 0:
        print("✅ Đồ thị tiền đề liền mạch: mọi thứ được yêu cầu đều đã được dạy trước đó.")
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
