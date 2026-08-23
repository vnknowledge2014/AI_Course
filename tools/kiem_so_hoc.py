#!/usr/bin/env python3
"""Kiểm mọi khẳng định số học viết trong văn xuôi bài học.

Vòng phản biện T2.1 tìm ra bảy lỗi TOÁN mà cả bốn cổng máy đều cho qua. Vài
ca trong số đó là số học thuần và máy kiểm được: "bẻ làm 4 phần cần 4 nhát"
(sai, 3), "0,25 không nhỏ hơn 0,251" (sai), "999 và 1002 liền nhau" (cách
nhau 3).

Bài Toán viết đầy câu dạng `3 × 4 = 12`, `100000 // 3 = 33333`, `0,1 + 0,2`.
Mỗi câu ấy là một lời hứa với người học, và một lời hứa sai trong bài toán
còn tệ hơn không có bài — người ta tin nó rồi mang đi dùng.

Cổng này KHÔNG hiểu toán. Nó chỉ làm đúng một việc: gặp một đẳng thức viết
bằng số, tính lại bằng Python, và so. Chuyện "3 × 4 nghĩa là lô cỡ 3 hay lô
cỡ 4" nằm ngoài tầm nó — đó vẫn là việc của người đọc.

    python3 tools/kiem_so_hoc.py
    python3 tools/kiem_so_hoc.py --json
"""
from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys
from fractions import Fraction

GOC = pathlib.Path(__file__).resolve().parent.parent
NOI_DUNG = GOC / "content"

# Dấu toán học trong văn xuôi tiếng Việt khác dấu trong mã.
DAU = {"×": "*", "·": "*", "−": "-", "–": "-", "÷": "/", ":": "/"}

# `12 × 3 = 36`, `100 // 3 = 33`, `7 % 2 = 1`. Chỉ bắt biểu thức TOÀN SỐ —
# có chữ cái là biểu thức đại số, và giá trị của nó phụ thuộc chỗ trống.
#
# Ba mảnh chặn ở hai đầu, mỗi mảnh chặn một dương-giả có thật đã bắt được:
#   - đầu trái `(?<![\w.,])` cộng `(?<![-+×·−–*/÷:] )`: `n − 5/3 = 13` là câu
#     ĐÚNG, chỉ sai khi đọc mỗi mẩu `5/3 = 13` sau khi vứt `n −` đi.
#   - đuôi `(?!\s*[…]|\.\.\.)`: `1/3 = 0,333…` cũng ĐÚNG — dấu ba chấm nói
#     rằng vế phải là xấp xỉ, và bài này DẠY đúng chuyện đó.
SO = r"\d[\d.,]*(?:\u00a0\d{3}| \d{3})*"
BIEU_THUC = re.compile(
    r"(?<![\w.,])"
    r"(?<![-+×·−–*/÷:] )"
    rf"({SO}(?:\s*(?:[×·+\-−–*/÷:]|//|%)\s*{SO})+)"
    r"\s*=\s*"
    # Vế phải cho phép là một phân số: bài phân số viết kết quả bằng phân số
    # nhiều hơn bằng số lẻ, và bản đầu của cổng này bỏ qua sạch chúng — nó
    # thấy dấu `/` sau vế phải rồi kết luận "biểu thức chưa hết".
    rf"({SO}(?:\s*/\s*{SO})?)"
    # `/` nằm trong lớp cấm chứ không chỉ `[\d.,]`: `7/4 = 4/4 + 3/4` là câu
    # ĐÚNG, nhưng vế phải `4/4` bị chặn bởi dấu `+` phía sau, nên regex lùi
    # xuống `4` và báo `7/4 = 4`. Cấm lùi thì nó phải bỏ cả câu — đúng ý.
    # `(?![\d.,/])` phải đứng TRƯỚC: thiếu nó thì `0,333…` không bị loại, nó
    # chỉ lùi một chữ số thành `0,333` → `0,33` cho tới khi dấu … khuất mắt.
    # Một mẫu regex né được chính cái chặn của mình còn tệ hơn không chặn.
    r"(?![\d.,/])"
    r"(?!\s*(?:…|\.\.\.))"
    r"(?![\d.,]*\s*(?:[×·+\-−–*÷:%]|//))"
)



