#!/usr/bin/env python3
"""Di trú 159 chương sách sang `content/legacy/` ở TIER-C.

MASTERPLAN §0 quyết định 13 chốt phạm vi v1.0: ~180 bài TIER-A cộng **toàn bộ
159 chương ở TIER-C chế độ đọc**. Lý do có tier C là để "không lĩnh vực nào
trống": một người vào tìm hiểu về DDD hay parser combinator phải đọc được cái
gì đó ngay, kể cả khi bài tương tác cho lĩnh vực ấy chưa viết.

Vì sao CHÉP chứ không đọc thẳng từ `fp/`:

1. `fp/*_Books` là READ-ONLY. Nó là nguồn lịch sử, và mọi thứ trong `content/`
   phải sửa được mà không đụng vào nguồn.
2. Chương gốc không có frontmatter, nên không có id ổn định. Tiến độ người học
   lưu theo id — không có id thì không lưu được gì.
3. `fp/` chứa cả `.rig_raw_data` nặng 144 MB và outline không phải chương.
   Trộn chúng vào build là kéo theo thứ không ai đọc.

Chương ở tier C KHÔNG phải `.lesson.md`: chúng không có bước, không chấm, không
cấp mastery ở mức bài. Đặt tên `.chapter.md` để trình biên dịch bài học không
nhặt nhầm.

    python3 tools/di_tru_sach.py            # chép và sinh frontmatter
    python3 tools/di_tru_sach.py --check    # chỉ kiểm, không ghi
"""
from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys

GOC = pathlib.Path(__file__).resolve().parent.parent
NGUON = GOC / "fp"
DICH = GOC / "content" / "legacy"

SACH = {
    "Python_Books": "python",
    "Rust_Books": "rust",
    "TypeScript_Books": "typescript",
}


def doc_tieu_de(noi_dung: str, du_phong: str) -> str:
    """Tiêu đề lấy từ dòng `# …` đầu tiên."""
    for dong in noi_dung.splitlines():
        if dong.startswith("# "):
            return dong[2:].strip()
    return du_phong


def doc_tom_tat(noi_dung: str) -> str:
    """Câu đầu của khối `> **Bạn sẽ học được**`, nếu có.

    Nhiều chương mở bằng một khối trích dẫn liệt kê thứ sẽ học. Lấy gạch đầu
    dòng đầu tiên làm tóm tắt: nó luôn là ý chính, và viết tay lại 159 lần thì
    vừa tốn vừa dễ lệch khỏi nội dung thật.
    """
    m = re.search(r"^>\s*-\s*(.+)$", noi_dung, re.M)
    if m:
        return re.sub(r"[*`]", "", m.group(1)).strip()[:160]
    return ""


def doc_phut(noi_dung: str) -> int | None:
    m = re.search(r"[Tt]hời gian đọc\**:?\s*~?\s*(\d+)", noi_dung)
    return int(m.group(1)) if m else None


def slug(s: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", s.lower()).strip("-")


def cac_chuong() -> list[tuple[str, pathlib.Path]]:
    ra: list[tuple[str, pathlib.Path]] = []
    for thu_muc, lang in SACH.items():
        goc = NGUON / thu_muc
        if not goc.is_dir():
            continue
        for t in sorted(goc.rglob("chapter_*.md")):
            # `_raw_data` là bản sao thô để tra cứu, không phải chương sách.
            if any(p.endswith("_raw_data") for p in t.relative_to(goc).parts):
                continue
            ra.append((lang, t))
    return ra


def main() -> int:
    ap = argparse.ArgumentParser(description="Di trú chương sách sang content/legacy")
    ap.add_argument("--check", action="store_true", help="chỉ kiểm, không ghi gì")
    ns = ap.parse_args()

    chuong = cac_chuong()
    if not chuong:
        print(f"Không tìm thấy chương nào trong {NGUON}", file=sys.stderr)
        return 1

    muc_luc = []
    da_ghi = 0
    trung: dict[str, str] = {}

    for lang, t in chuong:
        goc_sach = NGUON / next(k for k, v in SACH.items() if v == lang)
        rel = t.relative_to(goc_sach)
        phan = rel.parts[0] if len(rel.parts) > 1 else "khac"
        ten = t.stem

        noi_dung = t.read_text(encoding="utf-8")
        tieu_de = doc_tieu_de(noi_dung, ten.replace("_", " "))
        ma = f"legacy.{lang}.{slug(phan)}.{slug(ten)}"

        if ma in trung:
            print(f"❌ id trùng: {ma}\n   {trung[ma]}\n   {t}", file=sys.stderr)
            return 1
        trung[ma] = str(t)

        muc = {
            "id": ma,
            "lang": lang,
            "part": slug(phan),
            "chapter": slug(ten),
            "title": tieu_de,
            "summary": doc_tom_tat(noi_dung),
            "tier": "C",
            "sourcePath": str(t.relative_to(GOC)),
            "lines": len(noi_dung.splitlines()),
        }
        p = doc_phut(noi_dung)
        if p:
            muc["estimatedMinutes"] = p
        muc_luc.append(muc)

        if ns.check:
            continue

        dich = DICH / lang / slug(phan) / f"{slug(ten)}.chapter.md"
        dich.parent.mkdir(parents=True, exist_ok=True)
        fm = ["---"]
        for k, v in muc.items():
            fm.append(f"{k}: {json.dumps(v, ensure_ascii=False)}")
        fm.append("---\n")
        dich.write_text("\n".join(fm) + noi_dung, encoding="utf-8")
        da_ghi += 1

    if not ns.check:
        (DICH / "index.json").write_text(
            json.dumps({"tier": "C", "chapters": muc_luc}, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )

    theo_lang: dict[str, int] = {}
    for m in muc_luc:
        theo_lang[m["lang"]] = theo_lang.get(m["lang"], 0) + 1
    tong_dong = sum(m["lines"] for m in muc_luc)

    print(f"{len(muc_luc)} chương · {tong_dong:,} dòng")
    for k, v in sorted(theo_lang.items()):
        print(f"   {k:12s} {v:3d}")
    thieu = [m["id"] for m in muc_luc if not m["summary"]]
    if thieu:
        print(f"\n⚠ {len(thieu)} chương không có khối \"Bạn sẽ học được\" để lấy tóm tắt:")
        for m in thieu[:8]:
            print(f"   {m}")
    if ns.check:
        print("\n(--check: không ghi gì)")
    else:
        print(f"\n✅ đã ghi {da_ghi} chương → {DICH.relative_to(GOC)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
