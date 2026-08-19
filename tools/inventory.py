#!/usr/bin/env python3
"""Kiểm kê nội dung — nguồn sự thật duy nhất về số liệu giáo trình.

Vì sao cần: ba `book_outline.md` đều từng ghi số liệu sai, và sai theo cách khó
thấy (header nói 68 chương trong khi có 63 file). Con số viết tay sẽ trôi khỏi
thực tế ngay lần bổ sung nội dung tiếp theo. Công cụ này đo lại từ đĩa.

Dùng:
    python3 tools/inventory.py            # in bảng + ghi docs/generated/inventory.md
    python3 tools/inventory.py --check    # thoát 1 nếu outline lệch thực tế (dùng cho CI)
    python3 tools/inventory.py --fix      # ghi lại số liệu đúng vào các outline
"""

from __future__ import annotations

import argparse
import hashlib
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

GOC = Path(__file__).resolve().parent.parent
SACH = {
    "Python_Books": "Python",
    "Rust_Books": "Rust",
    "TypeScript_Books": "TypeScript",
}
BO_QUA = {".git", ".venv", "node_modules", "target", "dist"}


@dataclass
class Tep:
    duong_dan: Path
    dong: int
    code_block: dict[str, int]
    sha256: str
    phan: str
    so_bai_tap: int
    so_details: int
    fence_chua_dong: bool


@dataclass
class Sach:
    ten: str
    chuong: list[Tep] = field(default_factory=list)
    phu_luc: list[Tep] = field(default_factory=list)

    @property
    def tong_dong(self) -> int:
        return sum(t.dong for t in self.chuong + self.phu_luc)

    @property
    def tong_code(self) -> int:
        return sum(sum(t.code_block.values()) for t in self.chuong + self.phu_luc)

    def code_theo_ngon_ngu(self) -> dict[str, int]:
        ra: dict[str, int] = {}
        for t in self.chuong + self.phu_luc:
            for k, v in t.code_block.items():
                ra[k] = ra.get(k, 0) + v
        return ra


def doc_tep(p: Path) -> Tep:
    raw = p.read_bytes()
    t = raw.decode("utf-8", errors="replace")
    # Đếm theo TRẠNG THÁI mở/đóng, không đếm theo có-tên-hay-không: block không
    # gắn tên ngôn ngữ vẫn là block, và đếm nhầm khiến số liệu lệch âm thầm.
    fences: dict[str, int] = {}
    dang_mo = False
    # Fence có thể đứng ngay sau dấu đầu mục: "2. ```sql" hay "- ```rust".
    # Bỏ qua trường hợp này sẽ đếm lệch và — tệ hơn — báo "block chưa đóng" oan.
    re_fence = re.compile(r"^\s*(?:[-*+]|\d+[.)])?\s*```(.*)$")
    for dong in t.splitlines():
        m = re_fence.match(dong)
        if not m:
            continue
        if dang_mo:
            dang_mo = False
            continue
        dang_mo = True
        duoi = m.group(1).strip()
        lang = duoi.split()[0].lower() if duoi else "(không ghi)"
        fences[lang] = fences.get(lang, 0) + 1
    return Tep(
        fence_chua_dong=dang_mo,
        duong_dan=p.relative_to(GOC),
        dong=t.count("\n") + 1,
        code_block=fences,
        sha256=hashlib.sha256(raw).hexdigest()[:12],
        phan=p.parent.name,
        so_bai_tap=len(re.findall(r"\*\*Bài \d+", t)),
        so_details=len(re.findall(r"<details>", t)),
    )


def quet() -> dict[str, Sach]:
    ra: dict[str, Sach] = {}
    for thu_muc, ten in SACH.items():
        s = Sach(ten=ten)
        goc = GOC / "fp" / thu_muc
        for p in sorted(goc.rglob("*.md")):
            if any(x in BO_QUA or x.startswith(".") for x in p.relative_to(goc).parts[:-1]):
                continue
            if p.name.startswith("chapter_"):
                s.chuong.append(doc_tep(p))
            elif p.name.startswith("appendix_"):
                s.phu_luc.append(doc_tep(p))
        ra[thu_muc] = s
    return ra


def in_bang(sach: dict[str, Sach]) -> None:
    print(f"{'Sách':<14}{'Chương':>8}{'Phụ lục':>9}{'Dòng':>10}{'Code block':>12}")
    print("─" * 53)
    tc = tp = td = tb = 0
    for s in sach.values():
        print(
            f"{s.ten:<14}{len(s.chuong):>8}{len(s.phu_luc):>9}"
            f"{s.tong_dong:>10,}{s.tong_code:>12}".replace(",", ".")
        )
        tc += len(s.chuong)
        tp += len(s.phu_luc)
        td += s.tong_dong
        tb += s.tong_code
    print("─" * 53)
    print(f"{'TỔNG':<14}{tc:>8}{tp:>9}{td:>10,}{tb:>12}".replace(",", "."))