def so(s: str) -> Fraction | None:
    """Đọc số kiểu Việt (`45.000` hay `0,5`) thành Fraction.

    Dùng Fraction chứ không float: bài Toán đầy `0,1 + 0,2`, và kiểm một bài
    DẠY rằng số thực là xấp xỉ bằng chính số thực xấp xỉ thì vô nghĩa.
    """
    s = s.strip()
    # `190 000` — tiếng Việt phân nhóm nghìn bằng dấu cách cũng nhiều như bằng
    # dấu chấm, và nội dung dùng cả hai.
    s = re.sub(r"[ \u00a0](?=\d{3}\b)", "", s)
    # `45.000` = bốn mươi lăm nghìn (dấu chấm phân nhóm nghìn, ba chữ số).
    if re.fullmatch(r"\d{1,3}(\.\d{3})+", s):
        s = s.replace(".", "")
    # `0,5` = năm phần mười.
    s = s.replace(",", ".")
    try:
        return Fraction(s)
    except (ValueError, ZeroDivisionError):
        return None


def tinh(bt: str) -> Fraction | None:
    """Tính một biểu thức toàn số. `None` khi không tính được."""
    ma = bt
    for k, v in DAU.items():
        ma = ma.replace(k, v)
    # Tách số ra để chuẩn hoá dấu phân cách trước khi đưa vào eval.
    def thay(m: re.Match[str]) -> str:
        v = so(m.group(0))
        return f"Fraction({v.numerator},{v.denominator})" if v is not None else m.group(0)

    ma = re.sub(SO, thay, ma)
    try:
        kq = eval(ma, {"__builtins__": {}, "Fraction": Fraction})  # noqa: S307
    except Exception:
        return None
    return Fraction(kq) if isinstance(kq, (int, Fraction)) else None


MIEN_TRU = GOC / "content" / "curriculum" / "so-hoc-sai-co-y.yaml"


def doc_mien_tru() -> list[dict[str, str]]:
    """Đọc danh sách khẳng định sai CỐ Ý.

    Bài dạy bằng cách bác bỏ ngộ nhận buộc phải viết ngộ nhận ra: bài 35 viết
    `1/2 + 1/3 = 2/5` rồi dành cả bài chứng minh nó sai. Cổng không phân biệt
    được, nên chỗ ấy phải khai báo.

    Khoá là (tên file, biểu thức) chứ KHÔNG phải số dòng. Số dòng trôi mỗi lần
    sửa một chữ ở trên, và một miễn trừ trôi khỏi chỗ nó canh sẽ lặng lẽ tha
    cho một lỗi thật ở dòng mới — đúng cái chế độ hỏng cổng này sinh ra để
    chống. Biểu thức biến mất thì miễn trừ thành THỪA, và thừa là lỗi.
    """
    if not MIEN_TRU.exists():
        return []
    ra: list[dict[str, str]] = []
    hien: dict[str, str] = {}
    for dong in MIEN_TRU.read_text(encoding="utf-8").splitlines():
        d = dong.strip()
        if not d or d.startswith("#"):
            continue
        if d.startswith("- "):
            hien = {}
            ra.append(hien)
            d = d[2:].strip()
        if ":" in d:
            k, _, v = d.partition(":")
            hien[k.strip()] = v.strip().strip('"')
    return [x for x in ra if x.get("tep") and x.get("bieu_thuc")]


def gon(s: str) -> str:
    """Chuẩn hoá biểu thức để so khớp: mọi dấu cách thành một."""
    return " ".join(s.split())


def ten_goi(t: pathlib.Path) -> str:
    """Đường dẫn tương đối khi nằm trong repo, tuyệt đối khi ở ngoài.

    `relative_to` ném lỗi với file ngoài repo, và bộ tự-kiểm của chính cổng
    này chạy trên file ngoài repo.
    """
    try:
        return str(t.resolve().relative_to(GOC))
    except ValueError:
        return str(t)


def kiem(t: pathlib.Path) -> list[tuple[int, str, str, str]]:
    ra: list[tuple[int, str, str, str]] = []
    trong_ma = False
    for i, dong in enumerate(t.read_text(encoding="utf-8").splitlines(), 1):
        if dong.lstrip().startswith("```"):
            trong_ma = not trong_ma
            continue
        # Trong khối mã thì Python là trọng tài, không phải cổng này —
        # `tools/kiem_ma_bai_hoc.mjs` đã chạy chúng thật.
        if trong_ma:
            continue
        for m in BIEU_THUC.finditer(dong):
            trai, phai = tinh(m.group(1)), so(m.group(2))
            if trai is None or phai is None or trai == phai:
                continue
            dep = lambda f: str(f) if f.denominator != 1 else str(f.numerator)
            ra.append((i, m.group(0), dep(trai), dep(phai)))
    return ra


def main() -> int:
    ap = argparse.ArgumentParser(description="Kiểm khẳng định số học trong bài học")
    ap.add_argument("--json", action="store_true")
    ap.add_argument("tep", nargs="*")
    ns = ap.parse_args()

    bai = [pathlib.Path(x) for x in ns.tep] if ns.tep else sorted(NOI_DUNG.rglob("*.lesson.md"))
    mien = doc_mien_tru()
    da_dung: set[int] = set()
    sai = []
    n_bt = 0
    for t in bai:
        for dong, van, dung, viet in kiem(t):
            bo_qua = False
            for i, m in enumerate(mien):
                if t.name == pathlib.Path(m["tep"]).name and gon(m["bieu_thuc"]) == gon(van):
                    da_dung.add(i)
                    bo_qua = True
            if bo_qua:
                continue
            sai.append({
                "tep": ten_goi(t),
                "dong": dong, "van": van, "dung": dung, "viet": viet,
            })
        n_bt += len(BIEU_THUC.findall(t.read_text(encoding="utf-8")))

    # Chỉ soát miễn trừ thừa khi quét TOÀN BỘ: chạy trên một file lẻ thì mọi
    # miễn trừ của file khác đương nhiên chưa dùng tới.
    thua = [] if ns.tep else [m for i, m in enumerate(mien) if i not in da_dung]

    if ns.json:
        print(json.dumps({"so_bai": len(bai), "so_bieu_thuc": n_bt, "sai": sai, "mien_tru_thua": thua},
                         ensure_ascii=False, indent=2))
        return 1 if sai or thua else 0

    print(f"Đã kiểm {n_bt} đẳng thức trong {len(bai)} bài"
          f"{f', bỏ qua {len(da_dung)} chỗ sai cố ý' if da_dung else ''}.")
    if thua:
        print(f"❌ {len(thua)} miễn trừ THỪA — biểu thức không còn trong bài:\n")
        for m in thua:
            print(f"  ✗ {m['tep']}: `{m['bieu_thuc']}`")
        print("\n  Xoá chúng khỏi content/curriculum/so-hoc-sai-co-y.yaml.")
        print("  Miễn trừ còn sống sau khi chỗ nó canh đã đi là một cái bẫy để mở.")
    if not sai:
        if not thua:
            print("✅ Mọi khẳng định số học đều đúng.")
        return 1 if thua else 0
    print(f"❌ {len(sai)} khẳng định SAI:\n")
    for s in sai:
        print(f"  ✗ {s['tep']}:{s['dong']}")
        print(f"     viết: {s['van']}")
        print(f"     đúng: {s['dung']}")
    print("\n  Nếu một chỗ SAI CỐ Ý (bài dạy bằng cách bác bỏ ngộ nhận),")
    print("  khai báo nó ở content/curriculum/so-hoc-sai-co-y.yaml.")
    return 1


if __name__ == "__main__":
    sys.exit(main())