def ghi_bao_cao(sach: dict[str, Sach]) -> Path:
    ra = ["# Kiểm kê nội dung", "", "> Sinh tự động bởi `tools/inventory.py`. Đừng sửa tay.", ""]
    ra += ["| Sách | Chương | Phụ lục | Dòng | Code block |", "|---|---|---|---|---|"]
    for s in sach.values():
        ra.append(
            f"| {s.ten} | {len(s.chuong)} | {len(s.phu_luc)} | "
            f"{s.tong_dong:,} | {s.tong_code} |".replace(",", ".")
        )
    ra.append("")
    for s in sach.values():
        ra += [f"## {s.ten}", "", "Code block theo ngôn ngữ:", ""]
        for lang, n in sorted(s.code_theo_ngon_ngu().items(), key=lambda x: -x[1]):
            ra.append(f"- `{lang}`: {n}")
        ra += ["", "| Tệp | Phần | Dòng | Code | Bài tập | details | sha256 |", "|---|---|---|---|---|---|---|"]
        for t in s.chuong + s.phu_luc:
            ra.append(
                f"| `{t.duong_dan.name}` | {t.phan} | {t.dong} | "
                f"{sum(t.code_block.values())} | {t.so_bai_tap} | {t.so_details} | `{t.sha256}` |"
            )
        ra.append("")
    dich = GOC / "docs" / "generated" / "inventory.md"
    dich.parent.mkdir(parents=True, exist_ok=True)
    dich.write_text("\n".join(ra), encoding="utf-8")
    return dich


def kiem_fence(sach: dict[str, Sach]) -> list[str]:
    """Tìm code block mở mà không đóng.

    Lỗi này im lặng: markdown vẫn render, chỉ là phần còn lại của chương bị nuốt
    vào trong khối code. Người đọc thấy nguyên nửa chương biến thành mã nguồn.
    """
    return [
        f"{t.duong_dan}: có code block mở mà không đóng"
        for s in sach.values()
        for t in s.chuong + s.phu_luc
        if t.fence_chua_dong
    ]


def kiem_outline(sach: dict[str, Sach]) -> list[str]:
    """Đối chiếu con số ghi trong outline với thực tế."""
    loi = []
    for thu_muc, s in sach.items():
        p = GOC / "fp" / thu_muc / "book_outline.md"
        if not p.exists():
            loi.append(f"{thu_muc}: thiếu book_outline.md")
            continue
        t = p.read_text(encoding="utf-8")
        for m in re.finditer(r"\*\*(\d+) chương", t):
            n = int(m.group(1))
            if n != len(s.chuong):
                loi.append(
                    f"{thu_muc}: outline ghi {n} chương, thực tế {len(s.chuong)}"
                )
        for m in re.finditer(r"\*\*(\d+) phụ lục", t):
            n = int(m.group(1))
            if n != len(s.phu_luc):
                loi.append(
                    f"{thu_muc}: outline ghi {n} phụ lục, thực tế {len(s.phu_luc)}"
                )
    return loi


def sua_outline(sach: dict[str, Sach]) -> list[str]:
    da_sua = []
    for thu_muc, s in sach.items():
        p = GOC / "fp" / thu_muc / "book_outline.md"
        if not p.exists():
            continue
        t = goc_t = p.read_text(encoding="utf-8")
        t = re.sub(r"\*\*\d+ chương", f"**{len(s.chuong)} chương", t)
        t = re.sub(r"\*\*\d+ phụ lục", f"**{len(s.phu_luc)} phụ lục", t)
        # Dòng TỔNG trong bảng thống kê
        t = re.sub(
            r"\| \*\*TỔNG\*\* \| \*\*[^|]*\*\* \| \*\*[\d.,]+\*\* \| \*\*[\d.,+]+\*\* \|",
            f"| **TỔNG** | **{len(s.chuong)} chương + {len(s.phu_luc)} phụ lục** | "
            f"**{s.tong_dong:,}**".replace(",", ".") + f" | **{s.tong_code}** |",
            t,
        )
        if t != goc_t:
            p.write_text(t, encoding="utf-8")
            da_sua.append(thu_muc)
    return da_sua


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--check", action="store_true", help="thoát 1 nếu outline lệch thực tế")
    ap.add_argument("--fix", action="store_true", help="ghi lại số liệu đúng vào outline")
    a = ap.parse_args()

    sach = quet()
    in_bang(sach)

    if a.fix:
        da = sua_outline(sach)
        print(f"\nĐã sửa outline: {', '.join(da) if da else '(không có gì lệch)'}")

    loi = kiem_outline(sach) + kiem_fence(sach)
    dich = ghi_bao_cao(sach)
    print(f"\nBáo cáo: {dich.relative_to(GOC)}")

    if loi:
        print("\n⚠️  Vấn đề:")
        for x in loi:
            print(f"   - {x}")
        if a.check:
            return 1
    else:
        print("✅ Outline khớp thực tế, không có code block nào chưa đóng.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
